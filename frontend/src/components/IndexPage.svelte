<script lang="ts">
  import FilterOutlineIcon from "svelte-material-icons/FilterOutline.svelte";
  import Searchbar from "./Searchbar.svelte";
  import ArrowUpIcon from "svelte-material-icons/ArrowUp.svelte";
  import PlusIcon from "svelte-material-icons/Plus.svelte";
  import FranchiseCard from "./FranchiseCard.svelte";
  import PictureCard from "./PictureCard.svelte";
  import HeartIcon from "svelte-material-icons/Heart.svelte";
  import AccountMultipleIcon from "svelte-material-icons/AccountMultiple.svelte";
  import DramaMasksIcon from "svelte-material-icons/DramaMasks.svelte";
  import MovieOpenIcon from "svelte-material-icons/MovieOpen.svelte";
  import Rating from "./Rating.svelte";
</script>

<div class="w-[100vw] h-[100vh] bg-base-500 flex flex-row">
  <div class="w-[80%] h-full flex flex-col gap-3 p-2">
    <div class="flex flex-row flex-nowrap gap-1 w-full relative items-center">
      <div>
        <FilterOutlineIcon size="2rem" />
      </div>

      <Searchbar />

      <div class="flex flex-row items-center gap-1">
        <ArrowUpIcon />
        Recent
      </div>

      <button
        class="absolute right-0 bg-base-300 flex flex-row items-center justify-center flex-nowrap rounded pr-2"
      >
        <PlusIcon />
        Add Franchise
      </button>
    </div>

    <div
      class="h-full overflow-y-scroll overflow-x-hidden grid grid-cols-5 gap-2 grid-flow-row"
    >
      {#each Array(15) as _}
        <FranchiseCard />
      {/each}
    </div>
  </div>

  <div
    class="w-[20%] h-full bg-base-600 rounded-tl-3xl rounded-bl-3xl p-4 flex flex-col overflow-y-scroll gap-2"
  >
    <div class="rounded-xl bg-base-500 p-2 flex flex-col gap-1">
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

    <button class="rounded-xl bg-base-500 p-2 flex flex-row items-center gap-2">
      <HeartIcon size="1.5rem" />
      Favorites
    </button>

    <button class="rounded-xl bg-base-500 p-2 flex flex-row items-center gap-2">
      <AccountMultipleIcon size="1.5rem" />
      Creators
    </button>

    <button class="rounded-xl bg-base-500 p-2 flex flex-row items-center gap-2">
      <DramaMasksIcon size="1.5rem" />
      Genres
    </button>

    <button class="rounded-xl bg-base-500 p-2 flex flex-row items-center gap-2">
      <MovieOpenIcon size="1.5rem" />
      Media
    </button>

    <div class="flex flex-col">
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
</div>
