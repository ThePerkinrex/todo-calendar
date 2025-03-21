import { cached } from "./cached.js";
import * as paths from "./paths.js";

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
				`${endpoint(id)}?${new URLSearchParams(params).toString()}`
			).then((x) => x.json()),
		(id, params) => `${id}?${new URLSearchParams(params).toString()}`,
	];
}

const [loadTasks, clearTasks] = cached(...getJSON(paths.apiV1TasksAll()));
const [loadTask, clearTask] = cached(...getJSONWithId(paths.apiV1TasksGet));
const [loadColors, clearColors] = cached(...getJSON(paths.apiV1ColorsAll()));
const [loadColor, clearColor] = cached(...getJSONWithId(paths.apiV1ColorsGet));
const [loadCourses, clearCourses] = cached(...getJSON(paths.apiV1CoursesAll()));
const [loadCourse, clearCourse] = cached(...getJSONWithId(paths.apiV1CoursesGet));
const [loadCategories, clearCategories] = cached(...getJSON(paths.apiV1CategoriesAll()));
const [loadCategory, clearCategory] = cached(...getJSONWithId(paths.apiV1CategoriesGet));
const [loadStates, clearStates] = cached(...getJSON(paths.apiV1StatesAll()));
const [loadState, clearState] = cached(...getJSONWithId(paths.apiV1StatesGet));
const loadTime = getJSONWithId(paths.apiV1TimesGet)[0];


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
	loadTime
};
