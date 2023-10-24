import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const loadCardData = async () => {
  try {
    const cardDataBuffer = await readFile(resolve(__dirname, '../src/lib/data/card-data.json'));
    return JSON.parse(cardDataBuffer.toString());
  } catch (e) {
    console.log(e);
    return {};
  }
};

const loadJoinedList = async () => {
  const joinedListsBuffer = await readFile(resolve(__dirname, '../src/lib/data/joined-lists.json'));
  return JSON.parse(joinedListsBuffer.toString());
};

const wait = () => new Promise<void>((resolve) => setTimeout(() => resolve(), 100));

const writeCardData = async (joinedList) => {
  await writeFile(
    resolve(__dirname, '../src/lib/data/card-data.json'),
    JSON.stringify(joinedList, null, 2)
  );
};
const main = async () => {
  const cardData = await loadCardData();
  const joinedList = await loadJoinedList();
  for (const card in joinedList) {
    if (!(card in cardData)) {
      const rawCardData = await fetch(`https://api.scryfall.com/cards/named?exact=${card}`);
      const cardJson = await rawCardData.json();

      cardData[card] = cardJson;
      await wait();
      console.log(`Fetched ${card}`);
    } else {
      console.log(`Skipping ${card}`);
    }
  }
  await writeCardData(cardData);
};

main();
