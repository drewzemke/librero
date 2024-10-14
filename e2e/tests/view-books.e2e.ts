import { test, expect } from "@playwright/test";

test("view main page", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByRole("heading", { name: "Librero" })).toBeVisible();
});
