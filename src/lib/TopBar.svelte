<script lang="ts">
  import { books_list, search_keyword } from "./stores.ts";
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let author = "";
  let keyword = "";

  async function search() {
    search_keyword.set(keyword);
    if (keyword == "") {
      let result = await invoke("get_books");
      if (result) books_list.set(result);
    } else {
      let result = await invoke("search_books", { keyword: keyword });
      if (result) books_list.set(result);
    }
  }

  async function add_book() {
    if (name != "" && author != "") {
      if (
        await invoke("add_book_to_db", {
          book: name,
          author: author,
        })
      ) {
        if (keyword == "") {
          let result = await invoke("get_books");
          if (result) books_list.set(result);
        } else {
          let result = await invoke("search_books", { keyword: keyword });
          if (result) books_list.set(result);
        }
        name = "";
        author = "";
      }
    }
  }
</script>

<div>
  <form on:submit|preventDefault={add_book}>
    <input type="text" bind:value={name} placeholder="book name" />
    <input type="text" bind:value={author} placeholder="author name" />
    <button type="submit">Add</button>
  </form>
  <input
    type="text"
    bind:value={keyword}
    on:input={search}
    placeholder="Search"
  />
</div>

<style scoped>
  div,
  form {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 10px;
  }
  div {
    width: 100%;
    padding: 10px;
  }
  button,
  input {
    border: solid #525c7c 1px;
    border-radius: 5px;
    padding: 10px;
  }
  input {
    width: max(250px, 25vw);
    background-color: #dddeef;
  }
  button {
    width: 80px;
    background-color: #525c7c;
    color: #f2f2f2;
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
