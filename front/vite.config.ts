import { defineConfig } from "vite";
import basicSsl from "@vitejs/plugin-basic-ssl";
import solid from "vite-plugin-solid";

export default defineConfig({
  plugins: [solid(), basicSsl({ name: "test", domains: ["localhost"] })],
  server: {
    https: true,
    proxy: {
      "/api": {
        target: "https://localhost:3001",
        changeOrigin: true,
        secure: false
      }
    }
  },
});
