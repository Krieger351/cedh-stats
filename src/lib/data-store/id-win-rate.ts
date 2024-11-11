import { extractMoxfieldId, validDecklistUrl } from "@/lib/moxfield";
import { fetchCommanderData } from "@/lib/data-store/commander-data";

export const transformIdWinRate = (
  commanderData: Awaited<ReturnType<typeof fetchCommanderData>>,
) => {
  const acc = new Map<string, number>();
  for (const { winRate, decklist: decklistUrl } of commanderData) {
    if (!validDecklistUrl(decklistUrl)) {
      continue;
    }
    const id = extractMoxfieldId(decklistUrl);
    acc.set(id, winRate);
  }
  return acc;
};
