# Conway's Game of Life
Conway's game of life implemented in [Rust](https://www.rust-lang.org/) and compiled to [WebAssembly](https://webassembly.org/).

Used to test WebAssembly performance and experiment with Rust.

[Live page](https://r-oung.github.io/game-of-life/)


## Folders
```
root          Root directory
├─ wasm       Game of Life WASM and JavaScript API
└─ www        Web application
```


## tl;dr
Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), then:

```shell
cd wasm/
wasm-pack build
cd ../www/
npm i
npm run serve
```


## References
- [Rust and WebAssembly: Tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/primitives/array.html)