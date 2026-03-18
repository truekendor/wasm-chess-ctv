# wasm-chess-ctv

This project currently focused on replacing performance-critical sections of chess.js with compiled WASM bindings. 


To compile the rust code to WASM use `wasm-pack build --target web`

# Rust versions

- rustc: `1.94.0`
- cargo: `1.94.0`


# Acknowledgements

The project uses [shakmaty](https://github.com/niklasf/shakmaty) chess library to do all the heavy lifting. 
