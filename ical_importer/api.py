import requests

BASE_URL = 'http://localhost:5010/api/v1'

def get_json(path):
	r =  requests.get(BASE_URL + path)
	r.raise_for_status()
	return r.json()

def get_colors():
	print('Getting colors')
	try:
		return get_json('/colors/')
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
		return get_json('/courses/')
	except Exception as e:
		print("Error: " + e)
		return [{"id":1,"name":"Autonomous Racing Cars","color":2},{"id":2,"name":"Distributed System Technologies","color":7},{"id":3,"name":"TFG","color":9},{"id":4,"name":"Web Engineering","color":15}]

def get_categories():
	print('Getting categories')
	try:
		return get_json('/categories/')
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
		return get_json('/states/')
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