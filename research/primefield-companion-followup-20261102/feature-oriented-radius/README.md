# Oriented mode relations across radii

[Derivation](DERIVATION.md) describes the scalar-cancelling relation, nonzero
anchor guard, zero-mode constraints and exact oracle. The optional adapter
adds information to the predecessor; it does not replace the default route.

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/oriented-radius-check
timeout 60 /tmp/oriented-radius-check
timeout 60 /tmp/oriented-radius-check --inherited
```

`generate.py` copies hash-bound predecessor Rust files and generates the
initializer, adapter and independent oracle. `oriented-image.rs` and
`oriented-pair.rs` are authored. `record.py` captures the named validation02
binary and logs; later replays require new immutable validation paths.
