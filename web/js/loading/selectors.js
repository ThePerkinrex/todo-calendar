import { loadCategories, loadCourses, loadStates } from "../REST/client.js";
import { getColor } from "./color.js";

const config = {
	search: true,
	selectAll: true,
	listAll: true,
	width: "fit-content",
	dropdownWidth: "300px",
};

export async function loadSelectors() {
	const categories = await Promise.all(
		(
			await loadCategories()
		).map(async (c) => {
			const color = await getColor(c.color);
			const style = `background-color: rgb(from ${color} r g b / 0.2); border: 1px solid rgb(from ${color} r g b / 0.4); width: fit-content;`;
			const attr = {
				style,
			};
			return {
				value: c.id,
				text: c.name,
				// innerAttributes: attr,
				outerAttributes: attr,
			};
		})
	);

	for (const s of document.querySelectorAll("select.category-select")) {
		new MultiSelect(s, {
			...config,
			data: categories,
			placeholder: "Any category",
		});
	}

	const states = await Promise.all(
		(
			await loadStates()
		).map(async (c) => {
			const color = await getColor(c.color);
			const style = `background-color: rgb(from ${color} r g b / 0.2); border: 1px solid rgb(from ${color} r g b / 0.4); width: fit-content;`;
			const attr = {
				style,
			};
			return {
				value: c.id,
				text: c.name,
				// innerAttributes: attr,
				outerAttributes: attr,
				// selected: true
			};
		})
	);

	for (const s of document.querySelectorAll("select.state-select")) {
		new MultiSelect(s, {
			...config,
			data: states,
			placeholder: "Any state",
		});
	}

	const courses = await Promise.all(
		(
			await loadCourses()
		).map(async (c) => {
			const color = await getColor(c.color);
			const style = `background-color: rgb(from ${color} r g b / 0.2); border: 1px solid rgb(from ${color} r g b / 0.4); width: fit-content;`;
			const attr = {
				style,
			};
			return {
				value: c.id,
				text: c.name,
				// innerAttributes: attr,
				outerAttributes: attr,
			};
		})
	);

	for (const s of document.querySelectorAll("select.course-select")) {
		new MultiSelect(s, {
			...config,
			data: courses,
			placeholder: "Any course",
		});
	}
}
