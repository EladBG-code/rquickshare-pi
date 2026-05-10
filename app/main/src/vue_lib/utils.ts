import { Visibility } from '@martichou/core_lib/bindings/Visibility';
import { TauriVM } from './helper/ParamsHelper';
import { autostartKey, DisplayedItem, downloadPathKey, lastErrorReportPathKey, numberToVisibility, realcloseKey, startminimizedKey, stateToDisplay, visibilityKey, visibilityToNumber } from './types';
import { SendInfo } from '@martichou/core_lib/bindings/SendInfo';
import { ChannelMessage } from '@martichou/core_lib/bindings/ChannelMessage';
import { ChannelAction } from '@martichou/core_lib';
import { confirm } from '@tauri-apps/plugin-dialog';
import { open } from '@tauri-apps/plugin-shell';
import { gt, valid } from 'semver';

const repoUrl = 'https://github.com/EladBG-code/rquickshare-pi';
const releasesApiUrl = `${repoUrl.replace('https://github.com/', 'https://api.github.com/repos/')}/releases?per_page=30`;
const latestReleaseUrl = `${repoUrl}/releases/latest`;
const issueUrl = `${repoUrl}/issues/new?template=bug_report.yml`;

function _displayedItems(vm: TauriVM): Array<DisplayedItem> {
	const ndisplayed = new Array<DisplayedItem>();

	vm.endpointsInfo.forEach((el) => {
		const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
		if (idx !== -1) return;

		ndisplayed.push({
			id: el.id,
			name: el.name ?? 'Unknown',
			deviceType: el.rtype ?? 'Unknown',
			endpoint: true,
		})
	});

	vm.requests.filter((el) => stateToDisplay.includes(el.state ?? 'Initial')).forEach((el) => {
		const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
		const elem: DisplayedItem = {
			id: el.id,
			name: el.meta?.source?.name ?? 'Unknown',
			deviceType: el.meta?.source?.device_type ?? 'Unknown',
			endpoint: false,

			state: el.state ?? undefined,
			pin_code: el.meta?.pin_code ?? undefined,
			destination: el.meta?.destination ?? undefined,
			files: el.meta?.files ?? undefined,
			text_description: el.meta?.text_description ?? undefined,
			text_payload: el.meta?.text_payload ?? undefined,
			text_type: el.meta?.text_type ?? undefined,
			ack_bytes: (el.meta?.ack_bytes as number | undefined) ?? undefined,
			total_bytes: (el.meta?.total_bytes as number | undefined) ?? undefined,
		};

		if (idx !== -1) {
			ndisplayed.splice(idx, 1, elem);
		} else {
			ndisplayed.push(elem)
		}
	});

	return ndisplayed;
}

async function setAutoStart(vm: TauriVM, autostart: boolean) {
	if (autostart) {
		await vm.enable();
	} else {
		await vm.disable();
	}

	await vm.store.set(autostartKey, autostart);
	await vm.store.save();
	vm.autostart = autostart;
}

async function applyAutoStart(vm: TauriVM) {
	vm.autostart = await vm.store.get(autostartKey) ?? false;

	if (vm.autostart) {
		await vm.enable();
	} else {
		await vm.disable();
	}
}

async function setRealClose(vm: TauriVM, realclose: boolean) {
	await vm.store.set(realcloseKey, realclose);
	await vm.store.save();
	vm.realclose = realclose;
}

async function getRealclose(vm: TauriVM) {
	vm.realclose = await vm.store.get(realcloseKey) ?? false;
}

async function setStartMinimized(vm: TauriVM, startminimized: boolean) {
	await vm.store.set(startminimizedKey, startminimized);
	await vm.store.save();
	vm.startminimized = startminimized;
}

async function getStartMinimized(vm: TauriVM) {
	vm.startminimized = await vm.store.get(startminimizedKey) ?? false;
}

async function setVisibility(vm: TauriVM, visibility: Visibility) {
	await vm.invoke('change_visibility', { message: visibility });
	await vm.store.set(visibilityKey, visibilityToNumber[visibility]);
	await vm.store.save();
	vm.visibility = visibility;
}

async function getVisibility(vm: TauriVM) {
	vm.visibility = numberToVisibility[(await vm.store.get(visibilityKey) ?? 0) as number];
}

async function invertVisibility(vm: TauriVM) {
	if (vm.visibility === 'Temporarily') {
		return;
	}

	if (vm.visibility === 'Visible') {
		return await vm.setVisibility(vm, 'Invisible');
	}

	return await vm.setVisibility(vm, 'Visible');
}

async function clearSending(vm: TauriVM, ) {
	await vm.invoke('stop_discovery');
	vm.outboundPayload = undefined;
	vm.discoveryRunning = false;
	vm.endpointsInfo = [];
}

function removeRequest(vm: TauriVM, id: string) {
	const idx = vm.requests.findIndex((el) => el.id === id);

	if (idx !== -1) {
		vm.requests.splice(idx, 1);
	}
}

async function sendInfo(vm: TauriVM, eid: string) {
	if (vm.outboundPayload === undefined) return;

	const ei = vm.endpointsInfo.find((el) => el.id === eid);
	if (!ei || !ei.ip || !ei.port) return;

	const msg: SendInfo = {
		id: ei.id,
		name: ei.name ?? 'Unknown',
		addr: ei.ip + ":" + ei.port,
		ob: vm.outboundPayload,
	};

	await vm.invoke('send_payload', { message: msg });
}

async function sendCmd(vm: TauriVM, id: string, action: ChannelAction) {
	const cm: ChannelMessage = {
		id: id,
		direction: 'FrontToLib',
		action: action,
		meta: null,
		state: null,
		rtype: null,
	};
	console.log("js2rs:", cm);

	await vm.invoke('send_to_rs', { message: cm });
}

function blured() {
	(document.activeElement as any).blur();
}

function getProgress(item: DisplayedItem): string {
	const value = item.ack_bytes! / item.total_bytes! * 100;
	return `--progress: ${value}`;
}

function normalizeVersion(version?: string | null): string | null {
	if (!version) return null;

	const normalized = version.trim().replace(/^v/i, '').replace(/\s+/g, '-');
	return valid(normalized);
}

function formatUpdateVersion(version: string): string {
	const alpha = version.match(/^(\d+\.\d+\.\d+)-alpha(?:\.\d+)?$/);
	if (alpha) return `v${alpha[1]} alpha`;

	return `v${version}`;
}

function getHighestRelease(releases: unknown): { version: string, url: string } | null {
	if (!Array.isArray(releases)) return null;

	return releases.reduce<{ version: string, url: string } | null>((latest, release) => {
		if (!release || typeof release !== 'object' || 'draft' in release && release.draft) return latest;

		const tagName = 'tag_name' in release && typeof release.tag_name === 'string' ? release.tag_name : null;
		const version = normalizeVersion(tagName);
		if (!version) return latest;

		const url = 'html_url' in release && typeof release.html_url === 'string' ? release.html_url : latestReleaseUrl;
		if (!latest || gt(version, latest.version)) return { version, url };

		return latest;
	}, null);
}

async function setDownloadPath(vm: TauriVM, dest: string) {
	await vm.invoke('change_download_path', { message: dest });
	await vm.store.set(downloadPathKey, dest);
	await vm.store.save();
	vm.downloadPath = dest;
}

async function getDownloadPath(vm: TauriVM) {
	vm.downloadPath = await vm.store.get(downloadPathKey) ?? undefined;
}

async function setDeviceName(vm: TauriVM, deviceName: string) {
	const normalized = await vm.invoke('change_device_name', { message: deviceName });
	vm.hostname = normalized as string;
}

async function promptLatestErrorReport(vm: TauriVM) {
	const path = await vm.invoke('get_latest_error_report') as string | null;
	if (!path || await vm.store.get(lastErrorReportPathKey) === path) return;

	await vm.store.set(lastErrorReportPathKey, path);
	await vm.store.save();

	const shouldOpenIssue = await confirm(
		`RQuickShare Pi found an error report:\n\n${path}\n\nOpen GitHub Issues so you can upload it?`,
		{
			title: 'RQuickShare Pi error report',
			kind: 'warning',
			okLabel: 'Open issues',
			cancelLabel: 'Later',
		}
	);

	if (shouldOpenIssue) await open(issueUrl);
}

async function getLatestVersion(vm: TauriVM, prompt = false) {
	try {
		const response = await fetch(releasesApiUrl, {
			headers: {
				Accept: 'application/vnd.github+json',
			},
		});
		if (!response.ok) {
			throw new Error(`Error: ${response.status} ${response.statusText}`);
		}
		const latestRelease = getHighestRelease(await response.json());
		const currentVersion = normalizeVersion(vm.version);
		const newVersion = latestRelease?.version ?? null;
		const releaseUrl = latestRelease?.url ?? latestReleaseUrl;

		console.log(`Latest version: ${currentVersion} vs ${newVersion}`);

		if (!currentVersion || !newVersion || !gt(newVersion, currentVersion)) {
			vm.new_version = null;
			vm.latest_release_url = null;
			return;
		}

		vm.new_version = newVersion;
		vm.latest_release_url = releaseUrl;

		if (!prompt) return;

		const shouldUpdate = await confirm(
			`RQuickShare Pi ${formatUpdateVersion(newVersion)} is available.\n\nYou are running ${formatUpdateVersion(currentVersion)}. Open GitHub to download the latest release?`,
			{
				title: 'RQuickShare Pi update available',
				kind: 'info',
				okLabel: 'Open update',
				cancelLabel: 'Later',
			}
		);

		if (shouldUpdate) {
			await open(releaseUrl);
		}
	} catch (err) {
		console.error(err);
	}
}

// Default export
export const utils = {
	_displayedItems,
	setAutoStart,
	applyAutoStart,
	setRealClose,
	getRealclose,
	setVisibility,
	getVisibility,
	invertVisibility,
	clearSending,
	removeRequest,
	sendInfo,
	sendCmd,
	blured,
	getProgress,
	setDownloadPath,
	getDownloadPath,
	setDeviceName,
	promptLatestErrorReport,
	getLatestVersion,
	setStartMinimized,
	getStartMinimized
};
export type UtilsType = typeof utils;
