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
    if (result) books_list.update((value) => (value = result));
  }
</script>

<div>
  <span class="book">{name}</span>
  <span class="author">{author}</span>
  <span>{read_state}</span>
  <button class="star">
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
  button {
    background-color: transparent;
    border: none;
    border-radius: 50%;
  }
</style>
