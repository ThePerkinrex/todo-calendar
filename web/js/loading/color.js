import { loadColor } from "../REST/client.js";

export function setColorBlock(block, color, tooltip) {
	block.style.background = `${color}`;
	block.classList.add('color-block')
	if (tooltip) {
		block.title = tooltip;
	}
}

export function genColorBlock(color, tooltip) {
	const block = document.createElement('div')
	setColorBlock(block, color, tooltip)
	return block
}

export async function getColor(id) {
	return (await loadColor(id)).fallback;
}
