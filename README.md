## farm-plugin-webp-to-png

A pwa rust plugin for farm

### Install

```bash
pnpm add -D farm-plugin-webp-to-png
```

### Usage

```ts
import { defineConfig } from "@farmfe/core";
import webpToPng from 'farm-plugin-webp-to-png';

export default defineConfig({
  plugins: [
    webpToPng()
  ],
});
```
