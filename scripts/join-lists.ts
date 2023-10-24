import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

type List = string[];
type JoinedLists = Record<string, number>;
const getLists = async (): Promise<List[]> => {
  const rawLists = JSON.parse(
    (await readFile(resolve(__dirname, '../src/lib/data/lists.json'))).toString()
  );

  return rawLists;
};

const writeJoinedList = async (joinedList: JoinedLists) => {
  await writeFile(
    resolve(__dirname, '../src/lib/data/joined-lists.json'),
    JSON.stringify(joinedList, null, 2)
  );
};
const main = async () => {
  const lists = await getLists();

  const joinedList: JoinedLists = {};

  for (const list of lists) {
    for (const card of list) {
      if (card in joinedList) {
        joinedList[card] = joinedList[card] + 1;
      } else {
        joinedList[card] = 1;
      }
    }
  }
  await writeJoinedList(joinedList);
};

main();
