# swirlr-wasm

Experimental in-browser version of [swirlr](https://github.com/willdady/swirlr) which samples from your web cam. Built with Rust and Web Assembly!

### Compiling and running

1. From the root directory run `wasm-pack build`
2. From `pkg/` run `npm link`
3. From `www/` run `npm install`
4. From `www/` run `npm link swirlr-wasm`
5. From `www/` run `npm run start` to start a local server or `npm run build` to build
