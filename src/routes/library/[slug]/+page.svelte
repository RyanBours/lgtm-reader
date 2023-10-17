<script>
	/** @type {import('./$types').PageData} */
	export let data;
</script>

<div>{@html data.Title}</div>
<div>{@html data.Description}</div>
<div>{@html data.Artist}</div>
<div>{@html data.Magazine}</div>
<div>{@html data.Pages}</div>
<div>{@html data.Parody}</div>
<div>{@html data.Tags}</div>
<div><a href={data.URL}>{@html data.URL}</a></div>
