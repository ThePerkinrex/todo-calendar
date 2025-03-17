export function cached(
	fetch,
	keyFn = (...args) => JSON.stringify(args),
	staleness = 5 * 60 * 1000 /* 5 minutes */
) {
	let data = {};
	async function load(...args) {
		let key = keyFn(...args);
		if (key === undefined) {
			const hashed = JSON.stringify(args);
			key = hashed;
		}
		const now = new Date().getTime();
		if (data[key] === undefined || data[key].time + staleness < now) {
			console.log("reloading", data, args, key);
			data[key] = { data: await fetch(...args), time: now };
		}
		return data[key].data;
	}
	function clear() {
		data = undefined;
	}
	return [load, clear];
}
