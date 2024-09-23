import { Command } from "commander";
import { checkCache, readCache, writeCache } from "@/lib/cache";
import {
  allCardsKey,
  decklistKey,
  idWinRateKey,
  validIdsKey,
} from "@/lib/cache-keys";
import { loadALlCards, loadAllLists, loadDecklist } from "@/lib/cache-load";
import { checkCacheKeysExit } from "@/lib/check-cache-keys-exit";

export const metadataValidIds = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  console.log(`Building list of valid ids ${commander_name}`);
  console.log("Checking cache...");
  const CACHE_KEY = validIdsKey(commander_name);
  checkCacheKeysExit(skipCache, CACHE_KEY);
  const allLists = await loadAllLists(commander_name);

  const validLists: string[] = [];
  for (const list of allLists) {
    if (await checkCache(decklistKey(commander_name, list))) {
      validLists.push(list);
    }
  }

  await writeCache(validIdsKey(commander_name), validLists);
};

export const registerMetadataValidIds = (program: Command) => {
  program
    .command("metadata-valid-ids")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataValidIds);
};
