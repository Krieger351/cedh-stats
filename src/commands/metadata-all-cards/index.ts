import type { Command } from "commander";
import { loadCommanderData, loadDecklist } from "@/lib/data";
import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { checkCache, readCache, writeCache } from "@/lib/cache";

export const metadataAllCards = async (commander_name: string) => {
  console.log(`Building full list of cards for ${commander_name}`);
  console.log("Checking cache...");
  const CACHE_KEY = `meta/${encodeURIComponent(commander_name)}/all-cards.json`;
  if (await checkCache(CACHE_KEY)) {
    console.log("Data cached.");
    return;
  }
  console.log("Building Set of all cards");
  const allCards = new Set();
  const commanderData = await loadCommanderData(commander_name);

  for (const { winRate, decklist: decklistUrl } of commanderData) {
    if (!validDecklistUrl(decklistUrl) || !winRate) {
      continue;
    }
    const id = extractMoxfieldId(decklistUrl);

    console.log(`Reading data for ${id}`);
    const CACHE_KEY = `decklists/${encodeURIComponent(commander_name)}/${id}.json`;
    if (!(await checkCache(CACHE_KEY))) {
      console.log(`Cache missing ${id}`);
      continue;
    }
    const rawList = await readCache(CACHE_KEY);
    const list = JSON.parse(rawList);
    for (const card of list) {
      allCards.add(card);
    }
  }
  console.log("Writing all cards to cache");
  await writeCache(CACHE_KEY, [...allCards]);
};

export const registerMetadataAllCards = (program: Command) => {
  program
    .command("metadata-all-cards")
    .argument("<commander_name>")
    .action(metadataAllCards);
};
