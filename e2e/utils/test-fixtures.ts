import { test as base } from "@playwright/test";

type Fixtures = {
  testId: string;
  clientPort: string;
};

export const test = base.extend<Fixtures>({
  testId: async ({}, use) => {
    await use(crypto.randomUUID());
  },

  clientPort: async ({ testId, request }, use) => {
    // set up
    const response = await request.post("http://localhost:4000/testing/start", {
      data: { testId },
    });

    // TODO: get type from API deefs
    const { clientPort } = (await response.json()) as { clientPort: string };

    // use in tests
    await use(clientPort);

    // tear down
    await request.post("http://localhost:4000/testing/stop", {
      data: { testId },
    });
  },
});
