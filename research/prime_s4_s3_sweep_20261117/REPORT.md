# Prime-field S₄ 3-decomposition: S₃-anchored pair sweeps

Measured 2026-11-17/18, on the `a = p − 3` deployed-shape rungs (P-256 class
`p ≡ 3 (mod 4)`, GOST CryptoPro-B class `p ≡ 1 (mod 4)`), extending the
prime `decomposition` ladder from 24 to 28 bits.

**No exponent change is claimed.** The relation family is the same `S₄` /
three-summand family the ledger already records; the change is a constant
factor in the decomposition oracle plus two new 28-bit rungs.
Subexponential prime-field IC needs a decomposition whose cost per target is
*sublinear in the factor-base size*; this is throughput for the measurement
lane, not that.

## 1. What the oracle does

A 3-decomposition of `R = aG + bQ` is a signed triple of factor-base points
with `ε_i F_i + ε_j F_j + ε_k F_k = R`, equivalently a base abscissa `x_k`
with `S₄(x_R, x_i, x_j, x_k) = 0`.

`find_one_relation_s4_counted` (kept, still tested) expands that quartic for
each of the `B²/2` base pairs and runs Cantor–Zassenhaus over `F_p` on it:
`X^p mod f`, equal-degree splitting with `(X + c)^((p−1)/2) mod f` trials,
gcds, exact divisions — thousands of `BigUint` multiplications per pair.

`find_one_relation_s4_via_s3_counted` anchors one base point instead.  For
`F_i` and both signs it forms `R′ = R ∓ F_i` and decomposes `R′` as a
**two**-point sum: for each `x_j ∈ base`, solve the *quadratic*
`S₃(x_j, X, x_{R′}) = 0` and keep the roots that are base abscissae.
`S₄(x_R, x_i, x_j, x_k) = 0` holds exactly when such a signed triple exists,
so the decided family is identical — the yield per trial is the same
`≈ 2B³/p` relations — while a candidate costs ~10 field multiplications and
one square root.

Supporting changes in the same file: `quadratic_roots_fp` (variable-time
solve), `inv_vartime` in place of the constant-time `inv` at every inversion
in the root-finder hot path, and `poly_rem` skipping the inversion for a
monic modulus.  `ec_index_calculus_dlp_s4_s3_staged` is the staged driver;
`ec_index_calculus_dlp_s4_staged` runs the quartic oracle on the identical
skeleton, so a paired run measures the oracle alone.

New rungs: `a3-bench-28bit-p256class` (`p = 268435399`, `n = 268407199`) and
`a3-bench-28bit-cryptoproclass` (`p = 268435361`, `n = 268448867`), from
`examples/find_a3_bench_curves` (largest prime below `2^28` in each residue
class mod 4; smallest `b` with prime order; smallest-`x` generator).  The
ladder example re-checks `n` prime and `[n]G = O` on every run.

## 2. Correctness evidence

1. **Exact set equivalence, not sampling.**
   `s3_anchor_sweep_matches_the_quartic_triple_census` fixes one 16-bit
   target known to decompose, enumerates every triple each oracle admits,
   and compares both with a brute-force census that re-adds all eight sign
   patterns of every base triple.  The anchored set equals the census
   exactly; the quartic set is a subset of it (CZ may miss a root it fails
   to split) and contains the planted triple.
2. **Every relation is re-added in the group** before it is used
   (`try_finalise_s4`), in both oracles.
3. **Every rung recovers a known-answer log**: `[x]G = Q`, `x = x_sidecar`
   (never a solver input), and the paired ρ control on the identical public
   point returns the same scalar.  All 22 rows in `ladder.jsonl`, all 7 in
   `two_decomp.jsonl` and all 4 control rows pass.
4. `s3_staged_driver_matches_the_quartic_driver` checks both staged drivers
   return the same log and the same relation count.  The module's 37 tests
   pass.

## 3. Anchored 3-decomposition ladder (`ladder.jsonl`)

`fb = 120`, `+8` extra relations, 128 rows per rung, one rung at a time.

| bits | class | seeds | median IC wall | median ms/relation | median trials/rel |
|--:|:--|--:|--:|--:|--:|
| 16 | p256 | 3 | 0.0185 s | 0.145 | 1 |
| 20 | p256 | 3 | 0.485 s | 3.79 | 1 |
| 20 | cryptopro | 3 | 0.581 s | 4.54 | 1 |
| 24 | p256 | 3 | 14.59 s | 114.0 | 6 |
| 24 | cryptopro | 3 | 20.31 s | 158.7 | 5 |
| 28 | p256 | 3 | 255.2 s | 1994 | 72.5 |
| 28 | cryptopro | 3 | 362.5 s | 2832 | 87.5 |

`trials` counts random `(a, b)` draws, so a rung whose first draw already
decomposes reads `1.0` — censored below, not a rate estimate.  Wall times
come from a shared host (load average 6–24 on 14 cores) and are
**exploratory**, not an isolation-qualified speedup.

Paired oracle comparison, same seed and fixture, only the oracle differs:

| bits | seed | quartic/CZ | S₃-anchored | ratio |
|--:|--:|--:|--:|--:|
| 16 | 20261006 | 291.53 s | 0.0185 s | 1.6 × 10⁴ |
| 16 | 20261005 | 82.68 s (HEAD~1 control) | 0.0185 s | 4.5 × 10³ |

## 4. The 2-decomposition driver on the same tree (`two_decomp.jsonl`)

Same fixtures and policy, one column fewer per row:

| bits | class | IC wall | ms/relation | median trials/rel |
|--:|:--|--:|--:|--:|
| 16 | p256 | 0.021 s | 0.166 | 2 |
| 20 | p256 | 0.331 s | 2.59 | 19.5 |
| 20 | cryptopro | 0.423 s | 3.31 | 24 |
| 24 | p256 | 8.28 s | 64.7 | 530 |
| 24 | cryptopro | 9.62 s | 75.1 | 360 |
| 28 | p256 | 121.4 / 126.0 s | 966 | 6397 |
| 28 | cryptopro | 166.5 s | 1301 | 7531 |

At 28 bits the anchored 3-decomposition is ~2× the 2-decomposition wall for
the same row count (245–390 s vs 121–167 s).  It is no longer the 8× penalty
the 2026-10-06 record measured at 16 bits (3.3 s vs 0.4 s per relation).

**The recorded a = −3 rows are stale, and this is not a regression.**
`control_before.jsonl` runs the tree *before* this change (HEAD~1, same host,
same seed): 20-bit 2-decomposition 0.55 s, 24-bit 8.3–9.6 s, against the
ledger's recorded 18.7–37.2 s and 272–513 s for the same fixtures.  Those
records predate the word-sized square-root and field fast paths that landed
since 2026-10-05; they should be re-recorded, not read as a speedup from
2026-11-18.

The variable-time inversions alone are worth ~1.6× on the 2-decomposition
driver (16-bit 0.0332 → 0.0213 s; 20-bit 0.5505 → 0.3315 s, same host and
seed).  They do not measurably move the quartic/CZ oracle, which is bound by
polynomial arithmetic rather than inversions.

## 5. Cost accounting, and what this does *not* buy

Per trial the anchored sweep visits `2B²` candidates at cost `c` and yields
`≈ 2B³/p` relations, so a relation costs `≈ p·c/B` and a full log with `B`
rows costs `≈ p·c` — independent of `B`, with `c` a constant.  That is the
`O(p)` wall the ledger already records for every enumeration-based
prime-field IC variant, and why ρ (`p^{1/2}`) still wins every `vs_rho` gate:
the measured gap is ≳95× at 24 bits and ≳470× at 28 bits.

What moved is `c`: from a quartic plus a full `F_p` root find (~10³–10⁴
multiplications per candidate) to one quadratic solve.  The subexponential
question is untouched — it needs a decomposition whose per-target cost is
sublinear in `B`, which is the algebraic (FFD/Gröbner) route measured in
`research/prime_fall_degree_20261027`, or a factor base with an algebraic
membership condition.

Claim boundary: public synthetic known-answer fixtures only; no deployed
curve, no key recovery, no `vs_rho` claim, no asymptotic claim.

## 6. Recorded cells that are not measurements

- 20-bit p256 **quartic** oracle, seed 20261005: the run completed at
  2026-11-18T08:49:39Z but the recording pipeline was cut while it finished,
  so no JSON row survives.  It is not reported and not used.
  (`progress.txt` keeps the note.)

## 7. Reproduce

```bash
cargo run --release --example find_a3_bench_curves              # the 28-bit rungs
cargo run --release --example a3_s4_ladder -- 28 20261005 p256 s3
cargo run --release --example a3_s4_ladder -- 16 20261006 p256 quartic
cargo run --release --example a3_stage_ladder -- 28 20261005 p256
cargo test --release --lib cryptanalysis::ec_index_calculus
```

Data files: `ladder.jsonl` (anchored + quartic), `two_decomp.jsonl`
(2-decomposition), `control_before.jsonl` (HEAD~1 control), `progress.txt`
(per-run UTC timestamps, including the abandoned cell), `summarise.py`,
`build_ledger_block.py`, `update_ledger_rest.py` (the ledger numbers are
generated from the recordings, not typed).
