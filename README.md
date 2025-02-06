# grump
functional programming language that compiles to WebAssembly written in Rust

## Progress?
Nowhere close to being done

## Why do we need this?

Not sure tbh... seemed to be na idea with all the right buzz terms in it

## How to run?

1. Compile the rust code 
```
cargo build --release
```

2. Run the compiler 
```
./target/release/grump
```

3. Execute the Wasm Binary
```
wasmtime output.wasm --invoke main
```
