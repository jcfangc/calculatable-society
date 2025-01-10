<template>
	<Combobox v-model="selectedPerson">
		<ComboboxInput
			@change="query = $event.target.value"
			class="combobox-input"
		/>

		<ComboboxOptions
			class="combobox-options"
			v-if="filteredPeople.length !== 0"
		>
			<ComboboxOption
				class="combobox-option"
				v-for="person in filteredPeople"
				:key="person"
				:value="person"
			>
				{{ person }}
			</ComboboxOption>
		</ComboboxOptions>
	</Combobox>
</template>

<script setup lang="ts">
	import { ref, computed } from "vue";
	import {
		Combobox,
		ComboboxInput,
		ComboboxOptions,
		ComboboxOption,
	} from "@headlessui/vue";

	const people = [
		"门前大桥下",
		"游过一群鸭",
		"快来快来数一数",
		"二四六七八",
		"嘎嘎嘎嘎",
		"真呀真多呀",
		"数不清",
		"数不清",
		"真呀真多呀",
		"数不清",
		"到底有多少鸭",
	];
	const selectedPerson = ref(people[0]);
	const query = ref("");

	const filteredPeople = computed(() =>
		query.value === ""
			? people
			: people.filter((person) => {
					return person
						.toLowerCase()
						.includes(query.value.toLowerCase());
			  })
	);
</script>

<style lang="scss">
	@use "@/assets/style/default/index.scss" as *;

	.combobox-input {
		@include default-config;
		cursor: pointer;
	}

	.combobox-options {
		@include default-config($layout-config: false, $effects-config: false);
		@include effects-config(
			$active-transform: none,
			$hover-border-color: transparent
		);
		@include layout-config(
			$flex-direction: column,
			$align-items: start,
			$justify-content: start,
			$gap: var(--spacing-sm),
			$overflow: auto
		);

		@include scrollbar-config;

		max-height: calc(var(--container-max-height) * 0.5);
	}

	.combobox-option {
		@include default-config;
		cursor: pointer;
	}
</style>
