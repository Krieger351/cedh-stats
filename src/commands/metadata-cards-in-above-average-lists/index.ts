import { Command } from "commander";
import { buildLoaders } from "@/lib/cache-load";
import { getQuartiles } from "@/lib/filter-outliers";
import * as console from "node:console";

export const metadataCardsInAboveAverageDecks = async (
  commander_name: string,
) => {
  const loaders = buildLoaders(commander_name);
  const card_list_map = await loaders.card_list_map.read();
  const id_win_rate = await loaders.id_win_rate.read();
  const { winrate_data } = await loaders.winrate_commander_average.read();

  const set_of_above_average_decks = new Set();
  const cards_in_above_average_decks = new Map();
  const { Q1 } = getQuartiles(
    Object.entries(id_win_rate),
    ([_, number]) => number,
  );
  for (const [id, win_rate] of Object.entries(id_win_rate)) {
    if (win_rate >= Q1 && win_rate > winrate_data) {
      set_of_above_average_decks.add(id);
    }
  }
  for (const [card, list] of Object.entries(card_list_map)) {
    const intersection = set_of_above_average_decks.intersection(new Set(list));
    if (intersection.size > 5) {
      cards_in_above_average_decks.set(card, intersection.size);
    }
  }

  const set_of_common = new Set<string>();
  const number_of_above_average_decks = set_of_above_average_decks.size;
  cards_in_above_average_decks.forEach((value, card) => {
    if (value === number_of_above_average_decks) {
      set_of_common.add(card);
      cards_in_above_average_decks.delete(card);
    }
  });

  const cards_not_in_all_above_average_decks = Array.from(
    cards_in_above_average_decks.entries(),
  );

  console.log(`
Common Cards: (${set_of_common.size})
============
${Array.from(set_of_common.values()).join("\n")}
  
Uncommon Cards: (${cards_not_in_all_above_average_decks.length})
==============
${Array.from(cards_not_in_all_above_average_decks.entries().map(([_, [card]]) => card)).join("\n")}
`);
};

export const registerMetadataCardsInAboveAverageDecks = (program: Command) => {
  program
    .command("metadata-cards-in-above-average-decks")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataCardsInAboveAverageDecks);
};
