import { expect } from "@playwright/test";
import { test } from "../utils/test-fixtures";

test("add a book to collection", async ({ page, clientPort }) => {
  const url = `http://localhost:${clientPort}/`;
  console.log("navigating to:", url);

  await page.goto(url);
  await expect(page.getByRole("heading", { name: "Librero" })).toBeVisible();

  await page.getByRole("button", { name: "Add Book" }).click();
  await expect(page.getByRole("listitem", { name: /book/i })).toBeVisible();
});
