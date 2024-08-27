/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      gridTemplateAreas: {
        "post-listing": [
          "vote thumbnail title",
          "vote thumbnail to",
          "vote thumbnail actions",
        ],
        "post-listing-mobile": [
          "title title thumbnail",
          "to to thumbnail",
          "vote actions thumbnail",
        ],
      },
      gridTemplateColumns: {
        "post-listing": "min-content min-content auto",
        "post-listing-mobile": "min-content auto min-content",
      },
      gridAutoRows: {
        "post-listing": "1fr 2fr 1fr",
        "post-listing-mobile": "1fr 1fr 1fr",
      },
      aria: {
        "current-page": 'current="page"',
      },
      keyframes: {
        "color-cycle": {
          "0%, 100%": { color: "#f87171" },
          "6%": { color: "#fb923c" },
          "12%": { color: "#fbbf24" },
          "18%": { color: "#facc15" },
          "24%": { color: "#a3e635" },
          "30%": { color: "#4ade80" },
          "36%": { color: "#34d399" },
          "42%": { color: "#2dd4bf" },
          "48%": { color: "#22d3ee" },
          "54%": { color: "#38bdf8" },
          "60%": { color: "#60a5fa" },
          "66%": { color: "#818cf8" },
          "72%": { color: "#a78bfa" },
          "78%": { color: "#c084fc" },
          "84%": { color: "#e879f9" },
          "90%": { color: "#f472b6" },
          "95%": { color: "#fb7185" },
        },
      },
      animation: {
        "color-cycle": "color-cycle 6s linear infinite",
      },
      spacing: {
        "1/20": "5%",
        "2/20": "10%",
        "3/20": "15%",
        "4/20": "20%",
        "5/20": "25%",
        "6/20": "30%",
        "7/20": "35%",
        "8/20": "40%",
        "9/20": "45%",
        "10/20": "50%",
        "11/20": "55%",
        "12/20": "60%",
        "13/20": "65%",
        "14/20": "70%",
        "15/20": "75%",
        "16/20": "80%",
        "17/20": "85%",
        "18/20": "90%",
        "19/20": "95%",
        "1/10": "10%",
        "2/10": "20%",
        "3/10": "30%",
        "4/10": "40%",
        "5/10": "50%",
        "6/10": "60%",
        "7/10": "70%",
        "8/10": "80%",
        "9/10": "90%",
      },
    },
  },
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [require("daisyui"), require("@savvywombat/tailwindcss-grid-areas")],
  daisyui: {
    themes: ["light", "dark", "retro"],
  },
};
