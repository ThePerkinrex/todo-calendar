import icalendar
from pathlib import Path
import api
import inquirer
import datetime

def build_base_questions(categories, courses, states):
	return [
		inquirer.List('category', message='What category does it belong to?', choices=categories),
		inquirer.List('course', message='What course does it belong to?', choices=courses),
		inquirer.List('state', message='What state is it in?', choices=states),
		inquirer.Confirm('has_time', message='Has it got a time?', default = True)

	]

def get_calendar():
	ics_path = Path("data/24-25 SS_88af9d2b57efa24721baaf64072b858600e830b02a82b5923874d3e460c0e0fc@group.calendar.google.com.ics")
	return icalendar.Calendar.from_ical(ics_path.read_bytes())

def get_datetime_start(event):
	if event.start:
		return datetime.datetime.fromisoformat(event.start)
	

def get_datetime_end(event):
	if event.end:
		d = datetime.datetime.fromisoformat(event.end)
		if isinstance(event.end, datetime.date):
			d.hour = 23
			d.minute = 59
		return d

def main():
	categories = api.get_categories()
	courses = api.get_courses()
	states = api.get_states()
	calendar = get_calendar()


	category_ids = [None, *map(lambda x: x['id'], categories)]
	category_names = list(map(lambda x: x['name'], categories))
	course_ids = [None, *map(lambda x: x['id'], courses)]
	course_names = ['None', *map(lambda x: x['name'], courses)]
	state_ids = [None, *map(lambda x: x['id'], states)]
	state_names = ['None', *map(lambda x: x['name'], states)]

	base_questions = build_base_questions(category_names, course_names, state_names)


	for event in calendar.events:
		print('Event:', event.get('SUMMARY'))
		answer = inquirer.prompt([inquirer.List('action', message='What to do?', choices=['Process', 'Skip', 'End'])])

		match answer:
			case "Skip":
				continue
			case "End":
				break

		questions = [
			inquirer.Text('name', message='What name should it have?', default=event.get('SUMMARY')),
			*base_questions,
		]
		answers = inquirer.prompt(questions)
		print(answers)
		if answers.has_time:
			start = get_datetime_start(event)
			start_time = inquirer.prompt([inquirer.Text('start', message='Start time: ', default=start.isoformat()),])
		break

if __name__ == "__main__":
	main()
