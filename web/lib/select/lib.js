export class Select {
	constructor(element, data = [], options = {}) {
		const defaultOptions = {hasNone: element.dataset.optional !== undefined, noneText:  element.dataset.optional ? element.dataset.optional : 'None', selected: undefined}
		this.options = {...defaultOptions, ...options}
		this.data = data
		this.name = element.name
		this.id = element.id
		if (!this.id) {
			this.id = "select-" + Math.floor(Math.random() * 1000000);
		}
		this.classList = element.classList
		this.element = element
		this.justClicked = false

		if(!this.options.hasNone) {
			this.options.selected = 0
		}

		let i = 0;
		for(const e of element.children) {
			this.data.push({
				value: e.value,
				content: e.innerHTML
			})
			if(e.selected) {
				this.options.selected = i
			}
			i++
		}

		this.build()
		document.addEventListener('click', (ev) => this._on_click_outside(ev))
	}

	_on_click_outside(ev) {
		
		if (
			!ev.target.closest("#" + this.id) &&
			!ev.target.closest('label[for="' + this.id + '"]')
		) {
			if(this.justClicked) {
				this.justClicked = false
			}else{
				this.dropdown.classList.remove('select-open')
			}
		}
		
	}

	_on_click_header() {
		this.dropdown.classList.add('select-open')
		// this.justClicked = true
	}

	_on_click_element(element) {
		const i = element.dataset.idx
		this.options.selected = i
		if(i === undefined) {
			this.input.removeAttribute('value')
		}else{
			this.input.value = this.data[i].value
		}
		this.header.innerHTML = element.innerHTML
		for(const e of this.element.querySelectorAll('.select-selected')) {
			e.classList.remove('select-selected')
		}
		element.classList.add('select-selected')
		this.dropdown.classList.remove('select-open')
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
		if(!this.options.hasNone) {
			input.required = true
		}
		this.input = input

		container.appendChild(input)

		const header = document.createElement('div')
		header.classList.add('select-header')
		if(this.options.selected !== undefined) {
			header.innerHTML = this.data[this.options.selected].content
		}else{
			header.innerHTML = this.options.noneText
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
			e.onclick = () => this._on_click_element(e)

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
			e.onclick = () => this._on_click_element(e)
			dropdown.appendChild(e)
		}

		container.appendChild(dropdown)

		this.element.replaceWith(container)
		this.element = container
		this.header.onclick = (ev) => this._on_click_header(ev)
		this.dropdown = dropdown
		console.log(this)
	}
}