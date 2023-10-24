import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import puppeteer, { Browser } from 'puppeteer';

const loadUrls = async () => {
  const rawText = await readFile(resolve(__dirname, '../src/lib/data/list-urls.txt'));
  return rawText
    .toString()
    .split('\n')
    .map((cur) => cur.trim());
};

const openBrowser = async () => {
  const browser = await puppeteer.launch({ headless: false });

  const context = browser.defaultBrowserContext();
  await context.overridePermissions('https://www.moxfield.com/', ['clipboard-read']);
  return browser;
};

const sanitizeList = (list: string) =>
  list
    .split('\n')
    .map((cur) => cur.trim())
    .slice(0, 98)
    .map((card) => card.replace('1 ', ''));
const getList = (browser: Browser) => async (url: string) => {
  const page = await browser.newPage();
  await page.goto(url, {
    waitUntil: 'networkidle0'
  });
  await page.click('#subheader-more');
  await page.click('body > div.dropdown-menu.show > div > div > div > a:nth-child(1)');
  await page.waitForSelector('div.modal-body > button:nth-child(4)');
  await page.click('div.modal-body > button:nth-child(4)');

  const list = await page.evaluate(() => navigator.clipboard.readText());
  await page.close();

  return sanitizeList(list);
};
const main = async () => {
  const files = await loadUrls();

  const browser = await openBrowser();

  const lists = await Promise.all(files.map(getList(browser)));

  await browser.close();

  await writeFile(resolve(__dirname, '../src/lib/data/lists.json'), JSON.stringify(lists, null, 2));
};

main();
