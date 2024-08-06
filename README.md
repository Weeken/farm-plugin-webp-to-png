## farm-plugin-webp-to-png

A rust plugin for farm to automatically convert your webp to png, and compress all the png.

You don't need to change anything in your code, the plugin can automatically replace all the `.webp` to `.png`.

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
