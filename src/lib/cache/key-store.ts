export const buildKeyStore = (commander_name: string) => {
  const uri_commander_name = encodeURIComponent(commander_name);
  return {
    cards_in_above_average_decks: () =>
      `${uri_commander_name}/meta/cards_in_above_average_decks.json`,
    deck_list: (id: string) => `${uri_commander_name}/decklists/${id}.json`,
    all_cards: () => `${uri_commander_name}/meta/all-cards.json`,
    id_win_rate: () => `${uri_commander_name}/meta/id-win-rate.json`,
    commander_data: () => `${uri_commander_name}/commander-data.json`,
    valid_list_ids: () => `${uri_commander_name}/meta/valid-list-ids.json`,
    card_list_map: () => `${uri_commander_name}/meta/card-list-map.json`,
    all_pairs: () => `${uri_commander_name}/meta/all-pairs.json`,
    uncommon_cards: () => `${uri_commander_name}/meta/uncommon-cards.json`,
    common_cards: () => `${uri_commander_name}/meta/common-cards.json`,
    bundle_data: () => `${uri_commander_name}/meta/bundle-data.json`,
    positive_winrate_ids: () =>
      `${uri_commander_name}/meta/bundle-positive-winrate-ids.json`,
    winrate_uncommon_cards: () =>
      `${uri_commander_name}/meta/winrate-uncommon-cards.json`,
    winrate_commander_average: () =>
      `${uri_commander_name}/meta/winrate-commander-average.json`,
  };
};
