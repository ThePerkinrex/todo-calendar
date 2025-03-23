import { genColorBlock, setColorBlock } from "/js/loading/color.js"
import { loadCategories, loadColor, loadColors, loadCourses, loadStates } from "/js/REST/client.js"
import { Select } from "/lib/select/lib.js"
import * as submitters from './js/submitters.js'

async function loadDetails(details, getAll, getColor = async (x) => (await loadColor(x.color)).fallback) {
	const template = details.querySelector('TEMPLATE')
	const list = details.querySelector('ul')
	const form = details.querySelector('form')
	formSubmitSetup(form, submitters[form.dataset.submit])
	list.innerHTML = ''
	for (const x of await getAll()) {
		const content = template.content.cloneNode(true)
		const color = await getColor(x)
		content.querySelector('.name').innerText = x.name
		setColorBlock(content.querySelector('.color-block'), color)
		const li = document.createElement('li')
		li.appendChild(content)
		li.dataset.id = x.id
		list.appendChild(li)	
	}
}

function formSubmitSetup(form, withData) {
	form.onsubmit = (ev) => {
		ev.preventDefault();
		const data = new FormData(form);
		(async () => {
			await withData(data)
			location.reload()
		})();
		return false;
	}
}

async function colorDataSource() {
	return (await loadColors()).map(color => ({content: `${genColorBlock(color.fallback).outerHTML} ${color.name}`, value: color.id}))
}

export async function main() {
	loadDetails(document.querySelector('.categories'), loadCategories)
	loadDetails(document.querySelector('.courses'), loadCourses)
	loadDetails(document.querySelector('.states'), loadStates)
	loadDetails(document.querySelector('.colors'), loadColors, async x => x.fallback)
	for(const e of document.getElementsByClassName('html-select')) {
		new Select(e, await colorDataSource())
	}
}