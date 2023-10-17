<script>
	/** @type {import('./$types').PageData} */
	export let data;
	const { info, thumbnail, pages } = data;
</script>

<div class="flex">
	<img class="max-w-[14rem] aspect-auto" src={thumbnail} alt="" />
	<div>
		<div>{@html info.Title}</div>
		<div>{@html info.Description}</div>
		<div>{@html info.Artist}</div>
		<div>{@html info.Magazine}</div>
		<div>{@html info.Pages}</div>
		<div>{@html info.Parody}</div>
		<div>{@html info.Tags}</div>
		<div><a href={info.URL}>{@html info.URL}</a></div>
	</div>
</div>
<div class="flex flex-wrap">
	{#each pages as page}
		<a href={location.href + '/01.png'} target="_self">
			<img class="max-w-[10rem] m-1 aspect-auto" src={'data:image/png;base64,' + page} alt="" />
		</a>
	{/each}
</div>
