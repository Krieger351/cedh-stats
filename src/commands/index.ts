import { Command, program } from "commander";
import { registerFetchTopCommanders } from "@/commands/fetch-top-commanders";
import { registerFetchCommanderEntries } from "@/commands/fetch-commander-entries";
import { registerFetchDecklists } from "@/commands/fetch-decklists";
import { registerBundleData } from "@/commands/bundle-data";
import { registerMetadataIdWinRate } from "@/commands/metadata-id-win-rate-map";
import { registerMetadataAllCards } from "@/commands/metadata-all-cards";

export const registerCommands = (program: Command) => {
  registerFetchTopCommanders(program);
  registerFetchCommanderEntries(program);
  registerFetchDecklists(program);
  registerBundleData(program);
  registerMetadataIdWinRate(program);
  registerMetadataAllCards(program);
};
