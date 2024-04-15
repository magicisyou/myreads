<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Book from "./lib/Book.svelte";
  import TopBar from "./lib/TopBar.svelte";
  import SearchIndicator from "./lib/SearchIndicator.svelte";
  import { books_list } from "./lib/stores.ts";

  async function get_books() {
    let result = await invoke("get_books");
    if (result) books_list.update((value) => (value = result));
  }
  get_books();
</script>

<div>
  <TopBar />
  <SearchIndicator />
  {#each $books_list as book}
    <Book {...book} />
  {/each}
</div>

<style scoped>
  div {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    flex-flow: wrap;
    gap: 10px;
    width: 100%;
  }
</style>
