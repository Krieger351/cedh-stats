export const allCardsKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/all-cards.json`;

export const idWinRateKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/id-win-rate.json`;

export const decklistKey = (commander_name: string, id: string) =>
  `decklists/${encodeURIComponent(commander_name)}/${id}.json`;

export const commanderDataKey = (commander_name: string) =>
  `commanders/${encodeURIComponent(commander_name)}.json`;

export const validIdsKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/valid-ids.json`;

export const cardListMapKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/card-list-map.json`;
export const allPairsKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/all-pairs.json`;
export const uncommonCardsKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/uncommon-cards.json`;

export const commonCardsKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/common-cards.json`;

export const bundleDataKey = (commander_name: string) =>
  `meta/${encodeURIComponent(commander_name)}/bundle-data.json`;
