import { buildCache } from "@/lib/cache";
import { fetchCommanderData } from "@/lib/data-store/commander-data";
import { fetchDecklist } from "@/lib/data-store/decklist";
import { transformIdWinRate } from "@/lib/data-store/id-win-rate";
import { getQuartiles } from "@/lib/filter-outliers";

type StringOrFunction<T extends (...args: any) => string> = string | T;
const build_wrap =
  (cache: ReturnType<typeof buildCache>) =>
  <T, K extends (...params: any) => string>(
    key: StringOrFunction<K>,
    get_data: (...args: any) => Promise<T>,
    encode: (data: T) => string = JSON.stringify,
    decode: (s: string) => T = JSON.parse,
  ) => {
    const wrapFunction = async (
      ...params: K extends string ? unknown[] : Parameters<K>
    ): Promise<T> => {
      let string_key = typeof key === "function" ? key(...params) : key;
      if (await cache.check(string_key)) {
        return await cache.read(string_key, decode);
      }
      const data = await get_data(...params);
      await cache.write(string_key, data, encode);
      return data;
    };

    wrapFunction.skip = get_data;
    return wrapFunction;
  };

export const buildDataStore = function (commander_name: string) {
  const cache = buildCache(commander_name);
  const wrap = build_wrap(cache);

  const store = {
    commander_entries: wrap(
      "commander-entries",
      async () => await fetchCommanderData(commander_name),
    ),
    id_win_rate: wrap(
      "meta/id-win-rate",
      async () => {
        const commanderData = await store.commander_entries();
        const id_win_rate = transformIdWinRate(commanderData);
        return id_win_rate;
      },
      (data) => JSON.stringify(Object.fromEntries(data.entries())),
      (string) => new Map<string, number>(Object.entries(JSON.parse(string))),
    ),
    decklist: wrap(
      (id: string) => `decklists/${id}`,
      async (id: string) => {
        const decklist = await fetchDecklist(id);
        return decklist ? new Set(decklist) : false;
      },
      (set) => (set ? JSON.stringify(Array.from(set)) : "false"),
      (string) => {
        try {
          return new Set<string>(JSON.parse(string));
        } catch {
          return false;
        }
      },
    ),
    all_cards: wrap(
      "meta/all-cards",
      async () => {
        const id_list = (await store.id_win_rate()).keys();

        const all_cards_set = new Set<string>();

        for (const id of id_list) {
          const list = await store.decklist(id);
          if (list) {
            for (const card of list) {
              all_cards_set.add(card);
            }
          }
        }
        return all_cards_set;
      },
      (set) => JSON.stringify(Array.from(set)),
      (string) => new Set(JSON.parse(string)),
    ),
    ids_valid: wrap(
      "ids-valid",
      async () => {
        const id_list = (await store.id_win_rate()).keys();
        const valid_ids = new Set<string>();

        for (const id of id_list) {
          if (await store.decklist(id)) {
            valid_ids.add(id);
          }
        }
        return valid_ids;
      },
      (set) => JSON.stringify(Array.from(set)),
      (string) => new Set(JSON.parse(string)),
    ),
    ids_positive_win_rate: wrap(
      "meta/ids-positive-win-rate",
      async () => {
        const id_win_rate = await store.id_win_rate();
        const positive_win_rate = new Set<string>();
        for (const [id, winrate] of id_win_rate) {
          if (winrate >= 0.25) {
            positive_win_rate.add(id);
          }
        }
        return positive_win_rate;
      },
      (set) => JSON.stringify(Array.from(set)),
      (string) => new Set(JSON.parse(string)),
    ),
    cards_in_positive_win_rate_decks: wrap(
      "meta/cards-in-positive-win-rate-decks",
      async () => {
        const ids_positive_win_rate = await store.ids_positive_win_rate();
        let all_cards = new Set();
        for (const id of ids_positive_win_rate) {
          const decklist = await store.decklist(id);
          if (decklist) {
            all_cards = all_cards.union(decklist);
          }
        }
        return all_cards;
      },
      (set) => JSON.stringify(Array.from(set)),
      (string) => new Set(JSON.parse(string)),
    ),
    ids_top_decks: wrap(
      "meta/ids-top-decks",
      async () => {
        const id_win_rate: Map<string, number> = await store.id_win_rate();
        const { Q1 } = getQuartiles(Array.from(id_win_rate.values()));
        for (const [id, winrate] of id_win_rate) {
          if (winrate < Q1) {
            id_win_rate.delete(id);
          }
        }

        return new Set(id_win_rate.keys());
      },
      (set) => JSON.stringify(Array.from(set)),
      (string) => new Set(JSON.parse(string)),
    ),
    cards_in_top_decks: wrap(
      (offset = 5) => `meta/cards-in-top-decks-${offset}`,
      async (offset = 5) => {
        const ids_top_decks = await store.ids_top_decks();
        const card_list_map = await store.card_list_map();
        const all_cards = await store.all_cards();
        const cards_in_top_decks = new Set<string>();
        for (const card of all_cards) {
          if (
            (card_list_map.get(card)?.intersection(ids_top_decks).size || 0) >=
            (offset > 1 ? offset : ids_top_decks.size * offset)
          ) {
            cards_in_top_decks.add(card);
          }
        }
        return cards_in_top_decks;
      },
      (set) => JSON.stringify(Array.from(set)),
      (string) => new Set(JSON.parse(string)),
    ),
    card_list_map: wrap(
      "meta/card-list-map",
      async () => {
        const all_cards = await store.all_cards();
        const ids_valid = await store.ids_valid();

        const card_list_map = new Map<string, Set<string>>();
        for (const card of all_cards) {
          card_list_map.set(card, new Set());
        }
        for (const id of ids_valid) {
          const decklist = (await store.decklist(id)) || [];
          for (const card of decklist) {
            card_list_map.set(
              card,
              card_list_map.get(card)?.add(id) || new Set([id]),
            );
          }
        }
        return card_list_map;
      },
      (map) =>
        JSON.stringify(
          Object.fromEntries(
            Array.from(map.entries()).map(([key, set]) => [
              key,
              Array.from(set),
            ]),
          ),
        ),
      (string) =>
        new Map(
          Object.entries(JSON.parse(string) as Record<string, string[]>).map(
            ([key, set]) => [key, new Set(set)],
          ),
        ),
    ),
  };
  return store;
};
