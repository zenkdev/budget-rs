module.exports = {
  globDirectory: "dist/.stage",
  globPatterns: ["**/*.{js,wasm,css,html,json,ico,png,ttf}"],
  swDest: "dist/.stage/sw.js",
  ignoreURLParametersMatching: [/^utm_/, /^fbclid$/],
  sourcemap: false,
  maximumFileSizeToCacheInBytes: 10 * 1024 * 1024, // 10MB
  runtimeCaching: [
    {
      urlPattern:
        "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined",
      handler: "CacheFirst",
      options: {
        cacheName: "cdn-cache-workbox-cli",
        // Ensure only valid responses (e.g., status 200) are cached
        cacheableResponse: {
          statuses: [0, 200],
        },
        // Optional: configure expiration
        expiration: {
          maxEntries: 60, // Maximum number of entries
          maxAgeSeconds: 30 * 24 * 60 * 60, // 30 days
        },
      },
    },
    {
      urlPattern: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4",
      handler: "CacheFirst",
      options: {
        cacheName: "cdn-cache-workbox-cli",
        // Ensure only valid responses (e.g., status 200) are cached
        cacheableResponse: {
          statuses: [0, 200],
        },
        // Optional: configure expiration
        expiration: {
          maxEntries: 60, // Maximum number of entries
          maxAgeSeconds: 30 * 24 * 60 * 60, // 30 days
        },
      },
    },
  ],
};
