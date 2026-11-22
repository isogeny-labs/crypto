# 36-bit prime-field MITM rung

**Date:** 2026-11-22. **Status:** measured, `[pcc:unverified]`.
**Scope:** public synthetic known-answer fixture only; no deployed-curve,
`vs_rho`, or asymptotic claim.

## Fixture

```text
p = 68719476731 = 2^36 - 5   (p ≡ 3 mod 4)
a = p - 3 = 68719476728
b = 33
n = 68719022539              (prime)
G = (19, 72892532)           [n]G = O checked
```

Found by `examples/find_a3_bench_curves 36` in **2.1 s**. The finder needed a
new counter for this rung: `count_points_small` is Θ(p) modular
exponentiations (fine at 20 bits, impossible at 36), so it now has a
Hasse-interval (Mestre) path that scans `|k| ≤ 2√p` for `kP = ±(p+1)P` and
accepts only a prime candidate. A prime `m` in the Hasse interval divisible
by `ord(P)` forces `#E = m` (otherwise `#E = m² > p+1+2√p`), so the count is
exact, not heuristic.

Independently re-verified outside the crate (pure Python group law):
`G` on curve, `[n]G = O`, `n` prime, `|p+1-n| = 454193 ≤ 2√p`.

Added as `a3-bench-36bit-p256class` in `src/cryptanalysis/research_bench.rs`.

## Timing methodology (read this before the numbers)

The bench host is shared with other agent sessions; the load average during
these runs was **55–150**, so *wall time is not comparable across runs* (the
same configuration measured 82 s and 116 s wall). Every ladder invocation is
therefore wrapped in `/usr/bin/time -p` and the **CPU seconds (user + sys)**
are the reported quantity — they came out at 30–33 s across all runs, i.e.
stable to a few percent. The in-process stage split
(`pair_table_ms` / `relations_ms` / `linear_algebra_ms`) is wall-clock, so
only its shares are meaningful here, not its absolute values.

Reproduce: `research/prime_36bit_20261122/run_rung.sh` (writes every artifact
used below), then `python3 summarise.py`.

## B probe (seed 20261121, sparse LA, ladder rho arm skipped)

| B | IC CPU ms | table ms | relations ms | LA ms | trials | IC ok |
|--:|--:|--:|--:|--:|--:|:--|
| 1600 | **30290** | 8022 | 30460 | 712 | 21609 | yes |
| 2048 | **30260** | 17040 | 25666 | 1051 | 13358 | yes |
| 2560 | 34700 | 24371 | 22070 | 3330 | 8997 | yes |
| 3072 | 43820 | 35797 | 20388 | 5186 | 7058 | yes |

B = 1600 and B = 2048 are tied in CPU cost; 2560 and 3072 are strictly worse.
**B = 2048 is used for the seed row** because it halves the trial count at no
CPU cost, and relation collection is the noisier stage.

Note the `(2p)^{1/3}` heuristic that was used up to 28 bits would pick
**B ≈ 5158** here — about 2.5× the measured optimum. The reason is visible in
the stage shares: the pair-table build is no longer small next to relation
collection, so the heuristic's cost model (relations ≫ table) has stopped
holding at this size.

## Four paired seeds at B = 2048

| seed | IC CPU ms | table ms | relations ms | LA ms | trials | tuned rho ms | rho ops | S=ops/√n | IC / tuned rho | scalars agree |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 20261121 | 33120 | 19082 | 29458 | 1687 | 13211 | 22.7 | 419269 | 1.599 | 1459× | yes |
| 20261122 | 32700 | 24252 | 46194 | 1876 | 13243 | 19.6 | 217018 | 0.828 | 1668× | yes |
| 20261123 | 33040 | 19981 | 32693 | 1366 | 13435 | 22.9 | 353344 | 1.348 | 1443× | yes |
| 20261124 | 31440 | 15585 | 31476 | 1868 | 13279 | 16.0 | 193877 | 0.740 | 1965× | yes |

IC CPU is 31.4–33.1 s and every seed recovers the planted scalar
(`ic_matches_truth: true`), with the tuned rho returning the identical value
on the identical public point (`rho_agrees: true` in
`a3_36bit_rho_paired.jsonl`).

## Paired rho arm comes from the `cryptanalysis` repo

The ladder's built-in `rho_ms` is a **single unoptimised walk** with no
negation map and no distinguished points; at 36 bits it exhausts its 5M-step
budget on every seed and never answers. It is a fixture sanity check, not a
baseline. The paired arm is the tuned reference implementation:

```sh
cd ~/cryptanalysis && make cli
./build/ca solve --alg rho --group ec \
  --p 68719476731 --a 68719476728 --b 33 --order 68719022539 \
  --g 19,72892532 --h <public_target_q>
```

It answers in 16–23 ms at `S = ops/√n = 0.74–1.60`, i.e. at or below the
1.25 textbook constant (r-adding walk + negation map + distinguished points,
single thread, same host, same load). The ladder now takes `--no-rho` so the
redundant arm can be skipped; `rho_ms` / `rho_ok` are `null` in that mode.

**So the honest 36-bit statement is: the S₄ 3-decomposition MITM IC costs
~1.5×10³ × a tuned rho on the same instance.** Against rho it is nowhere
near competitive; the rung's value is the scaling curve, not a speed record.

## What the rung does say

- The MITM path is correct at 36 bits: same scalar as the tuned rho, every
  candidate re-added in the group before use, no false relations.
- IC cost is dominated by the pair-table build (35–48% of the run) and by
  relation collection; LA is ~5%.
- The table build is `B(B+1)/2` **modular inversions**: `build_pair_sum_table`
  calls `Point::add`, the affine BigUint add, per pair. The "u64 fast path"
  only converts the resulting abscissa to a `u64` key. Batching the
  inversions (Montgomery's trick) is the obvious next lever, and it is what
  the ledger's `next_target` now records.
- **32 → 36 bits costs ~10× CPU**, at the tuned B on the same four seeds:
  3.19–3.33 s at 32 bits (`a3_32bit_cpu.txt`) against 31.4–33.1 s here. The
  `p^{2/3}` projection of this family predicts 2^(8/3) = 6.3×, so this one
  4-bit step is *steeper* than the projection — one step is not a slope, but
  it is the opposite of a subexponential gain and worth watching on the next
  rung.

## Artifacts

| file | contents |
|:--|:--|
| `run_rung.sh` | every command below, including the paired arms |
| `a3_36bit_sweep.log` | B probe: JSON row + `real/user/sys` per B |
| `a3_36bit_u64_seeds.jsonl` | four seed rows at B=2048 |
| `a3_36bit_seeds_timing.txt` | `/usr/bin/time -p` per seed + load average |
| `a3_36bit_rho_paired.jsonl` | tuned rho on the identical four targets |
| `a3_32bit_rho_paired.jsonl` | same arm on the four archived 32-bit targets |
| `a3_32bit_cpu.txt` | 32-bit CPU-time control at B=1600, for the 32→36 step |
| `summarise.py` | renders the two tables above |
| `update_ledger.py` | folds this rung (and the 32-bit correction) into `docs/ic/boundary_targets.json` |
