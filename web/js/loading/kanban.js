import { loadStates } from "../REST/client.js"
import { apiV1TasksPut } from "../REST/paths.js"
import { genColorBlock, getColor } from "./color.js"

export async function buildKanban(element) {
	let states = await loadStates()
	let stateColumnIds = [...states.map(x => x.id), null]
	stateColumnIds.reverse()
	let stateColumnNames = [...(await Promise.all(states.map(async x => {
		const color = await getColor(x.color)
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

		for (let i = 0; i < stateColumnNames.length; i++) {
			const name = stateColumnNames[i]
			const cid = stateColumnIds[i]
			const column = document.createElement('div')
			column.classList.add('kanban-column')
			const title = document.createElement('div')
			title.classList.add('kanban-column-title')
			title.innerHTML = name
			column.appendChild(title)
			element.appendChild(column)
			this.columns.push(column)
			const drag_into = (ev) => {
				const isCard = ev.dataTransfer.types.includes("card-task");
				if(isCard) {
					const task = JSON.parse(ev.dataTransfer.getData('card-task'))
					
					const closestCard = ev.target.closest('.kanban-card')
					if(closestCard && closestCard.dataset.id == task.id) {
						ev.preventDefault()
						return
					}

					for (const x of document.getElementsByClassName('kanban-card-drag')) {
						x.remove()
					}
					// const html = ev.dataTransfer.getData('text/html')
					const card = buildBanbanCard(task)
					card.classList.add('kanban-card-drag')
					const closestColumn = ev.target.closest('.kanban-column')
					if (closestCard) {
						closestCard.before(card)
					}else if(closestColumn){
						closestColumn.appendChild(card)
					}
				}
			}

			column.ondragenter = drag_into
			column.ondragover = drag_into
			column.ondrop = (ev) => {
				const card = document.querySelector('.kanban-card-drag')
				card.classList.remove('kanban-card-drag')
				const task = JSON.parse(ev.dataTransfer.getData('card-task'))
				task.state = cid
				fetch(apiV1TasksPut(task.id), {
					method: 'PUT',
					body: JSON.stringify(task),
					headers: {
						'Content-Type': 'application/json'
					}
				})
			}
		}
		
	}

	add(task) {
		const idx = this.stateColumnIds.indexOf(task.state)
		console.log(task.state, this.stateColumnIds, idx)
		this.columns[idx].appendChild(buildBanbanCard(task))
	}
}

function buildBanbanCard(task) {
	const element = document.createElement('div')
	element.classList.add('kanban-card')
	element.draggable = true
	const nameE = document.createElement('div')
	nameE.classList.add('kanban-card-name')
	nameE.innerText = task.name
	element.appendChild(nameE)
	const catE = document.createElement('div')
	catE.classList.add('kanban-card-cat')
	catE.innerText = task.categoryData.name
	element.appendChild(catE)

	if (task.courseData) {
		const courseE = document.createElement('div')
		courseE.classList.add('kanban-card-course')
		courseE.innerText = task.courseData.name
		element.appendChild(courseE)
	}
	// const timeE = document.createElement('div')
	// timeE.classList.add('kanban-card-time')

	// element.innerText = `TODO: ${task.name} - ${task.categoryData.name}`
	element.dataset.id = task.id
	element.ondragstart = (ev) => {
		ev.dataTransfer.setData("text/html", ev.target.outerHTML);
		ev.dataTransfer.setData("card-task", JSON.stringify(task));
		ev.dataTransfer.effectAllowed = 'move'
		// element.classList.add('kanban-card-drag')
	}
	element.ondragend = (ev) => {
		if(ev.dataTransfer.dropEffect === 'move') {
			element.remove()
		}
	}
	return element
}