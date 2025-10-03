# Terminal Game

I'm Trying to Learn how Terminal Programs Talk to Terminals


https://claude.ai/chat/356b533c-dc7f-496c-8dd1-781afed193ec


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

