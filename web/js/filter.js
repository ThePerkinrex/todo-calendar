import { loadTaskList } from "./loading/taskList.js";

export async function applyFilter(form) {
	if (form === undefined) form = getFilterForm();

	const data = new FormData(form);
	data.append('from_end', new Date().toISOString()) // FIXME Timezones?
	data.append('include_no_time', true)
	await loadTaskList(data);
}

function getFilterForm() {
	return document.getElementById("filter-form");
}

export function setupFiltering() {
	const form = getFilterForm();
	form.onsubmit = (ev) => {
		ev.preventDefault();
		applyFilter(ev.target);
		return false;
	};
}
