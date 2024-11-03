import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";

const calc_average = <T extends string>(
  lists: Set<T>,
  id_win_rate: Record<T, number>,
) => {
  let sum = 0;
  let count = 1;
  for (const list of lists.values()) {
    sum = sum + id_win_rate[list];
    count = count + 1;
  }
  return sum / count;
};

export const metadataWinrateUncommonCards = async (commander_name: string) => {
  const loaders = buildLoaders(commander_name);
  const uncommon_cards = await loaders.uncommon_cards.read();
  const card_list_map = await loaders.card_list_map.read();
  const id_win_rate = await loaders.id_win_rate.read();

  const set_lists = new Set(Object.keys(id_win_rate));

  const winrate_data: Record<
    string,
    {
      contains_average: number;
      excludes_average: number;
      contains_count: number;
      excludes_count: number;
    }
  > = {};

  for (const card of uncommon_cards) {
    const set_contains = new Set(card_list_map[card]);
    const set_excludes = set_lists.difference(set_contains);
    const contains_average = calc_average(set_contains, id_win_rate);
    const excludes_average = calc_average(set_excludes, id_win_rate);

    winrate_data[card] = {
      contains_average,
      excludes_average,
      contains_count: set_contains.size,
      excludes_count: set_excludes.size,
    };
  }

  await loaders.winrate_uncommon_cards.write(winrate_data);
};

export const registerMetadataWinrateUncommonCards = (program: Command) => {
  program
    .command("metadata-winrate-uncommon-cards")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataWinrateUncommonCards);
};
