/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        "base-100": "#D9E4E9",
        "base-200": "#AAB8C2",
        "base-300": "#6B7B8C",
        "base-400": "#42526E",
        "base-500": "#233040",
        "base-600": "#19212A",
      },
    },
  },
  plugins: [],
};
