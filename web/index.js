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
const DEADLINE_TEMPLATE = document.getElementById("deadline-template")

async function loadDeadlines() {
	let deadlines = await fetch("/deadlines").then(t => t.json())
	for (const deadline of deadlines) {
		const template = DEADLINE_TEMPLATE.content.cloneNode(true);
		const course = await fetch(`/courses/${deadline.course}`).then(x => x.json())
		const cat = await fetch(`/deadlines/category/${deadline.category}`).then(x => x.json())
		template.querySelector('.course-color').style.background = `#${course.color}`;
		template.querySelector('.cat-color').style.background = `#${cat.color}`;
		template.querySelector('.deadline-name').innerText = deadline.name;
		template.querySelector('.course-name').innerText = course.name;
		template.querySelector('.cat-name').innerText = cat.name;
		DEADLINES.appendChild(template)
	}
}

loadDeadlines()