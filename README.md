## farm-plugin-webp-to-png

A Rust plugin for Farm that can:

- convert `.webp` assets to `.png`
- rewrite `.webp` references to `.png` during production build
- compress all `.png` assets
- optionally recompress `.webp` assets as smaller WebP files when conversion is disabled

### Install

```bash
pnpm add -D farm-plugin-webp-to-png
```

### Usage

```ts
import { defineConfig } from "@farmfe/core";
import webpToPng from "farm-plugin-webp-to-png";

export default defineConfig({
  plugins: [
    webpToPng()
  ],
});
```

### Options

```ts
import { defineConfig } from "@farmfe/core";
import webpToPng from "farm-plugin-webp-to-png";

export default defineConfig({
  plugins: [
    webpToPng({
      is_convert: false,
      quality: 80,
    }),
  ],
});
```

#### `is_convert`

- type: `boolean`
- default: `false`

When `is_convert` is `true`:

- `.webp` assets are converted to `.png`
- `.webp` references in generated content are replaced with `.png`
- all `.png` assets are compressed

When `is_convert` is `false`:

- `.webp` assets stay as `.webp`
- `.webp` assets are recompressed with WebP lossy encoding
- all `.png` assets are compressed

#### `quality`

- type: `number`
- default: `80`
- range: `0 ~ 100`

Used only when `is_convert` is `false`.

A lower value gives smaller output size with lower image quality. A higher value gives better image quality with larger output size.
