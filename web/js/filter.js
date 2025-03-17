import { loadTaskList } from "./loading/taskList.js";

async function applyFilter(ev) {
	ev.preventDefault()
	const data = new FormData(ev.target)
	loadTaskList(data)
	return false;
}


export function setupFiltering() {
	const form = document.getElementById('filter-form')
	form.onsubmit = applyFilter
}