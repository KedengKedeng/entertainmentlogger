<script lang="ts">
  interface Props {
    imgSources: string[];
    width: string;
    height: string;
  }

  //TODO: make sure that all sets of image sources that come in will result in an even square
  let { imgSources, width, height }: Props = $props();

  let container: HTMLDivElement | undefined = $state();

  let containerWidth = $derived(container?.offsetWidth ?? 0);
  let containerHeight = $derived(container?.offsetHeight ?? 0);

  let imagesInRow = $derived(Math.sqrt(imgSources.length));
  let imageWidth = $derived(containerWidth / imagesInRow);
  let imageHeight = $derived(containerHeight / imagesInRow);
</script>

<div
  class="grid overflow-hidden"
  style="width: {width}; height: {height}; grid-template-columns: repeat({imagesInRow}, {imageWidth}px); grid-template-rows: repeat({imagesInRow}, {imageHeight}px);"
  bind:this={container}
>
  {#each imgSources as imgSrc}
    <img
      class="object-fill"
      src={imgSrc}
      alt=""
    />
  {/each}
</div>
