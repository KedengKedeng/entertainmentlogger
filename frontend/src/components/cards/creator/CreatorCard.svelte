<script lang="ts">
  import ImageCollage from "src/components/ImageCollage.svelte";

  interface Props {
    imageCollageUrls: string[];
    creatorImage: string;
    creatorName: string;
    creatorKnownFor: string[];
    creationAmount: number;
  }

  let {
    imageCollageUrls,
    creatorImage,
    creatorName,
    creatorKnownFor,
    creationAmount,
  }: Props = $props();
</script>

<div class="w-full aspect-square relative rounded">
  <div class="absolute top-0 left-0 w-full h-full">
    <ImageCollage imgSources={imageCollageUrls} width="100%" height="100%" />
  </div>

  <div
    class="w-full h-1/2 bg-base-500 absolute bottom-0 flex flex-col items-center p-2 pt-[10%]"
  >
    <h2 class="font-bold">{creatorName}</h2>

    <p class="text-xs opacity-30">{creationAmount} franchises</p>

    <p class="text-xs break-all text-ellipsis overflow-hidden line-clamp-3">
      {creatorKnownFor.join(", ")}
    </p>
  </div>

  <img
    class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-3/4 rounded-full w-1/3 aspect-square"
    src={creatorImage}
    alt=""
  />
</div>
