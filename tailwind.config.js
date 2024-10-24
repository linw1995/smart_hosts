module.exports = {
  darkMode: ['class', '[data-theme="night"]'],
  mode: "jit",
  content: ["./src/**/*.rs", "index.html"],
  theme: {
    extend: {
    },
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: [
      "light",
      "night"
    ],
  },
}
