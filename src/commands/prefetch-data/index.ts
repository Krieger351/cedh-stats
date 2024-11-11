import { Command } from "commander";
import { buildDataStore } from "@/lib/data-store";
import { mean } from "@/lib/statistics/mean";
import { standardDeviation } from "@/lib/statistics/standard-deviation";

const prefetchData = async (commander_name: string) => {
  const store = buildDataStore(commander_name);

  await store.commander_data();
  const id_win_rate = await store.id_win_rate();

  for (const id of id_win_rate.keys()) {
    await store.decklist(id);
  }

  console.log(await store.cards_in_top_decks.skip(0.5));
  // const card_list_map = await store.card_list_map();
};
export const registerPrefetchData = (program: Command) => {
  program
    .command("prefetch-data")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(prefetchData);
};
