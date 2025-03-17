import { LocalDate, UTCDate } from "./date.js";

export class TimeSelect {
	constructor(container, data = {}) {
		const dataDefaults = {
			start_from: null,
			start_to: null,
			end_from: new UTCDate().toLocal(),
			end_to: null,
			excludeNoTime: false,
			timeFilters: ['end_from'],
			open: false,
		};
		this.data = { ...dataDefaults, ...data };
		this.container = container;
		this.name = this.container.getAttribute("name")
		? this.container.getAttribute("name")
		: "timeselect-" + Math.floor(Math.random() * 1000000);
		this._build_inner();
	}

	_build_single(parent, key, name, nameText) {
		// const row = document.createElement('div')
		// row.classList.add('timeselect-row')

		const label = document.createElement("label");
		label.htmlFor = name;
		label.innerText = nameText;
		parent.appendChild(label);

		const input = document.createElement("input");
		input.type = "datetime-local";
		// input.name = name;
		input.id = name;

		
		const hInput = document.createElement('input');
		hInput.type = 'hidden'

		if (this.data[key]) {
			console.log('Setting', key, this.data[key], this.data[key].toLocalISOString(), this.data[key].toISOString())
			input.value = this.data[key].toLocal().toDatetimeLocalString();
			hInput.value = this.data[key].toISOString();
		}


		input.onchange = () => {
			this.data.timeFilters.filter(x => x === key)
			if (input.value) {
				this.data[key] = new LocalDate(input.value)
				hInput.value = this.data[key].toISOString()
				this.data.timeFilters.push(key)
				this.excludeNoTime.disabled = false
			}else{
				delete hInput.value
				if(this.data.timeFilters.length == 0) this.excludeNoTime.disabled = true
			}
		}

		parent.appendChild(input);
		parent.appendChild(hInput);

		// return row
	}

	_build_inner() {
		this.container.innerHTML = "";
		const small = document.createElement("div");
		small.classList.add("timeselect-header");
		small.innerText = "Any time";

		const content = document.createElement("div");
		content.classList.add("timeselect-content");
		if (this.data.open) content.classList.add("timeselect-open");

		const grid = document.createElement("div");
		grid.classList.add("timeselect-content-grid");

		this._build_single(
			grid,
			"start_from",
			"from_start",
			"Start from"
		);
		this._build_single(grid, "start_to", "to_start", "Start to");
		this._build_single(grid, "end_from", "from_end", "End from");
		this._build_single(grid, "end_to", "to_end", "End to");

		content.appendChild(grid);

		const excludeNoTimeContainer = document.createElement('div')
		excludeNoTimeContainer.classList.add('timeselect-exclude-no-time')
		const excludeNoTimeLabel = document.createElement('label')
		excludeNoTimeLabel.htmlFor = 'exclude_no_time'
		excludeNoTimeLabel.innerHTML = 'Exclude no time'
		const excludeNoTimeInput = document.createElement('input')
		excludeNoTimeInput.id = 'exclude_no_time'
		excludeNoTimeInput.name = 'exclude_no_time'
		excludeNoTimeInput.type = 'checkbox'
		excludeNoTimeInput.checked = this.data.excludeNoTime
		excludeNoTimeInput.value = 'true'
		this.excludeNoTime = excludeNoTimeInput;
		if(this.data.timeFilters.length == 0) this.excludeNoTime.disabled = true
		excludeNoTimeContainer.appendChild(excludeNoTimeLabel)
		excludeNoTimeContainer.appendChild(excludeNoTimeInput)

		content.appendChild(excludeNoTimeContainer)


		this.container.innerHTML = "";
		this.container.appendChild(small);
		this.container.appendChild(content);
		this.container.classList.add(this.name)

		small.onclick = (ev) => {
			this.data.open = !this.data.open;
			console.log(this.name, this.data.open)
			content.classList.toggle("timeselect-open");
			// ev.preventDefault()
			ev.stopPropagation()
		};

		document.addEventListener("click", (ev) => {
			if(this.data.open) console.log(this.name, ev)
			if (
				this.data.open &&
				!ev.target.closest("." + this.name) &&
				!ev.target.closest('label[for="' + this.container.id + '"]')
			) {
				content.classList.remove("timeselect-open");
				this.data.open = false;
			}
		});
	}
}
