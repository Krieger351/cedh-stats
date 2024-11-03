import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

export const metadataWinrateCommanderAverage = async (
  commander_name: string,
) => {
  const loaders = buildLoaders(commander_name);
  const id_win_rate = await loaders.id_win_rate.read();

  let sum = 0;
  let count = 0;
  for (const winrate of Object.values(id_win_rate)) {
    sum = sum + winrate;
    count = count + 1;
  }
  const winrate_data = sum / count;

  await loaders.winrate_commander_average.write({ winrate_data });
};

export const registerMetadataWinrateCommanderAverage = (program: Command) => {
  program
    .command("metadata-winrate-commander-average")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataWinrateCommanderAverage);
};
