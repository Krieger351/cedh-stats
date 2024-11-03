import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

export const metadataUncommonCards = async (commander_name: string) => {
  const loaders = buildLoaders(commander_name);
  console.log(
    `Building lists of common and uncommon cards for ${commander_name}`,
  );

  if (
    (await loaders.uncommon_cards.check()) &&
    (await loaders.common_cards.check())
  ) {
    process.exit();
  }

  const cardListMap = await loaders.all_cards.read();

  const validIds = await loaders.valid_ids.read();

  const commonCards: string[] = [];
  const uncommonCards: string[] = [];
  for (const [card, lists] of Object.entries(cardListMap)) {
    if (lists.length >= validIds.length) {
      commonCards.push(card);
    } else {
      uncommonCards.push(card);
    }
  }

  await loaders.uncommon_cards.write(uncommonCards);
  await loaders.common_cards.write(commonCards);
};

export const registerMetadataUncommonCards = (program: Command) => {
  program
    .command("metadata-uncommon-cards")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataUncommonCards);
};
