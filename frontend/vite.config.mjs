import wasm from "vite-plugin-wasm";
import { defineConfig } from 'vite';
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
      wasm(),
      topLevelAwait()
  ],
  build: {
      target:"esnext",
    // Relative to the root
  },
});