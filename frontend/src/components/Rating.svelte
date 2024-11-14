<script lang="ts">
  import StarIcon from "svelte-material-icons/Star.svelte";

  export let rating: number;
  export let starSize = 20;
  let numberList: number[] = [];

  $: numberList = Array(Math.ceil(rating))
    .fill(0)
    .map((_, i) => (i += 1));
</script>

<div class="flex flex-row flex-nowrap items-center">
  {#each numberList as i}
    <div
      class="overflow-hidden flex flex-row justify-start p-0 m-0 items-center"
      style="width: {i > rating ? starSize * (rating % 1) : starSize}px"
    >
      <div style="width: {starSize}px">
        <StarIcon size={starSize} color="yellow" />
      </div>
    </div>
  {/each}
</div>
