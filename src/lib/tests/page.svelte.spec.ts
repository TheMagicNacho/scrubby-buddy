import { page } from 'vitest/browser';
import { describe, expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import Page from '../../routes/+page.svelte';

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
	open: vi.fn()
}));

vi.mock('@tauri-apps/plugin-opener', () => ({
	openPath: vi.fn()
}));

describe('+page.svelte', () => {
	describe('Major components are present', () => {
		it('renders the kawaii bear', async () => {
			// Arrange & Act
			const { container } = await render(Page);

			// Assert
			const bearContainerEl = container.querySelector('.bear-container');
			await expect.element(page.elementLocator(bearContainerEl!)).toBeInTheDocument();
		});

		it('renders the menu button', async () => {
			// Arrange & Act
			await render(Page);

			// Assert
			await expect.element(page.getByRole('button', { name: 'MENU' })).toBeInTheDocument();
		});

		it('renders the scrub view button', async () => {
			// Arrange & Act
			await render(Page);

			// Assert
			await expect.element(page.getByRole('button', { name: 'Scrub' })).toBeInTheDocument();
		});

		it('renders the inspect view button', async () => {
			// Arrange & Act
			await render(Page);

			// Assert
			await expect.element(page.getByRole('button', { name: 'Inspect' })).toBeInTheDocument();
		});
	});

	describe('Button hover shows bear speech bubble', () => {
		it('shows the speech bubble when the scrub view button is hovered', async () => {
			// Arrange
			await render(Page);
			const scrubButton = page.getByRole('button', { name: 'Scrub' });

			// Act - force hover to bypass actionability checks (the button has a CSS float animation)
			await scrubButton.hover({ force: true });

			// Assert - the speech bubble appears with the scrub-view message
			await expect
				.element(page.getByText(/Switch to scrub mode to scrub out all that pesky metadata/))
				.toBeInTheDocument();
		});

		it('shows the speech bubble when the inspect view button is hovered', async () => {
			// Arrange
			await render(Page);
			const inspectButton = page.getByRole('button', { name: 'Inspect' });

			// Act - force hover to bypass actionability checks (the button has a CSS float animation)
			await inspectButton.hover({ force: true });

			// Assert - the speech bubble appears with the inspect-view message
			await expect
				.element(page.getByText(/Let's switch to inspect mode/))
				.toBeInTheDocument();
		});
	});

	describe('Menu drawer', () => {
		it('opens the drawer when the menu button is clicked', async () => {
			// Arrange
			await render(Page);

			// Act - force click to bypass actionability checks (the menu button has a CSS float animation)
			await page.getByRole('button', { name: 'MENU' }).click({ force: true });

			// Assert - drawer is shown (the drawer-container is the first dialog in the DOM)
			await expect.element(page.getByRole('dialog').first()).toBeInTheDocument();
		});

		it('drawer contains a text field to change the save directory', async () => {
			// Arrange
			await render(Page);
			await page.getByRole('button', { name: 'MENU' }).click({ force: true });

			// Assert
			await expect.element(page.getByLabelText('Save Directory Name')).toBeInTheDocument();
		});

		it('drawer contains a button to reset the application state', async () => {
			// Arrange
			await render(Page);
			await page.getByRole('button', { name: 'MENU' }).click({ force: true });

			// Assert
			await expect.element(page.getByRole('button', { name: 'Reset' })).toBeInTheDocument();
		});
	});
});
