import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

export const metadataValidIds = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  const loaders = buildLoaders(commander_name);
  console.log(`Building list of valid ids ${commander_name}`);
  console.log("Checking cache...");

  if (skipCache && (await loaders.valid_ids.check())) {
    process.exit();
  }

  const allLists = Object.keys(await loaders.id_win_rate.read());

  const validLists: string[] = [];
  for (const list of allLists) {
    if (await loaders.deck_list.check(list)) {
      validLists.push(list);
    }
  }

  await loaders.valid_ids.write(validLists);
};

export const registerMetadataValidIds = (program: Command) => {
  program
    .command("metadata-valid-ids")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataValidIds);
};
