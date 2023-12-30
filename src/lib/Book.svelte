<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { books_list } from "./stores.ts";

  export let name = "";
  export let author = "";
  export let read_state = "";
  export let starred = false;

  async function deleteBook() {
    let result = await invoke("delete_book", {
      book: this.parentElement.children[0].innerText,
      author: this.parentElement.children[1].innerText,
    });
    if (result) books_list.set(result);
  }

  async function toggleStarred() {
    let result = await invoke("change_starred", {
      book: this.parentElement.children[0].innerText,
      author: this.parentElement.children[1].innerText,
    });
    if (result) books_list.set(result);
  }

  async function changeReadState() {
    let result = await invoke("change_read_state", {
      book: this.parentElement.children[0].innerText,
      author: this.parentElement.children[1].innerText,
    });
    if (result) books_list.set(result);
  }
</script>

<div>
  <span class="book">{name}</span>
  <span class="author">{author}</span>
  <button class="read-state" on:click={changeReadState}>{read_state}</button>
  <button class="star" on:click={toggleStarred}>
    <img src={starred ? "star-filled.svg" : "star.svg"} alt="star" />
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
    background-color: #213;
    border: solid gold 2px;
    border-radius: 6px;
    height: 300px;
    width: 200px;
    padding: 10px;
    color: gold;
    &:hover {
      background-color: #324;
    }
  }
  .book,
  .author {
    height: 75px;
  }
  .book {
    font-weight: bold;
    text-align: center;
  }
  .star {
    position: absolute;
    top: 5px;
    right: 5px;
  }
  .delete {
    position: absolute;
    bottom: 5px;
    right: 5px;
  }
  .star,
  .delete {
    background-color: transparent;
    border: none;
    border-radius: 50%;
  }
  .read-state {
    width: 100%;
    background-color: #0003;
    border: none;
    color: gold;
  }
</style>
