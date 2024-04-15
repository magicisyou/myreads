<script lang="ts">
  import { books_list, search_keyword } from "./stores.ts";
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let author = "";
  let keyword = "";

  $: {
    search_keyword.set(keyword);
    draw();
  }

  function closeSearch() {
    keyword = "";
  }

  async function add_book() {
    if (name != "" && author != "") {
      if (
        await invoke("add_book_to_db", {
          book: name,
          author: author,
        })
      ) {
        name = "";
        author = "";
        draw();
      }
    }
  }

  async function draw() {
    if (keyword == "") {
      let result = await invoke("get_books");
      if (result) books_list.set(result);
    } else {
      let result = await invoke("search_books", { keyword: keyword });
      if (result) books_list.set(result);
    }
  }
</script>

<div>
  <form on:submit|preventDefault={add_book}>
    <input type="text" bind:value={name} placeholder="book name" />
    <input type="text" bind:value={author} placeholder="author name" />
    <button class="add" type="submit">Add</button>
  </form>
  <span class="search">
    <input type="text" bind:value={keyword} placeholder="Search" />
    {#if keyword != ""}
      <button class="close-search" on:click={closeSearch}>
        <img src="/close.svg" alt="close" />
      </button>
    {/if}
  </span>
</div>

<style scoped>
  div {
    position: sticky;
    top: 0;
    background-color: #899ebd;
    width: 100%;
    padding: 10px;
    z-index: 1;
  }
  div,
  form {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 10px;
  }
  .add,
  input {
    border: solid #525c7c 1px;
    border-radius: 5px;
    padding: 10px;
  }
  input {
    width: max(250px, 25vw);
    background-color: #dddeef;
  }
  .add {
    width: 80px;
    background-color: #525c7c;
    color: #f2f2f2;
  }
  .search {
    position: relative;
  }
  .close-search {
    position: absolute;
    top: 20%;
    border: none;
    background-color: transparent;
    right: 10px;
    height: 75%;
  }
  @media (prefers-color-scheme: dark) {
    input,
    .add {
      background-color: #111;
      color: #f2f2f2;
      border: solid #666 1px;
    }
    div {
      background-color: #101321;
    }
  }
</style>
