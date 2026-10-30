# Commands

Auto run `cargo check`, `cargo clippy`, `cargo test` and `cargo run` sequentially:
```
cargo watch -x check -x clippy -x test -x run
```

Generate `sqlx` metadata for both code and tests
```
cargo sqlx prepare -- --all-targets
```