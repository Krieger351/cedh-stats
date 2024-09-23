import { checkCache } from "@/lib/cache";

export const checkCacheKeysExit = async (
  skipCache: boolean | undefined,
  ...keys: string[]
) => {
  if (skipCache) {
    return;
  }
  console.log("Checking cache...");
  const allKeys = await Promise.all(keys.map(checkCache));

  if (allKeys.every((a) => a)) {
    console.log("Data cached.");
    process.exit();
  }
};
