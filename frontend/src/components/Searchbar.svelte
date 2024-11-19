<script lang="ts" generics="searchResultItem">
  import MagnifyingIcon from "svelte-material-icons/Magnify.svelte";

  interface Props {
    onSearch: (query: string) => Promise<searchResultItem[]>;
    searchResults: searchResultItem[];
  }

  let { onSearch, searchResults = $bindable() }: Props = $props();
</script>

<div
  class="rounded bg-base-400 flex flex-row gap-1 flex-nowrap w-72 h-full text-base-300 items-center pl-1"
>
  <MagnifyingIcon size="2rem" />

  <input
    type="text"
    placeholder="Type to Search"
    class="w-full h-full bg-transparent focus:outline-none text-white"
    oninput={async (e) => {
      searchResults = await onSearch(e.target!.value);
    }}
  />
</div>
