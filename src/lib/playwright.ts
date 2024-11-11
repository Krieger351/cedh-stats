import { chromium } from "playwright";
import { parseList } from "@/lib/moxfield";

export const getList = async (url: string) => {
  const browser = await chromium.launch({ headless: true });

  const context = await browser.newContext();
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);

  const page = await context.newPage();

  await page.goto(url);

  await page.waitForLoadState("networkidle");

  if (page.url().includes("/404")) {
    console.log("Not found");
    await browser.close();
    return false;
  }

  try {
    await page.getByLabel("More Options").click();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await page.getByRole("button", { name: "Copy for MTGO" }).click();

    const handle = await page.evaluateHandle(() =>
      // @ts-ignore
      navigator.clipboard.readText(),
    );
    const rawList = await handle.jsonValue();

    await browser.close();
    return parseList(rawList);
  } catch {
    await browser.close();
    return false;
  }
};
