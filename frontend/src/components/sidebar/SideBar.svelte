<script lang="ts">
  import HeartIcon from "svelte-material-icons/Heart.svelte";
  import AccountMultipleIcon from "svelte-material-icons/AccountMultiple.svelte";
  import DramaMasksIcon from "svelte-material-icons/DramaMasks.svelte";
  import MovieOpenIcon from "svelte-material-icons/MovieOpen.svelte";
  import PictureCard from "../cards/PictureCard.svelte";
  import HomeIcon from "svelte-material-icons/Home.svelte";
  import Rating from "../Rating.svelte";
  import { SideBarPageEnum } from "./sideBarPageEnum";

  interface Props {
    currentPage: SideBarPageEnum;
  }

  let { currentPage }: Props = $props();
</script>

<div
  class="w-[20%] h-full bg-base-600 rounded-tl-3xl rounded-bl-3xl pt-4 pb-4 pr-4 flex flex-col overflow-y-scroll gap-2"
>
  <div class="rounded-xl bg-base-500 p-2 flex flex-col gap-1 ml-4">
    <p>Bookmarks</p>

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
          <p class="break-all">blabla,blabla</p>
        </div>
      {/each}
    </div>
  </div>

  <button
    class="p-2 flex flex-row items-center gap-2 {currentPage ===
    SideBarPageEnum.FRANCHISES
      ? 'bg-base-400 rounded-r-xl pl-6'
      : 'ml-4 bg-base-500 rounded-xl'}"
  >
    <HomeIcon size="1.5rem" />
    Franchises
  </button>

  <button
    class="p-2 flex flex-row items-center gap-2 {currentPage ===
    SideBarPageEnum.FAVORITES
      ? 'bg-base-400 rounded-r-xl pl-6'
      : 'ml-4 bg-base-500 rounded-xl'}"
  >
    <HeartIcon size="1.5rem" />
    Favorites
  </button>

  <button
    class="p-2 flex flex-row items-center gap-2 {currentPage ===
    SideBarPageEnum.CREATORS
      ? 'bg-base-400 rounded-r-xl pl-6'
      : 'ml-4 bg-base-500 rounded-xl'}"
  >
    <AccountMultipleIcon size="1.5rem" />
    Creators
  </button>

  <button
    class="p-2 flex flex-row items-center gap-2 {currentPage ===
    SideBarPageEnum.GENRES
      ? 'bg-base-400 rounded-r-xl pl-6'
      : 'ml-4 bg-base-500 rounded-xl'}"
  >
    <DramaMasksIcon size="1.5rem" />
    Genres
  </button>

  <button
    class="p-2 flex flex-row items-center gap-2 {currentPage ===
    SideBarPageEnum.MEDIA
      ? 'bg-base-400 rounded-r-xl pl-6'
      : 'ml-4 bg-base-500 rounded-xl'}"
  >
    <MovieOpenIcon size="1.5rem" />
    Media
  </button>

  <div class="flex flex-col ml-4">
    <p>Bored? Get back into...</p>

    <div class="flex flex-row gap-1 h-full">
      <div class="w-1/2 aspect-[3/4]">
        <PictureCard
          imgSrc="https://picsum.photos/200/300"
          height="100%"
          width="100%"
        />
      </div>

      <div class="w-1/2 flex flex-col justify-items-start h-full">
        <p class="font-bold">BLABLA</p>

        <Rating rating={2.5} />

        <p class="text-xs">Last edited</p>
        <p>2nd June 2024</p>

        <div class="flex flex-col gap-1 justify-self-end">
          <p>Notes:</p>
          <p class="line-clamp-3">
            Lorem ipsum, dolor sit amet consectetur adipisicing elit. Itaque
            autem facere repellendus et aperiam suscipit, eius quod vero alias
            asperiores cumque odio, sunt, libero ut ex veritatis facilis
            consectetur reprehenderit.
          </p>
        </div>
      </div>
    </div>
  </div>
</div>
