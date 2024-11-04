# rustbook
play around, code snippets while reading the rustbook


# rb01

```sh
cargo new mycargo
cargo build --locked
cargo build --release
cargo run
cargo check
```

# rb02

```rs
# prelude? https://doc.rust-lang.org/stable/std/prelude/index.html
```

- Q: how to find gen_range() from `rand` in doc? - trait (rb10?)

```sh
cargo doc --open
```

- `match` @ rb06 rb18

## rb03 variables and control flow

- Q: variable shadowing when decompiled

## rb04

- `Drop` (ref:https://doc.rust-lang.org/stable/std/ops/trait.Drop.html#tymethod.drop) is called when an object go out of scope
- `Copy` Trait desides whether a type is deep copied (on assign) or shallow copied. Conflict with `Drop` (compile time check)
