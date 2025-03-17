import { loadCourse } from "../REST/client.js";
import { getColor } from "./color.js";

const emptyCourse = {
	name: "",
	color: "white",
};

export async function getCourse(id) {
	if (id === null || id === undefined) return emptyCourse;
	let data = await loadCourse(id);
	return {
		name: data.name,
		color: await getColor(data.color),
	};
}
