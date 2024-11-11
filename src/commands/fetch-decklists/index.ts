import type { Command } from "commander";
import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { getList } from "@/lib/playwright";
import { buildLoaders } from "@/lib/cache-load";

export const fetchDecklists = async (commander_name: string) => {
  const loader = buildLoaders(commander_name);

  if (!(await loader.commander_data.check())) {
    console.log("Data missing, you need to run load-data-store-for-commander");
    return;
  }
  const commanderData = await loader.commander_data.read();
  console.log("Starting load");
  for (const [index, { decklist: decklistUrl }] of commanderData.entries()) {
    if (!validDecklistUrl(decklistUrl)) {
      continue;
    }
    console.group(`Fetching ${index}`);

    const id = extractMoxfieldId(decklistUrl);
    if (await loader.deck_list.check(id)) {
      console.log(`Cache hit for ${id}`);
      console.groupEnd();
      continue;
    }
    console.log(`Fetching ${id}`);
    const decklist = await getList(decklistUrl);
    if (decklist) {
      console.log("Writing to cache");
      await loader.deck_list.write(decklist, id);
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
