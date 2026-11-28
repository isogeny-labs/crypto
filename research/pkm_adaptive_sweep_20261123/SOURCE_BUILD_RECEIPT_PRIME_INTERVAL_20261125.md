# Source build receipt — `prime-interval` bench registration — 2026-11-25

## Change

`src/bin/ic/bench.rs` now imports `PrimeIntervalBase` from
`cryptanalysis::ic_framework::pkm_semaev` and registers it as the
`prime-interval` factor base in `run_prime`:

```text
prime-interval:t=T,lo=L   # x in [lo, lo + 2^T) mod p
```

The matching oracle is already supported by `PkmSemaevOracle::from_spec`:
`pkm-semaev:m=2,engine=f4,degree=8,bits=T,lo=L,mode=chain`. The error message
now lists `prime-interval`. No PKM arithmetic, tower construction or oracle
code changed.

This exposes shifted power-of-two intervals. It does not add a
non-power-of-two width; a 21-abscissa 14-bit candidate still has no exact
interval control in the available `t=5/6` shifts.

## Build

```bash
export PATH=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin:$PATH
export RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc
cargo build --release --locked --bin ic
```

Compiler `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Build finished in 5 m 53 s
with warnings; the `rust-objcopy` stripping helper again failed to load
`libLLVM.dylib`, but Cargo returned success and the binary runs.

Binary SHA-256:
`1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08`
(size 57 MiB). Retained at
`/Volumes/SSD990-2/.codex/tmp/pkm-1ae5-target/release/ic`.

## Source hashes

| Source | SHA-256 |
|:--|:--|
| `src/bin/ic/bench.rs` | `5dd0f8b8be2d2b1e5340b4a6040cb518da8e2802cd79734acc783de3f7545944` |
| `src/cryptanalysis/ic_framework/pkm_semaev.rs` | `5be69350d2457954091ea2ef4994d1e007a8a24fb69ea2a867fb1dcc0c25cb4a` |
| `src/cryptanalysis/ic_framework/prime_tower.rs` | `85609ee73f57eb819e921f713931772c162bada2d9c7fe0a6fb373ad02ef2133` |

The `pkm_semaev.rs` hash differs from the `a5503f4` receipt
(`f645d0c8231dd9d02e1b21ede8704612c53e4b452cc9b75b7e065bce19e74e16`)
by one line: the `PkmMode::Chain` maximum `m` guard changed from 5 to 6. Every
run in this panel uses `m = 2`, so the change is inert for this comparison;
`prime_tower.rs` is unchanged.
