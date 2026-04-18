import { page } from 'vitest/browser';
import { describe, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import KawaiBear from './KawaiBear.svelte';

describe('KawaiBear', () => {
	it('renders the bear container', async () => {
		// Arrange & Act
		const { container } = await render(KawaiBear, { state: 'waiting', message: '' });

		// Assert
		const bearContainerEl = container.querySelector('.bear-container');
		await expect.element(page.elementLocator(bearContainerEl!)).toBeInTheDocument();
	});

	it('applies the waiting class when state is waiting', async () => {
		// Arrange & Act
		const { container } = await render(KawaiBear, { state: 'waiting', message: '' });

		// Assert
		const bearEl = container.querySelector('.bear.waiting');
		await expect.element(page.elementLocator(bearEl!)).toBeInTheDocument();
	});

	it('applies the talking class when state is talking', async () => {
		// Arrange & Act
		const { container } = await render(KawaiBear, { state: 'talking', message: 'Hello!' });

		// Assert
		const bearEl = container.querySelector('.bear.talking');
		await expect.element(page.elementLocator(bearEl!)).toBeInTheDocument();
	});

	it('applies the looking class when state is looking', async () => {
		// Arrange & Act
		const { container } = await render(KawaiBear, { state: 'looking', message: '' });

		// Assert
		const bearEl = container.querySelector('.bear.looking');
		await expect.element(page.elementLocator(bearEl!)).toBeInTheDocument();
	});

	it('shows a speech bubble with the message text when state is talking', async () => {
		// Arrange
		const testMessage = 'Squeaky clean, just for you!';

		// Act
		await render(KawaiBear, { state: 'talking', message: testMessage });

		// Assert
		await expect.element(page.getByText(testMessage)).toBeInTheDocument();
	});

	it('does not show a speech bubble when state is not talking', async () => {
		// Arrange & Act
		await render(KawaiBear, { state: 'waiting', message: 'Hidden message' });

		// Assert
		await expect.element(page.getByText('Hidden message')).not.toBeInTheDocument();
	});
});
