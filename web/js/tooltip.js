export function addTooltip(container, text) {
	container.classList.add('tooltip')
	const span = document.createElement('span')
	span.classList.add('tooltiptext')
	span.innerText = text
	container.appendChild(span)
}