<script lang="ts">
  interface Props {
    username: string;
  }

  let { username }: Props = $props();
</script>

<div class="flex flex-row items-center h-full gap-2">
  <!-- TODO: replace with blob -->
  <img
    src="https://picsum.photos/200/300"
    alt=""
    class="rounded-full overflow-hidden aspect-square h-full object-cover"
  />

  <p>{username}</p>
</div>
