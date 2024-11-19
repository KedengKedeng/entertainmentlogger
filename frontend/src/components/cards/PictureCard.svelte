<script lang="ts">
  interface Props {
    imgSrc: string;
    height: string;
    width: string;
  }

  let { imgSrc, height, width }: Props = $props();
</script>

<div
  class="rounded-lg overflow-hidden flex flex-row justify-center items-center"
  style="height: {height}; width: {width};"
>
  <img src={imgSrc} alt="" class="w-full h-full object-cover" />
</div>
