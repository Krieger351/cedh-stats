<script lang="ts">
  import cardData from '../lib/data/card-data.json';
  import joinedLists from '../lib/data/joined-lists.json';
  import Mana from '../components/mana.svelte';
  import ManaCost from '../components/mana-cost.svelte';

  const sortedList = Object.entries(joinedLists).sort((a, b) => b[1] - a[1]);
</script>

<svelte:head>
  <title>cEDH Stats</title>
  <meta name="description" content="cEDH stats helper" />
</svelte:head>

<h1>cEDH stats helper</h1>

<table class="pure-table">
  <thead>
    <tr><th>index</th><th> Name </th><th>count</th><th>type</th><th>MV</th><th>MC</th></tr>
  </thead>
  <tbody>
    {#each sortedList as [card, count], index}
      <tr>
        <td>{index + 1}</td>
        <td>{card}</td>
        <td>{count}</td>
        <td>{cardData[card].type_line}</td>
        <td><Mana mana={cardData[card].cmc} /></td>
        <td> <ManaCost mana_cost={cardData[card].mana_cost} /></td>
      </tr>
    {/each}
  </tbody>
</table>
