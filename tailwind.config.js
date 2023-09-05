/** @type {import('tailwindcss').Config} */
export const content = {
  files: ["*.html", "./src/**/*.rs"],
};
export const darkMode = "class";
export const theme = {
  extend: {
    colors: {
      primaryBg: "#14213d", 

      // BUTTONS
      primaryBtn: "#fca311", 
      secondaryBtn: "#219ebc",
      warnBtn: "#ff444f",

      primaryBtnHover: "#ed9505", 
      warnBtnHover: "#e63946",

    },
  }, 
};
export const plugins = [];
