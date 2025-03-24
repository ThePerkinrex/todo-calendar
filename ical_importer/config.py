import json, os.path

CONFIG_PATH = "config.json"

class Config:
	def __init__(self):
		self.before = {}
		self.after = {}
		self.contains = {}
		self.refs = {}
		if os.path.exists(CONFIG_PATH):
			with open(CONFIG_PATH) as f:
				data = json.load(f)

				self.refs = data['refs']
				self.before = data['before']
				self.after = data['after']
				self.contains = data['contains']
	
	def _get_v(self, v):
		if 'ref' in v:
			return self.refs[v['ref']]
		return v
	
	def parse_info(self, name: str):
		name = name.strip()
		x = {'category': None, 'course': None, 'state': None}
		for k, v in self.before.items():
			if name.startswith(k):
				v = self._get_v(v)
				for otherk in x.keys():
					if otherk in v and x[otherk] is None:
						x[otherk] = v[otherk]
						name = name.removeprefix(k).strip()
		for k, v in self.after.items():
			if name.endswith(k):
				v = self._get_v(v)
				for otherk in x.keys():
					if otherk in v and x[otherk] is None:
						x[otherk] = v[otherk]
						name = name.removesuffix(k).strip()
		for k, v in self.contains.items():
			if k in name:
				v = self._get_v(v)
				for otherk in x.keys():
					if otherk in v and x[otherk] is None:
						x[otherk] = v[otherk]

		return {'expected': name, **x}