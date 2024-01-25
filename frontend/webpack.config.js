const path = require('path');

module.exports = {
resolve: {
  fallback: {
    "fs": false,
    "tls": false,
    "net": false,
    "path": false,
    "zlib": false,
    "http": false,
    "https": false,
    "stream": false,
    "crypto": false,
    "crypto-browserify": require.resolve('crypto-browserify'), //if you want to use this module also don't forget npm i crypto-browserify 
  } 
},
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
