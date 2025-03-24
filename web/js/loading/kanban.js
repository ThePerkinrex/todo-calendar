import { loadStates } from "../REST/client.js"
import { genColorBlock, getColor } from "./color.js"

export async function buildKanban(element) {
	let states = await loadStates()
	let stateColumnIds = [...states.map(x => x.id), null]
	stateColumnIds.reverse()
	let stateColumnNames = [...(await Promise.all(states.map(async x => {
		const color = getColor(x.color)
		return `${genColorBlock(color).outerHTML} ${x.name}` 
	}))), 'No state']
	stateColumnNames.reverse()
	return new KanbanBoard(element, stateColumnIds, stateColumnNames)
}

export class KanbanBoard {
	constructor(element, stateColumnIds, stateColumnNames) {
		this.stateColumnIds = stateColumnIds
		this.element = element
		this.columns = []

		element.innerHTML = ''

		for (const name of stateColumnNames) {
			const column = document.createElement('div')
			column.classList.add('kanban-column')
			const title = document.createElement('div')
			title.classList.add('kanban-column-title')
			title.innerHTML = name
			column.appendChild(title)
			element.appendChild(column)
			this.columns.push(column)
		}
		
	}

	add(task) {
		
		const idx = this.stateColumnIds.indexOf(task.state)
		console.log(task.state, this.stateColumnIds, idx)
		const element = document.createElement('div')
		element.classList.add('kanban-card')
		element.innerText = `TODO: ${task.name} - ${task.categoryData.name}`
		this.columns[idx].appendChild(element)
	}
}