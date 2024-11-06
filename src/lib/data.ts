import { readCache } from "@/lib/cache_";
import { LoadDataForCommanderQuery } from "@/graphql/graphql";

export const loadCommanderData = async (commander: string) => {
  const commanderDataCacheKey = `commanders/${encodeURIComponent(commander)}.json`;
  const rawCommanderData = await readCache(commanderDataCacheKey);

  return JSON.parse(
    rawCommanderData,
  ) as LoadDataForCommanderQuery["commander"]["entries"];
};

export const loadDecklist = async (commander: string, id: string) => {
  const cacheKey = `decklists/${encodeURIComponent(commander)}/${id}.json`;
  try {
    const rawDecklist = await readCache(cacheKey);
    return JSON.parse(rawDecklist) as string[];
  } catch {
    return false;
  }
};
