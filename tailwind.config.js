const defaultTheme = require('tailwindcss/defaultTheme');

module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
      extend: { 
        colors: {
          'custom-green-dark': '#025259',
          'custom-green': '#007172',
          'custom-pink-cream': '#F4E2DE',
          'custom-orange-light': '#F29325',
          'custom-orange-dark': '#D94F04'

        }
      },
  },
  plugins: [],
}
