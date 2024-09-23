import type { Command } from "commander";
import { loadCommanderData, loadDecklist } from "@/lib/data";
import { extractMoxfieldId } from "@/lib/moxfield";
import { bundleDataKey, decklistKey } from "@/lib/cache-keys";
import {
  loadCardListMap,
  loadIdWinRate,
  loadValidLists,
} from "@/lib/cache-load";
import { writeCache } from "@/lib/cache";
import { checkCacheKeysExit } from "@/lib/check-cache-keys-exit";

export const bundleData = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  const CACHE_KEY = bundleDataKey(commander_name);

  await checkCacheKeysExit(skipCache, CACHE_KEY);

  const cardData: Record<string, { contains: number; excludes: number }> = {};
  const idWinRate = await loadIdWinRate(commander_name);

  const validIdsSet = new Set(await loadValidLists(commander_name));

  const cardListMap = await loadCardListMap(commander_name);

  const calcSetAverage = (ids: Set<string>) => {
    const numbers = [...ids].map((id) => idWinRate[id]);
    const sum = numbers.reduce((acc, cur) => acc + cur, 0);
    return sum / numbers.length;
  };
  const count = validIdsSet.size;
  const averageWinRate = calcSetAverage(validIdsSet);

  console.log(`Total entries: ${count}`);
  console.log(`Average win rate: ${averageWinRate}`);

  for (const card in cardListMap) {
    const containsCard = new Set(cardListMap[card]);
    const excludesCard = validIdsSet.difference(containsCard);
    cardData[card] = {
      contains: calcSetAverage(containsCard),
      excludes: calcSetAverage(excludesCard),
    };
  }

  await writeCache(CACHE_KEY, {
    averageWinRate,
    totalEntries: count,
    cardData,
  });
};

export const registerBundleData = (program: Command) => {
  program
    .command("bundle-data")
    .option("--skip-cache")
    .argument("<commander_name>")
    .action(bundleData);
};
