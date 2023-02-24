Using wasm-pack 0.10.3, index.html has the following in the console:

"hello"

This is what is expected.

Dependencies modified:

```
cargo update -p web-sys --precise 0.3.60
cargo update -p js-sys --precise 0.3.60
cargo update -p wasm-bindgen --precise 0.2.83
```

To build, run:

`wasm-pack build --dev --target web`
