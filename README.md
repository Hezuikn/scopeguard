## right:
```rust
let guard = scopeguard::defer(|| xyz);
```
## wrong:
```rust
let _ = scopeguard::defer(|| xyz);
```
