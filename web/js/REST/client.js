import { cached } from "./cached.js";

function getJSON(endpoint) {
	return () => fetch(endpoint).then((x) => x.json());
}

function getJSONWithId(endpoint) {
	return (id) => fetch(`${endpoint}/${id}`).then((x) => x.json());
}

const [loadTasks, clearTasks] = cached(getJSON("/tasks"));
const [loadTask, clearTask] = cached(getJSONWithId("/tasks"));
const [loadColors, clearColors] = cached(getJSON("/colors"));
const [loadColor, clearColor] = cached(getJSONWithId("/colors"));
const [loadCourses, clearCourses] = cached(getJSON("/courses"));
const [loadCourse, clearCourse] = cached(getJSONWithId("/courses"));
const [loadCategories, clearCategories] = cached(getJSON("/categories"));
const [loadCategory, clearCategory] = cached(getJSONWithId("/categories"));
const [loadStates, clearStates] = cached(getJSON("/states"));
const [loadState, clearState] = cached(getJSONWithId("/states"));

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
