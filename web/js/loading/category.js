import { loadCategory } from "../REST/client.js";
import { getColor } from "./color.js";

const emptyCategory = {
	name: "",
	color: "white",
};

export async function getCategory(id) {
	if (id === null || id === undefined) return emptyCategory;
	let data = await loadCategory(id);
	return {
		name: data.name,
		color: await getColor(data.color),
	};
}