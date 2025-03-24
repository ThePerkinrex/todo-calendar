import datetime
import inquirer
from zoneinfo import ZoneInfo

def get_tz():
	return datetime.datetime.now(datetime.timezone.utc).astimezone().tzinfo

def get_datetime(data, def_time):
	if data:
		date = None
		if isinstance(data, datetime.datetime):
			date = data
		else:
			date = datetime.datetime.combine(data, def_time).astimezone()
		return date

def get_datetime_start(event, time = None):
	return get_datetime(event.start, datetime.time(00,00) if time is None else time)
	

def get_datetime_end(event):
	return get_datetime(event.end, datetime.time(23,59))

def edit_time(time):
	while True:
		action = inquirer.prompt([inquirer.List('action', message='What to do?', choices=['Nothing', 'Change TZ', 'Edit', 'Change to start of day', 'Change to end of day'])])['action']
		if action == 'Nothing':
			break
		if action == 'Change TZ':
			current_tz = get_tz()
			current_tz_name = str(current_tz)
			try:
				current_tz_name = current_tz.tzname(time)
			except:
				pass
			tz_1 = inquirer.prompt([inquirer.List('tz', message='Interpret as what timezone?', choices=[current_tz_name, 'UTC', 'Other'])])['tz']
			to_tz = None
			if tz_1 == current_tz_name:
				to_tz = current_tz
			elif tz_1 == 'UTC':
				to_tz = datetime.timezone(0, 'UTC')
			if to_tz is None:
				tz_2 = inquirer.prompt([inquirer.Text('tz', message='IANA TZ name?')])['tz']
				try:
					to_tz = ZoneInfo(tz_2)
				except ValueError:
					print('Invalid tz')
					continue
			time = datetime.datetime.combine(time.date(), time.time(), tzinfo=to_tz)
		elif action == 'Edit':
			new_time = inquirer.prompt([inquirer.Text('t', message='Time in ISO Format (Example: ' + datetime.datetime.now().now(datetime.timezone.utc).astimezone().isoformat() + ')', default=time.isoformat())])['t']
			time = datetime.datetime.fromisoformat(new_time)
		elif action == 'Change to start of day':
			time = datetime.datetime.combine(time.date(), datetime.time(00,00), time.tzinfo)
		elif action == 'Change to end of day':
			time = datetime.datetime.combine(time.date(), datetime.time(23,59), time.tzinfo)
		
		print('New time:', time.astimezone())
	return time

def inquire_times(event, answers):
	start = get_datetime_start(event, datetime.time(23,59) if answers['category'] == 'Assignment' or answers['category'] == 'Task' else None)
	print('Start time, or deadline time:', start.astimezone())
	start = edit_time(start)

	has_end = event.end is not None
	if answers['category'] == 'Assignment' or answers['category'] == 'Task':
		has_end = False

	has_end = inquirer.prompt([inquirer.Confirm('has_end', message='Has end?', default=has_end)])['has_end']
	end = None
	if has_end:
		end = get_datetime_end(event)
		if end is None:
			new_time = inquirer.prompt([inquirer.Text('t',message='Time in ISO Format (Example: ' + datetime.datetime.now().now(datetime.timezone.utc).astimezone().isoformat() + ')')])['t']
			end = datetime.datetime.fromisoformat(new_time)
		else:
			print('End time:', end.astimezone())
			end = edit_time(end)

	return {'start': start.isoformat(), 'end': None if end is None else end.isoformat()}


	# start_time = inquirer.prompt([inquirer.Text('start', message='Start time: ', default=start.isoformat()),])
