import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from 'farm-plugin-webp-to-png';

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
    sourcemap: false
  },
  plugins: [
    react({ runtime: "automatic" }),
    farmPlugin()
  ],
});
