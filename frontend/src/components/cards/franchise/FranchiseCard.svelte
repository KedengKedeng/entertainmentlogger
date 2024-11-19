<script lang="ts">
  import Rating from "../../Rating.svelte";
  import PictureCard from "../PictureCard.svelte";
  import FranchiseCardHover from "./FranchiseCardHover.svelte";

  let visible = $state(false);
  let mousePosition = $state({ x: 0, y: 0 });
</script>

<div
  class="w-full aspect-[3/4] relative"
  onmouseenter={(e) => {
    console.log(e);
    visible = true;
    mousePosition = { x: e.clientX, y: e.clientY };
  }}
  onmouseleave={() => {
    visible = false;
  }}
>
  <PictureCard
    imgSrc="https://picsum.photos/200/300"
    height="100%"
    width="100%"
  />

  <div class="absolute bottom-1 left-1">
    <Rating rating={4.5} />
  </div>
</div>

<FranchiseCardHover
  {visible}
  pos={mousePosition}
  franchiseName="BLABLA"
  franchiseRating={4.5}
  franchiseDescription="BLABLA"
/>
