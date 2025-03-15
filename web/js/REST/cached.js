export function cached(fetch) {
	let data = {};
	async function load(...args) {
		const hashed = JSON.stringify(args)
		if (data[hashed] === undefined) {
			data[hashed] = fetch(...args)
		}
		return data[hashed]
	}
	function clear() {
		data = undefined
	}
	return [load, clear]
}