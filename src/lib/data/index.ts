import { buildCache } from "@/lib/cache";
import { fetchCommanderData } from "@/lib/data/fetch-commander-data";

export const buildDataStore = (commander_name: string) => {
  const cache = buildCache(commander_name);

  return {
    commander_data: async () => {
      if (await cache.commander_data.check()) {
        return await cache.commander_data.read();
      }
      const data = await fetchCommanderData(commander_name);
      //   await cache.commander_data.write(data)
      // return data
    },
  };
};
