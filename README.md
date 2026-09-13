# OwO Scrubby Buddy

ooohhh! hawwwugh wooooahhh, pwincess!!! >.<

I made this to make you feel more comfortable posting pictures.
When you post online, most of the time you want everything to stay in the virtual world. Nobody wants the interwebs leaking.
There are a lot of bad people in the world, and they can use the information hidden in your photos to help find you IRL.

This is an easy to use tool to help remove all the hiden meta data from your photos before you post it.
You can also inspect photos before and after i do the srubbby dubbies so you have confidence in them.

# How to scrub

1. Select a folder of photos.
2. Press "clean"
3. I'll make u a brand new directory with da photos that are allll nice and clean fur u

# how to inspect

1. click the inspect tab button
2. select a photo
3. I'll show you allll the metadata that I can find in your phoootooo oooooweeeeoooooo

# Development & Quality Checks

Run the following commands locally to validate changes before pushing:

- **Frontend formatting check**: `npm run lint`
- **Frontend auto-format**: `npm run format`
- **TypeScript & Svelte type check**: `npm run check`
- **Frontend unit tests**: `npm test`
- **Rust backend formatting**: `cargo fmt --check` (in `src-tauri`)
- **Rust backend linter**: `cargo clippy -- -D warnings` (in `src-tauri`)
- **Frontend production build**: `npm run build`
- **Tauri desktop build**: `npx tauri build`

# Release

This project uses an automated multi-platform CI/CD pipeline to package installers and publish GitHub Releases.

## Release Tagging Convention

Releases follow the convention `r<number>.<number>.<number>` (for example, `r1.0.0`, `r0.1.0`).

## How to Create a Release

You can trigger a release using either a Git tag or a release branch:

### Option A: Using a Release Tag (Recommended)

1. Ensure your local `main` branch is up to date and all changes are committed:
   ```bash
   git checkout main
   git pull origin main
   ```
2. Create a release tag matching the convention:
   ```bash
   git tag r1.0.0
   ```
3. Push the release tag to GitHub:
   ```bash
   git push origin r1.0.0
   ```

### Option B: Using a Release Branch

1. Create a release branch named with the release convention:
   ```bash
   git checkout -b r1.0.0
   ```
2. Push the release branch to GitHub:
   ```bash
   git push origin r1.0.0
   ```

## What the Automated Pipeline Does

Once a release tag or branch matching `r*.*.*` is pushed to GitHub:

1. **Validation**: The pipeline runs code quality checks:
   - Frontend formatting (`prettier`) and TypeScript type-checking (`svelte-check`).
   - Rust backend formatting (`rustfmt`) and linter checks (`cargo clippy`).
   - Frontend unit tests with Playwright.
   - CycloneDX Software Bill of Materials (SBOM) generation and vulnerability scanning.
2. **Multi-Platform Packaging**: A build matrix compiles and packages native installers concurrently across:
   - **Windows** (`windows-latest`): Produces the Windows MSI installer (`.msi`).
   - **macOS** (`macos-latest`): Produces the macOS DMG installer (`.dmg`).
   - **Linux** (`ubuntu-22.04`): Produces Linux Debian packages (`.deb`) and AppImages (`.AppImage`).
3. **Artifact Storage & Release Publishing**:
   - Stores all built platform installers as GitHub Actions artifacts (30-day retention).
   - Detects the release marker, creates/updates the release tag, and publishes a new GitHub Release via `softprops/action-gh-release@v2`.
   - Attaches the macOS DMG, Linux DEB & AppImage, and Windows MSI installers, along with auto-generated release notes and marking it as the latest release.
