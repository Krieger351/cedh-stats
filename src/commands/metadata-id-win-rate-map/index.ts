import type { Command } from "commander";
import { loadCommanderData, loadDecklist } from "@/lib/data";
import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { writeCache } from "@/lib/cache";
import { idWinRateKey } from "@/lib/cache-keys";
import { checkCacheKeysExit } from "@/lib/check-cache-keys-exit";

export const metadataIdWinRate = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  console.log(`Building Id/WinRate map for ${commander_name}`);
  console.log("Checking cache...");
  const CACHE_KEY = idWinRateKey(commander_name);
  checkCacheKeysExit(skipCache, CACHE_KEY);

  console.log("Loading commander data");
  const commanderData = await loadCommanderData(commander_name);
  console.log("Building map");
  const acc: Record<string, number> = {};
  for (const { winRate, decklist: decklistUrl } of commanderData) {
    if (!validDecklistUrl(decklistUrl) || typeof winRate !== "number") {
      continue;
    }
    const id = extractMoxfieldId(decklistUrl);
    acc[id] = winRate;
  }
  console.log("Writing to cache");
  await writeCache(CACHE_KEY, acc);
};

export const registerMetadataIdWinRate = (program: Command) => {
  program
    .command("metadata-id-win-rate-map")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataIdWinRate);
};
