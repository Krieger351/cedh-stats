<script lang="ts">
  import Mana from './mana.svelte';
  const tokenize = (mana_cost) => {
    if (mana_cost) {
      return mana_cost.match(/{.+?}/g);
    }
    return [];
  };
  export let mana_cost;

  $: tokens = tokenize(mana_cost);
</script>

<span>
  {#each tokens as token}
    <Mana mana={token} />
  {/each}
</span>
