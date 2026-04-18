<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { openPath } from '@tauri-apps/plugin-opener';
	import { fly } from 'svelte/transition';
	import KawaiBear from '$lib/components/KawaiBear.svelte';

	let folderPath = $state('');
	let numberOfFiles = $state(-1);
	let thinking = $state(false);
	let cleanPath = $state('');
	let saveDirectory = $state('OwO-Clean');
	let isDrawerOpen = $state(false);

	let activeView: 'scrub' | 'inspect' = $state('scrub');

	// Inspect view state
	let imagePath = $state('');
	let imageMetadata = $state<Record<string, string> | null>(null);
	let inspecting = $state(false);

	// Bear state management
	type BearState = 'waiting' | 'talking' | 'looking';
	let bearState: BearState = $state('waiting');
	let bearMessage = $state('');

	function setBearState(state: BearState, message: string = '', duration: number = 3000) {
		bearState = state;
		bearMessage = message;
		if (state !== 'waiting' && duration > 0) {
			setTimeout(() => {
				bearState = 'waiting';
				bearMessage = '';
			}, duration);
		}
	}

	// Button tool tips
	const buttonDescriptions: Record<string, string> = {
		'select-folder':
			"I'll scrub all the images in the folder you choose. I'm sorry Daddy, but I can only clean JPEG right now...",
		scrub:
			"Let's clean those images! I'll remove all metadata making them squeaky clean, and save them in a new folder for you! The original images will stay unchanged! You can change the folder name I save to by clicking on the MENU in the top right!",
		reset: 'This will wash away everything so we can start fresh!',
		'select-image':
			"Pick an image and I'll show you all the hidden metadata inside! Even clean files have a small amount of metadata to help tell devices how to render the image.",
		'scrub-view': 'Switch to scrub mode to scrub out all that pesky metadata from your images!',
		'inspect-view': "Let's switch to inspect mode and take a peek at that sweet sweet image data!"
	};
	// Things the bear says in reaction to user actions
	const reactionScript: Record<string, string> = {
		scrubbing: "Pweeese wait, I'm scrubbing as fast as I can!",
		'all-done': "I'm done now, Daddy. Squeaky clean, just for you!",
		found: 'Here are all the images I could find in the folder you gave me.',
		'no-find': "I'm sorry, daddy! I couldn't find any images for you.",
		thinking: 'hummmmm...',
		'what-i-found': "Look, Daddy! Here's what I found hiding inside your image!"
	};

	function onButtonHover(buttonId: string) {
		const description = buttonDescriptions[buttonId];
		if (description) {
			bearState = 'talking';
			bearMessage = description;
		}
	}

	function onButtonLeave() {
		bearState = 'waiting';
		bearMessage = '';
	}

	function openDrawer() {
		isDrawerOpen = true;
	}

	function closeDrawer() {
		isDrawerOpen = false;
	}

	function resetState(event: Event) {
		event.preventDefault();
		folderPath = '';
		numberOfFiles = -1;
		cleanPath = '';
		imagePath = '';
		imageMetadata = null;
	}

	async function scrubDirectory(event: Event) {
		event.preventDefault();
		thinking = true;
		setBearState('talking', reactionScript['scrubbing'], 10000);
		// console.log("Scrubbing folder:", folderPath);
		const result = String(
			await invoke('scrub_images', {
				path: folderPath,
				saveDirectory: saveDirectory
			})
		);
		thinking = false;
		setBearState('talking', reactionScript['all-done'], 10000);
		await openPath(result);
		cleanPath = result;
	}

	async function selectFolder(event: Event) {
		event.preventDefault();
		thinking = true;
		setBearState('looking', "I'm looking for your images", 8000);
		const selected = await open({
			directory: true,
			multiple: false,
			title: 'Select a folder'
		});
		if (typeof selected === 'string') {
			folderPath = selected;
			// console.log("Selected folder:", folderPath);
			numberOfFiles = await invoke('count_images', { path: folderPath });
			if (numberOfFiles > 0) {
				setBearState('talking', reactionScript['found'], 3000);
			} else {
				setBearState('talking', reactionScript['no-find'], 10000);
			}
		} else {
			// console.log("No folder selected");
			setBearState('waiting', '');
		}
		thinking = false;
	}

	function createImageMetadataObject(rawData: [string, string][]) {
		let output: Record<string, string> = {};

		for (let dataPair of rawData) {
			// console.log(`dataPair: ${dataPair}`);
			const [key, value] = dataPair;
			output[key] = value;
		}
		return output;
	}

	async function selectImage(event: Event) {
		event.preventDefault();
		inspecting = true;
		imageMetadata = null;
		setBearState('looking', reactionScript['thinking'], 10000);
		const selected = await open({
			directory: false,
			multiple: false,
			title: 'Select an image to inspect'
		});
		if (typeof selected === 'string') {
			imagePath = selected;
			const metadata = await invoke('inspect_image', { path: imagePath });
			imageMetadata = createImageMetadataObject(metadata as [string, string][]);
			setBearState('talking', reactionScript['what-i-found'], 10000);
		} else {
			console.log('No image selected');
			setBearState('waiting', '');
		}
		inspecting = false;
	}
</script>

{#if isDrawerOpen}
	<div
		class="drawer-container"
		role="dialog"
		tabindex="0"
		onclick={closeDrawer}
		onkeydown={() => {}}
	>
		<div
			class="drawer"
			role="dialog"
			tabindex="0"
			onkeydown={() => {}}
			onclick={(event) => event.stopPropagation()}
			transition:fly={{ x: 250, duration: 300 }}
		>
			<label for="save-dir">Save Directory Name</label>
			<input id="save-dir" type="text" bind:value={saveDirectory} />
			<hr />
			<label for="reset-state">Reset application state</label>
			<form id="reset-state" onsubmit={resetState}>
				<button type="submit">Reset</button>
			</form>
		</div>
	</div>
{/if}

<main class="container">
	<!-- Kawaii decorative elements -->
	<div class="kawaii-star-1">⭐</div>
	<div class="kawaii-star-2">✨</div>
	<div class="kawaii-star-3">🌟</div>

	<h1>0w0 scrubby buddy</h1>
	{#if !isDrawerOpen}
		<button class="hamburger-menu" onclick={openDrawer}><p>MENU</p></button>
	{/if}

	<div class="view-switcher">
		<button
			class:active={activeView === 'scrub'}
			onclick={() => (activeView = 'scrub')}
			onmouseenter={() => onButtonHover('scrub-view')}
			onmouseleave={onButtonLeave}
			>Scrub
		</button>
		<button
			class:active={activeView === 'inspect'}
			onclick={() => (activeView = 'inspect')}
			onmouseenter={() => onButtonHover('inspect-view')}
			onmouseleave={onButtonLeave}
			>Inspect
		</button>
	</div>

	<!--    SCRUBBING SECTION -->
	{#if activeView === 'scrub'}
		<div transition:fly={{ x: -200, duration: 300 }}>
			<form class="row" onsubmit={selectFolder}>
				<button
					type="submit"
					onmouseenter={() => onButtonHover('select-folder')}
					onmouseleave={onButtonLeave}
					>Select Folder
				</button>
			</form>

			{#if folderPath}
				{#if numberOfFiles === -1}
					<p>Counting images...</p>
				{/if}
				{#if numberOfFiles === 0}
					<p>No images found in the selected folder.</p>
				{/if}

				{#if numberOfFiles > 0}
					<p>Number of images found: {numberOfFiles}</p>
					<form class="row" onsubmit={scrubDirectory}>
						<button
							type="submit"
							onmouseenter={() => onButtonHover('scrub')}
							onmouseleave={onButtonLeave}
							>Scrub Me Daddy
						</button>
					</form>
				{/if}
			{/if}
			{#if cleanPath}
				<p>I'm all squeaky clean: {cleanPath}</p>
				<form class="row" onsubmit={resetState}>
					<button
						type="submit"
						onmouseenter={() => onButtonHover('reset')}
						onmouseleave={onButtonLeave}
						>Reset
					</button>
				</form>
			{/if}
		</div>
	{/if}

	<!--INSPECTION SECTION-->
	{#if activeView === 'inspect'}
		<div transition:fly={{ x: 200, duration: 300 }}>
			<form class="row" onsubmit={selectImage}>
				<button
					type="submit"
					onmouseenter={() => onButtonHover('select-image')}
					onmouseleave={onButtonLeave}
					>Select Image
				</button>
			</form>

			{#if inspecting}
				<p>Inspecting...</p>
			{/if}

			{#if imagePath && imageMetadata}
				<p>Showing metadata for: {imagePath}</p>
				<table class="metadata-table">
					<thead>
						<tr>
							<th>Key</th>
							<th>Value</th>
						</tr>
					</thead>
					<tbody>
						{#each Object.entries(imageMetadata) as [key, value]}
							<tr>
								<td>{key}</td>
								<td>{value}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
	{/if}

	<div class="bear">
		<KawaiBear message={bearMessage} state={bearState} />
	</div>
</main>

<style>
	/** Bear container styles **/
	.bear {
		position: absolute;
		bottom: 1.5rem;
		left: 1.5rem;
		width: 200px;
	}

	/** Hamburger menu styles **/
	.hamburger-menu {
		animation: float 3.5s ease-in-out infinite;
		height: 110px;
		width: 110px;
		border-radius: 50%;
		position: relative;

		background: radial-gradient(
			circle at 75% 30%,
			white 5px,
			var(--kawaii-cream) 8%,
			var(--kawaii-hot-pink) 60%,
			var(--kawaii-soft-pink) 100%
		);
		box-shadow:
			inset 0 0 20px #fff,
			inset 10px 0 46px #eaf5fc,
			inset 88px 0px 60px #c2d8fe,
			inset -20px -60px 100px #fde9ea,
			inset 0 50px 140px #fde9ea,
			inset 0 0 20px #fff,
			0 0 90px #fff;

		display: flex;
		align-items: center;
		justify-content: center;
		position: fixed;
		top: 1rem;
		right: 1rem;
		animation: gentleFloat 4s ease-in-out infinite;
		border: none;
	}
	.hamburger-menu p {
		font-family: 'Nunito', sans-serif;
		font-size: 1.2em;
		text-shadow: 1px 1px 2px var(--kawaii-shadow);
		font-weight: 800;
		color: var(--kawaii-white);
	}
	/**
    .hamburger-menu {
        position: fixed;
        top: 1rem;
        right: 1rem;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 80px;
        height: 80px;
        background: linear-gradient(135deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));
        border: 3px solid var(--kawaii-white);
        border-radius: 50%;
        cursor: pointer;
        z-index: 1001;
        box-shadow: 0 4px 15px var(--kawaii-shadow);
        transition: all 0.3s ease;
        animation: gentleFloat 3s ease-in-out infinite;
        font-family: 'Nunito', sans-serif;
        font-weight: 800;
        font-size: 1.2em;
        color: var(--kawaii-white);
        text-shadow: 1px 1px 2px var(--kawaii-shadow);
    }
    **/

	.hamburger-menu:hover {
		transform: translateY(-3px) scale(1.1);
		box-shadow: 0 8px 25px var(--kawaii-shadow);
	}

	.drawer-container {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		z-index: 1000;
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		height: 100%;
		width: 280px;
		background: linear-gradient(
			135deg,
			var(--kawaii-white) 0%,
			var(--kawaii-light-pink) 50%,
			var(--kawaii-soft-pink) 100%
		);
		border: 3px solid var(--kawaii-bright-pink);
		border-right: none;
		border-radius: 20px 0 0 20px;
		box-shadow: -5px 0 25px var(--kawaii-shadow);
		padding: 30px;
		z-index: 1001;
		display: flex;
		align-items: center;
		flex-direction: column;
		gap: 2rem;
		/*animation: gentleFloat 4s ease-in-out infinite;*/
	}

	.drawer::before {
		content: '🌟';
		position: absolute;
		top: 20px;
		left: 20px;
		font-size: 1.5em;
		animation: sparkle 2s ease-in-out infinite;
	}

	.drawer label {
		font-family: 'Nunito', sans-serif;
		font-weight: 700;
		color: var(--kawaii-text-dark);
		font-size: 1.1em;
		text-align: center;
	}

	.drawer hr {
		border: none;
		height: 3px;
		background: linear-gradient(90deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));
		border-radius: 5px;
		width: 100%;
		margin: 1rem 0;
	}

	/* Button and input styling */
	input,
	button {
		border-radius: 20px;
		border: 3px solid var(--kawaii-bright-pink);
		padding: 0.8em 1.5em;
		font-size: 1.1em;
		font-weight: 700;
		font-family: 'Nunito', sans-serif;
		color: var(--kawaii-white);
		background: linear-gradient(135deg, var(--kawaii-hot-pink) 0%, var(--kawaii-bright-pink) 100%);
		transition: all 0.3s ease;
		box-shadow: 0 4px 15px var(--kawaii-shadow);
		position: relative;
		overflow: hidden;
	}

	button {
		cursor: pointer;
		transform: translateY(0);
	}

	button::before {
		content: '';
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
		transition: left 0.5s;
	}

	button:hover {
		border-color: var(--kawaii-hot-pink);
		transform: translateY(-3px);
		box-shadow: 0 8px 25px var(--kawaii-shadow);
		animation: gentleFloat 2s ease-in-out infinite;
	}

	button:hover::before {
		left: 100%;
	}

	button:active {
		transform: translateY(-1px);
		box-shadow: 0 4px 15px var(--kawaii-shadow);
	}

	input {
		color: var(--kawaii-text-dark);
		background: var(--kawaii-white);
		border-color: var(--kawaii-soft-pink);
	}

	input:focus {
		outline: none;
		border-color: var(--kawaii-hot-pink);
		box-shadow: 0 0 0 3px rgba(255, 20, 147, 0.2);
	}

	/* Paragraph styling */
	p {
		font-family: 'Nunito', sans-serif;
		font-weight: 600;
		color: var(--kawaii-text-dark);
		font-size: 1.1em;
		margin: 1rem 0;
	}

	/* Row container */
	.row {
		display: flex;
		justify-content: center;
		margin: 1.5rem 0;
	}

	/*@media (prefers-color-scheme: dark) {*/
	/*    :root {*/
	/*        color: var(--kawaii-white);*/
	/*        background: linear-gradient(135deg, #2D1B69 0%, #8B008B 50%, var(--kawaii-hot-pink) 100%);*/
	/*    }*/

	/*    .hamburger-menu div {*/
	/*        background: var(--kawaii-white);*/
	/*    }*/

	/*    .drawer {*/
	/*        background: linear-gradient(135deg, #4A0E4E 0%, #8B008B 50%, var(--kawaii-hot-pink) 100%);*/
	/*        color: var(--kawaii-white);*/
	/*    }*/

	/*    .drawer label {*/
	/*        color: var(--kawaii-white);*/
	/*    }*/

	/*    .view-switcher button {*/
	/*        border-color: var(--kawaii-bright-pink);*/
	/*    }*/

	/*    .view-switcher button.active {*/
	/*        border-color: var(--kawaii-hot-pink);*/
	/*        background: linear-gradient(135deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));*/
	/*    }*/

	/*    .metadata-table th,*/
	/*    .metadata-table td {*/
	/*        border: 1px solid var(--kawaii-bright-pink);*/
	/*    }*/

	/*    .metadata-table th {*/
	/*        background: linear-gradient(135deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));*/
	/*    }*/

	/*    a:hover {*/
	/*        color: var(--kawaii-bright-pink);*/
	/*    }*/

	/*    input {*/
	/*        color: var(--kawaii-text-dark);*/
	/*        background: var(--kawaii-white);*/
	/*        border-color: var(--kawaii-bright-pink);*/
	/*    }*/

	/*    .speech-bubble {*/
	/*        background: linear-gradient(135deg, var(--kawaii-white) 0%, var(--kawaii-pastel-pink) 100%);*/
	/*        border-color: var(--kawaii-bright-pink);*/
	/*    }*/

	/*    p {*/
	/*        color: var(--kawaii-white);*/
	/*    }*/

	/*    h1 {*/
	/*        background: linear-gradient(45deg, var(--kawaii-white), var(--kawaii-pastel-pink));*/
	/*        -webkit-background-clip: text;*/
	/*        -webkit-text-fill-color: transparent;*/
	/*        background-clip: text;*/
	/*        line-height: 2;*/
	/*        padding: 0.5rem 0;*/
	/*    }*/
	/*}*/

	@font-face {
		font-family: 'Nunito';
		src: url('/fonts/nunito-regular.woff2') format('woff2');
		font-weight: 400;
		font-display: swap;
	}

	:root {
		/* Kawaii Pink Color Palette */
		--kawaii-hot-pink: #ff1493;
		--kawaii-bright-pink: #ff69b4;
		--kawaii-soft-pink: #ffb6c1;
		--kawaii-pastel-pink: #ffc0cb;
		--kawaii-light-pink: #ffcccb;
		--kawaii-cream: #fff8dc;
		--kawaii-white: #fffafa;
		--kawaii-shadow: rgba(255, 20, 147, 0.3);
		--kawaii-text-dark: #4a0e4e;
		--kawaii-text-light: #8b008b;

		font-family: 'Nunito', 'Comic Neue', Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 24px;
		font-weight: 600;

		color: var(--kawaii-text-dark);
		background: linear-gradient(
			135deg,
			var(--kawaii-cream) 0%,
			var(--kawaii-light-pink) 50%,
			var(--kawaii-soft-pink) 100%
		);
		min-height: 100vh;

		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		-webkit-text-size-adjust: 100%;

		overflow-x: hidden;
		width: 100%;
	}

	/* Kawaii floating animations */
	@keyframes gentleFloat {
		0%,
		100% {
			transform: translateY(0px);
		}
		50% {
			transform: translateY(-8px);
		}
	}

	@keyframes sparkle {
		0%,
		100% {
			opacity: 0.4;
			transform: scale(1);
		}
		50% {
			opacity: 1;
			transform: scale(1.2);
		}
	}

	@keyframes heartbeat {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.05);
		}
	}

	.container {
		margin: 0;
		/*padding-top: 10vh;*/
		display: flex;
		flex-direction: column;
		justify-content: center;
		text-align: center;
	}

	h1 {
		text-align: center;
		font-family: 'Nunito', sans-serif;
		font-weight: 800;
		font-size: 2.5rem;
		background: linear-gradient(45deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));
		line-height: 5rem;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		text-shadow: 2px 2px 4px var(--kawaii-shadow);
		margin: 2rem 0;
		animation: gentleFloat 4s ease-in-out infinite;
		position: relative;
	}

	h1::before {
		content: '✨';
		position: absolute;
		left: -2rem;
		top: 0;
		animation: sparkle 2s ease-in-out infinite;
	}

	h1::after {
		content: '💖';
		position: absolute;
		right: -2rem;
		top: 0;
		animation: heartbeat 1.5s ease-in-out infinite;
	}

	/*.logo {*/
	/*    height: 6em;*/
	/*    padding: 1.5em;*/
	/*    will-change: filter;*/
	/*    transition: 0.75s;*/
	/*}*/

	/*.logo.tauri:hover {*/
	/*    filter: drop-shadow(0 0 2em #24c8db);*/
	/*}*/

	.view-switcher {
		display: flex;
		justify-content: center;
		margin-bottom: 2rem;
		gap: 1rem;
		animation: gentleFloat 2s ease-in-out infinite;
	}

	.view-switcher button {
		background: linear-gradient(135deg, var(--kawaii-white), var(--kawaii-light-pink));
		border: 3px solid var(--kawaii-bright-pink);
		color: var(--kawaii-text-dark);
		font-family: 'Nunito', sans-serif;
		font-weight: 700;
		position: relative;
		overflow: hidden;
	}

	.view-switcher button.active {
		border-color: var(--kawaii-hot-pink);
		background: linear-gradient(135deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));
		color: var(--kawaii-white);
		box-shadow: 0 4px 15px var(--kawaii-shadow);
	}

	.view-switcher button::after {
		content: '✨';
		position: absolute;
		top: -5px;
		right: -5px;
		font-size: 0.8em;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.view-switcher button.active::after {
		opacity: 1;
		animation: sparkle 1s ease-in-out infinite;
	}

	.metadata-table {
		margin: 2rem auto;
		border-collapse: collapse;
		width: 80%;
		max-width: 600px;
		border-radius: 15px;
		overflow: hidden;
		box-shadow: 0 8px 25px var(--kawaii-shadow);
		border: 3px solid var(--kawaii-bright-pink);
	}

	.metadata-table th,
	.metadata-table td {
		border: 1px solid #ddd;
		padding: 8px;
		text-align: left;
	}

	.metadata-table th {
		background: linear-gradient(135deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));
		color: var(--kawaii-white);
		font-family: 'Nunito', sans-serif;
		font-weight: 700;
		padding: 12px;
	}

	.metadata-table td {
		background: var(--kawaii-white);
		color: var(--kawaii-text-dark);
		font-family: 'Nunito', sans-serif;
		font-weight: 500;
		padding: 10px 12px;
		border-bottom: 1px solid var(--kawaii-light-pink);
	}

	.metadata-table td:first-child {
		white-space: nowrap;
	}

	.metadata-table td:last-child {
		word-break: break-word;
		overflow-wrap: break-word;
	}

	/* Link styling */
	a {
		font-weight: 600;
		color: var(--kawaii-hot-pink);
		text-decoration: none;
		position: relative;
	}

	a::after {
		content: '';
		position: absolute;
		width: 0;
		height: 2px;
		bottom: -2px;
		left: 50%;
		background: linear-gradient(90deg, var(--kawaii-hot-pink), var(--kawaii-bright-pink));
		transition: all 0.3s ease;
	}

	a:hover::after {
		width: 100%;
		left: 0;
	}

	a:hover {
		color: var(--kawaii-bright-pink);
		transform: translateY(-1px);
		transition: all 0.3s ease;
	}

	/* Kawaii decorative elements */
	body::before,
	body::after {
		content: '';
		position: fixed;
		pointer-events: none;
		z-index: -1;
	}

	/* Floating hearts */
	.container::before {
		content: '💖';
		position: fixed;
		top: 10%;
		right: 15%;
		font-size: 2rem;
		animation: gentleFloat 6s ease-in-out infinite;
		z-index: 1;
		pointer-events: none;
	}

	.container::after {
		content: '🌸';
		position: fixed;
		bottom: 20%;
		right: 10%;
		font-size: 1.5rem;
		animation: sparkle 4s ease-in-out infinite;
		z-index: 1;
		pointer-events: none;
	}

	/* Kawaii stars scattered around */
	@keyframes floatingStar {
		0%,
		100% {
			transform: translateY(0) rotate(0deg);
			opacity: 0.6;
		}
		50% {
			transform: translateY(-15px) rotate(180deg);
			opacity: 1;
		}
	}

	/* Additional kawaii element positioning */
	.kawaii-star-1 {
		position: fixed;
		top: 25%;
		left: 8%;
		font-size: 1.2rem;
		animation: floatingStar 8s ease-in-out infinite;
		z-index: 1;
		pointer-events: none;
	}

	.kawaii-star-2 {
		position: fixed;
		top: 70%;
		right: 25%;
		font-size: 1rem;
		animation: sparkle 5s ease-in-out infinite 2s;
		z-index: 1;
		pointer-events: none;
	}

	.kawaii-star-3 {
		position: fixed;
		top: 15%;
		left: 85%;
		font-size: 0.8rem;
		animation: gentleFloat 7s ease-in-out infinite 1s;
		z-index: 1;
		pointer-events: none;
	}
</style>
