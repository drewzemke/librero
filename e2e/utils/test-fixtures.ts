import { test as base } from "@playwright/test";

// TODO: get this from an env file?
const SERVER_URL = "http://localhost:3000";

type Fixtures = {
  testId: string;
  clientPort: string;
};

export const test = base.extend<Fixtures>({
  // deno-lint-ignore no-empty-pattern
  testId: async ({}, use) => {
    await use(crypto.randomUUID());
  },

  clientPort: async ({ testId, request }, use) => {
    // set up
    const response = await request.post(`${SERVER_URL}/testing/start`, {
      data: { testId },
    });

    // TODO: get type from API deefs
    const { clientPort } = (await response.json()) as { clientPort: string };

    // use in tests
    await use(clientPort);

    // tear down
    await request.post(`${SERVER_URL}/testing/stop`, {
      data: { testId },
    });
  },
});
