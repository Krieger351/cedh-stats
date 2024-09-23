import type { Command } from "commander";
import { graphql } from "@/graphql/gql.js";
import { execute } from "@/graphql/execute.js";
import { checkCache, writeCache } from "@/lib/cache";

const CACHE_KEY = "top-commanders.json";
const query = graphql(`
  query TopCommanders {
    topCommanders {
      name
    }
  }
`);
const fetchTopCommanders = async () => {
  console.log("Fetching Top Commanders...");
  console.log("Checking cache...");
  const exists = await checkCache(CACHE_KEY);
  if (exists) {
    console.log("Data cached.");
    return;
  }
  console.log("Executing query...");
  const data = await execute(query);

  console.log("Writing to cache...");
  await writeCache(CACHE_KEY, data);

  console.log("Finished");
};

export const registerFetchTopCommanders = (program: Command) => {
  program
    .command("fetch-top-commanders")
    .description("load and store the top commanders from edhtop16.com")
    .action(fetchTopCommanders);
};
