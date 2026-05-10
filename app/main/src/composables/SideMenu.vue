<script setup lang="ts">
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { PropType } from 'vue';
import { open } from '@tauri-apps/plugin-shell';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emits = defineEmits(['invertVisibility', 'clearSending']);

const pluralize = (n: number, s: string) => n === 1 ? s : `${s}s`;

function openKofi() {
	void open('https://ko-fi.com/eladbg');
}
</script>

<template>
	<div class="w-72 p-3 flex flex-col min-h-full" v-if="props.vm.outboundPayload === undefined">
		<div>
			<p class="mt-4 mb-2 pt-3 px-3">
				Visibility state
			</p>
			<h4
				tabindex="0" role="button" class="btn font-medium flex flex-row !justify-between w-full
	            items-center rounded-xl active:scale-95 transition duration-150 ease-in-out p-3" @click="emits('invertVisibility')">
				<span v-if="props.vm.visibility === 'Visible'">Always visible</span>
				<span v-else-if="props.vm.visibility === 'Invisible'">Hidden from everyone</span>
				<span v-else>Temporarily visible</span>

				<svg
					xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
					:class="{'rotate-180': props.vm.visibility === 'Invisible'}">
					<path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
				</svg>
			</h4>
			<p class="text-xs mt-2 pb-3 px-3">
				<span v-if="props.vm.visibility === 'Visible'">
					Nearby devices can share files with you, but you'll always be
					notified and have to approve each transfer before receiving it.
				</span>
				<span v-else-if="props.vm.visibility === 'Invisible'">
					No one can see your device at the moment. However, keep in mind that if another
					device has saved yours before, it might still attempt to start a transfer with you.
					<br>
					<br>
					You will get a notification when someone nearby is sharing
					giving you the ability to become visible for 1 minute.
				</span>
				<span v-else>
					You are temporarily visible to everyone.
				</span>
			</p>
		</div>
		<div class="mt-auto px-3 pb-3">
			<div class="flex items-center justify-between border-t border-rose-100 pt-3 text-xs text-gray-600">
				<span>© EladBG, Pi fork</span>
				<button
					type="button"
					class="btn px-2 rounded-xl active:scale-95 transition duration-150 ease-in-out"
					aria-label="Donate on Ko-fi"
					title="Donate on Ko-fi"
					@click="openKofi">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="8 16 48 34" width="27" height="27">
						<path d="M14 18h33c4 0 7 3 7 7v3c0 5-4 9-9 9h-2c-3 7-10 11-20 11h-2C13 48 8 43 8 35V24c0-3 3-6 6-6Zm34 8v3c0 2-1 4-3 5h1c3 0 5-2 5-5v-3c0-2-2-4-4-4h-1c1 1 2 2 2 4ZM21 23c-5 0-9 4-9 9v3c0 6 4 9 10 9h2c10 0 16-5 16-14v-4c0-2-1-3-3-3H21Z" />
						<path d="M25 38c-6-4-10-8-10-12 0-3 2-5 5-5 2 0 4 1 5 3 1-2 3-3 5-3 3 0 5 2 5 5 0 4-4 8-10 12Z" />
					</svg>
				</button>
			</div>
		</div>
	</div>
	<div class="w-72 p-6 flex flex-col justify-between" v-else>
		<div>
			<p class="mt-4 mb-2">
				Sharing {{ props.vm.outboundPayload.Files.length }} {{ pluralize(props.vm.outboundPayload.Files.length, "file") }}
			</p>
			<div class="bg-white w-32 h-32 rounded-2xl mb-2 flex justify-center items-center">
				<svg
					xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
					class="w-8 h-8">
					<!-- eslint-disable-next-line -->
                    <path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" />
				</svg>
			</div>
			<p v-for="f in props.vm.outboundPayload.Files" :key="f" class="overflow-hidden whitespace-nowrap text-ellipsis">
				{{ f.split('/').pop() }}
			</p>

			<p class="text-xs mt-3">
				Make sure both devices are unlocked, close together, and have bluetooth turned on. Device you're sharing with need
				Quick Share turned on and visible to you.
			</p>
		</div>

		<p
			@click="emits('clearSending')"
			class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out w-fit">
			Cancel
		</p>
	</div>
</template>
