import { graphql } from "@/graphql";
import type { Command } from "commander";
import { checkCache, writeCache } from "@/lib/cache_";
import { execute } from "@/graphql/execute";
import { filterOutliers } from "@/lib/filter-outliers";
import { CommanderEntriesQuery } from "@/graphql/graphql";

const query = graphql(`
  query CommanderEntries($name: String!, $filters: EntryFilters) {
    commander(name: $name) {
      entries(filters: $filters) {
        decklist
        winRate
      }
    }
  }
`);

const filterInvalidEntries = (
  entries: CommanderEntriesQuery["commander"]["entries"],
) =>
  entries.filter(
    ({ decklist, winRate }) =>
      typeof winRate === "number" && typeof decklist === "string",
  );

export const fetchCommanderEntries = async (
  commander_name: string,
): Promise<void> => {
  console.log(`Fetching data for ${commander_name}...`);

  console.log("Checking cache...");
  const CACHE_KEY = `commanders/${encodeURIComponent(commander_name)}.json`;
  const exists = await checkCache(CACHE_KEY);

  if (exists) {
    console.log("Data cached.");
    return;
  }

  console.log("Executing query...");
  const result = await execute(query, {
    name: commander_name,
    filters: {
      minSize: 64,
      minDate: "2024/01/01",
    },
  });

  console.log("Filtering outliers...");
  const data = filterOutliers(
    filterInvalidEntries(result.commander.entries),
    (d) => d!.winRate as number,
  );

  console.log("Writing to cache...");
  writeCache(CACHE_KEY, data);

  console.log("Finished");
};

export const registerFetchCommanderEntries = (program: Command) => {
  program
    .command("fetch-commander-entries")
    .argument("<commander_name>")
    .action(fetchCommanderEntries);
};
