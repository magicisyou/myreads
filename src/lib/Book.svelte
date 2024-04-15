<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { books_list, search_keyword } from "./stores.ts";

  export let name = "";
  export let author = "";
  export let read_state = "";
  export let starred = false;

  async function deleteBook() {
    if (
      await invoke("delete_book", {
        book: this.parentElement.children[0].innerText,
        author: this.parentElement.children[1].innerText,
      })
    ) {
      draw();
    }
  }

  async function toggleStarred() {
    if (
      await invoke("change_starred", {
        book: this.parentElement.children[0].innerText,
        author: this.parentElement.children[1].innerText,
      })
    ) {
      draw();
    }
  }

  async function changeReadState() {
    if (
      await invoke("change_read_state", {
        book: this.parentElement.children[0].innerText,
        author: this.parentElement.children[1].innerText,
      })
    ) {
      draw();
    }
  }
  async function draw() {
    if ($search_keyword == "") {
      let result = await invoke("get_books");
      if (result) books_list.set(result);
    } else {
      let result = await invoke("search_books", { keyword: $search_keyword });
      if (result) books_list.set(result);
    }
  }
</script>

<div class={read_state}>
  <span class="book">{name}</span>
  <span class="author">{author}</span>
  <button class="read-state" on:click={changeReadState}>{read_state}</button>
  <button on:click={toggleStarred} class="star">
    <img src={starred ? "/star-black.svg" : "star.svg"} alt="star" />
  </button>
  <button class="delete" on:click={deleteBook}>
    <img src="delete.svg" alt="delete" />
  </button>
</div>

<style scoped>
  div {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    background-color: #d3d6d8;
    border: solid #435 2px;
    border-radius: 6px;
    height: 300px;
    width: 200px;
    padding: 10px;
    color: #101;
  }
  div:hover {
    outline: solid #234 1px;
  }
  .book,
  .author {
    height: 75px;
    width: 100%;
    text-align: center;
  }
  .book {
    font-weight: bold;
  }
  .star {
    top: 5px;
  }
  .delete {
    bottom: 5px;
  }
  .star,
  .delete {
    background-color: transparent;
    border: none;
    border-radius: 50%;
    right: 5px;
    position: absolute;
  }
  .read-state {
    width: 100%;
    background-color: #0003;
    border: none;
    color: inherit;
    padding: 5px;
    font-size: 16px;
  }
  .NotCompleted {
    background-color: #d3d6d8;
  }
  .Reading {
    background-color: #546b96;
    color: #f2f2f2;
  }
  .Completed {
    background-color: #40826d;
    color: #f2f2f2;
  }
  .WishToRead {
    background-color: #b08d57;
  }

  @media (prefers-color-scheme: dark) {
    div {
      border: solid #f2f2f2 2px;
    }
    div:hover {
      outline: solid #f2f2f2 1px;
    }
  }
</style>
