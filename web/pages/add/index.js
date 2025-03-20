import { Select } from '/lib/select/lib.js'
import * as dataSources from './js/data_source.js'
import { LocalDate, UTCDate } from '/js/date.js'

function timeSetup() {
	const start = document.getElementById('startTime')
	const startNow = document.getElementById('startTimeNow')
	const end = document.getElementById('endTime')
	const endNow = document.getElementById('endTimeNow')

	start.onchange = () => {
		if(start.value) {
			end.disabled = false
			endNow.disabled = false
		}else{
			end.disabled = true
			endNow.disabled = true
			end.removeAttribute('value')
		}
	}

	startNow.onclick = () => {
		start.value = new UTCDate().toDatetimeLocalString();
		start.onchange()
	}

	endNow.onclick = () => {
		end.value = new UTCDate().toDatetimeLocalString();
	}
}

async function addTask(data) {
	let time;
	console.log(data)
	if(data.get('start')) {
		const body = {
			start: new UTCDate(new Date(data.get('start'))).toISOString(),
			end: data.get('end') ? new UTCDate(new Date(data.get('end'))).toISOString() : undefined
		}
		console.log('time', body)
		const res = await fetch('/times', {method: 'POST', body: JSON.stringify(body), headers: {'Content-Type': 'application/json'}}).then(x => x.json())
		time = res
	}
	const body = {
		name: data.get('name'),
		category: parseInt(data.get('category')),
		course: parseInt(data.get('course')),
		state: parseInt(data.get('state')),
		parent: parseInt(data.get('parent')),
		time
	}
	console.log('task', body)
	await fetch('/tasks', {method: 'POST', body: JSON.stringify(body), headers: {'Content-Type': 'application/json'}})
}

function formSubmitSetup() {
	const form = document.getElementById('task-form');
	form.onsubmit = (ev) => {
		ev.preventDefault();
		const data = new FormData(form)
		addTask(data)
		return false;
	}
}

export async function main() {
	timeSetup()
	for(const e of document.getElementsByClassName('html-select')) {
		const source = dataSources[e.dataset.source]
		new Select(e, await source())
	}
	formSubmitSetup()
}
