## right:
```rust
let _guard = scopeguard::defer(|| xyz);
```
## wrong:
```rust
let _ = scopeguard::defer(|| xyz);
```
