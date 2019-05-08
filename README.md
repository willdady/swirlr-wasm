# swirlr-wasm

Experimental in-browser version of [swirlr](https://github.com/willdady/swirlr) which samples from your web cam. Built with Rust and Web Assembly!

### Demo

https://willdady.github.io/swirlr-wasm/

### Compiling and running

1. Install [Rust](https://www.rust-lang.org/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
2. From the root directory run `wasm-pack build`
3. From `pkg/` run `npm link`
4. From `www/` run `npm install`
5. From `www/` run `npm link swirlr-wasm`
6. From `www/` run `npm run start` to start a local server or `npm run build` to build
