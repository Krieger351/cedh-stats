import { Command } from "commander";
import { buildDataStore } from "@/lib/data-store";
import { standardDeviation } from "@/lib/statistics/standard-deviation";
import * as console from "node:console";

const cardsInBestDecks = async (commander_name: string) => {
  const store = buildDataStore(commander_name);

  const cards = await store.cards_in_top_decks();

  // console.log(`Top ${cards.size} cards:  `);
  // console.log(
  //   Array.from(cards.values())
  //     .map((card) => `1 ${card}`)
  //     .join("\n"),
  // );

  const winrate_map = await store.winrate_cards_in_top_decks();
  const winrate = await store.winrate_top_decks();

  const standard_div = standardDeviation(Array.from(winrate_map.values()));

  const better = new Map();
  const worse = new Map();
  const within_one = new Map();

  for (const [card, rate] of winrate_map) {
    const dif = rate - winrate;
    if (Math.abs(dif) >= standard_div) {
      if (rate > winrate) {
        better.set(card, dif);
      } else {
        worse.set(card, dif);
      }
    } else {
      within_one.set(card, dif);
    }
  }

  console.group("Better");
  for (const [card, dif] of better) {
    console.log(`${card}: ${(dif * 100).toFixed(2)}%`);
  }
  console.groupEnd();

  console.group("Worse");
  for (const [card, dif] of worse) {
    console.log(`${card}: ${(dif * 100).toFixed(2)}%`);
  }
  console.groupEnd();

  console.log(winrate_map.get("Doomsday"), winrate);
};
export const registerCardsInBestDecks = (program: Command) => {
  program
    .command("cards-in-best-decks")
    .argument("<commander_name>")
    .action(cardsInBestDecks);
};
