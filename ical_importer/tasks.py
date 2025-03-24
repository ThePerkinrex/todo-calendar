import json, os.path

TASKS_FILE = 'tasks.json'

class Tasks:
	def __init__(self):
		self.tasks = {}
		if os.path.exists(TASKS_FILE):
			with open(TASKS_FILE) as f:
				self.tasks = json.load(f)
	
	def __contains__(self, id):
		return str(id) in self.tasks
	
	def get(self, id):
		return self.tasks[id]
	
	def add(self, id, task, times = None):
		task['time'] = times
		self.tasks[str(id)] = task
	
	def ids(self):
		return self.tasks.keys()

	def save(self):
		with open(TASKS_FILE, 'w') as f:
			json.dump(self.tasks, f, indent='\t', sort_keys=True)