<script lang="ts">
  import { books_list } from "./stores.ts";
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let author = "";

  async function add_book() {
    if (name != "" && author != "") {
      let result = await invoke("add_book_to_db", {
        book: name,
        author: author,
        readState: "NotRead",
      });
      if (result) {
        books_list.update((value) => result);
        name = "";
        author = "";
      }
    }
  }
</script>

<form class="main" on:submit|preventDefault={add_book}>
  <input type="text" bind:value={name} placeholder="book name" />
  <input type="text" bind:value={author} placeholder="author name" />
  <button type="submit">Add</button>
</form>

<style scoped>
  .main {
    width: 100%;
    display: flex;
    padding: 10px;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 10px;
  }
  button,
  input {
    border: solid #aaa 1px;
    padding: 10px;
  }
  input {
    width: max(250px, 25vw);
  }
  button {
    width: 80px;
  }
  @media (prefers-color-scheme: dark) {
    input,
    button {
      background-color: #111;
      color: #f2f2f2;
      border: solid #666 1px;
    }
  }
</style>
