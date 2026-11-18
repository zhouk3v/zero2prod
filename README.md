# Commands

Auto run `cargo check`, `cargo clippy`, `cargo test` and `cargo run` sequentially:
```
cargo watch -x check -x clippy -x test -x run
```

Generate `sqlx` metadata for both code and tests
```
cargo sqlx prepare -- --all-targets
```

Delete test databases if tests are failing due to migration failure:
```
docker exec -i postgres psql -U postgres -d postgres <<'SQL'
SELECT format('DROP DATABASE %I WITH (FORCE);', datname)
FROM pg_database
WHERE datname ~* '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
\gexec
SQL
```