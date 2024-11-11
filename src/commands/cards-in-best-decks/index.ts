import { Command } from "commander";
import { buildDataStore } from "@/lib/data-store";

const cardsInBestDecks = async (commander_name: string) => {
  const store = buildDataStore(commander_name);

  const cards = await store.cards_in_top_decks();

  console.log(`Top ${cards.size} cards:  `);
  console.log(
    Array.from(cards.values())
      .map((card) => `1 ${card}`)
      .join("\n"),
  );
};
export const registerCardsInBestDecks = (program: Command) => {
  program
    .command("cards-in-best-decks")
    .argument("<commander_name>")
    .action(cardsInBestDecks);
};
