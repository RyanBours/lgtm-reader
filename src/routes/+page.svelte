<script lang="ts">
	import ThumbnailCard from '$lib/thumbnail-card.svelte';
	import { readDir } from '@tauri-apps/api/fs';

	let files = [];

	const entries = readDir('library', {
		recursive: true
	});

	entries.then((entries) => {
		files = [];
		const regex = /\.(cbz|cbr)$/;

		for (const entry of entries) {
			if (regex.test(entry.path)) {
				files.push(entry);
			}
			if (entry.children) {
				for (const child of entry.children) {
					if (regex.test(child.path)) {
						files.push(child);
					}
				}
			}
		}
	});
</script>

<main class="container">
	<div class="flex flex-wrap">
		{#each files as file}
			<ThumbnailCard {file} />
		{:else}
			<p>No files found</p>
		{/each}
	</div>
</main>
