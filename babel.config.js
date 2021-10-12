module.exports = {
  presets: [["@babel/preset-env", { useBuiltIns: false }]],
  plugins: [
    require("@babel/plugin-syntax-import-meta"),
    ["@babel/transform-runtime"],
  ],
};
