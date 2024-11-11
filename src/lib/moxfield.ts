import path from "node:path";

export const parseList = (list: string) => {
  const cards = [];

  const regex = /^\d+ (.+)/g;

  for (const line of list.split("\n")) {
    if (line.trim().length === 0) {
      return cards;
    }
    cards.push(line.trim().replace(/^\d+ /, ""));
  }
  return cards;
};

export const extractMoxfieldId = (url: string): string =>
  url.trim().replace(/\/$/, "").split("/").slice(-1)[0];

export const validDecklistUrl = (url: unknown): url is string =>
  typeof url === "string" && !url.includes("compare");

export const buildDecklistUrl = (id: string): string =>
  `https://www.moxfield.com/decks/${id}`;
