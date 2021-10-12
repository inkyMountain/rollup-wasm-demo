import rust from "@wasm-tool/rollup-plugin-rust";
import { defineConfig } from "rollup";
import { babel } from "@rollup/plugin-babel";

// @ts-ignore
export default defineConfig({
  input: "index.js",
  output: {
    file: "dist.js",
    format: "cjs",
  },
  plugins: [
    rust({ inlineWasm: false, nodejs: true }),
    babel({ babelHelpers: "runtime" }),
  ],
});
