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

---

## Correction (2026-11-22, same day): the rho arm was not a tuned rho

**Status:** the IC-vs-rho sentences above are withdrawn. The `rho_ms` column
there is a *single unoptimised walk* inside `examples/a3_s4_ladder.rs`
(`pollard_rho_ecdlp`, no negation map, no distinguished points, one walker),
so `IC/rho 0.19–1.16` compares IC against a reference that is roughly three
orders of magnitude slower than the tuned reference implementation in the
`cryptanalysis` repo. The IC-internal numbers (before/after u64 probe path,
stage split, four-seed spread) are unaffected.

**Paired re-measurement on the identical four public targets**, using
`cryptanalysis`'s rho (r-adding walk, negation map, distinguished points,
batched affine addition; `ops/sqrt(n)` reported as `S`):

```sh
cd ~/cryptanalysis && make cli
./build/ca solve --alg rho --group ec \
  --p 4294967291 --a 4294967288 --b 267 --order 4295072597 \
  --g 1,2811385579 --h <public_target_q>
```

Two independent paired runs, because rho is randomised and its single-run
spread at this size is itself a factor of ~7:

| seed | IC ms (archived) | ladder `rho_ms` | tuned rho ms (A / artifact B) | rho ops (A / B) | IC / tuned rho (A / B) | scalar agrees |
|--:|--:|--:|--:|--:|--:|:--|
| 20261121 | 2536.9 | 3610.5 | 8.6 / 12.5 | 167671 / 86364 | 295× / 203× | yes |
| 20261122 | 2450.0 | 12694.8 | 6.0 / 5.5 | 97775 / 76974 | 408× / 445× | yes |
| 20261123 | 2827.7 | 4941.9 | 2.2 / 3.7 | 61692 / 57713 | 1285× / 765× | yes |
| 20261124 | 3120.4 | 2682.8 | 3.0 / 1.9 | 30539 / 47908 | 1040× / 1674× | yes |

Column B is the committed artifact
`research/prime_36bit_20261122/a3_32bit_rho_paired.jsonl` (written by
`run_rung.sh`). S = ops/√n over the eight observations spans 0.47–2.56, i.e.
around the 1.25 textbook constant. All eight tuned-rho runs return the
archived `x_truth` exactly, so the two arms agree on the answer and differ
only in cost.

**Consequence.** The 32-bit rung does not put IC "in the same band" as rho;
it is **≈2×10²–1.7×10³** away from a tuned rho at 32 bits, the spread coming
almost entirely from rho's own run-to-run variance. Any `vs_rho` statement
for the prime regime has to be made against `cryptanalysis`
`ca solve --alg rho` (or an equivalent tuned arm), never against the
ladder's built-in walk. The ladder's `rho_ms` field should be read as a
*fixture sanity check that the target is a genuine DLP instance*, not as a
cost baseline.
