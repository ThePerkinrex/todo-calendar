function readFile(file) {
	return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.onload = () => {
			resolve(reader.result);
		};
		reader.onerror = (e) => {
			reject(e);
		};
		reader.readAsText(file);
	});
}

async function upload() {
	const IMPORT_INPUT = document.getElementById("import-file");
	if (IMPORT_INPUT.value !== "") {
		const file = IMPORT_INPUT.files[0];
		let data = {};

		let text = await readFile(file);

		try {
			data = JSON.parse(text);
		} catch (e) {
			alert("Invalid data: " + e);
		}

		let res = await fetch("/data", {
			method: "POST",
			body: JSON.stringify(data),
			headers: {
				"Content-Type": "application/json",
			},
		});
		if (!res.ok) {
			alert("Invalid data: " + (await res.text()));
		} else {
			document.location.reload();
		}
	} else {
		alert("No file selected");
	}
}

async function clearData() {
	await fetch("/data", {
		method: "DELETE",
	});
	document.location.reload();
}

async function addCourse(e) {
	e.preventDefault();
	const formData = new FormData(e.target);
	let data = {
		name: formData.get("name"),
		color: formData.get("color"),
	};
	await fetch("/courses", {
		method: "POST",
		body: JSON.stringify(data),
		headers: {
			"Content-Type": "application/json",
		},
	});
	document.location.reload();
}

async function addDeadline(e) {
	e.preventDefault();
	const formData = new FormData(e.target);
	const timestamp = new Date(formData.get("timestamp")).toISOString();
	let data = {
		name: formData.get("name"),
		course: parseInt(formData.get("course")),
		category: parseInt(formData.get("category")),
		timestamp,
	};
	await fetch("/deadlines", {
		method: "POST",
		body: JSON.stringify(data),
		headers: {
			"Content-Type": "application/json",
		},
	});
	document.location.reload();
}
