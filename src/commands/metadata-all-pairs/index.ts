import { Command } from "commander";
import { allPairsKey } from "@/lib/cache-keys";
import { checkCache, writeCache } from "@/lib/cache";
import { loadALlCards, loadUncommonCards } from "@/lib/cache-load";

const cacheFlow = async (skipCache: boolean | undefined, ...keys: string[]) => {
  if (skipCache) {
    return;
  }
  console.log("Checking cache...");
  const allKeys = await Promise.all(keys.map(checkCache));

  if (allKeys.every((a) => a)) {
    console.log("Data cached.");
    process.exit();
  }
};

export const metadataAllPairs = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
): Promise<void> => {
  console.log(`Building set of all pairs of cards for ${commander_name}`);
  const CACHE_KEY = allPairsKey(commander_name);

  cacheFlow(skipCache, CACHE_KEY);
  const uncommonCards = await loadUncommonCards(commander_name);

  const length = uncommonCards.length;

  const pairs: [string, string][] = [];
  for (let i = 0; i < length - 1; i++) {
    for (let j = i + 1; j < length; j++) {
      pairs.push([uncommonCards[i], uncommonCards[j]]);
    }
  }

  console.log(`Writing all ${pairs.length} pairs.`);
  await writeCache(CACHE_KEY, pairs);
};

export const registerMetadataAllPairs = (program: Command) => {
  program
    .command("metadata-all-pairs")
    .option("--skip-cache")
    .argument("<commander_name>")
    .action(metadataAllPairs);
};
