import { Command } from "commander";
import { buildDataStore } from "@/lib/data-store";

const prefetchData = async (commander_name: string) => {
  const store = buildDataStore(commander_name);

  console.log("commander_entries");
  await store.commander_entries.skip();
  console.log("id_win_rate");
  const id_win_rate = await store.id_win_rate();

  console.group("Loading Lists");
  for (const [id] of id_win_rate) {
    process.stdout.write(`Current List: ${id}`);
    await store.decklist(id);
    process.stdout.clearLine(0);
    process.stdout.cursorTo(0);
  }
  console.groupEnd();
};
export const registerPrefetchData = (program: Command) => {
  program
    .command("prefetch-data")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(prefetchData);
};
