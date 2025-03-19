import {loadCategories} from '/js/REST/client.js'
import {getColor} from '/js/loading/color.js'

export async function category() {
	console.log('Loading categories')
	return await Promise.all((await loadCategories()).map(async (c) => {
		const color = await getColor(c.color)
		return {
			value: c.id,
			content: c.name
		}
	}))
}

export async function course() {
	return [
	]
}

export async function state() {
	return [
	]
}

export async function task() {
	return [
	]
}