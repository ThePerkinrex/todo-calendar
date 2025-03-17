import { loadTasks } from "../REST/client.js";
import { getCategory } from "./category.js";
import { setColorBlock } from "./color.js";
import { getCourse } from "./course.js";
import { getState } from "./state.js";
import { getTime } from "./time.js";

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
		const time = await getTime(task.time)

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

		if (time !== null) {
			console.log("What time: ", time)
			const start = new Date(time.start)
			console.log("start: ", start)
			if (time.end !== null) {
				const end = new Date(time.end)
				
				const start_date = start.toLocaledateString();
				const start_time = start.toLocaleTimeString();
				const end_date = end.toLocaledateString();
				const end_time = end.toLocaleTimeString();
				
				let datestring;
				if(start_date === end_date) {
					datestring = `${start_date} ${start_time} - ${end_time}`
				}else{
					datestring = `${start_date} ${start_time} - ${end_date} ${end_time}`
				}
				row.querySelector(".time").innerText = datestring
			}else{
				row.querySelector(".time").innerText = start.toLocaleString();
			}
		}

		body.appendChild(row);
	}
}
