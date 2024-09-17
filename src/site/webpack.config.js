const path = require('path');
const webpack = require('webpack');
const CopyPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const dotenv = require('dotenv').config({
  path: path.join(__dirname, '.env')
});

module.exports = () => {

  const environment = process.env.NODE_ENV ?? 'development';

  console.log(`Building version: ${dotenv.parsed.VERSION}`);
  console.log(`Environment: ${environment}`);

  return {
    entry: [ "./js/index.js", "./js/cipher.js", "./js/cookieconsent.js", "./style.css" ],
    output: {
      path: path.resolve(__dirname, "dist"),
      filename: "bundle.js",
      publicPath: "auto",
      clean: true
    },
    mode: environment,
    experiments: {
      syncWebAssembly: true
    },
    devServer: {
    },
    module: {
      rules: [
        {
          test: /.s?css$/,
          use: [MiniCssExtractPlugin.loader, "css-loader"],
        },
      ],
    },
    optimization: {
      minimizer: [
        `...`,
        new CssMinimizerPlugin(),
      ],
    },    
    plugins: [
      new webpack.DefinePlugin({
        APP_VERSION: JSON.stringify(dotenv.parsed.VERSION)
      }),
      new MiniCssExtractPlugin(),
      new CopyPlugin({
        "patterns": [
        { from: 'assets', to: 'assets' },
        { from: 'cipher.html', to: 'cipher.html' },
        { from: 'index.html', to: 'index.html' },
      ]})
    ]
  };
};