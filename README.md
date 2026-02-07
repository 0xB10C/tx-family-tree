# tx-family-tree
Tool to visualize parent and child (ancestors and descendants) relationships of transaction sets (blocks, mempool, ...)


```
$ rustup target add wasm32-unknown-unknown
```

```
# Debug build (faster compilation, larger output)
$ wasm-pack build --target web --dev

# Release build (optimized for production)
$ wasm-pack build --target web --release
```