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

const copyIcon = `
	<svg viewBox="0 0 24 24" aria-hidden="true">
		<rect x="8" y="8" width="10" height="10" rx="2"></rect>
		<path d="M6 16H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
	</svg>
`;
const copiedIcon = `
	<svg viewBox="0 0 24 24" aria-hidden="true">
		<path d="m5 12 4 4L19 6"></path>
	</svg>
`;

async function copyText(text) {
	if (navigator.clipboard?.writeText) {
		await navigator.clipboard.writeText(text);
		return;
	}

	const textarea = document.createElement('textarea');
	textarea.value = text;
	textarea.setAttribute('readonly', '');
	textarea.className = 'clipboard-fallback';
	document.body.appendChild(textarea);
	textarea.select();
	document.execCommand('copy');
	textarea.remove();
}

document.querySelectorAll('pre').forEach((block) => {
	const code = block.querySelector('code');
	const button = document.createElement('button');
	button.type = 'button';
	button.className = 'copy-code';
	button.setAttribute('aria-label', 'Copy command');
	button.innerHTML = `${copyIcon}<span>Copy</span>`;
	block.appendChild(button);

	button.addEventListener('click', async () => {
		try {
			await copyText((code ?? block).textContent.trim());
			button.classList.add('copied');
			button.setAttribute('aria-label', 'Copied');
			button.innerHTML = `${copiedIcon}<span>Copied</span>`;
			window.setTimeout(() => {
				button.classList.remove('copied');
				button.setAttribute('aria-label', 'Copy command');
				button.innerHTML = `${copyIcon}<span>Copy</span>`;
			}, 1450);
		} catch {
			button.classList.add('copy-failed');
			button.querySelector('span').textContent = 'Error';
			window.setTimeout(() => {
				button.classList.remove('copy-failed');
				button.querySelector('span').textContent = 'Copy';
			}, 1450);
		}
	});
});
