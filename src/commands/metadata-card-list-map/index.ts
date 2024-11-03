import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

export const metadataCardListMap = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  const loaders = buildLoaders(commander_name);
  console.log(`Building card list map for ${commander_name}`);
  console.log("Checking cache...");

  if (skipCache && (await loaders.card_list_map.check())) {
    process.exit();
  }

  console.log("Building object for all cards...");
  const allCards = await loaders.all_cards.read(); //loadAllCards(commander_name);
  const lists = await loaders.valid_ids.read();

  const map: Record<string, string[]> = {};

  for (const card of allCards) {
    map[card] = [];
  }

  for (const list of lists) {
    const cards = await loaders.deck_list.read(list);

    for (const card of cards) {
      try {
        map[card].push(list);
      } catch {
        console.log(card);
      }
    }
  }
  await loaders.card_list_map.write(map);
};

export const registerMetadataCardListMap = (program: Command) => {
  program
    .command("metadata-card-list-map")
    .option("--skip-cache")
    .argument("<commander_name>")
    .action(metadataCardListMap);
};
