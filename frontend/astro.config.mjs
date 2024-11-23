import { defineConfig } from "astro/config";
import node from "@astrojs/node";
import svelte from "@astrojs/svelte";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  integrations: [
    svelte(),
    tailwind(),
  ],
  output: "server",
  adapter: node({
    mode: "standalone",
  }),
  vite: {
    build: {
      assetsInlineLimit: 4096 * 16,
      rollupOptions: {
        output: {
          entryFileNames: "static/e[hash].js",
          chunkFileNames: "static/c[hash].js",
        },
      },
    },
    server: {
      proxy: {
        "/api": {
          target: "http://localhost",
          changeOrigin: true,
          headers: {
            "oauth-url": "http://localhost:4321",
          },
        },
      },
    },
  },
});