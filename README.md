# TODO: change repo name to Chess.WASM or wasm-chess

# wasm-chess

`wasm-chess` provides a chess.js-inspired API while using the excellent Rust chess ecosystem underneath for correctness, performance, and reliability.

Built with:

* Rust
* wasm-bindgen
* shakmaty
* tsify

---

## Features

* chess.js-inspired API
* SAN and UCI move support
* Full legal move generation
* PGN parsing and export
* Comments, suffix annotations, and NAG support
* Move history with historical position access
* WASM-first architecture
* TypeScript-friendly bindings
* Divergence/transposition TODO

---

## Installation TODO

```bash

```

---

## Quick Example

```js
import { WasmChess } from 'wasm-chess'

const chess = new WasmChess()

chess.move('e4')
chess.move('e5')
chess.move('Nf3')

console.log(chess.fen())
console.log(chess.pgn())
```

---

## API Overview

TODO

---

## Basic Game Control

```js
chess.load(fen)
chess.reset()
chess.clear()
```

## Moves

```js
chess.move('e4')
chess.move('Nf3')
chess.undo()
```

## Move Generation

```js
chess.legalMovesSAN()
chess.legalMovesUCI()
chess.legalMovesVerbose()
```

## Position Information

```js
chess.fen()
chess.board()
chess.turn()
chess.isCheck()
chess.isCheckmate()
chess.isDraw()
```

## Historical Queries

```js
chess.fenAt(index)
chess.turnAt(index)
chess.moveAt(index)
```

## PGN Support

```js
chess.loadPgn(pgn)
chess.pgn()
```

---

## PGN Features

The PGN implementation supports:

* headers
* comments
* suffix annotations
* NAGs
* SAN move export
* PGN parsing

Example:

```pgn
[Event "Example"]

1. e4 $1 {Best by test} e5 2. Nf3! *
```

---

## Direct Board Manipulation

Some low-level board editing APIs commonly found in chess.js are intentionally not yet implemented:

* `clear()`
* `put()`
* `remove()`
* castling-right mutation APIs (`setCastling`, etc.)

his library is built on top of `shakmaty`, which enforces legal and internally consistent chess positions

That means seemingly simple editing operations become significantly more complex because they may invalidate:

* king safety
* castling rights
* side-to-move consistency
* en passant state
* move counters
* general position legality

These features are planned for a future release once a clean and well-defined API design is finalized.

---

# Acknowledgements

## chess.js

This project was heavily inspired by:

* the chess.js API design
* the developer experience of chess.js
* chess.js test suite

Many tests were ported and adapted from the chess.js repository to Rust in order to verify compatibility and behavior.

Huge thanks to the chess.js maintainers and contributors for creating one of the most approachable chess libraries in the JavaScript ecosystem.

Repository:

* [https://github.com/jhlywa/chess.js](https://github.com/jhlywa/chess.js)

---

## shakmaty

This project is fundamentally powered by the incredible `shakmaty` ecosystem.

Core chess logic, move legality, SAN handling, position management and more
are built on top of:

* shakmaty
* shakmaty pgn-reader

Without shakmaty this project would not exist.

Thanks to shakmaty project and all contributors for building one of the strongest chess libraries available in Rust

Repositories:

* [https://github.com/niklasf/shakmaty](https://github.com/niklasf/shakmaty)
* [https://github.com/niklasf/shakmaty/tree/master/pgn-reader](https://github.com/niklasf/shakmaty/tree/master/pgn-reader)

---

# Development

Build:

```bash
wasm-pack build
```

Run tests:

```bash
cargo test
```
