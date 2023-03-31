const buildMode = process.env.BUILD_MODE;

module.exports = {
  plugins: [
    require('postcss-import')(),
    require( "postcss-url" )(),
    require('tailwindcss/nesting')(),
    require('tailwindcss')(),
    require('postcss-reporter')({ clearReportedMessages: true }),
    buildMode === 'release' ? require('autoprefixer')() : null,
    buildMode === 'release' ? require('cssnano')({ preset: 'default' }) : null,
  ],
};
