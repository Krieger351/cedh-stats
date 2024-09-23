import type { Command } from "commander";
import { checkCache, readCache, writeCache } from "@/lib/cache";
import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { getList } from "@/lib/playwright";
import { commanderDataKey, decklistKey } from "@/lib/cache-keys";

export const fetchDecklists = async (commander_name: string) => {
  const commanderDataCacheKey = commanderDataKey(commander_name);

  if (!(await checkCache(commanderDataCacheKey))) {
    console.log("Data missing, you need to run load-data-for-commander");
    return;
  }
  const commanderData = JSON.parse(await readCache(commanderDataCacheKey));
  console.log("Starting load");
  for (const [index, { decklist: decklistUrl }] of commanderData.entries()) {
    if (!validDecklistUrl(decklistUrl)) {
      continue;
    }
    console.group(`Fetching ${index}`);

    const id = extractMoxfieldId(decklistUrl);
    const DECKLIST_CACHE_KEY = decklistKey(commander_name, id);
    const decklistExists = await checkCache(DECKLIST_CACHE_KEY);
    if (decklistExists) {
      console.log(`Cache hit for ${id}`);
      console.groupEnd();
      continue;
    }
    console.log(`Fetching ${id}`);
    const decklist = await getList(decklistUrl);
    if (decklist) {
      console.log("Writing to cache");
      await writeCache(DECKLIST_CACHE_KEY, decklist);
    }
    console.groupEnd();
  }
};

export const registerFetchDecklists = (program: Command) => {
  program
    .command("fetch-decklists")
    .argument("<commander_name>")
    .action(fetchDecklists);
};
