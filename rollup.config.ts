import rust from "@wasm-tool/rollup-plugin-rust";
import { defineConfig } from "rollup";

export default defineConfig({
  input: "index.js",
  output: {
    file: "dist.js",
  },
  plugins: [rust({ inlineWasm: true })],
  watch: true
});
