/** @type {import('tailwindcss').Config} */
export const content = {
  files: ["*.html", "./src/**/*.rs"],
};
export const darkMode = "class";
export const daisyui = {
  themes: [
    {
      mytheme: {
        "base-100": "#14213d",
        "primary": "#fca311",
        "warning": "#ff444f",
      }
    }
  ]
};
export const plugins = [require("daisyui")];
