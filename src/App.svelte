<script lang="ts">
  import ThumbnailCard from "./lib/thumbnail-card.svelte";
  import Test from "./lib/test.svelte";

  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";
  const entries = readDir("library", {
    recursive: true,
  });
  console.log(entries);
  let files = [];
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
