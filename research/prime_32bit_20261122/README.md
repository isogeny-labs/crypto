# 32-bit prime-field MITM rung and u64 probe

**Date:** 2026-11-22. **Status:** measured, `[pcc:unverified]`.
**Scope:** public synthetic known-answer fixture only; no deployed-curve,
`vs_rho`, or asymptotic claim.

## Fixture

```text
p = 4294967291 = 2^32 - 5   (p ≡ 3 mod 4)
a = p - 3 = 4294967288
b = 267
n = 4295072597             (prime)
G = (1, 2811385579)         [n]G = O checked
```

Added as `a3-bench-32bit-p256class` in `src/cryptanalysis/research_bench.rs`.

## Change

`PairSumTable` mirrors `x` keys and factor-base points in `u64` when
`p < 2^63`. The relation probe uses u64 affine addition with extended-Euclid
inverses; every hit is still finalised and re-added in the `BigUint` group, so
the fast path cannot create a false relation. The `BigUint` table is skipped
when the fast path is active.

## Correctness

`prime_mitm_check` on the frozen 16-bit targets: S3, MITM and quartic recover
the same scalar on 3/3 targets; all `mitm_ok`.

## Measurements (B = 1600, seed 20261121)

| stage | before u64 | after u64 |
|:--|--:|--:|
| pair table | 1322 ms | 1223–1490 ms |
| relations | 3145 ms | **1016–1314 ms** |
| LA | 245 ms | 239 ms |
| total IC | 4770 ms | **2450–3120 ms** |

Four seeds:

| seed | IC ms | rho ms | IC/rho |
|--:|--:|--:|--:|
| 20261121 | 2536.9 | 3610.5 | 0.703 |
| 20261122 | 2450.0 | 12694.8 | 0.193 |
| 20261123 | 2827.7 | 4941.9 | 0.572 |
| 20261124 | 3120.4 | 2682.8 | 1.163 |

The rho arm is a single unoptimised walk with high target-luck variance. A
negation-folded/tuned rho would likely be ~2× faster, so this is **not** a
`vs_rho` claim. It is the first 32-bit prime rung where IC is in the same band
as rho rather than ~400× above it.

## Reproduce

```sh
PATH="$HOME/.rustup/toolchains/1.97.0-aarch64-apple-darwin/bin:$PATH" \
  cargo build --release --example a3_s4_ladder
./target/release/examples/a3_s4_ladder 32 20261121 p256 mitm 1600
```

Raw logs: `a3_32bit_mitm_u64.log`, `a3_32bit_u64_seeds.jsonl`,
`a3_32bit_sweep.log`.
