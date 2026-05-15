const tabs = Array.from(document.querySelectorAll('.wiki-tab'));
const pages = Array.from(document.querySelectorAll('.wiki-page'));
const search = document.querySelector('#wikiSearch');

function showTopic(topic) {
	tabs.forEach((tab) => tab.classList.toggle('active', tab.dataset.topic === topic));
	pages.forEach((page) => page.classList.toggle('active', page.dataset.topic === topic));
}

tabs.forEach((tab) => {
	tab.addEventListener('click', () => {
		search.value = '';
		showTopic(tab.dataset.topic);
	});
});

search.addEventListener('input', () => {
	const query = search.value.trim().toLowerCase();

	if (!query) {
		showTopic(document.querySelector('.wiki-tab.active')?.dataset.topic ?? tabs[0].dataset.topic);
		return;
	}

	const match = pages.find((page) => {
		const haystack = `${page.dataset.keywords ?? ''} ${page.textContent}`.toLowerCase();
		return haystack.includes(query);
	});

	if (match) {
		showTopic(match.dataset.topic);
	}
});

const observer = new IntersectionObserver((entries) => {
	entries.forEach((entry) => {
		if (entry.isIntersecting) {
			entry.target.classList.add('seen');
		}
	});
}, { threshold: 0.14 });

document.querySelectorAll('section, article').forEach((item) => observer.observe(item));
