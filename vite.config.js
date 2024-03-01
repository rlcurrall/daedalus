import { defineConfig } from "vite";
import laravel from "laravel-vite-plugin";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [
    laravel({
      input: ["resources/css/app.css", "resources/js/app.ts"],
    }),
    react(),
  ],

  resolve: {
    alias: {
      "@": "/resources/ts",
    },
  },

  server: {
    watch: {
      ignored: ["**/target/**", "**/node_modules/**"],
    },
  },
});
