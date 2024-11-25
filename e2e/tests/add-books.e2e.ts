import { expect } from "@playwright/test";
import { test } from "../utils/test-fixtures.ts";

test("add a book to collection", async ({ page, clientPort }) => {
  await page.route(/openlibrary/, (route) => {
    // TODO: try using `schemars` and `json-schema-to-typescript`
    // make this mock more strongly-typed
    route.fulfill({
      json: {
        docs: [{
          title: "Test Book",
          author_name: ["Test Author"],
          author_key: ["test-author"],
          isbn: ["1111111111"],
        }],
      },
    });
  });

  await page.goto(`http://localhost:${clientPort}/`);
  await expect(page.getByRole("heading", { name: "Librero" })).toBeVisible();

  await page.getByRole("link", { name: "My Library" }).click();

  await page.getByRole("button", { name: "Add Book" }).click();
  await page.getByRole("searchbox").fill("Test Book");

  const searchResult = page.getByRole("listitem", { name: /Test Book/ });
  await searchResult.getByRole("button", { name: "Add" }).click();

  await expect(page.getByRole("searchbox")).toBeHidden();

  const bookList = page.getByRole("list", { name: "My Books" });
  await expect(bookList.getByRole("listitem", { name: "Test Book" }))
    .toBeVisible();
});
