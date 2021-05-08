const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const basePath = path.resolve(__dirname, './');
module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
  devServer: {
    hot: true,
    watchOptions: {
      poll: true,
      ignored: [/node_modules/, /static/]
    },
    contentBase: path.relative(basePath, 'static/'),
  },
};
