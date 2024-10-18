import { defineConfig, createLogger } from "vite";
import react from "@vitejs/plugin-react";
import "dotenv/config";

console.log(process.env.VITE_SERVER_PORT);

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
    port: Number(process.env.VITE_CLIENT_PORT) || 5173,
    proxy: {
      "/api": {
        target: `http://localhost:${process.env.VITE_SERVER_PORT}`,
      },
      "/covers": {
        target: `http://localhost:${process.env.VITE_SERVER_PORT}`,
      },
    },
  },
});
