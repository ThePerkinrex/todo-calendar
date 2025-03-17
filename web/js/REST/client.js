import { cached } from "./cached.js";

function getJSON(endpoint) {
	return [
		(params) =>
			fetch(`${endpoint}?${new URLSearchParams(params).toString()}`).then(
				(x) => x.json()
			),
		(params) => new URLSearchParams(params).toString(),
	];
}

function getJSONWithId(endpoint) {
	return [
		(id, params) =>
			fetch(
				`${endpoint}/${id}?${new URLSearchParams(params).toString()}`
			).then((x) => x.json()),
		(id, params) => `${id}?${new URLSearchParams(params).toString()}`,
	];
}

const [loadTasks, clearTasks] = cached(...getJSON("/tasks"));
const [loadTask, clearTask] = cached(...getJSONWithId("/tasks"));
const [loadColors, clearColors] = cached(...getJSON("/colors"));
const [loadColor, clearColor] = cached(...getJSONWithId("/colors"));
const [loadCourses, clearCourses] = cached(...getJSON("/courses"));
const [loadCourse, clearCourse] = cached(...getJSONWithId("/courses"));
const [loadCategories, clearCategories] = cached(...getJSON("/categories"));
const [loadCategory, clearCategory] = cached(...getJSONWithId("/categories"));
const [loadStates, clearStates] = cached(...getJSON("/states"));
const [loadState, clearState] = cached(...getJSONWithId("/states"));

export {
	loadTasks,
	loadTask,
	clearTasks,
	clearTask,
	loadColors,
	loadColor,
	clearColors,
	clearColor,
	loadCategories,
	loadCategory,
	clearCategories,
	clearCategory,
	loadCourses,
	loadCourse,
	clearCourses,
	clearCourse,
	loadStates,
	loadState,
	clearStates,
	clearState,
};
