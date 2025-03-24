export function constructAllTabbed() {
	for(const e of document.getElementsByClassName('tabbed')) {
		buildTabs(e)
	}
}



function buildTabs(element) {
	const tabTitles = element.querySelectorAll('.tab-title')
	const tabs = element.querySelectorAll('.tab')


	if(tabTitles.length !== tabs.length) throw new Error("Incorrectly setup tabbed structure");

	let openTab = 0;

	const setOpenTab = (n) => {
		tabTitles[openTab].classList.remove('tab-open')
		tabs[openTab].classList.remove('tab-open')
		openTab = n;
		tabTitles[openTab].classList.add('tab-open')
		tabs[openTab].classList.add('tab-open')
	}

	for(let i = 0; i < tabs.length; i++) {
		tabTitles[i].dataset.idx = i;
		tabs[i].dataset.idx = i;
		tabTitles[i].onclick = () => {
			setOpenTab(i)
		}
	}

	setOpenTab(0);
}