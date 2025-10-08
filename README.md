# Terminal Game

I'm learning how terminal programs talk to the terminal.  Running this program will place a cursor on the terminal.  The arrow keys can move that cursor.  Pressing 'q' or ctrl+c ends the experience.

## Run the program natively

```
cargo run
```


## Run vias wasix

```
cargo wasix run
```

## Build and run the WASM

Registry - Getting Started: https://docs.wasmer.io/registry/get-started

```
# cargo build --target=wasm32-wasip1
cargo wasix build --release
wasmer run .
```

There is a bug, you need to press enter after you press an enter key.

## Notes

So... according to [wasix.org](https://wasix.org/) the point of WASIX is to fill gaps in wasm, at it does!  A patch was sent to https://github.com/wasix-org/libc to allow this program to function correctly.  It's currently pointed at my fork and should work as-is.

> WASIX provides functions to control Terminal I/O.

I'm not sure what that means but I guess I should ask an actual human?

More documentation here: https://wasix.org/docs/explanation/extensions-to-wasi#wasix-extended-functions

