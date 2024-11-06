import { checkCache, readCache, writeCache } from "@/lib/cache_";
import { buildKeyStore } from "@/lib/cache-keys";

const buildLoader = <T, K extends (...args: any) => any>(keyBuilder: K) => ({
  read: async (...params: Parameters<K>): Promise<T> =>
    JSON.parse(await readCache(keyBuilder(...params))),
  write: async (data: T, ...params: Parameters<K>) =>
    await writeCache(keyBuilder(...params), data),
  check: async (...params: Parameters<K>) =>
    await checkCache(keyBuilder(...params)),
});

export const buildLoaders = (commander_name: string) => {
  const keys = buildKeyStore(commander_name);

  return {
    all_cards: buildLoader<string[], typeof keys.all_cards>(keys.all_cards),
    deck_list: buildLoader<string[], typeof keys.deck_list>(keys.deck_list),
    all_pairs: buildLoader(keys.all_pairs),
    commander_data: buildLoader<
      { decklist: string; winrate: number }[],
      typeof keys.commander_data
    >(keys.commander_data),
    id_win_rate: buildLoader<Record<string, number>, typeof keys.id_win_rate>(
      keys.id_win_rate,
    ),
    valid_ids: buildLoader<string[], typeof keys.valid_ids>(keys.valid_ids),
    bundle_data: buildLoader(keys.bundle_data),
    card_list_map: buildLoader<
      Record<string, string[]>,
      typeof keys.card_list_map
    >(keys.card_list_map),
    uncommon_cards: buildLoader<string[], typeof keys.uncommon_cards>(
      keys.uncommon_cards,
    ),
    common_cards: buildLoader<string[], typeof keys.common_cards>(
      keys.common_cards,
    ),
    winrate_uncommon_cards: buildLoader(keys.winrate_uncommon_cards),
    winrate_commander_average: buildLoader<
      { winrate_data: number },
      typeof keys.winrate_commander_average
    >(keys.winrate_commander_average),
  };
};
