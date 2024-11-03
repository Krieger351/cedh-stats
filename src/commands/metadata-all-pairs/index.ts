import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

export const metadataAllPairs = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
): Promise<void> => {
  const loaders = buildLoaders(commander_name);
  console.log(`Building set of all pairs of cards for ${commander_name}`);

  if (skipCache && (await loaders.all_pairs.check())) {
    process.exit();
  }
  const uncommonCards = await loaders.uncommon_cards.read();

  const length = uncommonCards.length;

  const pairs: [string, string][] = [];
  for (let i = 0; i < length - 1; i++) {
    for (let j = i + 1; j < length; j++) {
      pairs.push([uncommonCards[i], uncommonCards[j]]);
    }
  }

  console.log(`Writing all ${pairs.length} pairs.`);
  await loaders.all_pairs.write(pairs);
};

export const registerMetadataAllPairs = (program: Command) => {
  program
    .command("metadata-all-pairs")
    .option("--skip-cache")
    .argument("<commander_name>")
    .action(metadataAllPairs);
};
