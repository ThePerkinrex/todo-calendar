import { Select } from '/lib/select/lib.js'
import * as dataSources from './js/data_source.js'

export async function main() {
	for(const e of document.getElementsByClassName('html-select')) {
		const source = dataSources[e.dataset.source]
		new Select(e, await source())
	}
}
