import { test, expect } from "@playwright/test";

test("add a book to collection", async ({ page }) => {
  const url = `http://localhost:${process.env.CLIENT_PORT}/`;
  console.log(url);

  await page.goto(url);
  await expect(page.getByRole("heading", { name: "Librero" })).toBeVisible();

  await page.getByRole("button", { name: "Add Book" }).click();
  await expect(page.getByRole("listitem", { name: /book/i })).toBeVisible();
});
