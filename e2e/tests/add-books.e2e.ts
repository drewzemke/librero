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

  await page.getByRole("searchbox").fill("Test Book");

  const searchResult = page.getByRole("listitem", { name: /Test Book/ });
  await searchResult.getByRole("button", { name: "Add" }).click();

  const recentAdditions = page.getByRole("list", { name: "Recent Additions" });
  await expect(recentAdditions.getByRole("listitem", { name: "Test Book" }))
    .toBeVisible();
});
