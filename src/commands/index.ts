import type { Command } from "commander";
import { registerPrefetchData } from "@/commands/prefetch-data";
import { registerCardsInBestDecks } from "@/commands/cards-in-best-decks";

const commands = [registerCardsInBestDecks, registerPrefetchData];

export const registerCommands = (program: Command) => {
  commands.forEach((registerCommand) => registerCommand(program));
};
