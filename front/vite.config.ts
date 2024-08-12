import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

export default defineConfig({
  plugins: [solid()],
  server: {
    proxy: {
      "/api/state": {
        target: "http://localhost:3000",
        changeOrigin: true
      }
    }
  }
})
