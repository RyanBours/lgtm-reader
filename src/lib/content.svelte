<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  type Info = {
    Artist: string[];
    Description: string;
    Pages: number;
    Parody: string[];
    Publisher: string[];
    Released: number;
    Tags: string[];
    Thumbnail: number;
    Title: string;
    URL: string;
  };

  let info: Info;

  const fetchContent = async () => {
    const res = await invoke<Info>("cbz_info", {
      fileLocation: "F:/projects/lgtm-reader/src-tauri/library/test.cbz",
    });

    info = res;

    console.log(res);
  };
</script>

<div>
  <!-- TODO: page view transition -->
  card <button on:click={fetchContent}> test </button>
  <div>
    {#if info}
      <div>
        <div>Artist: {info.Artist}</div>
        <div>Description: {info.Description}</div>
        <div>Pages: {info.Pages}</div>
        <div>Parody: {info.Parody}</div>
        <div>Publisher: {info.Publisher}</div>
        <div>Released: {info.Released}</div>
        <div>Tags: {info.Tags}</div>
        <div>Thumbnail: {info.Thumbnail}</div>
        <div>Title: {info.Title}</div>
        <div>URL: {info.URL}</div>
      </div>
    {/if}
  </div>
</div>
