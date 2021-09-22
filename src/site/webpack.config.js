const path = require('path');
const webpack = require('webpack');
const dotenv = require('dotenv').config({
  path: path.join(__dirname, '.env')
});

module.exports = () => {

  console.log(`Building version: ${dotenv.parsed.VERSION}`);

  return {
    entry: [ "./js/index.js", "./js/cipher.js" ],
    output: {
      path: path.resolve(__dirname, "dist"),
      filename: "bundle.js",
      publicPath: "/dist/"
    },
    mode: "development",
    plugins: [
      new webpack.DefinePlugin({
        APP_VERSION: JSON.stringify(dotenv.parsed.VERSION)
      })
    ]
  };
};