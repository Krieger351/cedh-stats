import type { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

export const bundleData = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  const loaders = buildLoaders(commander_name);

  const cardData: Record<string, { contains: number; excludes: number }> = {};
  const idWinRate = await loaders.id_win_rate.read();

  const validIdsSet = new Set(await loaders.valid_ids.read());

  const cardListMap = await loaders.card_list_map.read();

  const calcSetAverage = (ids: Set<string>) => {
    const numbers = [...ids].map((id) => idWinRate[id]);
    const sum = numbers.reduce((acc, cur) => acc + cur, 0);
    return sum / numbers.length;
  };
  const count = validIdsSet.size;
  const averageWinRate = calcSetAverage(validIdsSet);

  console.log(`Total entries: ${count}`);
  console.log(`Average win rate: ${averageWinRate}`);

  for (const card in cardListMap) {
    const containsCard = new Set(cardListMap[card]);
    const excludesCard = validIdsSet.difference(containsCard);
    cardData[card] = {
      contains: calcSetAverage(containsCard),
      excludes: calcSetAverage(excludesCard),
    };
  }
  await loaders.bundle_data.write({
    averageWinRate,
    totalEntries: count,
    cardData,
  });
};

export const registerBundleData = (program: Command) => {
  program
    .command("bundle-data")
    .option("--skip-cache")
    .argument("<commander_name>")
    .action(bundleData);
};
