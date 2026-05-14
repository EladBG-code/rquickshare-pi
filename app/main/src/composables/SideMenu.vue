<script setup lang="ts">
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { PropType } from 'vue';
import { open } from '@tauri-apps/plugin-shell';
import kofiIcon from '../vue_lib/assets/icons8-ko-fi-96.png';

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
	            items-center rounded-xl active:scale-95 transition duration-150 ease-in-out !px-3" @click="emits('invertVisibility')">
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
					class="btn h-9 w-9 !p-1.5 rounded-xl active:scale-95 transition duration-150 ease-in-out"
					aria-label="Donate on Ko-fi"
					title="Donate on Ko-fi"
					@click="openKofi">
					<img :src="kofiIcon" alt="" class="h-full w-full object-contain" draggable="false">
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
