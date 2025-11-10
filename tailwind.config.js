module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: "#19e619",
        "background-light": "#f6f8f6",
        "background-dark": "#112111",
      },
      fontFamily: {
        display: ["Space Grotesk"],
      },
      borderRadius: {
        DEFAULT: "0.125rem",
        lg: "0.25rem",
        xl: "0.5rem",
        full: "0.75rem",
      },
    },
  },
};
