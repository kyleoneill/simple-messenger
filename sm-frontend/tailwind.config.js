/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'media',
  theme: {
    extend: {
      colors: {
        void: {
          0: '#0E0B16'
        },
        fuschia: {
          0: '#A239CA'
        },
        jewel: {
          0: '#4717F6'
        },
        stark: {
          0: '#E7DFDD'
        }
      }
    },
  },
  plugins: [],
}
