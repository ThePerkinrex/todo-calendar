import requests, json, pathlib, datetime

BASE_URL = 'http://localhost:5010/api/v1'
CACHE_LOCATION = 'cache.json'


def get_json(path):
	r =  requests.get(BASE_URL + path)
	r.raise_for_status()
	return r.json()
class Cache:
	def __init__(self, location = CACHE_LOCATION):
		self.p = pathlib.Path(location)
		if self.p.exists():
			with open(self.p) as f:
				self.data = json.load(f)
		else:
			self.data = {}
	
	def save(self):
		with open(self.p, 'w') as f:
			json.dump(self.data, f, indent='\t', sort_keys=True)
	
	def get(self, path, getter = get_json):
		str_path = str(path)
		if str_path not in self.data or self.data[str_path]['time'] < datetime.datetime.now().timestamp() - 15 * 60:
			self.data[str_path] = {
				'time': datetime.datetime.now().timestamp(),
				'data': getter(path)
			}
			self.save()
		return self.data[str_path]['data']

CACHE = Cache()

def get_colors():
	print('Getting colors')
	try:
		return CACHE.get('/colors/')
	except Exception as e:
		print("Error: " + e)
		return [
				{
					"id": 1,
					"name": "Black",
					"fallback": "#000000"
				},
				{
					"id": 2,
					"name": "Charcoal",
					"fallback": "#233d4d"
				},
				{
					"id": 3,
					"name": "Electric violet",
					"fallback": "#8a00ff"
				},
				{
					"id": 4,
					"name": "Fuchsia",
					"fallback": "#ff38f3"
				},
				{
					"id": 5,
					"name": "Gold",
					"fallback": "#ffd700"
				},
				{
					"id": 6,
					"name": "Neon green",
					"fallback": "#5cff38"
				},
				{
					"id": 7,
					"name": "Olivine",
					"fallback": "#a1c181"
				},
				{
					"id": 8,
					"name": "Picton Blue",
					"fallback": "#38b6ff"
				},
				{
					"id": 9,
					"name": "Pumpkin",
					"fallback": "#fe7f2d"
				},
				{
					"id": 10,
					"name": "Sunglow",
					"fallback": "#fcca46"
				},
				{
					"id": 11,
					"name": "White",
					"fallback": "#ffffff"
				},
				{
					"id": 12,
					"name": "Zomp",
					"fallback": "#619b8a"
				},
				{
					"id": 13,
					"name": "orange",
					"fallback": "#F3722C"
				},
				{
					"id": 14,
					"name": "red",
					"fallback": "#F94144"
				},
				{
					"id": 15,
					"name": "turquoise",
					"fallback": "#43AA8B"
				},
				{
					"id": 16,
					"name": "yellow",
					"fallback": "#F9C74F"
				}
			]
	
def get_courses():
	print('Getting courses')
	try:
		return CACHE.get('/courses/')
	except Exception as e:
		print("Error: " + e)
		return [{"id":1,"name":"Autonomous Racing Cars","color":2},{"id":2,"name":"Distributed System Technologies","color":7},{"id":3,"name":"TFG","color":9},{"id":4,"name":"Web Engineering","color":15}]

def get_categories():
	print('Getting categories')
	try:
		return CACHE.get('/categories/')
	except Exception as e:
		print("Error: " + e)
		return [
        {
            "id": 1,
            "name": "Assignment",
            "color": 4
        },
        {
            "id": 2,
            "name": "Event",
            "color": 3
        },
        {
            "id": 3,
            "name": "Exam",
            "color": 8
        },
        {
            "id": 4,
            "name": "Presentation",
            "color": 5
        },
        {
            "id": 5,
            "name": "Task",
            "color": 6
        }
    ]


def get_states():
	print('Getting states')
	try:
		return CACHE.get('/states/')
	except Exception as e:
		print("Error: " + e)
		return [
        {
            "id": 1,
            "name": "Done",
            "color": 12
        },
        {
            "id": 2,
            "name": "In Progress",
            "color": 16
        },
        {
            "id": 3,
            "name": "To-Do",
            "color": 14
        }
    ]