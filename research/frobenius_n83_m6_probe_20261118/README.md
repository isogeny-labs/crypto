# n = 83 m = 6 orbit decomposition: hit-count arithmetic and small-n E-model calibration (2026-11-18)

**Class: research design/analysis. Verdict: the m = 6 "TOOLING, likely affordable"
call in `research/frobenius_n83_relation_diag_20261118/REPORT.md` is wrong.**
Decompositions exist (`E ≈ 561`, and the calibration below shows the mean is
essentially exact). But *finding* one is a bichromatic collision in a group of
size `r/(2n)`. Every list-join method, the proposed `K³` table included, pays
at least **≈1.6× a complete Frobenius-rho DLP for ONE relation**. A rank-600
relation set costs **≈39–65× rho** with unlimited memory and **≈3,400× rho**
within 256 GB. m = 6 sits behind the same square-root MITM wall as
`research/slope_beta_design_20261118/`. It is FUNDAMENTAL relative to rho, not
tooling. No pod probe is proposed: none can change that verdict
(see "Decision"). All n = 83 figures are **estimates** (formulas shown). The
small-n numbers are **measured** (exhaustive, group-checked).
Claims here are unregistered with pcc: `[pcc:unverified]`.

## 1. Arithmetic (no compute; estimates)

Notation: `r = 2417851639230796216685689` (`log2 r = 81.000`), `n = 83`, `K = 600` orbits,
`N = 2nK = 99,600` signed-orbit points (closed under `±τ`, all in `<G>`, cofactor-projected:
`koblitz_n83_factor_base_sweep_20261008/README.md:149`). Raw arithmetic: `raw/n83_arithmetic.txt` (bc).

**Rho reference.** The repository's floor for signed-Frobenius rho is `√(π/4n)` steps per `√r`
(`docs/ic/LEADERBOARD.md:232`), i.e. `ρ = √(π r / (4n)) = √(πr/2)/√(2·83) = 1.51e11 = 2^37.14`
group operations, with canonicalisation and DP-table work unpriced (§5a: the same unpriced
classes, canonicalisation and hash probes, also appear in every MITM arm below and are priced at 0 there,
which favours MITM).

**Core collision law.** Let `L1` be a table of `A` triple-sums (stored by `±τ`-canonical class)
and `L2` a set of `B` triple-sums `t` queried as `canon(R − t)`. A query matches if
`R − t = ±τ^j s`. Since the base is `±τ`-closed, `±τ^j s` is again a triple-sum, so every match is
a 6-term decomposition. For a target independent of the lists each (entry, query) pair matches
with probability `2n/r = 166/r`, so

    H(A, B) = 166 · A · B / r          (expected verified decompositions)

and it is capped by the total `20·E` (a 6-set has C(6,3) = 20 triple splits). One hit needs
`A·B = r/166 = 2^73.6`. Total work `A + B ≥ 2√(r/166) = 2^37.81`, so

    one m=6 relation / rho  ≥  2√(r/166) / √(πr/(4n)) = 2/√(π/2) = 1.60     (independent of r)

| design | table `A` | lookups `B` | expected hits | ops for one hit | vs rho DLP |
|---|---:|---:|---:|---:|---:|
| REPORT.md probe: `K³` triples of orbit **representatives**, one target | `K³/6` | same | **2.7e-11** | – | – |
| same, + all 166 rotations/signs of R | `K³/6` | ×166 | **4.4e-9** | – | – |
| same, generous: 2K signed reps, 166 rotations | `(2K)³/6` | ×166 | **2.8e-7** | – | – |
| best balanced MITM (Frobenius-canonical table, unlimited memory: 1.2e11 entries ≈ 2 TB) | 2^36.8 | 2^36.8 | 1 | 2^37.8 | **1.60×** |
| memory-capped, 256 GB (`A = 2^34` at 16 B/entry) | 2^34 | 2^39.6 | 1 | 2^39.7 | **5.7×** |
| van Oorschot–Wiener parallel collision on `f1(i)=s_i`, `f2(j)=R−t_j` (memoryless) | – | – | 1 | ≈ `√(π r/(2·166))`·O(1) | **≈ 1–2×** (a rho walk in a different guise) |

So the proposed `K³ = 2.16e8` probe would return **zero with certainty (≈1e-8 – 3e-7 expected
hits)**. A zero from it says nothing about the base. The objection raised against it is correct.

**Variants that do not help.**
- *Frobenius normalisation of one summand / rotating R*: this is the factor `2n = 166` already
  inside `H`. It is a constant and is already counted.
- *Schroeppel–Shamir / Howgrave-Graham–Joux / Wagner k-tree*: these need either an ordering compatible
  with addition (to merge sorted sum-lists) or a quotient map to a small group (to
  constrain partial sums to a coset). A prime-order `r` subgroup has no nontrivial quotients, and EC
  coordinates are not additive. The 4-list tricks reduce *memory*, never the
  `√(r/2n)` *time*. Memoryless collision search (vOW) already achieves that time with no table.
- *Time–memory tradeoff (Hellman / Bernstein–Lange precomputation)*: this amortises across
  *many* DLPs. It applies equally to rho (precomputed rho: `r^{2/3}` precomputation,
  `r^{1/3}` per DLP), so it does not change the comparison.
- *Algebraic solvers (Semaev / Gröbner / XOR-SAT)*: these are the only route around the generic
  collision bound, and they need algebraic structure in the factor base (e.g. x in an
  `F_2`-subspace). This base is a union of 600 point orbits with no such structure. Invariant
  subspace dimensions are only 0, 1, 82, 83 (`koblitz_n83_factor_base_sweep_20261008/README.md:137`).
  For this base a SAT solve is an enumeration in disguise, which is consistent with every
  natural m = 5/6 call stopping at its cap while the planted control solved in 787 conflicts.

**Total cost of index calculus at m = 6.** The unknowns are the `K = 600` orbit logs plus log Q, so about
601 relations (each from `R = aG + bQ`; no separate descent). Linear algebra on a 601×601
six-sparse system is negligible. Relation collection with one shared table of size `A`:

    T(A) = A + 601 · r / (166 A),   minimised at A* = √(601 r/166) = 2^41.4  →  T = 2√601 · √(r/166)

| memory regime | `A` | total ops | vs rho |
|---|---:|---:|---:|
| unlimited (A* = 3.0e12 > 9.9e11 distinct triple classes, so not attainable) | 2^41.4 | 2^42.4 | 39× |
| table = all `N³/6/166 = 9.9e11` triple classes (≈16 TB) | 2^39.85 | 2^43.2 | **65×** |
| 256 GB (`A = 2^34`) | 2^34 | 2^48.9 | **3,370×** |

In general `IC_m6 / rho ≈ 1.6·√(#relations)`. Rho needs one collision. This IC needs
about K collisions of the same size. **No m = 6 decomposition search at n = 83 beats rho per
relation, and the full system is ≥ ~40× rho.** This is the β = 2/3 − 1/2 wall again: the
`slope_beta_design` m = 3 balance `c² + r/c` generalises to `A + K'·r/(2nA)`.

## 2. Empirical calibration of `E = N^m / m! / r` at small n (measured)

Tool: `tools/m6_calib` (Rust; field arithmetic is the repo's `src/binary_ecc/f2m.rs` included by
`#[path]`, because the full crate does not fit the disk budget; only the affine Koblitz formulas are new).
It enumerates `<G>`, so every base point has an exact log, and counts m-decompositions of **every**
nonzero target by a subset-sum DP over `Z/r`. The counts are exhaustive over all targets, not
sampled. Checks: moduli irreducible (`z^{2^n}=z`), panel orders/cofactors reproduced (unit test),
`G^r = O` with no earlier `O`, orbit size exactly `2n` with `τ ↦ λ`, count invariance under
`R ↦ τR, −R` (1,000 samples per row, all pass), and one extracted decomposition per run re-added
with point arithmetic (30/30 group-checks ok, `raw/calib-stderr.log`).

Panel = `ecbench_frobenius_orbit_vs_rho_20261014` curves with `r < 2^21`, and the same `c` (orbit count) per
curve. Bases: min-x orbits (seed 0) plus 4 random orbit selections (seeds 1–4) per curve.
Variant "pairfree" = distinct points and never both P and −P (the non-degenerate decompositions).
Raw rows: `raw/calib-*.jsonl` (300 rows; hashes in `raw/SHA256SUMS`); aggregated: `raw/summary.jsonl`.

| curve | N | r | m=2 | m=3 | m=4 | m=5 | m=6 | dispersion var/mean (all m) | P(≥1 decomp) / Poisson(1−e^−mean) |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| a0 n13 c1 | 26 | 2003 | 0.924 | 0.781 | 0.601 | 0.416 | 0.256 | 0.54–1.92 | 0.90–1.00 |
| a1 n17 c1 | 34 | 65587 | 0.941 | 0.830 | 0.684 | 0.523 | 0.369 | 1.24–4.13 | **0.53**–0.92 |
| a0 n19 c2 | 76 | 130873 | 0.974 | 0.922 | 0.850 | 0.760 | 0.660 | 0.58–1.37 | 0.89–1.00 |
| a0 n23 c3 | 138 | 2095853 | 0.986 | 0.957 | 0.915 | 0.862 | 0.800 | 0.75–2.50 | 0.91–1.00 |
| a1 n29 c1 | 58 | 42457 | 0.966 | 0.899 | 0.806 | 0.695 | 0.575 | 0.57–1.14 | 0.93–1.00 |
| a0 n31 c2 | 124 | 1439393 | 0.984 | 0.952 | 0.906 | 0.848 | 0.779 | 0.97–1.59 | 0.91–1.00 |

(The m columns give measured mean decompositions per target ÷ `E_model`, pair-free. Every seed gives the same mean.)

**Reading.** The mean is **not** a model. It is a counting identity. Every pair-free m-subset of the
base sums to some element of `<G>`, so `mean = C(N/2, m)·2^m / r` exactly (up to the `R = O` row),
and `mean/E = Π_{i<m} (1 − 2i/N)`. The table reproduces this to all printed digits, with the same
value for all five bases. The deflation is pure finite-N. **At n = 83 (N = 99,600, m = 6) the factor is
0.9997, so the mean is 560.6 against `E = 560.8`.** The diagnosis's "possibly off by one or more orders" caveat is
wrong for the mean. Signed-Frobenius structure cannot deflate it, because count(R) is constant on
`±τ`-orbits of R and the orbits partition the targets. What structure *can* change is the spread.
Measured dispersion stays within 0.5–4.1× Poisson, and the fraction of targets with ≥ 1
decomposition is 0.53–1.0× the Poisson prediction (worst: a1 n17, c = 1, where one orbit is the whole
base). Not tested: n = 37/41 (`r` too large to enumerate `<G>`). Not reproduced: the panel's exact
orbit choices (`c` matched, orbit choice = min-x / random). The identity makes the mean
independent of that choice.

So E was never the problem, and existence was never the bottleneck. The bottleneck is the
search, and section 1 shows it is generic.

## 3. Decision: no pod probe

The task's condition for a proposal was an n = 83 design with ≥ ~3 expected hits in ≤ 24 h on one
32-vCPU / 256 GB pod.
- At the only measured n = 83 throughput in the record (~4,000 group adds/s per core, compact probes,
  REPORT.md citing `RESULT.md:94-109`), a 24 h pod does 1.1e10 ops. The best split gives
  `H = 166 (D/2)² / r = 2.1e-3` expected hits.
- Three hits within 256 GB need `2^41.2 = 2.56e12` canonicalised adds plus random DRAM probes, i.e. **≥ 9.3e5 per
  core-second**. That is 230× the measured rate, from a kernel that does not exist, and it is unmeasured.
- Even if it ran and hit, it would be **uninformative**. The hit rate is fixed in advance by
  `H = 166AB/r`, the mean is an exact identity, and Poisson-like spread is measured here. A
  success would show only that the code works, which the n = 23–31 runs above already show for free.
  It cannot move the cost per relation below 1.6× rho, which is the decision-relevant quantity.

So `PROPOSAL.md` is deliberately absent. **Implication for the n = 83 verdict:** re-label m = 6 from
"TOOLING, likely affordable" to **FUNDAMENTAL relative to rho (generic-collision wall)**. The same
argument covers m = 5 (a 3+2 split, with `E < 1` as an extra penalty) and any m. For a structureless orbit base,
IC relation finding is rho-sized per relation, and the system needs ~K of them. Further spend on the
n = 83 orbit-base relation phase is not justified. A future lever has to change the *base*
(algebraic structure a non-generic solver can exploit) or amortise one table across many DLPs. A larger
search engine will not help.

## Reproduce

    cd tools/m6_calib && CARGO_TARGET_DIR=../../target cargo test --release --offline   # 24 tests incl. f2m.rs's own
    ../../target/release/m6_calib <n> <a> <c> 6 0 1 2 3 4 > raw/calib-a<a>-n<n>-c<c>.jsonl

Walls (one local core): n13 0.02 s, n17 0.3 s, n19 1.2 s, n23 56 s, n29 0.3 s, n31 34 s.

## Gaps against house rules (stated, not hidden)

- No PDF/graph (`research-visuals`), no `bench.record`, no pcc claim, no research-kb receipt in this
  worktree pass. Disk was at 98% and the sandbox had no Python. This is a local analysis, not a benchmark row.
- Not a controlled experiment in the `controlled-experiment` sense. The central claim (mean identity) is an
  exact computation, and the dispersion figures are descriptive. The A/A analogue is the 5 base selections per curve.

## Sources checked

- `research/frobenius_n83_relation_diag_20261118/REPORT.md` (E table, K³ probe, m = 6 verdict)
- `research/koblitz_n83_factor_base_sweep_20261008/README.md:30,137,149` (curve, invariant subspaces, subgroup-projected base)
- `research/ecbench_frobenius_orbit_vs_rho_20261014/README.md` (panel, `c`, cofactors)
- `research/slope_beta_design_20261118/{README,PREREGISTRATION}.md` (MITM `r^{2/3}` balance)
- `docs/ic/LEADERBOARD.md:232` (rho floor `√(π/4n)`), `AGENTS.md` §5a
