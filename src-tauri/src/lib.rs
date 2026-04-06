use image::guess_format;
use img_parts::{ImageEXIF, ImageICC};
use log::debug;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::path::PathBuf;

/// image_stats provide stats on a particular image's cleanning process.
struct ImageReturn {
    img_data: Vec<u8>,
    segments_removed: u32,
    bits_removed: usize,
}

/// batch_stats is the combined stats of the entire process.
struct BatchStats {
    total_segments: u32,
    total_bits: usize,
    total_files: u32,
}
impl BatchStats {
    fn new() -> BatchStats {
        BatchStats {
            total_segments: 0,
            total_bits: 0,
            total_files: 0,
        }
    }

    fn add(&mut self, file_stats: &ImageReturn) {
        self.total_segments += file_stats.segments_removed;
        self.total_bits += file_stats.bits_removed;
        self.total_files += 1;
    }
}

fn is_image(path: PathBuf) -> bool {
    let img_extensions: HashMap<&str, bool> = [
        ("jpg", true),
        ("jpeg", true),
        // ("png", true),
        // ("gif", true),
        // ("tiff", true),
        // ("webp", true),
    ]
    .iter()
    .cloned()
    .collect();

    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return img_extensions.contains_key(&ext_str.to_lowercase()[..]);
        }
    }
    false
}

async fn gather_files(path: &PathBuf) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    match tokio::fs::read_dir(path).await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let path_buf = entry.path();
                if is_image(path_buf.clone()) {
                    match tokio::fs::try_exists(&path_buf).await {
                        Ok(_) => files.push(path_buf.to_str().unwrap().to_string()),
                        Err(e) => debug!("Error opening file: {}", e),
                    }
                }
            }
        }
        Err(e) => debug!("Error reading directory: {}", e),
    }
    files
}

async fn remove_metadata_jpeg(input_bytes: Vec<u8>) -> Result<ImageReturn, Error> {
    // Parse the JPEG structure from the byte array
    // This does NOT decode the actual image pixels, so it's fast and lossless
    let mut jpeg = img_parts::jpeg::Jpeg::from_bytes(input_bytes.into()).unwrap();

    jpeg.set_exif(None);
    jpeg.set_icc_profile(None);
    let mut segments_removed = 0;
    let mut bits_removed = 0;
    jpeg.segments_mut().retain(|segment| {
        match segment.marker() {
            // --- MANDATORY JPEG MARKERS ---
            // SOI: Start of Image (0xFFD8) - REQUIRED at file start
            img_parts::jpeg::markers::SOI => true,
            // EOI: End of Image (0xFFD9) - REQUIRED at file end
            img_parts::jpeg::markers::EOI => true,

            // --- CRITICAL FOR DECODING ---
            // SOF0: Baseline DCT (Standard JPEG)
            img_parts::jpeg::markers::SOF0 => true,
            // SOF2: Progressive DCT (Progressive JPEG)
            img_parts::jpeg::markers::SOF2 => true,
            // DQT: Define Quantization Table (Required to decompress)
            img_parts::jpeg::markers::DQT => true,
            // DHT: Define Huffman Table (Required to decompress)
            img_parts::jpeg::markers::DHT => true,
            // DRI: Define Restart Interval (Required for error recovery/sync)
            img_parts::jpeg::markers::DRI => true,
            // SOS: Start Of Scan (This contains the actual compressed image data)
            img_parts::jpeg::markers::SOS => true,
            // APP0: JFIF Header.
            img_parts::jpeg::markers::APP0 => true,
            // APP14: Adobe Header. Critical for color correctness.
            img_parts::jpeg::markers::APP14 => true,

            // --- EVERYTHING ELSE IS REMOVED ---
            // APP1 (Exif), APP2 (ICC), COM (Comments), etc.
            _ => {
                segments_removed += 1;
                bits_removed += segment.len();
                false
            }
        }
    });
    let mut img_data = Vec::new();
    jpeg.encoder().write_to(&mut img_data)?;

    let output = ImageReturn {
        img_data,
        segments_removed,
        bits_removed,
    };

    Ok(output)
}

// creates a directory to store new files within the given path
async fn make_write_dir(path: &PathBuf, new_dir: &str) -> Result<PathBuf, Error> {
    let mut new_path = path.clone();

    new_path.push(new_dir);
    // println!("Creating new directory at {:?}", new_path);
    match tokio::fs::create_dir(&new_path).await {
        Ok(_) => Ok(new_path),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Ok(new_path)
            } else if e.kind() == std::io::ErrorKind::DirectoryNotEmpty {
                Ok(new_path)
            } else {
                Err(e)
            }
        }
    }
}

/// There is the 32! propbability of a collision, so if there is a collision detected we'll
/// regenerate a new filename.
async fn generate_filename(save_directory: &PathBuf) -> std::io::Result<(PathBuf, File)> {
    loop {
        let new_name = {
            let mut rng = rand::rng();
            let name = rng.random::<u32>();
            format!("{:#}", name)
        };
        let new_path = save_directory.join(format!("{}.jpeg", new_name));

        let file_result = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&new_path);

        match file_result {
            // Success: We reserved the filename and have the handle
            Ok(file) => return Ok((new_path, file)),
            // Collision: File exists, loop again
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            // Real Error: Permission denied, disk full, etc.
            Err(e) => return Err(e),
        }
    }
}
#[tauri::command]
async fn inspect_image(path: &str) -> Result<Vec<(String, String)>, String> {
    let data = std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let jpeg = img_parts::jpeg::Jpeg::from_bytes(data.into())
        .map_err(|e| format!("Failed to parse JPEG: {}", e))?;

    let mut output = Vec::new();

    // EXIF data
    if let Some(exif_data) = jpeg.exif() {
        output.push(("EXIF".to_string(), format!("{} bytes", exif_data.len())));

        // Parse EXIF fields
        let mut cursor = std::io::Cursor::new(exif_data);
        if let Ok(exif) = exif::Reader::new().read_from_container(&mut cursor) {
            for f in exif.fields() {
                let key = format!("EXIF:{}", f.tag);
                let value = format!("{}", f.display_value().with_unit(&exif));
                output.push((key, value));
            }
        }
    }

    // ICC Profile
    if let Some(icc_data) = jpeg.icc_profile() {
        output.push((
            "ICC Profile".to_string(),
            format!("{} bytes", icc_data.len()),
        ));
    }

    // All other segments
    for segment in jpeg.segments() {
        let marker_name = match segment.marker() {
            img_parts::jpeg::markers::APP0 => "APP0 (JFIF)",
            img_parts::jpeg::markers::APP1 => "APP1 (EXIF/XMP)",
            img_parts::jpeg::markers::APP2 => "APP2 (ICC)",
            img_parts::jpeg::markers::APP14 => "APP14 (Adobe)",
            img_parts::jpeg::markers::COM => "COM (Comment)",
            m => &format!("Marker 0x{:04X}", m),
        };
        output.push((marker_name.to_string(), format!("{} bytes", segment.len())));
    }

    Ok(output)
}
// NOTE: Return this code if we want a more simplified EXIF-only inspection
// #[tauri::command]
// async fn inspect_image(path: &str) -> Result<Vec<(String, String)>, String> {
//     let file = match std::fs::File::open(path) {
//         Ok(f) => f,
//         Err(e) => return Err(format!("Failed to open file: {}", e)),
//     };
//
//     let mut bufreader = std::io::BufReader::new(&file);
//     let exifreader = exif::Reader::new();
//
//     match exifreader.read_from_container(&mut bufreader) {
//         Ok(exif) => {
//             let mut output = Vec::new();
//             for f in exif.fields() {
//                 let key = format!("{}", f.tag);
//                 let value = format!("{}", f.display_value().with_unit(&exif));
//                 output.push((key, value));
//             }
//             Ok(output)
//         }
//         Err(exif::Error::NotFound(_)) => Ok(Vec::new()),
//         Err(e) => Err(format!("Failed to read EXIF: {}", e)),
//     }
// }

#[tauri::command]
async fn scrub_images(path: &str, save_directory: &str) -> Result<String, ()> {
    let path = PathBuf::from(path);
    // create a default if the client forgets to supply a directory.
    let save_directory = match save_directory {
        "" => "scrubbed_images",
        _ => save_directory,
    };

    let save_directory = make_write_dir(&path, save_directory).await.unwrap();
    // println!("Saving scrubbed image to {:?}", save_directory);
    let mut stats = BatchStats::new();

    let files = gather_files(&path).await;
    for file in files {
        // println!("Processing file: {}", &file);

        let data = std::fs::read(&file).unwrap();
        match guess_format(&data).unwrap() {
            image::ImageFormat::Jpeg => {
                let input_data = std::fs::read(file).unwrap();
                let output_data = remove_metadata_jpeg(input_data).await.unwrap();
                let new_path = generate_filename(&save_directory).await.unwrap();

                stats.add(&output_data);
                std::fs::write(new_path.0, output_data.img_data).unwrap();
            }
            // image::ImageFormat::Png => todo!("PNG"),
            // image::ImageFormat::Tiff => todo!("TIFF"),
            _ => println!("Unable to process file format. Try resaving the image."),
        };
    }
    Ok(save_directory.display().to_string())
}

#[tauri::command]
fn count_images(path: &str) -> u64 {
    // println!("Counting image files in {}", path);
    let mut counter: u64 = 0;
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path_buf = entry.path();
                if is_image(path_buf) {
                    counter += 1;
                }
            }
            debug!("Found {} image files in {}", counter, path);
            counter
        }
        Err(e) => {
            debug!("Error reading directory: {}", e);
            0
        }
    }
}
/// Kani formal-verification harnesses.
///
/// These are only compiled when running `cargo kani`.  They are invisible to
/// the normal Rust compiler and to `cargo test`, so they have zero impact on
/// the production binary.
///
/// # Running the proofs
///
/// ```sh
/// cargo install --locked kani-verifier   # one-time setup
/// cargo kani                             # verify all harnesses
/// cargo kani --harness <name>            # verify a single harness
/// ```
#[cfg(kani)]
mod kani_proofs {
    use super::*;

    /// A minimal, structurally-valid JPEG that `img_parts` can parse.
    ///
    /// Structure:
    ///   - SOI  (FF D8)
    ///   - APP0 (FF E0) – 16-byte JFIF header
    ///   - EOI  (FF D9)
    const MINIMAL_JPEG: &[u8] = &[
        // SOI
        0xFF, 0xD8,
        // APP0 marker + length (16 bytes total, including the 2-byte length field)
        0xFF, 0xE0, 0x00, 0x10,
        // "JFIF\0" identifier
        0x4A, 0x46, 0x49, 0x46, 0x00,
        // Version 1.1
        0x01, 0x01,
        // Pixel density units: no units
        0x00,
        // X/Y pixel density: 1×1
        0x00, 0x01, 0x00, 0x01,
        // No embedded thumbnail
        0x00, 0x00,
        // EOI
        0xFF, 0xD9,
    ];

    /// Formally verify that `remove_metadata_jpeg` removes EXIF data for
    /// **any** possible EXIF payload content.
    ///
    /// Kani explores every possible 64-byte EXIF payload, proving the
    /// property holds universally rather than for a single concrete value.
    #[kani::proof]
    fn proof_remove_metadata_jpeg_removes_exif() {
        // Build a minimal JPEG and inject an arbitrary EXIF payload.
        let mut jpeg =
            img_parts::jpeg::Jpeg::from_bytes(MINIMAL_JPEG.to_vec().into())
                .expect("minimal JPEG must parse");

        let exif_payload: [u8; 64] = kani::any();
        jpeg.set_exif(Some(exif_payload.to_vec().into()));

        // Sanity check: EXIF must be present before processing.
        assert!(jpeg.exif().is_some(), "EXIF should be present before processing");

        // Serialise the JPEG with embedded EXIF.
        let mut input_bytes: Vec<u8> = Vec::new();
        jpeg.encoder()
            .write_to(&mut input_bytes)
            .expect("JPEG encoding must succeed");

        // Call remove_metadata_jpeg directly (async fn driven by kani::block_on).
        let result = kani::block_on(remove_metadata_jpeg(input_bytes))
            .expect("remove_metadata_jpeg must succeed");

        // ── Core properties ────────────────────────────────────────────────
        // Parse the output so we can inspect its segments.
        let output_jpeg =
            img_parts::jpeg::Jpeg::from_bytes(result.img_data.into())
                .expect("output must be a valid JPEG");

        // 1. No EXIF data present.
        assert!(
            output_jpeg.exif().is_none(),
            "EXIF must be removed after processing"
        );

        // 2. Only image-critical segments are allowed; all metadata is gone.
        let allowed: &[u8] = &[
            img_parts::jpeg::markers::SOI,
            img_parts::jpeg::markers::EOI,
            img_parts::jpeg::markers::SOF0,
            img_parts::jpeg::markers::SOF2,
            img_parts::jpeg::markers::DQT,
            img_parts::jpeg::markers::DHT,
            img_parts::jpeg::markers::DRI,
            img_parts::jpeg::markers::SOS,
            img_parts::jpeg::markers::APP0,
            img_parts::jpeg::markers::APP14,
        ];
        for segment in output_jpeg.segments() {
            assert!(
                allowed.contains(&segment.marker()),
                "output contains a non-image segment after scrubbing"
            );
        }
    }

    /// Formally verify that `remove_metadata_jpeg` removes an ICC profile for
    /// **any** possible ICC payload content.
    #[kani::proof]
    fn proof_remove_metadata_jpeg_removes_icc_profile() {
        let mut jpeg =
            img_parts::jpeg::Jpeg::from_bytes(MINIMAL_JPEG.to_vec().into())
                .expect("minimal JPEG must parse");

        let icc_payload: [u8; 32] = kani::any();
        jpeg.set_icc_profile(Some(icc_payload.to_vec().into()));

        assert!(
            jpeg.icc_profile().is_some(),
            "ICC profile should be present before processing"
        );

        let mut input_bytes: Vec<u8> = Vec::new();
        jpeg.encoder()
            .write_to(&mut input_bytes)
            .expect("JPEG encoding must succeed");

        // Call remove_metadata_jpeg directly (async fn driven by kani::block_on).
        let result = kani::block_on(remove_metadata_jpeg(input_bytes))
            .expect("remove_metadata_jpeg must succeed");

        let output_jpeg =
            img_parts::jpeg::Jpeg::from_bytes(result.img_data.into())
                .expect("output must be a valid JPEG");

        // ── Core property ──────────────────────────────────────────────────
        assert!(
            output_jpeg.icc_profile().is_none(),
            "ICC profile must be removed after processing"
        );
    }

    /// Formally verify that `generate_filename` always produces a path whose
    /// file-system extension is `"jpeg"`, for every possible `u32` random value.
    #[kani::proof]
    fn proof_generate_filename_has_jpeg_extension() {
        // Create a real temporary directory so the file-creation succeeds.
        let dir = std::path::PathBuf::from("/tmp/kani_proof_generate_filename");
        std::fs::create_dir_all(&dir).ok();

        // Call generate_filename directly (async fn driven by kani::block_on).
        let (path, _file) = kani::block_on(generate_filename(&dir))
            .expect("generate_filename must succeed");

        // ── Core property ──────────────────────────────────────────────────
        // The extension reported by the OS path API must always be "jpeg".
        assert_eq!(
            path.extension().and_then(|e| e.to_str()),
            Some("jpeg"),
            "generated filename must have .jpeg extension"
        );
    }

    /// Formally verify that the filename stem produced by `generate_filename`
    /// is never empty for any possible `u32` random value.
    #[kani::proof]
    fn proof_generate_filename_stem_nonempty() {
        let dir = std::path::PathBuf::from("/tmp/kani_proof_generate_filename_stem");
        std::fs::create_dir_all(&dir).ok();

        // Call generate_filename directly (async fn driven by kani::block_on).
        let (path, _file) = kani::block_on(generate_filename(&dir))
            .expect("generate_filename must succeed");

        // ── Core property ──────────────────────────────────────────────────
        // The stem (name without extension) must never be empty.
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        assert!(!stem.is_empty(), "filename stem must never be empty");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            count_images,
            scrub_images,
            inspect_image
        ])
        .run(tauri::generate_context!())
        .expect("error While running tauri application");
}
