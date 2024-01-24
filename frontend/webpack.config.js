const path = require('path');

module.exports = {
    experiments: {
        asyncWebAssembly: true,
    },
  entry: './index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
      open: true,
      static: {
          directory: path.join(__dirname,'public'),
      },
      //mimeTypes: { 'application/wasm': ['wasm'] },
      compress: true,
      port: 9000,
  }
};
