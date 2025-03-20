import {loadCategories, loadCategory, loadCourses, loadCourse, loadStates, loadState, loadTasks} from '/js/REST/client.js'
import {getColor, genColorBlock} from '/js/loading/color.js'

async function genericDataSource(data, mapping = async (c) => {
	const color = await getColor(c.color)
	const colorBlock = genColorBlock(color)
	return {
		value: c.id,
		content: `${colorBlock.outerHTML} ${c.name}`
	}
}) {
	return await Promise.all(data.map(mapping))
}

export async function category() {
	return await genericDataSource(await loadCategories())
}

export async function course() {
	return await genericDataSource(await loadCourses())
}

export async function state() {
	return await genericDataSource(await loadStates())
}

async function getOptionalColorBlock(loader, id) {
	if(!id) {
		return genColorBlock('#0000')
	}
	const data = await loader(id)
	const color = await getColor(data.color)
	return genColorBlock(color, data.name)
}

export async function task() {
	return await genericDataSource(await loadTasks(), async (t) => {
		const categoryBlock = await getOptionalColorBlock(loadCategory, t.category)
		const courseBlock = await getOptionalColorBlock(loadCourse, t.course)
		const stateBlock = await getOptionalColorBlock(loadState, t.state)

		return {
			value: t.id,
			content: `${stateBlock.outerHTML} ${categoryBlock.outerHTML} ${courseBlock.outerHTML} ${t.name}`
		}
	})
}