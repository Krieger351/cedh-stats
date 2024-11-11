import type { Command } from "commander";
import { loadCommanderData } from "@/lib/data";
import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { buildLoaders } from "@/lib/cache-load";

export const metadataIdWinRate = async (
  commander_name: string,
  { skipCache }: { skipCache?: boolean },
) => {
  const loaders = buildLoaders(commander_name);
  console.log(`Building Id/WinRate map for ${commander_name}`);
  console.log("Checking cache...");
  if (skipCache && (await loaders.id_win_rate.check())) {
    process.exit();
  }

  console.log("Loading commander data-store");
  const commanderData = await loadCommanderData(commander_name);
  console.log("Building map");
  const acc: Record<string, number> = {};
  for (const { winRate, decklist: decklistUrl } of commanderData) {
    if (!validDecklistUrl(decklistUrl) || typeof winRate !== "number") {
      continue;
    }
    const id = extractMoxfieldId(decklistUrl);
    acc[id] = winRate;
  }
  console.log("Writing to cache");
  await loaders.id_win_rate.write(acc);
};

export const registerMetadataIdWinRate = (program: Command) => {
  program
    .command("metadata-id-win-rate-map")
    .argument("<commander_name>")
    .option("--skip-cache")
    .action(metadataIdWinRate);
};
