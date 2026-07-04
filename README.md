# Blockvy

Blockvy is a Tetris-like puzzle game written in Rust with the Bevy game engine.

The project focuses on a clean, grid-based game architecture: gameplay logic is discrete and deterministic, while rendering remains a separate visual layer.

## Features

- Classic 10 x 20 Tetris-style playfield.
- Seven tetromino pieces: I, O, T, S, Z, J, and L.
- Seven-bag piece randomizer.
- SRS rotation states.
- SRS wall kicks enabled by default.
- Hard drop enabled by default.
- Soft drop support.
- Ghost piece support.
- Hold disabled in the base ruleset.
- Bevy ECS-based architecture.
- UI panels for score, level, lines, and upcoming pieces.

## Controls

| Action | Key |
| --- | --- |
| Move left | Left arrow |
| Move right | Right arrow |
| Soft drop | Down arrow |
| Hard drop | Space |
| Rotate clockwise | X or Up arrow |
| Rotate counter-clockwise | Z |

## Requirements

- Rust toolchain with edition 2024 support.
- Cargo.

For WebAssembly builds:

- `wasm32-unknown-unknown` Rust target.
- `wasm-bindgen-cli`.

If you use `mise`, the project tool configuration installs `wasm-bindgen-cli`.
The wasm target still needs to be installed through Rust:

```sh
rustup target add wasm32-unknown-unknown
```

The main runtime dependencies are:

- `bevy`
- `rand`

## Run

```sh
cargo run
```

## Build

```sh
cargo build
```

For an optimized release build:

```sh
cargo build --release
```

## WebAssembly Build

Build the browser-ready WebAssembly package with:

```sh
./scripts/build-web.sh
```

The script:

- builds `blockvy` with the `release-web` profile for `wasm32-unknown-unknown`;
- runs `wasm-bindgen` with the `web` target;
- writes the generated `blockvy.js` and `blockvy_bg.wasm` files into `web/`;
- copies `assets/` into `web/assets/` when assets are present.

To build and serve the game locally:

```sh
./scripts/build-web.sh --serve
```

By default this serves `web/` at `http://localhost:8000`.
Set `PORT` to override the port:

```sh
PORT=8080 ./scripts/build-web.sh --serve
```

Do not open `web/index.html` directly from the filesystem. Serve the `web/`
directory through a local HTTP server so the browser can load the generated
wasm module and assets correctly.

## Project Structure

```text
src/
  audio/        Music and sound effects.
  board/        Grid state, collision, gravity, line clearing, locking, and active piece movement.
  game/         Game rules, state transitions, scoring, startup flow, and piece spawning.
  input/        Keyboard controls, soft drop timing, and horizontal auto-shift.
  persistence/  Settings persistence.
  pieces/       Tetromino definitions, rotations, wall kicks, and seven-bag logic.
  ui/           Board rendering, layout, menus, options, score, pause, and game-over screens.
```
