import { check, read, write } from "@/lib/cache/interactions";
import { buildKeyStore } from "@/lib/cache/key-store";

const buildCacheItem = <T, K extends (...args: any) => any>(keyBuilder: K) => ({
  read: async (...params: Parameters<K>): Promise<T> =>
    JSON.parse(await read(keyBuilder(...params))),
  write: async (data: T, ...params: Parameters<K>) =>
    await write(keyBuilder(...params), data),
  check: async (...params: Parameters<K>) => await check(keyBuilder(...params)),
});

export const buildCache = (commander_name: string) => {
  const keys = buildKeyStore(commander_name);

  return {
    all_cards: buildCacheItem<string[], typeof keys.all_cards>(keys.all_cards),
    deck_list: buildCacheItem<string[], typeof keys.deck_list>(keys.deck_list),
    all_pairs: buildCacheItem(keys.all_pairs),
    commander_data: buildCacheItem<
      { decklist: string; winrate: number }[],
      typeof keys.commander_data
    >(keys.commander_data),
    id_win_rate: buildCacheItem<
      Record<string, number>,
      typeof keys.id_win_rate
    >(keys.id_win_rate),
    valid_ids: buildCacheItem<string[], typeof keys.valid_ids>(keys.valid_ids),
    bundle_data: buildCacheItem(keys.bundle_data),
    card_list_map: buildCacheItem<
      Record<string, string[]>,
      typeof keys.card_list_map
    >(keys.card_list_map),
    uncommon_cards: buildCacheItem<string[], typeof keys.uncommon_cards>(
      keys.uncommon_cards,
    ),
    common_cards: buildCacheItem<string[], typeof keys.common_cards>(
      keys.common_cards,
    ),
    winrate_uncommon_cards: buildCacheItem(keys.winrate_uncommon_cards),
    winrate_commander_average: buildCacheItem<
      { winrate_data: number },
      typeof keys.winrate_commander_average
    >(keys.winrate_commander_average),
  };
};
