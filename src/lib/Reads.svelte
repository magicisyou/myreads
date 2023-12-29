<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Book from "./Book.svelte";
  import AddBook from "./AddBook.svelte";
  import { books_list } from "./stores.ts";

  let books = [];

  async function get_books() {
    let result = await invoke("get_books");
    if (result) {
      books = result;
    }
  }
  get_books();
</script>

<div>
  <AddBook />
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
