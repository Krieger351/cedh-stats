import path from "node:path";
import fs from "node:fs/promises";
import * as prettier from "prettier";

const cachePath = path.resolve(process.cwd(), ".cache");

export const check = async (cacheId: string): Promise<boolean> => {
  try {
    await fs.access(path.resolve(cachePath, cacheId));
  } catch {
    return false;
  }
  return true;
};

export const write = async (cacheId: string, data: unknown): Promise<void> => {
  const filePath = path.resolve(cachePath, cacheId);
  const dirname = path.dirname(filePath);

  await fs.mkdir(dirname, { recursive: true });

  await fs.writeFile(
    filePath,
    await prettier.format(JSON.stringify(data), { parser: "json" }),
  );
};

export const read = async (cacheId: string): Promise<string> => {
  const buffer = await fs.readFile(path.resolve(cachePath, cacheId));

  return buffer.toString();
};
