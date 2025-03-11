export function buildCalendar(container, events, deadlines) {
	// Clear any existing content.
	container.innerHTML = "";

	// Create a grid container for 7 columns (one per day)
	const calendarGrid = document.createElement("div");
	calendarGrid.style.display = "grid";
	calendarGrid.style.gridTemplateColumns = "repeat(7, 1fr)";
	calendarGrid.style.border = "1px solid #ccc";
	calendarGrid.style.gap = "1px";

	// Today at midnight
	const today = new Date();
	today.setHours(0, 0, 0, 0);

	// Define the timeline container height (e.g., 24 hours * 30px per hour = 720px)
	const timelineHeight = 24 * 30;

	for (let i = 0; i < 7; i++) {
		// Compute the date for this column
		const dayDate = new Date(today.getTime() + i * 24 * 60 * 60 * 1000);

		// Create a column container for the day
		const dayColumn = document.createElement("div");
		dayColumn.style.position = "relative";
		dayColumn.style.backgroundColor = "#fff";
		dayColumn.style.borderRight = "1px solid #ccc";

		// Create a header showing the day name and date
		const header = document.createElement("div");
		header.textContent = dayDate.toLocaleDateString(undefined, {
			weekday: "long",
			month: "short",
			day: "numeric",
		});
		header.style.textAlign = "center";
		header.style.fontWeight = "bold";
		header.style.padding = "5px";
		header.style.backgroundColor = "#eee";
		dayColumn.appendChild(header);

		// Create a timeline container for this day (for event positioning)
		const timelineContainer = document.createElement("div");
		timelineContainer.style.position = "relative";
		timelineContainer.style.height = timelineHeight + "px";
		timelineContainer.style.borderTop = "1px solid #ccc";

		// Define start and end for the day
		const dayStart = new Date(dayDate);
		dayStart.setHours(0, 0, 0, 0);
		const dayEnd = new Date(dayDate);
		dayEnd.setHours(23, 59, 59, 999);

		// --- Place events ---
		// Select events that “touch” this day.
		const eventsForDay = events.filter((e) => {
			const eventStart = new Date(e.start);
			const eventEnd = new Date(e.end);
			return eventStart <= dayEnd && eventEnd >= dayStart;
		});

		eventsForDay.forEach((e) => {
			const eventStart = new Date(e.start);
			const eventEnd = new Date(e.end);
			// Determine the effective start/end for this day (for multi‑day events)
			const effectiveStart =
				eventStart < dayStart ? dayStart : eventStart;
			const effectiveEnd = eventEnd > dayEnd ? dayEnd : eventEnd;

			// Calculate minutes offset relative to the start of the day
			const minutesFromStart = (effectiveStart - dayStart) / (1000 * 60);
			const eventDurationMinutes =
				(effectiveEnd - effectiveStart) / (1000 * 60);

			// Create the event block
			const eventBlock = document.createElement("div");
			eventBlock.style.position = "absolute";
			eventBlock.style.left = "5px";
			eventBlock.style.right = "5px";
			eventBlock.style.top =
				(minutesFromStart / (24 * 60)) * timelineHeight + "px";
			eventBlock.style.height =
				(eventDurationMinutes / (24 * 60)) * timelineHeight + "px";
			eventBlock.style.backgroundColor = "#aaf";
			eventBlock.style.border = "1px solid #55f";
			eventBlock.style.boxSizing = "border-box";
			eventBlock.style.padding = "2px";
			eventBlock.style.fontSize = "12px";
			eventBlock.style.overflow = "hidden";
			eventBlock.title = `Event ID: ${e.id}`;
			// Show the event name and times. (Times may be adjusted if spanning days.)
			eventBlock.textContent = `${e.name} (${new Date(
				e.start
			).toLocaleTimeString()} - ${new Date(e.end).toLocaleTimeString()})`;

			timelineContainer.appendChild(eventBlock);
		});

		// --- Place deadlines ---
		const deadlinesForDay = deadlines.filter((d) => {
			const deadlineDate = new Date(d.timestamp);
			return deadlineDate >= dayStart && deadlineDate <= dayEnd;
		});

		deadlinesForDay.forEach((d) => {
			const deadlineTime = new Date(d.timestamp);
			const minutesFromStart = (deadlineTime - dayStart) / (1000 * 60);
			const topPosition = (minutesFromStart / (24 * 60)) * timelineHeight;

			// Create a label element for the deadline
			const deadlineLabel = document.createElement("div");
			deadlineLabel.textContent = d.name;
			deadlineLabel.style.position = "absolute";
			// Position the label a few pixels above the marker
			deadlineLabel.style.top = `calc(${topPosition}px - 1.2em)`; // adjust 12px if needed
			deadlineLabel.style.left = "5px";
			deadlineLabel.style.right = "5px";
			deadlineLabel.style.fontSize = "10px";
			deadlineLabel.style.backgroundColor = "#fff";
			deadlineLabel.style.padding = "0 2px";
			deadlineLabel.style.whiteSpace = "nowrap";
			deadlineLabel.style.overflow = "hidden";
			deadlineLabel.style.textOverflow = "ellipsis";

			// Create a marker for the deadline
			const deadlineMarker = document.createElement("div");
			deadlineMarker.style.position = "absolute";
			deadlineMarker.style.left = "5px";
			deadlineMarker.style.right = "5px";
			deadlineMarker.style.top = topPosition + "px";
			deadlineMarker.style.height = "4px";
			deadlineMarker.style.backgroundColor = "#f55";
			deadlineMarker.title = `${
				d.name
			} at ${deadlineTime.toLocaleTimeString()}`;

			timelineContainer.appendChild(deadlineLabel);
			timelineContainer.appendChild(deadlineMarker);
		});

		dayColumn.appendChild(timelineContainer);
		calendarGrid.appendChild(dayColumn);
	}

	container.appendChild(calendarGrid);
}
