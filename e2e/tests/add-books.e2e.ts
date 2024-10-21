import { expect } from "@playwright/test";
import { test } from "../utils/test-fixtures";
import { execSync } from "child_process";

// todo: turn into something that runs before the whole test run
test.beforeAll(async () => {
  execSync("deno task --cwd ../client build --outDir=../server/assets/");
});

test("add a book to collection", async ({ page, clientPort }) => {
  const url = `http://localhost:${clientPort}/`;

  await page.goto(url);
  await expect(page.getByRole("heading", { name: "Librero" })).toBeVisible();

  await page.getByRole("button", { name: "Add Book" }).click();
  await expect(page.getByRole("listitem", { name: /book/i })).toBeVisible();
});
