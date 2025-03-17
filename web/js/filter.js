import { loadTaskList } from "./loading/taskList.js";

export async function applyFilter(form) {
	if (form === undefined) form = getFilterForm();

	const data = new FormData(form);
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
