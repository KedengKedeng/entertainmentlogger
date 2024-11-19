<script lang="ts">
  import Account from "./Account.svelte";
  import Searchbar from "../Searchbar.svelte";
  import ArrowUpIcon from "svelte-material-icons/ArrowUp.svelte";
  import FilterOutlineIcon from "svelte-material-icons/FilterOutline.svelte";
  import PlusIcon from "svelte-material-icons/Plus.svelte";

  interface Props {
    addType: string;
    addFunction: () => void;
  }

  let {
    addType,
    addFunction
  }: Props = $props();
</script>

<div class="flex flex-row justify-start flex-nowrap gap-1 items-center w-full h-[3.5vh] pl-2 pr-2 pt-2">
  <!-- TODO: replace with user data -->
  <Account username="username" />

  <div>
    <FilterOutlineIcon size="2rem" />
  </div>

  <Searchbar onSearch={() => {}} searchResults={[]} />

  <div class="flex flex-row items-center gap-1">
    <ArrowUpIcon />
    Recent
  </div>

  <button
    class="ml-auto bg-base-300 flex flex-row items-center justify-center flex-nowrap rounded pr-2"
    onclick={addFunction}
  >
    <PlusIcon />
    Add {addType}
  </button>
</div>
