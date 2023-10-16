<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let img_src: string;

  const fetchContent = async () => {
    const res = await invoke("cbz_page", {
      fileLocation: "F:/projects/lgtm-reader/src-tauri/library/test.cbz",
      page: 0,
    });

    img_src = "data:image/png;base64," + res;

    console.log(res);
  };
</script>

<div>
  <!-- TODO: page view transition -->
  card <button on:click={fetchContent}> test </button>
  <img src={img_src} alt="" />
</div>
