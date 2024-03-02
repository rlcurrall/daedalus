/** @type {import('tailwindcss').Config} */
export default {
  content: ["./resources/**/*.{ts,tsx,njk}"],
  theme: {
    extend: {},
  },
  plugins: [require("autoprefixer")],
};
