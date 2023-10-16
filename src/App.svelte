<script lang="ts">
  import ThumbnailCard from "./lib/thumbnail-card.svelte";
  import Test from "./lib/test.svelte";

  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";
  const entries = readDir("library", {
    recursive: true,
  });
  console.log(entries);

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
  {#each { length: 3 } as _, i}
    <ThumbnailCard />
  {/each}
  <br />
  <Test />
</main>
