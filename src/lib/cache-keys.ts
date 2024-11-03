export const buildKeyStore = (commander_name: string) => {
  const uri_commander_name = encodeURIComponent(commander_name);
  return {
    cards_in_above_average_decks: () =>
      `meta/${uri_commander_name}/cards_in_above_average_decks.json`,
    deck_list: (id: string) => `decklists/${uri_commander_name}/${id}.json`,
    all_cards: () => `meta/${uri_commander_name}/all-cards.json`,
    id_win_rate: () => `meta/${uri_commander_name}/id-win-rate.json`,
    commander_data: () => `commanders/${uri_commander_name}.json`,
    valid_ids: () => `meta/${uri_commander_name}/valid-ids.json`,
    card_list_map: () => `meta/${uri_commander_name}/card-list-map.json`,
    all_pairs: () => `meta/${uri_commander_name}/all-pairs.json`,
    uncommon_cards: () => `meta/${uri_commander_name}/uncommon-cards.json`,
    common_cards: () => `meta/${uri_commander_name}/common-cards.json`,
    bundle_data: () => `meta/${uri_commander_name}/bundle-data.json`,
    winrate_uncommon_cards: () =>
      `meta/${uri_commander_name}/winrate-uncommon-cards.json`,
    winrate_commander_average: () =>
      `meta/${uri_commander_name}/winrate-commander-average.json`,
  };
};
