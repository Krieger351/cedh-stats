import { graphql } from "@/graphql";
import { execute } from "@/graphql/execute";
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
    (entry): entry is { decklist: string; winRate: number } =>
      typeof entry.winRate === "number" && typeof entry.decklist === "string",
  );

export const fetchCommanderData = async (commander_name: string) => {
  const result = await execute(query, {
    name: commander_name,
    filters: {
      minSize: 64,
      minDate: "2024/01/01",
    },
  });
  return filterInvalidEntries(result.commander.entries);
};
