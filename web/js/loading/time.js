import { loadTime } from "../REST/client.js";

// const emptyTime = {
// 	start: null,
// 	end: null
// };

export async function getTime(id) {
	if (id === null || id === undefined) return null;
	let data = await loadTime(id);
	// console.log("Get time", id, data)
	return {
		start: data.start,
		end: data.end,
	};
}
