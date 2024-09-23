import { Command } from "commander";
import { checkCache, writeCache } from "@/lib/cache";
import { cardListMapKey } from "@/lib/cache-keys";
import { loadALlCards, loadDecklist, loadValidLists } from "@/lib/cache-load";
import { checkCacheKeysExit } from "@/lib/check-cache-keys-exit";

export const metadataCardListMap = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  console.log(`Building card list map for ${commander_name}`);
  console.log("Checking cache...");
  const CACHE_KEY = cardListMapKey(commander_name);

  checkCacheKeysExit(skipCache, CACHE_KEY);

  console.log("Building object for all cards...");
  const allCards = await loadALlCards(commander_name);
  const lists = await loadValidLists(commander_name);

  const map: Record<string, string[]> = {};

  for (const card of allCards) {
    map[card] = [];
  }

  for (const list of lists) {
    const cards = await loadDecklist(commander_name, list);

    for (const card of cards) {
      map[card].push(list);
    }
  }
  await writeCache(CACHE_KEY, map);
};

export const registerMetadataCardListMap = (program: Command) => {
  program
    .command("metadata-card-list-map")
    .option("--skip-cache")
    .argument("<commander_name>")
    .action(metadataCardListMap);
};
