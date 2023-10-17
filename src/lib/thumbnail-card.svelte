<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	export let file;
	let img_src: string;

	
	const fetchThumbnail = async () => {
		const res = await invoke('cbz_thumbnail', {
			fileLocation: file.path
		});
		img_src = 'data:image/png;base64,' + res;
	};

	onMount(() => {
		fetchThumbnail();
	});
</script>

<div class="">
	<!-- TODO: page view transition -->
	<a href={'/' + file.path}>
		<img class="max-w-[12rem] aspect-auto" src={img_src} alt="" />
	</a>
</div>
