<script lang="ts">
  export let imgSrc: string;

  export let height: string;
  export let width: string;
</script>

<div
  class="rounded-lg overflow-hidden flex flex-row justify-center items-center"
  style="height: {height}; width: {width};"
>
  <img src={imgSrc} alt="" class="w-full h-full object-cover" />
</div>
