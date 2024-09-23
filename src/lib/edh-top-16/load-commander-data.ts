import { graphql } from "@/graphql";
import { checkCache, readCache } from "@/lib/cache";
import { execute } from "@/graphql/execute";
import { LoadDataForCommanderQuery } from "@/graphql/graphql";

export type CommanderData = {
  decklist: string;
  winRate: number;
}[];

const validateDecklist = (decklist: unknown): decklist is string =>
  typeof decklist === "string" &&
  decklist.includes("moxfield") &&
  !decklist.includes("compare");

const validateWinrate = (winrate: unknown) => typeof winrate === "number";

const filterQuery = ({
  commander: { entries },
}: LoadDataForCommanderQuery): CommanderData => {
  const commanderData = [];

  for (const { decklist, winRate } of entries) {
    if (validateDecklist(decklist) && validateWinrate(winRate)) {
      commanderData.push({ decklist, winRate });
    }
  }
  return commanderData;
};

const query = graphql(`
  query LoadDataForCommander($name: String!, $filters: EntryFilters) {
    commander(name: $name) {
      entries(filters: $filters) {
        decklist
        winRate
      }
    }
  }
`);

export const loadCommanderData = async (
  commanderName: string,
): Promise<CommanderData> => {
  const CACHE_KEY = `commanders/${encodeURIComponent(commanderName)}.json`;

  console.log(`Loading data for ${commanderName}`);
  const exists = await checkCache(CACHE_KEY);

  if (exists) {
    console.log("Cached data found, returning");
    const rawCacheData = await readCache(CACHE_KEY);
    return JSON.parse(rawCacheData) as CommanderData;
  }

  console.log("No cached data found, executing query");

  const result = await execute(query, {
    name: commanderName,
    filters: {
      minSize: 64,
      minDate: "2024/01/01",
    },
  });

  return filterQuery(result);
};
