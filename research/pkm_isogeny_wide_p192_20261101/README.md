# The isogeny tower at NIST P-192: a wide system builder and the first solved F4 cells

2026-11-01 · follow-up to the auxiliary-curve open items (Conductor
T-5750): the u64 ceiling · instrument `examples/pkm_tower_pilot.rs`
`--builder wide --kinds isogeny` on the new
`src/cryptanalysis/ic_framework/prime_tower_wide.rs` · records
`rows_m2_isogeny_verified.jsonl` (and the first unverified pass,
`rows_m2_isogeny.jsonl`) · logs `run.log`, `run_verified.log`

**Class: stage diagnostic of the solver axis (AGENTS.md §2). No pipeline is
priced, no `S`, no leaderboard row.** This closes the recorded gap
(encyclopedia §0.2, §8): "no F4 run or relation phase has run at P-192,
because the plug-in and the isogeny system builder are `u64`."

## What was built

- `prime_tower_wide.rs` (library): the PKM auxiliary-curve construction
  generic over `fp_backend::PrimeField` — any backend, any prime below
  `2^{64N}` (`FpWide<8>`: 512 bits; `FpP192`: the NIST P-192 prime).  The
  same charged search as the `u64` plug-in (residue-class BSGS filter,
  Mestre order, the order-aware `Q` step, the `X₁(2, 8)` family) plus the
  supersingular towers (`#E' = p + 1`, no search; P-192: `v₂(p+1) = 64`,
  one draw).  `t ≤ 63`.  u128 alone cannot hold P-192's 192-bit modulus;
  the multi-limb Montgomery backends are the repo's answer.
- `pkm_tower_wide/wide.rs`: the `isogeny` kind — Vélu square rules
  `y_j² = y_j·y_{j+1} − ξ·y_{j+1} + ξ·y_j − τ` (top: `y_j² = (c+ξ)·y_j −
  (cξ+τ)`), the level equations, the check, and the exhaustive
  verification, generic over the backend; the pilot takes
  `--builder wide --kinds isogeny` (engine `generic`).
- Cost caveat kept honest: the **ordinary** wide search is
  `Θ(p^{1/4})` group operations per curve (≈ `2^{45}` at P-192) — a
  small-prime instrument at NIST size; the supersingular tower is what
  runs there (the builder picks it whenever `v₂(p + 1) ≥ t`).

## Results

`m = 2`, planted target, control `tower`, supersingular auxiliary curve
`y² = x³ + x` at P-192 (`aux_curves_tried = 1`, `aux_search_gae = 0`,
`aux_tower_build_gae = 38`, `velu_evaluations_uncharged = 2t`), engine
`f4_tower_generic` over `FpP192`, every row verified exhaustively
(`--verify`: decompositions enumerated independently and re-checked in
`BigUint`; `planted_violations = 0`):

| t | N | D | columns | wall |
|--:|--:|--:|--:|--:|
| 4 | 8 | 5 | 206 | 0.1 s |
| 5 | 10 | 5 | 575 | 0.3 s |
| 6 | 12 | 5 | 1,182 | 0.7 s |
| 7 | 14 | 5 | 3,100 | 5.4 s |
| 8 | 16 | 5 | 8,279 | 25.2 s |

## Reading

- The isogeny tower's `D = 5` at `N = 8 … 16` matches the small-prime
  isogeny ladder (`D = 3, 5, 5, 5, 5, …` at `m = 2`, encyclopedia §0.2)
  and the supersingular D-equality claim (PR #1947 Q2) at a NIST prime:
  same leading forms, same degree.  Against the Kummer ladder at `p₀`
  (`D = 4` at `N = 8 … 10`), the isogeny tower runs one degree higher at
  these cells — as it already did at small primes.
- The 2-Sylow `Q`-step and `X₁(2, 8)` fixes of the `u64` plug-in (commit
  `c5dc87c3d4`) carry over: the wide module shares their structure, and
  its four unit tests cover the ordinary and `z2xz8` searches at a
  small prime (orders checked against exhaustive point counts), the
  supersingular tower at P-192 (order of `Q` by repeated doubling, the
  composed map recomputed in `BigUint`), determinism, and the limits
  (`t` bounds, `v₂(p+1) < t`, `p ≡ 1 mod 4` refused for `z2xz8`).
- Still open, unchanged: the priced `prime-isogeny` **plug-in**
  (ecbench `ic.pipeline`, `PrimeInstance`/`PrimeCurve`) is `u64`; a
  relation phase at P-192 needs that widening next.  The `m = 3 / m = 4`
  cells at NIST size are the lead-1 gate as before.

## Reproduce

```bash
cargo run --release --example pkm_tower_pilot -- --builder wide \
  --kinds isogeny --m 2 --controls tower --t-min 4 --max-t 8 \
  --planted 1 --random 0 --ladder-t none --budget 21600 \
  --p P-192 --engine generic --verify \
  --out research/pkm_isogeny_wide_p192_20261101/rows_m2_isogeny_verified.jsonl
```

## Addendum (later the same day): yield, the m = 2 staircase, and m = 3

Three further runs on the same instrument (`--builder wide --kinds
isogeny`, supersingular auxiliary curve, `--verify`):

### Random-target yield (rows_m2_isogeny_random.jsonl)

4 random targets per cell, `t = 4 … 8` (`N = 8 … 16`), 20 systems:
**0 of 20 decompose** — every system is the unit ideal, proved at
`D = 5` (the same degree as a solvable system at that cell), 0.0–10.7 s
per proof, all verified.  This is what the arithmetic predicts at a toy
tower on a NIST prime: a random point is a sum of `m` factor-base
points with probability ≈ `|V|^m/(m!·p)`, here `2^{16}/2^{192}` at its
best — so **random-relation collection at P-192 needs the PKM-optimal
tower size (`t ≈ 39` at `m = 4`, `|V| ≈ (m!·r·T)^{1/(m+1)}`), not toy
`t`**; planted (decomposition-oracle) systems are the right solver-axis
instrument at these sizes, which is what the ladder measures.

### The m = 2 staircase further (rows_m2_isogeny_t9_12.jsonl, in flight)

| t | N | D | columns | wall |
|--:|--:|--:|--:|--:|
| 9 | 18 | 5 | 13,009 | 88 s |
| 10 | 20 | **6** | 35,157 | 469 s |
| 11 | 22 | (running) | | |
| 12 | 24 | (running) | | |

`D = 6` at `N = 20` at P-192 matches the Kummer ladder at `p₀` (`D = 6`
at `N = 20 … 24`): the isogeny staircase at a NIST prime rises with
`N` exactly as at small primes.

### m = 3 (rows_m3_isogeny.jsonl, rows_m3_isogeny_t6.jsonl)

| t | N | D | columns | wall |
|--:|--:|--:|--:|--:|
| 4 | 12 | 6 | 5,088 | 18 s |
| 5 | 15 | **7** | 31,855 | 3,590 s |
| 6 | 18 | (running, `run_m3_t6.log`) | | |

`m = 3` starts one degree above `m = 2` (`D = 6` at `N = 12` against
`D = 5`) and reaches **`D = 7` at `N = 15`** — matching the small-prime
`m = 3` staircase exactly (`D = 6, 6, 7, 7` at `N = 9 … 18`, the
mega-pass lever-4 reading) at a NIST prime.  The `m = 3, N = 21` gate
cell is beyond this machine's memory at P-192 as at `p₀`.
