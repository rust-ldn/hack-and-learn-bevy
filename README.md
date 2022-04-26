# Template Bevy project with WebGL enabled (Upgraded to Bevy 0.7)

## Prerequisites

```
rustup components add wasm32-unknown-unknown
```

cargo install --locked trunk

## Build and serve WASM version

```
trunk serve
```
then point your browser to http://0.0.0.0:8080/

## Build and run native version
```
cargo run
```
