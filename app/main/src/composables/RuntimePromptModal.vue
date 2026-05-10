<script setup lang="ts">
import { computed, PropType } from 'vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { RuntimeStatus } from '../vue_lib/types';

const props = defineProps({
	status: {
		type: Object as PropType<RuntimeStatus | null>,
		required: false,
		default: null
	},
	open: {
		type: Boolean,
		required: true
	}
});

const emit = defineEmits(['close']);

const commandText = computed(() => {
	return props.status?.issues
		.map((issue) => issue.command)
		.filter((command): command is string => Boolean(command))
		.join('\n') ?? '';
});

async function copyCommands() {
	if (!commandText.value) return;
	await writeText(commandText.value);
}
</script>

<template>
	<div v-if="open && status && !status.ok" class="absolute z-20 w-full h-full flex justify-center items-center bg-black bg-opacity-25">
		<div class="bg-white rounded-xl shadow-xl p-4 w-[28rem] max-w-[calc(100%-2rem)]">
			<div class="flex flex-row justify-between items-center">
				<h3 class="font-medium text-xl">
					Pi setup needed
				</h3>
				<button type="button" class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('close')">
					Close
				</button>
			</div>

			<div class="py-4 flex flex-col gap-3">
				<p class="text-sm text-gray-600">
					RQuickShare Pi found a system setting that can block nearby discovery.
				</p>

				<div v-for="issue in status.issues" :key="issue.title" class="rounded-xl border border-rose-100 bg-rose-50 p-3">
					<h4 class="font-medium">
						{{ issue.title }}
					</h4>
					<p class="mt-1 text-sm text-gray-700">
						{{ issue.message }}
					</p>
					<code v-if="issue.command" class="mt-2 block rounded-lg bg-white px-3 py-2 text-xs text-gray-700">
						{{ issue.command }}
					</code>
				</div>

				<div class="flex justify-end gap-2">
					<button
						v-if="commandText"
						type="button"
						class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out"
						@click="copyCommands">
						Copy fix
					</button>
					<button type="button" class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('close')">
						Later
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
