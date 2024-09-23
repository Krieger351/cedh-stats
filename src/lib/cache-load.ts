import { readCache } from "@/lib/cache";
import {
  allCardsKey,
  cardListMapKey,
  decklistKey,
  idWinRateKey,
  uncommonCardsKey,
  validIdsKey,
} from "@/lib/cache-keys";

export const loadDecklist = async (
  commander_name: string,
  id: string,
): Promise<string[]> => {
  const rawData = await readCache(decklistKey(commander_name, id));
  return JSON.parse(rawData);
};

export const loadAllLists = async (commander_name: string) =>
  Object.keys(
    JSON.parse(await readCache(idWinRateKey(commander_name))),
  ) as string[];

export const loadALlCards = async (commander_name: string) =>
  JSON.parse(await readCache(allCardsKey(commander_name))) as string[];

export const loadUncommonCards = async (commander_name: string) =>
  JSON.parse(await readCache(uncommonCardsKey(commander_name))) as string[];

export const loadValidLists = async (commander_name: string) =>
  JSON.parse(await readCache(validIdsKey(commander_name))) as string[];

export const loadCardListMap = async (commander_name: string) =>
  JSON.parse(await readCache(cardListMapKey(commander_name))) as Record<
    string,
    string[]
  >;
export const loadIdWinRate = async (commander_name: string) =>
  JSON.parse(await readCache(idWinRateKey(commander_name))) as Record<
    string,
    number
  >;
