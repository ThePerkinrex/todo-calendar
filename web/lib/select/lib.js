class Select {
	constructor(element, data = [], options = {}) {
		const defaultOptions = {hasNone: element.dataset.optional === undefined, noneText:  element.dataset.optional ? element.dataset.optional : 'None', selected: undefined}
		this.options = {...defaultOptions, ...options}
		this.data = data
		this.name = element.name
		this.id = element.id
		this.classList = element.classList

		let i = 0;
		for(const e of element.children) {
			this.data.push({
				value: e.value,
				content: e.innerHTML
			})
			if(e.selected) {
				this.options.selectedValue = e.value
			}
			i++
		}

		this.build()
	}

	build() {
		const container = document.createElement('div')
		container.classList.add(...this.classList)
		container.classList.add('select-container')
		container.id = this.id

		const input = document.createElement('input')
		input.type = 'hidden'
		input.name = this.name
		if(this.options.selected !== undefined) {
			input.value = this.data[this.options.selected].value
		}
		this.input = input

		container.appendChild(input)

		const header = document.createElement('div')
		header.classList.add('select-header')
		if(this.options.selected !== undefined) {
			header.innerHTML = this.data[this.options.selected].content
		}

		this.header = header
		container.appendChild(header)

		const dropdown = document.createElement('div')
		dropdown.classList.add('select-dropdown')
		if(this.options.hasNone) {
			const e = document.createElement('div')
			e.classList.add('select-dropdown-element')
			if(this.options.selected === undefined) {
				e.classList.add('select-selected')
				e.dataset.selected = true
			}
			e.innerHTML = this.options.noneText

			dropdown.appendChild(e)
		}
		for(let i = 0; i < this.data.length; i++) {
			const d = this.data[i]
			const e = document.createElement('div')
			e.classList.add('select-dropdown-element')
			if(i === this.options.selected) {
				e.classList.add('select-selected')
				e.dataset.selected = true
			}
			e.dataset.value = d.value
			e.innerHTML = d.content
			e.dataset.idx = i
			dropdown.appendChild(e)
		}

		this.dropdown = dropdown
		container.appendChild(dropdown)


		this.element.replaceWith(container)
		
	}
}