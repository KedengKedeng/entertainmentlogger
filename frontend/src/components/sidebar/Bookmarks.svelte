<script lang="ts">
    import PictureCard from "../cards/PictureCard.svelte";

    // TODO: enter users bookmarks here. No longer hardcode
    interface Props {

    }

    let { }: Props = $props();
</script>

<div class="grid grid-cols-3 gap-3">
  {#each Array(9) as _}
    <div class="flex flex-col w-full">
      <div class="w-full aspect-square">
        <PictureCard
          imgSrc="https://picsum.photos/200/300"
          height="100%"
          width="100%"
        />
      </div>
      <p class="break-all">blabla</p>
    </div>
  {/each}
</div>
