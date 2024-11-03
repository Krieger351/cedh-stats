import { Command } from "commander";
import { registerFetchTopCommanders } from "@/commands/fetch-top-commanders";
import { registerFetchCommanderEntries } from "@/commands/fetch-commander-entries";
import { registerFetchDecklists } from "@/commands/fetch-decklists";
import { registerBundleData } from "@/commands/bundle-data";
import { registerMetadataIdWinRate } from "@/commands/metadata-id-win-rate-map";
import { registerMetadataAllCards } from "@/commands/metadata-all-cards";
import { registerMetadataCardListMap } from "@/commands/metadata-card-list-map";
import { registerMetadataValidIds } from "@/commands/metadata-valid-ids";
import { registerMetadataAllPairs } from "@/commands/metadata-all-pairs";
import { registerMetadataUncommonCards } from "@/commands/metadata-uncommon-cards";
import { registerMetadataWinrateUncommonCards } from "@/commands/metadata-winrate-uncommon-cards";
import { registerMetadataWinrateCommanderAverage } from "@/commands/metadata-winrate-commander-average";
import { registerMetadataCardsInAboveAverageDecks } from "@/commands/metadata-cards-in-above-average-lists";

export const registerCommands = (program: Command) => {
  registerFetchTopCommanders(program);
  registerFetchCommanderEntries(program);
  registerFetchDecklists(program);
  registerBundleData(program);
  registerMetadataIdWinRate(program);
  registerMetadataAllCards(program);
  registerMetadataCardListMap(program);
  registerMetadataValidIds(program);
  registerMetadataAllPairs(program);
  registerMetadataUncommonCards(program);
  registerMetadataWinrateUncommonCards(program);
  registerMetadataWinrateCommanderAverage(program);
  registerMetadataCardsInAboveAverageDecks(program);
};
