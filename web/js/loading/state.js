import { loadState } from "../REST/client.js";
import { getColor } from "./color.js";

const emptyState = {
	name: "",
	color: "white",
};

export async function getState(id) {
	if (id === null || id === undefined) return emptyState;
	let data = await loadState(id);
	return {
		name: data.name,
		color: await getColor(data.color),
	};
}