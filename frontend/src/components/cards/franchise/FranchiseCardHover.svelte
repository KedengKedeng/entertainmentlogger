<script lang="ts">
  import StarIcon from "svelte-material-icons/Star.svelte";

  interface Props {
    franchiseName: string;
    franchiseRating: number;
    franchiseDescription: string;

    visible: boolean;
    pos: { x: number; y: number };
  }

  let {
    franchiseName,
    franchiseRating,
    franchiseDescription,
    visible,
    pos,
  }: Props = $props();
</script>

{#if visible}
  <div
    class="flex flex-col rounded-lg z-10 bg-base-500 p-2 absolute pointer-events-none max-w-[10vw] min-h-0"
    style="left: {pos.x}px; top: {pos.y}px;"
  >
    <div class="flex flex-row justify-between items-center">
      <p>{franchiseName}</p>

      <div class="flex flex-row text-yellow-300 items-center">
        <StarIcon size="1.5rem" />
        <p>{franchiseRating}</p>
      </div>
    </div>

    <p class="text-xs text-wrap break-all">{franchiseDescription}</p>
  </div>
{/if}
