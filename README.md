# Terminal Game

I'm learning how terminal programs talk to the terminal.  Running this program will place a cursor on the terminal.  The arrow keys can move that cursor.  Pressing 'q' or ctrl+c ends the experience.


## Run the program natively

```
cargo run
```


## Build and run the WASM

Registry - Getting Started: https://docs.wasmer.io/registry/get-started

```
cargo build --target=wasm32-wasip1
cargo wasix build --release
wasmer run .
```


## Notes

So, It was good to build this. I stopped as far as realizing that WebAssembly does not support setting the standard in in a raw mode... 

I read somewhere that you can set the raw mode on the host side. So that would be X term. I guess I should Google that now.

So... according to [wasix.org](https://wasix.org/) the point of WASIX is to fill gaps in wasm,

> WASIX provides functions to control Terminal I/O.

I'm not sure what that means but I guess I should ask an actual human?

More documentation here: https://wasix.org/docs/explanation/extensions-to-wasi#wasix-extended-functions

