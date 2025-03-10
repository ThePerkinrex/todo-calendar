const NOW = new Date()

const COURSES = document.getElementById("courses")
const COURSE_TEMPLATE = document.getElementById("course-template")

async function loadCourses() {
	let courses = await fetch("/courses").then(t => t.json())
	for (const course of courses) {
		const template = COURSE_TEMPLATE.content.cloneNode(true);
		
		template.querySelector('.color-block').style.background = `#${course.color}`;
		template.querySelector('.course-name').innerText = course.name;
		COURSES.appendChild(template)
	}
}

loadCourses()


const DEADLINES = document.getElementById("deadlines")
const PAST_DEADLINES = document.getElementById("past-deadlines")
const NEXT_DEADLINES = document.getElementById("next-deadlines")
const DEADLINE_TEMPLATE = document.getElementById("deadline-template")

function buildDeadline(deadline, course, cat) {
	const template = DEADLINE_TEMPLATE.content.cloneNode(true);
	template.querySelector('.course-color').style.background = `#${course.color}`;
	template.querySelector('.cat-color').style.background = `#${cat.color}`;
	template.querySelector('.deadline-name').innerText = deadline.name + " - " + new Date(deadline.timestamp).toUTCString();
	template.querySelector('.course-name').innerText = course.name;
	template.querySelector('.cat-name').innerText = cat.name;
	return template;
}

async function loadDeadlines() {
	let current_date = NOW.toISOString()
	let past_params = new URLSearchParams()
	past_params.append("to", current_date)
	let next_params = new URLSearchParams()
	next_params.append("from", current_date)
	
	let past_deadlines = await fetch("/deadlines?" + past_params).then(t => t.json())
	for (const deadline of past_deadlines) {
		const course = await fetch(`/courses/${deadline.course}`).then(x => x.json())
		const cat = await fetch(`/deadlines/category/${deadline.category}`).then(x => x.json())

		PAST_DEADLINES.appendChild(buildDeadline(deadline, course, cat))
	}
	let next_deadlines = await fetch("/deadlines?" + next_params).then(t => t.json())
	for (const deadline of next_deadlines) {
		const course = await fetch(`/courses/${deadline.course}`).then(x => x.json())
		const cat = await fetch(`/deadlines/category/${deadline.category}`).then(x => x.json())

		NEXT_DEADLINES.appendChild(buildDeadline(deadline, course, cat))
	}
}

loadDeadlines()


const NOW_EL = document.getElementById("now")
NOW_EL.innerText = NOW.toUTCString()