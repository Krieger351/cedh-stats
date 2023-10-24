<script>
  export let mana;
  const mapManna = (mana) => {
    return `ms-${mana.toString().replace(/[{}]/g, '').toLowerCase()}`;
  };

  $: mappedMana = mapManna(mana);
</script>

<i class="ms ms-cost {mappedMana} ms-shadow" />
