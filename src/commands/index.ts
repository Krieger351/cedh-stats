import { Command } from "commander";
import { registerFetchTopCommanders } from "@/commands/fetch-top-commanders";
import { registerFetchDecklists } from "@/commands/fetch-decklists";
import { registerBundleData } from "@/commands/bundle-data";
import { registerMetadataIdWinRate } from "@/commands/metadata-id-win-rate-map";
import { registerMetadataCardListMap } from "@/commands/metadata-card-list-map";
import { registerMetadataValidIds } from "@/commands/metadata-valid-ids";
import { registerMetadataAllPairs } from "@/commands/metadata-all-pairs";
import { registerMetadataUncommonCards } from "@/commands/metadata-uncommon-cards";
import { registerMetadataWinrateUncommonCards } from "@/commands/metadata-winrate-uncommon-cards";
import { registerMetadataWinrateCommanderAverage } from "@/commands/metadata-winrate-commander-average";
import { registerMetadataCardsInAboveAverageDecks } from "@/commands/metadata-cards-in-above-average-lists";
import { registerPrefetchData } from "@/commands/prefetch-data";

const commands = [
  // registerFetchTopCommanders,
  // registerFetchCommanderEntries,
  // registerFetchDecklists,
  // registerBundleData,
  // registerMetadataIdWinRate,
  // registerMetadataAllCards,
  // registerMetadataCardListMap,
  // registerMetadataValidIds,
  // registerMetadataAllPairs,
  // registerMetadataUncommonCards,
  // registerMetadataWinrateUncommonCards,
  // registerMetadataWinrateCommanderAverage,
  // registerMetadataCardsInAboveAverageDecks,
  registerPrefetchData,
];

export const registerCommands = (program: Command) => {
  commands.forEach((registerCommand) => registerCommand(program));
};
