<script lang="ts">
    import Rating from "./Rating.svelte";
    import PictureCard from "./PictureCard.svelte";
</script>

<div class="w-full aspect-[3/4] relative">
    <PictureCard
        imgSrc="https://picsum.photos/200/300"
        height="100%"
        width="100%"
    />

    <div class="absolute bottom-1 left-1">
        <Rating rating={4.5} />
    </div>
</div>