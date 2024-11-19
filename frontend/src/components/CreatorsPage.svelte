<script lang="ts">
  import FranchiseCard from "./cards/franchise/FranchiseCard.svelte";
  import HeaderBar from "./header/HeaderBar.svelte";
  import SideBar from "./sidebar/SideBar.svelte";
  import { SideBarPageEnum } from "./sidebar/sideBarPageEnum";
</script>

<div class="w-[100vw] h-[100vh] bg-base-500 flex flex-row">
  <div class="w-[80%] h-full flex flex-col gap-3">
    <HeaderBar addType="creator" addFunction={() => {}} />

    <div
      class="h-full w-full overflow-y-scroll overflow-x-hidden grid grid-cols-5 gap-2 grid-flow-row bg-base-400 p-2"
    >
      {#each Array(15) as _}
        <FranchiseCard
          imgSrc="https://picsum.photos/200/300"
          rating={4.5}
          franchiseName="blabla"
          franchiseDescription="blabla"
        />
      {/each}
    </div>
  </div>

  <SideBar currentPage={SideBarPageEnum.CREATORS} />
</div>
