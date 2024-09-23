import type { Command } from "commander";
import { loadCommanderData, loadDecklist } from "@/lib/data";
import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { checkCache, writeCache } from "@/lib/cache";

export const metadataIdWinRate = async (commander: string) => {
  console.log(`Building Id/WinRate map for ${commander}`);
  console.log("Checking cache...");
  const CACHE_KEY = `meta/${encodeURIComponent(commander)}/id-win-rate.json`;
  if (await checkCache(CACHE_KEY)) {
    console.log("Data cached.");
    return;
  }
  console.log("Loading commander data");
  const commanderData = await loadCommanderData(commander);
  console.log("Building map");
  const acc: Record<string, number> = {};
  for (const { winRate, decklist: decklistUrl } of commanderData) {
    if (!validDecklistUrl(decklistUrl) || !winRate) {
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
    .action(metadataIdWinRate);
};
