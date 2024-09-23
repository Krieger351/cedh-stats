import { Command } from "commander";
import { checkCache, writeCache } from "@/lib/cache";
import {
  cardListMapKey,
  commonCardsKey,
  uncommonCardsKey,
} from "@/lib/cache-keys";
import {
  loadALlCards,
  loadAllLists,
  loadCardListMap,
  loadDecklist,
  loadValidLists,
} from "@/lib/cache-load";
import { checkCacheKeysExit } from "@/lib/check-cache-keys-exit";

export const metadataUncommonCards = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  console.log(
    `Building lists of common and uncommon cards for ${commander_name}`,
  );

  const UNCOMMON_CACHE_KEY = uncommonCardsKey(commander_name);
  const COMMON_CACHE_KEY = commonCardsKey(commander_name);
  await checkCacheKeysExit(false, UNCOMMON_CACHE_KEY, COMMON_CACHE_KEY);

  const cardListMap = await loadCardListMap(commander_name);

  const validIds = await loadValidLists(commander_name);

  const commonCards: string[] = [];
  const uncommonCards: string[] = [];
  for (const [card, lists] of Object.entries(cardListMap)) {
    if (lists.length >= validIds.length) {
      commonCards.push(card);
    } else {
      uncommonCards.push(card);
    }
  }

  await writeCache(COMMON_CACHE_KEY, commonCards);
  await writeCache(UNCOMMON_CACHE_KEY, uncommonCards);
};

export const registerMetadataUncommonCards = (program: Command) => {
  program
    .command("metadata-uncommon-cards")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataUncommonCards);
};
