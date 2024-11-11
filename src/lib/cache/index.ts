import { check, read, write } from "@/lib/cache/interactions";

export const buildCache = (commander_name: string) => {
  const uri_commander_name = encodeURIComponent(commander_name);
  const build_full_key = (key: string) => `${uri_commander_name}/${key}.json`;

  return {
    check: async (key: string): Promise<boolean> =>
      await check(build_full_key(key)),
    read: async <T>(
      key: string,
      decode: (s: string) => T = JSON.parse,
    ): Promise<T> => decode(await read(build_full_key(key))),
    write: async <T>(
      key: string,
      data: T,
      encode: (data: T) => string = JSON.stringify,
    ) => await write(build_full_key(key), encode(data)),
  };
};
