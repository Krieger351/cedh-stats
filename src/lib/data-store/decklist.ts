import { buildDecklistUrl } from "@/lib/moxfield";
import { getList } from "@/lib/playwright";

export const fetchDecklist = async (id: string) => {
  return await getList(buildDecklistUrl(id));
};
