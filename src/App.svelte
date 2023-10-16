<script lang="ts">
  import ThumbnailCard from "./lib/thumbnail-card.svelte";
  import Test from "./lib/test.svelte";

  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";
  const entries = readDir("library", {
    recursive: true,
  });

  let files = [];

  entries.then((entries) => {
    files = [];
    const regex = /\.(cbz|cbr)$/;
    for (const entry of entries) {
      if (regex.test(entry.path)) {
        console.log(entry);
        files.push(entry);
      }
    }
  });

  import { watch, watchImmediate } from "tauri-plugin-fs-watch-api";

  const stopRawWatcher = watchImmediate(
    ["library"],
    (event) => {
      const { type, paths, attrs } = event;
      console.log(type, paths, attrs);
    },
    {}
  );
</script>

<main class="container">
  <div class="flex flex-wrap">
    {#each files as file}
      <ThumbnailCard {file} />
    {/each}
  </div>
  <br />
  <Test />
</main>
