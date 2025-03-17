import { UTCDate } from "./date.js"

export class TimeSelect {
	constructor(container, data = {}) {
		const dataDefaults = {
			start_from: null,
			start_to: null,
			end_from: new UTCDate().toLocal(),
			end_to: null,
			includeNoTime: true,
			open: false
		}
		this.data = {...dataDefaults, ...data}
		this.container = container
		this._build_inner()
	}

	_build_single(parent, key, name, nameText) {
		// const row = document.createElement('div')
		// row.classList.add('timeselect-row')

		const label = document.createElement('label')
		label.htmlFor = name
		label.innerText = nameText
		parent.appendChild(label)

		const input = document.createElement('input')
		input.type = 'datetime-local'
		input.name = name
		input.id = name

		if (this.data[key]) {
			input.value = this.data[key].toLocalISOString()
		}

		parent.appendChild(input)

		// return row
	}

	_build_inner() {
		this.container.innerHTML = ''
		const small = document.createElement('div')
		small.classList.add('timeselect-header')
		small.innerText = 'Any time'

		const content = document.createElement('div')
		content.classList.add('timeselect-content')
		if(this.data.open) content.classList.add('timeselect-open')

		const grid = document.createElement('div')
		grid.classList.add('timeselect-content-grid')

		this._build_single(grid, 'start_from', 'from_start_timeselect', 'Start from')
		this._build_single(grid, 'start_to', 'to_start_timeselect', 'Start to')
		this._build_single(grid, 'end_from', 'from_end_timeselect', 'End from')
		this._build_single(grid, 'end_to', 'to_end_timeselect', 'End to')

		content.appendChild(grid)

		this.container.innerHTML = ''
		this.container.appendChild(small)
		this.container.appendChild(content)

		small.onclick = () => {
			this.data.open = !this.data.open
			content.classList.toggle('timeselect-open')
		}
		// TODO set events

	}
}