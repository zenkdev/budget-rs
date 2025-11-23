module.exports = {
  globDirectory: "dist/.stage",
  globPatterns: ["**/*.{js,wasm,css,html,json,ico,png,ttf}"],
  swDest: "dist/.stage/sw.js",
  ignoreURLParametersMatching: [/^utm_/, /^fbclid$/],
  sourcemap: false,
  maximumFileSizeToCacheInBytes: 10 * 1024 * 1024, // 10MB
};
