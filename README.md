# TIL - Rust

## Install
````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
``

### Check version
```
rustc --version
```

### Update
```
rustup update
```

## Cargo
### Check version
```
cargo --version
```

### Create new project
```
cargo new hello_cargo
```

### Compile
```
cargo build

cargo build --release # for optimized prod-ready builds
```

### Compile & run
```
cargo run
```

### Check code but not builds
```
cargo check
```