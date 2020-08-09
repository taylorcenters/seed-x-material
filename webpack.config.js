const path = require('path');
const autoprefixer = require('autoprefixer');
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = [
  {
    entry: './app.scss',
    output: {
      // This is necessary for webpack to compile
      // But we never use style-bundle.js
      filename: 'style-bundle.js',
    },
    mode: 'development',
    module: {
      rules: [{
        test: /\.scss$/,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: 'bundle.css',
            },
          },
          { loader: 'extract-loader' },
          { loader: 'css-loader' },
          {
            loader: 'postcss-loader',
            options: {
              plugins: () => [autoprefixer()],
            },
          },
          {
            loader: 'sass-loader',
            options: {
              includePaths: ['./node_modules'],
              implementation: require('dart-sass'),
              fiber: require('fibers'),
            }
          },
        ]
      }]
    },
  },
  {
    entry: ["@babel/polyfill", "./app.js"],
    output: {
      filename: "bundle.js"
    },
    mode: 'development',
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "app-rs"),
        args: "--log-level warn",
        extraArgs: "--target bundler",
        withTypescript: true,
        outDir: path.resolve(__dirname, "pkg"),
        outName: "package",
        forceMode: "development"
      })
    ],
    module: {
      rules: [
        {
          test: /\.js$/,
          use: [
            {
              loader: 'babel-loader',
              query: {
                presets: ["@babel/preset-env"]
              }
            }
          ]
        }
      ]
    },
    resolve: {
      extensions: ['.wasm', '.js', '.ts', '.tsx', '.json']
    }
  }
];
