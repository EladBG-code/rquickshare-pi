import { devtools } from '@vue/devtools'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { confirm } from '@tauri-apps/plugin-dialog'
import { open } from '@tauri-apps/plugin-shell'

import './vue_lib/assets/main.postcss'

import App from './App.vue'

const issueUrl = 'https://github.com/EladBG-code/rquickshare-pi/issues/new?template=bug_report.yml';

async function reportClientError(kind: string, value: unknown) {
	try {
		const message = value instanceof Error ? `${value.name}: ${value.message}\n${value.stack ?? ''}` : String(value);
		const path = await invoke('write_error_report', {
			message: `${kind}\n\n${message}\n\nUser agent:\n${navigator.userAgent}`,
		});
		const shouldOpenIssue = await confirm(
			`RQuickShare Pi saved an error report:\n\n${path}\n\nOpen GitHub Issues so you can upload it?`,
			{
				title: 'RQuickShare Pi error report',
				kind: 'error',
				okLabel: 'Open issues',
				cancelLabel: 'Later',
			}
		);

		if (shouldOpenIssue) await open(issueUrl);
	} catch (err) {
		console.error('Failed to write RQuickShare Pi error report', err);
	}
}

window.addEventListener('error', (event) => {
	void reportClientError('window.error', event.error ?? event.message);
});

window.addEventListener('unhandledrejection', (event) => {
	void reportClientError('unhandledrejection', event.reason);
});

if (process.env.NODE_ENV === 'development') {
	devtools.connect('http://localhost', 8098)
}

const pinia = createPinia();
const app = createApp(App)

app.use(pinia);

app.mount('#app')
