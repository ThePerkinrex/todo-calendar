import { applyFilter, setupFiltering } from "./filter.js";
import { loadSelectors } from "./loading/selectors.js";
// import { loadTaskList } from "./loading/taskList.js";

// async function loadCourses(courseTemplate, COURSES) {
// 	let courses = await fetch("/courses").then(t => t.json())
// 	const options = new DocumentFragment()

// 	for (const course of courses) {
// 		const template = courseTemplate.content.cloneNode(true);
// 		setColorBlock(template.querySelector('.color-block'), course.color);
// 		template.querySelector('.course-name').innerText = course.name;
// 		COURSES.appendChild(template)
// 		const option = document.createElement('option')
// 		option.value = course.id
// 		option.text = course.name
// 		options.appendChild(option)
// 	}
// 	for (const selector of document.getElementsByClassName('course-select')) {
// 		selector.appendChild(options.cloneNode(true))
// 	}
// }

// function buildDeadline(deadline, course, cat, DEADLINE_TEMPLATE) {
// 	const template = DEADLINE_TEMPLATE.content.cloneNode(true);
// 	setColorBlock(template.querySelector('.course-color'), course.color, course.name);
// 	setColorBlock(template.querySelector('.cat-color'), cat.color, cat.name);
// 	template.querySelector('.deadline-name').innerText = deadline.name + " - " + new Date(deadline.timestamp).toUTCString();
// 	template.querySelector('.course-name').innerText = course.name;
// 	template.querySelector('.cat-name').innerText = cat.name;
// 	return template;
// }

// async function loadDeadlines(NOW, PAST_DEADLINES, NEXT_DEADLINES, DEADLINE_TEMPLATE) {
// 	let current_date = NOW.toISOString()
// 	let past_params = new URLSearchParams()
// 	past_params.append("to", current_date)
// 	let next_params = new URLSearchParams()
// 	next_params.append("from", current_date)

// 	let past_deadlines = await fetch("/deadlines?" + past_params).then(t => t.json())
// 	for (const deadline of past_deadlines) {
// 		const course = await fetch(`/courses/${deadline.course}`).then(x => x.json())
// 		const cat = await fetch(`/deadlines/category/${deadline.category}`).then(x => x.json())

// 		PAST_DEADLINES.appendChild(buildDeadline(deadline, course, cat, DEADLINE_TEMPLATE))
// 	}
// 	let next_deadlines = await fetch("/deadlines?" + next_params).then(t => t.json())
// 	for (const deadline of next_deadlines) {
// 		const course = await fetch(`/courses/${deadline.course}`).then(x => x.json())
// 		const cat = await fetch(`/deadlines/category/${deadline.category}`).then(x => x.json())

// 		NEXT_DEADLINES.appendChild(buildDeadline(deadline, course, cat, DEADLINE_TEMPLATE))
// 	}
// }

// async function addDeadlineCatOptions() {
// 	const deadline_categories = await fetch('/deadlines/category').then(x => x.json())
// 	const options = new DocumentFragment()

// 	for (const deadline_cat of deadline_categories) {
// 		const option = document.createElement('option')
// 		option.value = deadline_cat.id
// 		option.text = deadline_cat.name
// 		options.appendChild(option)
// 	}

// 	for (const selector of document.getElementsByClassName('deadline-cat-select')) {
// 		selector.appendChild(options.cloneNode(true))
// 	}
// }

// function buildEvent(event, course, cat, EVENT_TEMPLATE) {
// 	console.log(event);
// 	const template = EVENT_TEMPLATE.content.cloneNode(true);
// 	setColorBlock(template.querySelector('.course-color'), course.color, course.name);
// 	setColorBlock(template.querySelector('.cat-color'), cat.color, cat.name);
// 	template.querySelector('.event-name').innerText = event.name + " - " + new Date(event.start).toUTCString() + "/" + new Date(event.end).toUTCString();
// 	template.querySelector('.course-name').innerText = course.name;
// 	template.querySelector('.cat-name').innerText = cat.name;
// 	return template;
// }

// async function loadEvents(NOW, PAST_EVENTS, NEXT_EVENTS, ONGOING_EVENTS, EVENT_TEMPLATE) {
// 	let current_date = NOW.toISOString()
// 	let past_params = new URLSearchParams()
// 	past_params.append("end_to", current_date)
// 	let next_params = new URLSearchParams()
// 	next_params.append("start_from", current_date)
// 	let ongoing_params = new URLSearchParams()
// 	ongoing_params.append("end_from", current_date)
// 	ongoing_params.append("start_to", current_date)

// 	let past_events = await fetch("/events?" + past_params).then(t => t.json())
// 	for (const event of past_events) {
// 		const course = await fetch(`/courses/${event.course}`).then(x => x.json())
// 		const cat = await fetch(`/events/category/${event.category}`).then(x => x.json())

// 		PAST_EVENTS.appendChild(buildEvent(event, course, cat, EVENT_TEMPLATE))
// 	}
// 	let next_events = await fetch("/events?" + next_params).then(t => t.json())
// 	for (const event of next_events) {
// 		const course = await fetch(`/courses/${event.course}`).then(x => x.json())
// 		const cat = await fetch(`/events/category/${event.category}`).then(x => x.json())

// 		NEXT_EVENTS.appendChild(buildEvent(event, course, cat, EVENT_TEMPLATE))
// 	}
// 	let ongoing_events = await fetch("/events?" + ongoing_params).then(t => t.json())
// 	for (const event of ongoing_events) {
// 		const course = await fetch(`/courses/${event.course}`).then(x => x.json())
// 		const cat = await fetch(`/events/category/${event.category}`).then(x => x.json())

// 		ONGOING_EVENTS.appendChild(buildEvent(event, course, cat, EVENT_TEMPLATE))
// 	}
// }

// async function addEventCatOptions() {
// 	const event_categories = await fetch('/events/category').then(x => x.json())
// 	const options = new DocumentFragment()

// 	for (const event_cat of event_categories) {
// 		const option = document.createElement('option')
// 		option.value = event_cat.id
// 		option.text = event_cat.name
// 		options.appendChild(option)
// 	}

// 	for (const selector of document.getElementsByClassName('event-cat-select')) {
// 		selector.appendChild(options.cloneNode(true))
// 	}
// }

// async function loadCalendar() {
// 	const weeklyCalendar = document.getElementById("weekly-calendar")
// 	const [events, deadlines] = await Promise.all([fetch("/events").then(x => x.json()), fetch("/deadlines").then(x => x.json())])
// 	buildCalendar(weeklyCalendar, events, deadlines)
// }

export function start() {
	const now = new Date();
	// const courses = document.getElementById("courses")
	// const courseTemplate = document.getElementById("course-template")
	// const DEADLINES = document.getElementById("deadlines")
	// const PAST_DEADLINES = document.getElementById("past-deadlines")
	// const NEXT_DEADLINES = document.getElementById("next-deadlines")
	// const DEADLINE_TEMPLATE = document.getElementById("deadline-template")
	// const PAST_EVENTS = document.getElementById("past-events")
	// const NEXT_EVENTS = document.getElementById("next-events")
	// const ONGOING_EVENTS = document.getElementById("ongoing-events")
	// const EVENT_TEMPLATE = document.getElementById("event-template")
	const NOW_EL = document.getElementById("now");
	// loadCourses(courseTemplate, courses);
	// loadDeadlines(now, PAST_DEADLINES, NEXT_DEADLINES, DEADLINE_TEMPLATE);
	// loadEvents(now, PAST_EVENTS, NEXT_EVENTS, ONGOING_EVENTS, EVENT_TEMPLATE);
	NOW_EL.innerText = now.toUTCString();
	// addDeadlineCatOptions()
	// addEventCatOptions()
	// loadCalendar()

	loadSelectors();

	applyFilter();

	setupFiltering();
}
