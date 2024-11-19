<script lang="ts">
  import Rating from "../../Rating.svelte";
  import PictureCard from "../PictureCard.svelte";
  import FranchiseCardHover from "./FranchiseCardHover.svelte";

  interface Props {
    imgSrc: string;
    rating: number;
    franchiseName: string;
    franchiseDescription: string;
  }

  let { imgSrc, rating, franchiseName, franchiseDescription }: Props = $props();

  let visible = $state(false);
  let mousePosition = $state({ x: 0, y: 0 });
</script>

<div
  class="w-full aspect-[3/4] relative"
  onmouseenter={(e) => {
    visible = true;
    mousePosition = { x: e.clientX, y: e.clientY };
  }}
  onmouseleave={() => {
    visible = false;
  }}
>
  <PictureCard
    {imgSrc}
    height="100%"
    width="100%"
  />

  <div class="absolute bottom-1 left-1">
    <Rating {rating} />
  </div>
</div>

<FranchiseCardHover
  {visible}
  pos={mousePosition}
  {franchiseName}
  franchiseRating={rating}
  {franchiseDescription}
/>
