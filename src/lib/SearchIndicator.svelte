<script lang="ts">
  import { search_keyword } from "./stores.ts";
</script>

{#if $search_keyword != ""}
  <p>Search results for <span>{$search_keyword}<span></span></span></p>
{/if}

<style scoped>
  p {
    width: 100%;
    text-align: center;
    background-color: #fff1;
    padding: 20px;
  }
  span {
    font-weight: bold;
    text-decoration: underline;
    color: #a45;
  }
</style>
