# Connect 4

Testing out web assembly with rust.

## Setup

```
rustup update
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

## Build

The debug build runs very slowly, the release build is much faster.

```
trunk serve --release
```
