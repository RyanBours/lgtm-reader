<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  const fetchContent = async () => {
    const res = await invoke("cbz_content", {
      fileLocation: "F:/projects/lgtm-reader/src-tauri/library/test.cbz",
    });

    console.log(res);
  };
</script>

<div>
  <!-- TODO: page view transition -->
  card <button on:click={fetchContent}> fetchContent </button>
</div>
