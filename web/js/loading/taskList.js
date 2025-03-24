import { loadTasks } from "../REST/client.js";
import { getCategory } from "./category.js";
import { setColorBlock } from "./color.js";
import { getCourse } from "./course.js";
import { buildKanban } from "./kanban.js";
import { getState } from "./state.js";
import { getTime } from "./time.js";

export async function loadTaskList(filter) {
	// console.log("Loading tasks:", filter);
	let tasks = await Promise.all((await loadTasks(filter)).map(async task => {
		task.timeData = await getTime(task.time);
		task.courseData = await getCourse(task.course);
		task.categoryData = await getCategory(task.category);
		task.stateData = await getState(task.state);
		return task
	}));

	tasks = tasks.sort((a, b) => {
		if (a.timeData == null) return -1;
		if (b.timeData == null) return 1;
		return new Date(a.timeData.start) - new Date(b.timeData.start)
	})

	const kanban = await buildKanban(document.querySelector('.kanban'))


	
	const template = document.getElementById("entry-template");
	const body = template.parentElement;

	for (let i = body.children.length - 1; i >= 0; i--) {
		const child = body.children[i];
		// console.log('children', child)
		if(child.tagName.toLowerCase() !== 'template') {
			// console.log('removing', child)
			body.removeChild(child);
		}
	}

	for (const task of tasks) {
		const course = task.courseData;
		const category = task.categoryData;
		const state = task.stateData;
		const time = task.timeData;

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
			// console.log("What time: ", time)
			const start = new Date(time.start)
			// console.log("start: ", start)
			if (time.end !== null) {
				const end = new Date(time.end)
				
				const start_date = start.toLocaleDateString();
				const start_time = start.toLocaleTimeString();
				const end_date = end.toLocaleDateString();
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
		kanban.add(task);
	}
}
