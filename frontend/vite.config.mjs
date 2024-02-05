import wasm from "vite-plugin-wasm";
import { defineConfig } from 'vite';
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
      wasm(),
      topLevelAwait()
  ],
  base:"tst/",
  build: {
      target:"esnext",
    // Relative to the root
  },
});
