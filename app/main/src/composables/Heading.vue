<script setup lang="ts">
import { PropType, ref } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	},
	openUrl: {
		type: Function as PropType<(url: string) => void>,
		required: true
	}
});

const emit = defineEmits(['openSettings']);
const editingName = ref(false);
const draftName = ref('');
const releasePageUrl = 'https://github.com/EladBG-code/rquickshare-pi/releases';

function formatVersion(version?: string | null) {
	if (!version) return '';

	const alpha = version.match(/^(\d+\.\d+\.\d+)-alpha(?:\.\d+)?$/);
	if (alpha) return `v${alpha[1]} alpha`;

	return `v${version}`;
}

function startEditingName() {
	draftName.value = props.vm.hostname ?? '';
	editingName.value = true;
}

async function saveDeviceName() {
	const name = draftName.value.trim();
	if (!name) {
		editingName.value = false;
		return;
	}

	await props.vm.setDeviceName(props.vm, name);
	editingName.value = false;
}
</script>

<template>
	<div class="flex flex-row justify-between items-center px-6 py-4">
		<!-- Header, Pc name left and options right -->
		<div>
			<h4 class="text-md">
				Device name
			</h4>
			<div class="flex items-center gap-2">
				<input
					v-if="editingName"
					v-model="draftName"
					class="text-2xl font-medium bg-white bg-opacity-70 rounded-xl px-2 py-1 w-56 focus:outline-none"
					maxlength="64"
					autofocus
					@keyup.enter="saveDeviceName"
					@keyup.escape="editingName = false"
					@blur="saveDeviceName">
				<h2 v-else class="text-2xl font-medium">
					{{ vm.hostname }}
				</h2>
				<button
					type="button"
					class="btn px-2 rounded-xl active:scale-95 transition duration-150 ease-in-out"
					aria-label="Edit device name"
					title="Edit device name"
					@click="startEditingName">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="19" height="19">
						<path d="M3 17.25V21h3.75L17.8 9.95 14.05 6.2 3 17.25Zm17.7-10.2c.4-.4.4-1 0-1.4L18.35 3.3a1 1 0 0 0-1.4 0l-1.85 1.85 3.75 3.75 1.85-1.85Z" />
					</svg>
				</button>
			</div>
		</div>
		<div class="flex justify-center items-center gap-3">
			<div
				class="btn flex items-center gap-2 text-sm transition duration-150 ease-in-out active:scale-95"
				title="Open releases"
				@click="openUrl(vm.new_version ? (vm.latest_release_url ?? releasePageUrl) : releasePageUrl)">
				<span v-if="vm.new_version">Update available</span>
				<p>
					{{ formatVersion(vm.version) }}
				</p>
				<p v-if="vm.new_version" class="text-lg">
					→
				</p>
				<p v-if="vm.new_version">
					{{ formatVersion(vm.new_version) }}
				</p>
			</div>
			<button
				type="button"
				class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out"
				aria-label="Open GitHub repository"
				title="Open GitHub repository"
				@click="openUrl('https://github.com/EladBG-code/rquickshare-pi')">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 98 96" width="24" height="24">
					<path fill-rule="evenodd" clip-rule="evenodd" d="M49 0C22 0 0 22 0 49c0 22 14 41 34 47 3 0 4-1 4-3v-9c-14 3-17-6-17-6-2-6-5-7-5-7-5-3 0-3 0-3 5 0 8 5 8 5 4 8 12 6 15 4 0-3 2-6 3-7-11-1-22-5-22-24 0-5 2-10 5-13-1-1-2-6 0-13 0 0 4-1 13 5 4-1 7-2 11-2s8 1 12 2c9-6 13-5 13-5 2 7 1 12 0 13 3 3 5 8 5 13 0 19-11 23-22 24 2 2 4 6 4 11v15c0 2 1 3 4 3 20-6 33-25 33-47C98 22 76 0 49 0Z" />
				</svg>
			</button>
			<button
				type="button"
				class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out"
				aria-label="Open settings"
				title="Open settings"
				@click="emit('openSettings')">
				<svg
					xmlns="http://www.w3.org/2000/svg" height="24"
					viewBox="0 -960 960 960" width="24">
					<!-- eslint-disable-next-line -->
						<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
				</svg>
			</button>
		</div>
	</div>
</template>
