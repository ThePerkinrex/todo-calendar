import { apiV1CategoriesPost, apiV1ColorsPost, apiV1CoursesPost, apiV1StatesPost } from "/js/REST/paths.js";

async function genericAdd(endpoint, data) {
	if (!(await fetch(endpoint, {method: 'POST', body: JSON.stringify({name: data.get('name'), color: parseInt(data.get('color'))}), headers: {'Content-Type': 'application/json'}})).ok) {
		throw new Error("Couldnt add data to " + endpoint);
	}
}

export async function category(data) {
	await genericAdd(apiV1CategoriesPost(), data)
}

export async function course(data) {
	await genericAdd(apiV1CoursesPost(), data)
}

export async function state(data) {
	await genericAdd(apiV1StatesPost(), data)
	
}

export async function color(data) {
	if (!(await fetch(apiV1ColorsPost(), {method: 'POST', body: JSON.stringify({name: data.get('name'), fallback: data.get('color')}), headers: {'Content-Type': 'application/json'}})).ok) {
		throw new Error("Couldnt add data to colors");
	}
}