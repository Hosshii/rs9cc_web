const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      static : {
        directory: distPath,
      },
      compress: argv.mode === 'production',
      port: 8000,
      host: '0.0.0.0',
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "todomvc.js",
      webassemblyModuleFilename: "todomvc.wasm"
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
      ],
    },
    experiments: {
      asyncWebAssembly: true, 
    },
    plugins: [
      new CopyWebpackPlugin({patterns: [
        { from: './static', to: distPath }
      ]}),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
