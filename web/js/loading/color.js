import { loadColor } from "../REST/client.js";

export function setColorBlock(block, color, tooltip) {
	block.style.background = `${color}`;
	if (tooltip) {
		block.title = tooltip;
	}
}

export async function getColor(id) {
	return (await loadColor(id)).fallback;
}