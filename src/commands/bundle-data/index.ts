import type { Command } from "commander";
import { loadCommanderData, loadDecklist } from "@/lib/data";
import { extractMoxfieldId } from "@/lib/moxfield";

export const bundleData = async (commander: string) => {
  const cardData = {};
  const commanderData = await loadCommanderData(commander);

  const count = commanderData.length;
  const averageWinRate =
    commanderData
      ?.map((cur) => cur.winRate)
      .filter<number>((a) => typeof a === "number")
      .reduce((acc, cur) => acc + cur, 0) / count;

  console.log(`Total entries: ${count}`);
  console.log(`Average win rate: ${averageWinRate}`);

  for (const commanderDatum of commanderData) {
    const moxfieldId = extractMoxfieldId(commanderDatum.decklist!);
    const decklist = await loadDecklist(commander, moxfieldId);
    if (!decklist) {
      continue;
    }
  }
};

export const registerBundleData = (program: Command) => {
  program
    .command("bundle-data")
    .argument("<commander_name>")
    .action(bundleData);
};
