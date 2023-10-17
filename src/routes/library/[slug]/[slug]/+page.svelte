<script>
	/** @type {import('./$types').PageData} */
	export let data;
	const { img, pages, slug } = data;
	const current_index = pages.indexOf(slug);
</script>

<div class="flex justify-center">
	<a href={location.href.replace(/\/[^\/]*$/, '/' + pages[current_index + 1])} target="_self">
		<img class="max-h-screen aspect-auto" src={'data:image/png;base64,' + img} alt="" />
	</a>
</div>
