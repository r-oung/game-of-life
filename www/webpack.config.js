const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "../docs"),
    filename: "bundle.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: 'public', to: "." },
      ],
    }),
  ],
  experiments: {
    syncWebAssembly: true,
  }
};
