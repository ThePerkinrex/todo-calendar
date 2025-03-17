import { loadTasks } from "../REST/client.js";
import { getCategory } from "./category.js";
import { setColorBlock } from "./color.js";
import { getCourse } from "./course.js";
import { getState } from "./state.js";

export async function loadTaskList(filter) {
	console.log("Loading tasks:", filter);
	const tasks = await loadTasks(filter);
	const template = document.getElementById("entry-template");
	const body = template.parentElement;

	for (let i = 1; i < body.children.length; i++) {
		body.removeChild(body.children[i]);
	}

	for (const task of tasks) {
		const course = await getCourse(task.course);
		const category = await getCategory(task.category);
		const state = await getState(task.state);

		const row = template.content.cloneNode(true);
		setColorBlock(
			row.querySelector(".course-block"),
			course.color,
			course.name
		);
		setColorBlock(
			row.querySelector(".category-block"),
			category.color,
			category.name
		);
		setColorBlock(
			row.querySelector(".state-block"),
			state.color,
			state.name
		);

		row.querySelector(".name-span").innerText = task.name;
		row.querySelector(".id").innerText = task.id;
		row.querySelector(".parent").innerText = task.parent;
		row.querySelector(".course").innerText = course.name;
		row.querySelector(".category").innerText = category.name;
		row.querySelector(".state").innerText = state.name;

		body.appendChild(row);
	}
}
