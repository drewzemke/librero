import { defineConfig, createLogger } from "vite";
import react from "@vitejs/plugin-react";

const logger = createLogger();
logger.info = (msg) => {
  console.info(msg);
};
logger.warn = (msg) => {
  console.warn(msg);
};
logger.error = (msg) => {
  console.error(msg);
};
logger.clearScreen = () => {};

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  customLogger: logger,
  build: {
    assetsDir: ".",
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:4000",
      },
      "/covers": {
        target: "http://localhost:4000",
      },
    },
  },
});
