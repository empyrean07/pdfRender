/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        academic: {
          50: '#f4f6f9',
          100: '#e9ecef',
          800: '#1b2a4a',
          900: '#0f172a',
        }
      }
    },
  },
  plugins: [],
}
