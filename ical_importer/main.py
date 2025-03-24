import icalendar
from pathlib import Path
import api
import inquirer
from tasks import Tasks
import hashlib, json
from config import Config

from dates import get_tz, inquire_times


def get_calendar():
	ics_path = Path("data/24-25 SS_88af9d2b57efa24721baaf64072b858600e830b02a82b5923874d3e460c0e0fc@group.calendar.google.com.ics")
	return icalendar.Calendar.from_ical(ics_path.read_bytes())

def process_event(config, tasks, id, event, category_ids, category_names, course_ids, course_names, state_ids, state_names):
	expectations = config.parse_info(event.get('SUMMARY'))

	questions = [
		inquirer.Text('name', message='What name should it have?', default=expectations['expected']),
		inquirer.List('category', message='What category does it belong to?', choices=category_names, default=expectations['category']),
		inquirer.List('course', message='What course does it belong to?', choices=course_names, default=expectations['course']),
		inquirer.List('state', message='What state is it in?', choices=state_names, default=expectations['state']),
		inquirer.Confirm('has_time', message='Has it got a time?', default = True)
	]
	answers = inquirer.prompt(questions)
	result = {
		'name': answers['name'],
		'category': category_ids[category_names.index(answers['category'])],
		'course': course_ids[course_names.index(answers['course'])],
		'state': state_ids[state_names.index(answers['state'])],
	}
	print(answers)
	print(result)
	times = None
	if answers['has_time']:
		times = inquire_times(event, answers)
		print(times)
	tasks.add(id, result, times)
	tasks.save()

def main():
	categories = api.get_categories()
	courses = api.get_courses()
	states = api.get_states()
	calendar = get_calendar()


	category_ids = list(map(lambda x: x['id'], categories))
	category_names = list(map(lambda x: x['name'], categories))
	course_ids = [None, *map(lambda x: x['id'], courses)]
	course_names = ['None', *map(lambda x: x['name'], courses)]
	state_ids = [None, *map(lambda x: x['id'], states)]
	state_names = ['None', *map(lambda x: x['name'], states)]

	config = Config()


	print("Working as if events are in tz:", get_tz())

	tasks = Tasks()
	processed = []

	for event in calendar.events:
		print()
		sid = str(event.get('SUMMARY'))+str(event.start)+str(event.end)
		id = hashlib.sha256(sid.encode()).hexdigest()
		print('Hash:', id, '(' + sid + ')')
		print('Event:', event.get('SUMMARY'))
		
		if id in tasks:
			processed.append(event)
			continue
		answer = inquirer.prompt([inquirer.List('action', message='What to do?', choices=['Process', 'Skip', 'End'])])['action']

		if answer == 'Skip':
			continue
		elif answer == 'End':
			break

		process_event(config, tasks, id, event, category_ids, category_names, course_ids, course_names, state_ids, state_names)

	print('Now, already processed events')
	for event in processed:
		print()
		sid = str(event.get('SUMMARY'))+str(event.start)+str(event.end)
		id = hashlib.sha256(sid.encode()).hexdigest()
		print('Hash:', id, '(' + sid + ')')
		print('Event:', event.get('SUMMARY'))
		
		print('Event already processed:', tasks.get(id)['name'])
		answer = inquirer.prompt([inquirer.List('action', message='What to do?', choices=['Reprocess', 'Skip', 'End'], default = 'Skip')])['action']

		if answer == 'Skip':
			continue
		elif answer == 'End':
			break

		process_event(config, tasks, id, event, category_ids, category_names, course_ids, course_names, state_ids, state_names)
	
	tasks.save()



if __name__ == "__main__":
	main()
