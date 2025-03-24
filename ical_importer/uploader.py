from api import BASE_URL
from tasks import Tasks
import requests
import datetime

def add_time(time):
	body = {'start': datetime.datetime.fromisoformat(time['start']).astimezone(datetime.timezone.utc).isoformat()}
	if 'end' in time and time['end'] is not None:
		body['end'] = datetime.datetime.fromisoformat(time['end']).astimezone(datetime.timezone.utc).isoformat()
	res = requests.post(BASE_URL + '/times/', json=body)
	res.raise_for_status()
	return res.json()

def add_task(task, time_id):
	body = {'name': task['name'], 'course': task['course'], 'category': task['category'], 'state': task['state']}
	if time_id is not None:
		body['time'] = time_id
	res = requests.post(BASE_URL + '/tasks/', json=body)
	res.raise_for_status()
	return res.json()

def main():
	tasks = Tasks()

	for task_id in tasks.ids():
		print(task_id)
		v = tasks.get(task_id)
		if 'id' not in v:
			time = v['time']
			time_id = None
			if time is not None and 'id' not in time:
				print('add time')
				time_id = add_time(time)
				time['id'] = time_id
			print('add task')
			task_id = add_task(v, time_id)
			v['id'] = task_id
			tasks.save()

if __name__ == '__main__':
	main()