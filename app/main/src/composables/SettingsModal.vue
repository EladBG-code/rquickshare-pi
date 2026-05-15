<script setup lang="ts">
import { utils } from '../vue_lib';
import { PropType } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { open } from '@tauri-apps/plugin-shell';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emit = defineEmits(['close']);

function openDownloadPicker() {
	props.vm.dialogOpen({
		title: "Select the destination for files",
		directory: true,
		multiple: false,
	}).then(async (el) => {
		if (el === null) {
			return;
		}

		await utils.setDownloadPath(props.vm, el as string);
	});
}

function openUpstream() {
	void open('https://github.com/Martichou/rquickshare');
}

function openLicense() {
	void open('https://github.com/EladBG-code/rquickshare-pi/blob/master/LICENSE');
}

function openLegalNotice() {
	void open('https://github.com/EladBG-code/rquickshare-pi/blob/master/LEGAL_NOTICE.md');
}
</script>

<template>
	<div v-if="vm.settingsOpen" class="absolute z-10 w-full h-full flex justify-center items-center bg-black bg-opacity-25">
		<div class="bg-white rounded-xl shadow-xl p-4 w-[24rem] max-h-[85vh] overflow-y-auto">
			<div class="flex flex-row justify-between items-center">
				<h3 class="font-medium text-xl">
					Settings
				</h3>
				<div class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('close')">
					Close
				</div>
			</div>
			<div class="py-4 flex flex-col">
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center" @click="utils.setAutoStart(vm, !vm.autostart)">
						<span class="label-text">Start on boot</span>
						<input type="checkbox" :checked="vm.autostart" class="checkbox focus:outline-none">
					</label>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center" @click="utils.setRealClose(vm, !vm.realclose)">
						<span class="label-text">Keep running on close</span>
						<input type="checkbox" :checked="!vm.realclose" class="checkbox focus:outline-none">
					</label>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center" @click="utils.setStartMinimized(vm, !vm.startminimized)">
						<span class="label-text">Start minimized</span>
						<input type="checkbox" :checked="vm.startminimized" class="checkbox focus:outline-none">
					</label>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-col items-start" @click="openDownloadPicker()">
						<span class="">Change download folder</span>
						<span class="overflow-hidden whitespace-nowrap text-ellipsis text-xs max-w-80">
							> {{ vm.downloadPath ?? 'OS User\'s download folder' }}
						</span>
					</label>
				</div>
				<details class="border-t border-gray-200 mt-3 pt-3 px-3 text-xs leading-relaxed text-gray-600 group">
					<summary class="btn rounded-xl !justify-between px-3 py-2 cursor-pointer list-none">
						<span>Credits & license</span>
						<svg
							xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" width="18" height="18"
							class="transition-transform duration-150 group-open:rotate-90">
							<path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
						</svg>
					</summary>
					<div class="pt-3">
						<p>
							Based on
							<button
								type="button"
								class="text-blue-600 hover:underline"
								@click="openUpstream">
								RQuickShare
							</button>
							by Martin ANDRE and contributors.
						</p>
						<p class="mt-2">
							This fork preserves the upstream GPL-3.0 license, copyright notices, credits, and project history.
							Original RQuickShare copyrights remain with their respective authors.
						</p>
						<p class="mt-2">
							© EladBG for RQuickShare Pi changes. This software is provided without warranty under the
							<button
								type="button"
								class="text-blue-600 hover:underline"
								@click="openLicense">
								GPL-3.0 license
							</button>.
						</p>
						<p class="mt-2">
							RQuickShare Pi is an independent community fork. It is not affiliated with, endorsed by,
							sponsored by, or officially supported by Google, Android, Samsung, Ko-fi, or the original
							RQuickShare maintainers.
						</p>
						<p class="mt-2">
							Quick Share, Android, Samsung, Ko-fi, and related names are used only to describe
							compatibility, platform behavior, or the donation link. All trademarks belong to their
							respective owners.
						</p>
						<button
							type="button"
							class="mt-2 text-blue-600 hover:underline"
							@click="openLegalNotice">
							Full legal notice
						</button>
					</div>
				</details>
			</div>
		</div>
	</div>
</template>
