# Certicom 131-bit index-calculus program: ECC2-131 and ECCp-131

**Status (2026-10-20).** Cost model, gates and two falsifiers, all measured
on public synthetic instances at toy sizes. **No sub-rho claim, no
key-recovery claim, no statement about the security of a deployed curve.**
The Certicom challenge targets are *named* here only to fix parameters and
the rho reference; nothing in this program is run against them.

**Class (AGENTS.md §3): measurement / accounting.** The gap to rho does not
close at any size measured; a gate is a falsification target, not a result.

| | |
|:--|:--|
| study directory | [`research/certicom131_ic_program_20261020/`](../../research/certicom131_ic_program_20261020/) (records, tables, figures, report PDF) |
| gate calculator | [`examples/certicom131_ic_gates.rs`](../../examples/certicom131_ic_gates.rs) → `gates.json`, `gates.md` |
| Falsifier A (binary) | [`examples/certicom131_fppr_dreg.rs`](../../examples/certicom131_fppr_dreg.rs), `ladder_measure_mb` and `chained_s3_solution_count_fast` in [`koblitz_bench.rs`](../../src/cryptanalysis/koblitz_bench.rs) |
| Falsifier B (prime) | [`examples/certicom131_prime_combined_count.rs`](../../examples/certicom131_prime_combined_count.rs); ecbench rows `pkm_ext/` |
| report | [`REPORT.pdf`](../../research/certicom131_ic_program_20261020/REPORT.pdf) |

## 1. The two targets and the rho reference

Parameters are the Certicom challenge list's (`BinaryCurve::certicom_challenge`
for ECC2-131, `CurveParams::certicom_eccp131` for ECCp-131, the latter taken
from the generated `certicom131-native-20261018/gpu/certicom131/params131.h`
and checked on the curve, `n` prime, `n·G = n·Q = O`, by
`certicom_eccp131_is_consistent`).

| | ECC2-131 | ECCp-131 |
|:--|:--|:--|
| field | `F_2[t]/(t^131 + t^13 + t^2 + t + 1)` | `F_p`, `p < 2^131` (131-bit prime) |
| curve | `y² + xy = x³ + a x² + b`, `a ∉ F_2`, random `b` | `y² = x³ + ax + b`, random |
| cofactor | `h = 2` | `h = 1` |
| `log₂ r` | 130.000 | 130.187 |
| rho with negation map, `√(πr/4)` | `2^64.83` GAE | `2^64.92` GAE |

Everything below is in group-addition equivalents (GAE), the unit of the
ledger. "Matched rho" is the negation-map rho in that unit. Both
families' unpriced native work must be priced at 0, 0.1 and 1 GAE per
operation before any two arms are ranked (AGENTS.md §5a); this program
ranks no arms, and the gates below are stated at two price points that
bracket the band (§3.1).

### What is structurally unavailable

| tool | why it does not apply |
|:--|:--|
| GHS / Weil descent to a genus-`g` cover | needs a proper subfield `F_{2^d}`, `1 < d < n`, over which the curve's coefficients lie. `131` is prime: the only subfield is `F_2`. |
| ECCp-131: any subfield or Weil-restriction route | `p` is prime, so `F_p` has no subfield. The only algebraic handle on `x ∈ F_p` is "integer modulo `p`". |
| Frobenius-orbit savings (ECC2K-style, factor `n`) | needs `a, b ∈ F_2`. ECC2-131 has `a ∉ F_2`, so the Frobenius is not an endomorphism of the curve. |
| Cyclotomic / Frobenius-stable factor bases | Frobenius-stable `F_2`-subspaces of `F_{2^131}` correspond to divisors of `x^131 − 1`; `ord_131(2) = 130`, so only the dimensions `0, 1, 130, 131` exist ([`RESEARCH_KOBLITZ_INDEX_CALCULUS.md`](../../RESEARCH_KOBLITZ_INDEX_CALCULUS.md) l.253–267). Even if they existed, they would need the Koblitz structure ECC2-131 lacks. |
| Negation-map halving | available and already in the rho reference (it is why rho is `√(πr/4)`, not `√(πr/2)`). |

What remains for ECC2-131 is the vector-space (Diem, FPPR) factor base
without Frobenius structure; for ECCp-131 it is the Petit–Kosters–Messeng
(PKM) tower base and the small-abscissa base with Coppersmith lattices.

## 2. Binary: FPPR cost model at `n = 131`

Factor base: the points with `x ∈ V`, `dim_{F₂} V = ℓ` (`ℓ ≈ n/m`), about
`2^{ℓ−1}` columns modulo negation. A random combination `aG + bQ` decomposes
into `m` base points with probability

    p_rel = min(1, 2^{mℓ−n} / m!).

Total cost, in GAE (`examples/certicom131_ic_gates.rs`):

    cost(ℓ) = 2^{ℓ−1} / p_rel · (1 + κ_dec · T_dec(D))  +  κ_LA · m · 2^{2(ℓ−1)}
    T_dec(D) = M(D)^ω / 64,    M(D) = Σ_{k≤D} C(N, k)

* `2^{2(ℓ−1)}` is the sparse linear algebra, `m` non-zeros per row
  (Wiedemann: `m · cols²`); the `1/log` refinement is an additional factor,
  not modelled.
* `T_dec` is the 64-bit word cost of eliminating the degree-`D` Macaulay
  matrix of the Weil-restricted `S_{m+1}` system, `D` its **solving degree**,
  `N` its number of Boolean unknowns. Two widths are priced: `N = mℓ` (the
  Weil-restricted `S_{m+1}` in the summand coordinates only, the favourable
  case for the attacker) and the **chained** width `N = (m−2)n + mℓ` that
  the measured systems actually have (`m−2` intermediate points).
* One group addition per attempt (forming the combination) is a floor no
  solver removes; `D = 0` is a free decomposition.
* Price points (the §5a band is not applicable to a model with no native
  counters, so two bracketing points are used instead):
  *generous* `κ_dec = 2^{-10}`, `κ_LA = 2^{-4}`, `ω = 2`; *plain*
  `κ_dec = κ_LA = 1`, `ω = 2.81`.

### Free-decomposition floors and gates

The **floor** is the cost with `T_dec = 0`: the best any decomposition oracle
could do for a given `m`. The **gate** `D*` is the largest solving degree at
which the optimised cost still reaches the matched rho `2^64.83`.

| `m` | floor, generous | floor, plain | gate `D*` `N = mℓ`: generous / plain | gate `D*` chained: generous / plain |
|--:|--:|--:|:--|:--|
| 2 | `2^86.6` (ℓ=45) | `2^88.0` | none | none |
| 3 | `2^65.2` (ℓ=34) | `2^67.2` | none | none |
| 4 | `2^52.5` (ℓ=28) | `2^54.8` | 3 / 1 | 2 / 1 |
| 5 | `2^44.5` (ℓ=24) | `2^46.8` | 4 / 2 | 3 / 1 |
| 6 | `2^38.7` (ℓ=21) | `2^41.1` | 5 / 2 | 3 / 1 |

Reading (model-derived, not measured):

1. **`m = 2, 3` cannot reach rho at `n = 131` even with a free oracle**: the
   `m = 3` floor `2^65.2`–`2^67.2` sits above `2^64.83` because
   `2^{2(ℓ−1)}` linear algebra at the balancing `ℓ = 34` already exceeds it.
   For ECC2-131 `m = 3` is closed by the linear algebra alone, at any
   solving degree; the program measures `D` at `m = 3` only because that is
   the arity the instrument reaches, as a calibration of the *mechanism*
   for the `m ≥ 4` rows.
2. **The first usable arity is `m = 4`**, with a gate of `D* ≤ 3`
   (generous) and `D* ≤ 1` (plain). `m = 5, 6` allow `D*` up to 4–5 only
   at the generous price and only with the favourable width.
3. The gates are **upper limits on D** for every model assumption that was
   favourable to the attacker (smaller matrix width, `κ_dec = 2^{-10}`,
   `ω = 2`, no Gröbner-basis fill-in). A measured `D` above them
   closes the family *under this model*; the two price points differ by up to 3 in `D*` (`m = 6`, `N = mℓ`: 5 against 2;
   `figures/fig_gates.svg` shows both), and the model has no sensitivity
   analysis beyond them.
4. The cost of deciding the gates' premise grows with `D` exactly as
   `C(N, D)^ω`; a gate of `D* ≤ 3` on 100–400 unknowns means the solver has
   to be essentially linear algebra on a degree-3 Macaulay matrix.

## 3. Prime: PKM tower and the Coppersmith limits at `p ≈ 2^131`

### 3.1 PKM tower (`prime_tower.rs`)

`V = { x : L(x) = 0 }`, `L` a composition of `t` low-degree maps whose
preimage tree splits over `F_p` (Kummer: `V = g·μ_{2^t}`, needs `2^t | p−1`;
isogeny form: no condition on `p`). The decomposition system has
`N = m·t` unknowns over `F_p` and `|V| = 2^t`. Cost:

    cost(t) = 2^t · m!·r / 2^{mt} · (1 + κ_dec · M_N(D)^ω)  +  κ_LA · m · 2^{2t}

| `m` | floor, generous | floor, plain | gate `D*`: generous / plain | `t*` at gate |
|--:|--:|--:|:--|:--|
| 2 | `2^87.5` | `2^88.7` | none | – |
| 3 | `2^66.2` (t=34) | `2^68.2` | none | – |
| 4 | `2^54.1` (t=27) | `2^56.3` | 3 / 1 | 33 / 31 |
| 5 | `2^45.8` (t=23) | `2^48.5` | 3 / 1 | 28 / 26 |
| 6 | `2^40.2` (t=20) | `2^42.8` | 4 / 2 | 24 / 24 |

The measured PKM reading on the toy ladder
([`RESEARCH_PKM_TOWER_ORACLE.md`](../../research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md) §16)
is `D = 6, 6, 7, 7` over `N = 9`–`18` at `m = 3`: a plateau at 7, "inconclusive",
not a growth law. Even a bounded `D = 7` is above every gate at every `m`
(cost at `D = 7`: `2^89`–`2^98` generous, `2^129`–`2^139` plain, `gates.md`).
**The gate for the PKM tower is therefore `D ≤ 3` at `m ≥ 4`, and a bounded
plateau at 7 does not meet it.** The PKM note's own lower bound (§3.3: XL
degree `≥ d + δ`, `δ ≈ N`) predicts growth; the observed plateau at 7 contradicts
the naive form of that bound and remains unexplained. Resolving it is the one
open item that could move this gate.

### 3.2 Small-abscissa base and the Coppersmith limit

Base `x ≤ B = p^δ`; an `m`-decomposition is a small root of
`S_{m+1}(x_1,…,x_m, x_R) ≡ 0 (mod p)`. Relations are free if the lattice
oracle is `O(1)`: `(m!/2^m)·p/B^{m−1} + κ_LA·m·B²`. The gate is the least `δ`
for which relation collection reaches rho; the reach of the Jochemsz–May
lattices is `δ_max = 1/(m(m+1)2^{m−2})`
([`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`](../../RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md) §2.2).

| `m` | `δ` needed (relations ≤ rho) | `δ` allowed (LA ≤ rho), generous / plain | lattice reach `δ_max` | shortfall |
|--:|--:|:--|--:|--:|
| 2 | 0.494 | 0.261 / 0.246 | 0.167 | 3.0× |
| 3 | 0.249 | 0.259 / 0.243 | 0.0417 | 6.0× |
| 4 | 0.169 | 0.257 / 0.242 | 0.0125 | 13.5× |
| 5 | 0.129 | 0.256 / 0.240 | 0.0042 | 31× |
| 6 | 0.106 | 0.255 / 0.239 | 0.0015 | 71× |

`m = 2` needs `δ = 0.494` but the linear algebra caps `δ ≤ 0.26`: no window.
`m ≥ 3` has a window, so the whole shortfall is the lattice reach (**the gate
is `δ ≥ 0.169` at `m = 4`**, against the reach `0.0125`). This reproduces
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.4 (first arity that could
beat rho is `m = 4`) at the exact `n = 131` constants.

## 4. Falsifier A (binary): the solving degree on random curves

*Pre-stated question.* Does the resolving degree `D` of the Weil-restricted
chained `S_{m+1}` system on **random (non-Koblitz) binary curves** at fixed
`m = 3`, `ℓ ≈ n/3`, stay at or below the gates of §2, grow slowly, or grow
like the Koblitz systems already measured?

*Instrument.* `ladder_measure_mb` is the repository's `dreg_ladder` pipeline
(exact Boolean solution count, `first_fall_degree`, `solving_degree_from` on
the sparse Macaulay matrices; a draw counts only if it has no solutions, so
`D` is the **refutation degree**, the protocol of
`research/dreg_*`), with the constant `b` of the summation polynomial free
instead of fixed to 1. Over `F_{2^n}` the summation polynomials of
`y² + xy = x³ + ax² + b` depend on `b` alone, so a random curve's system
differs from the Koblitz system in one place only. Each draw takes `b` from a
real random curve of `ic_boundary::random_binary_instance` (large
prime-order subgroup, ICV1-named) and measures the same `(V, x_R)` with
`b = 1` as the paired control.

*Limits stated up front.* The brief asked for `n` up to the `u64` cap of 63.
That is not reachable by this instrument: the chained system has
`n + 3ℓ` unknowns (84 at `n = 63`) and its degree-6 Macaulay matrix has
more than `10^8` columns. The reachable range is `n ≤ 13` (`ℓ ≤ 4`, ≤ 25
unknowns) with a handful of draws and `n = 17, 19` only as bounds if at all
(§4.2). The summation polynomial exact-count shortcut
`chained_s3_solution_count_fast` (`O(4^ℓ·n)`, equal to the exhaustive count
by test) is validated and could serve larger `n` in a different solver, but
the solver is the limit here, not the count.

### 4.1 Measured (seed 20261020; every draw verified unsatisfiable by exact count)

| n | ℓ | S = n−3ℓ | unknowns | draws | random-b `D` | `b = 1` `D`, same `(V, x_R)` | first fall degree | semi-regular reference |
|--:|--:|--:|--:|--:|:--|:--|:--|--:|
| 5 | 1 | 2 | 8 | 24 | 4 ×24 | 4 ×20 (4 satisfiable) | 2–3 | 4 |
| 7 | 2 | 1 | 13 | 24 | 5 ×24 | 5 ×23 (1 satisfiable) | 2–3 | 5 |
| 11 | 2 | 5 | 17 | 8 | 5 ×8 | 5 ×8 | 3 | 5 |
| 11 | 3 | 2 | 20 | 4 | 6 ×4 | 6 ×1 (3 not paired) | 3 | 6 |

(`research/certicom131_ic_program_20261020/tables/dreg_summary.md`, raw JSON
lines in `runs/c*.jsonl`; wall time is on a host at load ≈ 100 and is a
practicality note only: median 0.0 s, 0.1 s, 1.2 s and ≈ 2,300 s per draw
for ℓ = 1, 2, 2, 3.)

* **`D(ℓ) = 4, 5, 6` for `ℓ = 1, 2, 3`, zero spread over 60 draws; `D` equals
  the semi-regular reference exactly in every cell.** Read at
  fixed `ℓ`, `n = 7 → 11` (a 4-bit increase in `n`) leaves `D = 5` unchanged,
  as the earlier ladder found for `ℓ = 2` up to `n = 12`
  (`research/dreg_ell_grid_20260925/RESULTS.md`): `D` tracks `ℓ`, not `n`.
* **Random `b` is indistinguishable from Koblitz `b = 1`:** 52 of 52 paired
  draws where both sides are unsatisfiable have the same `D` (the other 5
  pairs have one satisfiable side, which has no refutation degree). The
  non-Koblitz constant `b` does **not** lower the solving degree in these
  cells.
* **First fall degree is 2–3 in all 60 draws** while `D` is 4–6: the gap
  `D − FFD` is 1–2 at `ℓ = 1`, 2–3 at `ℓ = 2` and 3 at `ℓ = 3`, i.e. it grows with `ℓ`.
  A first-fall-degree assumption (`D ≈ FFD + O(1)`) is therefore **not
  supported** on these systems, in agreement in kind with the
  Kosters–Yeo and Huang–Kosters–Yeo evidence against it (their examples
  are different systems: a group morphism to `F_2` that gives a linear
  polynomial after the descent of `S_3` (`m = 2`, first fall degree 2), and an
  HFE-like construction; this
  program reproduces only the statement that FFD under-estimates `D`).
  Kousidis–Wiemers place the degree fall of the *unchained* `S_{m+1}` descent
  at `m² − m + 1` (`7` at `m = 3`); the chained system measured here is a
  different presentation and its fall is 3.
* The semi-regular degree `D_sr` (a formula, `certicom131_ic_gates` last
  table) for the chained `m = 3` system with `ℓ = ⌊n/3⌉` is 5, 5, 7, 7, 8, 9,
  10, 12, 15, 21, **39** at `n = 5, 7, 11, 13, 17, 19, 23, 31, 43, 63, 131`
  and for `m = 4` it is **60** at `n = 131` (`ℓ = 33`).

### 4.2 What did not run, and why

Cells `(13,3)`, `(13,4)`, `(17,5)`, `(19,6)` were launched (degree cap 7 or 6,
50M row/column caps) and produced **no draw** in ≈ 3 h of wall time at a host
load of 90–170, so there are no results for `n ≥ 13`. A draw costs about
`×10³` more from `ℓ = 2` (1.2 s at `n = 11`) to `ℓ = 3` (`≈ 2,300` s wall at
`n = 11`, load-inflated by an unmeasured factor of order 5–10). This is the practical wall of the
instrument: the `u64` cap `n ≤ 63` is reachable for the *count*
(`chained_s3_solution_count_fast`) and for the *model* but not for the
Macaulay measurement. The previous committed cells at `ℓ = 4` (`n = 11, 13`,
`D = 6`) and `ℓ = 5` (`n = 10`, `D ≥ 7`) are Koblitz `b = 1`
(`research/dreg_ell_grid_20260925/RESULTS.md`) and are not repeated here.

### 4.3 `D(ℓ)` fit and the extrapolation to `n = 131`

Plain-Macaulay (F4/XL-type) resolving degree at `m = 3`, all draws of
this program plus the committed Koblitz ladder (`b = 1`):

| ℓ | 1 | 2 | 3 | 4 | 5 |
|:--|--:|--:|--:|--:|--:|
| `D` | 4 | 5 | 6 | 6 | ≥ 7 |

* Over `ℓ = 1…3` this is exactly `D = ℓ + 3` (three collinear points, no
  residual); including `ℓ = 4, 5` the slope falls below 1. The data cannot
  say whether `D` keeps growing linearly in `ℓ` (as `D_sr` does) or
  plateaus, as the PKM tower's `D = 7` plateau over `N = 9–18` did.
* **Two bracketing extrapolations to `n = 131`, `m = 3`, stated as such:**
  (H-sr) `D` tracks the semi-regular reference: `D ≈ 39` (`m = 3`), `≈ 60`
  (`m = 4`). (H-flat) `D` plateaus near its `ℓ = 3–5` value: `D ≈ 6–7`. Both
  exceed every binary gate of §2, including the most favourable cell
  (`m = 6`, `N = mℓ`, generous price, `D* = 5`).
  The uncertainty is the extrapolation range itself: the data end at `ℓ = 3`
  (60 draws) and `ℓ = 5` (committed Koblitz cells), against `ℓ ≈ 30` at
  `n = 131`, a factor 6–10 outside the measured range. **Neither bracket is
  a measurement at `n = 131`.**
* **The best instrument known in this repository goes below plain
  Macaulay**: the MutantXL closure refutes at degree **4** on `m = 3`
  (`ℓ ≤ 6`) and `m = 4` (`ℓ ≤ 4`, `N ≤ 42`) and rises past 4 at `m = 3`,
  `ℓ = 7` (`research/dreg_closure_scale_20261016/RESULTS.md`). At degree 4
  the gates of §2 are missed by 1–2 degrees at `m = 4` (model cost
  `2^65.3` against rho `2^64.83` at the most generous price and favourable
  width, `2^76.9` at the chained width), consistent with the study's own
  derived row (+7.6 bits over rho for ECC2K-130, a different price list and
  a Koblitz curve). **The open falsifier that would change the picture is
  the closure at `m ≥ 5`, `D = 4` constant** (untested; `m = 4, ℓ = 5, 6`
  is the next registered cell).

## 5. Falsifier B (prime): the combined-system solution count

*Pre-stated claim under test* ([`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`](../../RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md)
§4 item 2; listed as never run in `research_notes/Index calculus leads next
steps/crypto_prime_field_ic_docs.md` Q2): for a structured base `V ⊂ F_p`
the combined system `{S_{m+1}(x_1,…,x_m,x_R) = 0, L_V(x_i) = 0}` has
`≈ B^{m−1}` solutions over `F̄_p`, so any solver that enumerates them costs
`≥ B^{m−1}`; the lever is dead if the count is `≥ B^{m−1}`.

*What the count is.* `L_V = ∏_{v∈V}(X − v)` is square-free and splits over
`F_p`, so the ideal is radical and the number of solutions over `F̄_p`
**equals the number of `F_p`-solutions**: the decompositions of `x_R` over
`V`. That count is `≈ 2^m B^m / #E` (the random-oracle expectation on
on-curve abscissae), which is `≪ B^{m−1}` until `B ≈ #E`; **the note's dead
criterion `N ≥ B^{m−1}` is therefore never met at any `B < #E`, for any base,
and cannot discriminate between bases.** The informative quantity is the
**excess** `N / E` of a structured base against the random-oracle
expectation and against a random subset of the same size: an excess ≫ 1
would be an algebraic lever in the relation probability.

*Measured* (exact, `m = 3`, four 20–28-bit curves of the PKM ladder, eight
on-curve targets each, `B` from 2^6 to 2^11, 5 base types; cells with
expected count ≥ 20 per target pooled):

| base (on-curve abscissae of `V`) | cells pooled | Σ N₃ | Σ E₃ | N/E | 95% interval |
|:--|--:|--:|--:|--:|:--|
| coset `g·μ_{2^t}` | 8 | 11,324 | 11,322.7 | 1.000 | [0.968, 1.032] |
| subgroup, order a non-power of 2 | 7 | 16,348 | 16,470.8 | 0.993 | [0.966, 1.019] |
| interval `[1, B]` | 8 | 11,591 | 11,901.4 | 0.974 | [0.943, 1.005] |
| random subset (control) | 7 | 10,445 | 10,726.1 | 0.974 | [0.941, 1.006] |
| Kummer `μ_{2^t}` | 7 | 11,107 | 11,461.6 | 0.969 | [0.938, 1.000] |

(Per-cell rows: `tables/fb_cells.md`; records `runs/fb_*.jsonl`, 1,600 + 800 +
960 + 960 lines. Intervals are Poisson on triples, each unordered triple being
counted three times; the expectation `E₃ = 8B³/(6·#E)·3` per target was
validated against the `m = 3` counts: 65.1 observed vs 65.2 expected for the
20-bit `μ_{2^9}` cell.)

No structured base departs from the random-oracle expectation by more than
its sampling interval: `μ_{2^t}` and its coset, an interval `[1,B]` and a
subgroup of non-2-power order are all within 3% of expectation, and the
random control sits at 0.974 (the expectation ignores repeated-coordinate
and sign-collision terms, a ~2.5% bias common to all bases). **Verdict
(null): no excess; the lever "a structured base has more decompositions
per element" is not supported on these sizes.** The `B^{m−1}` dead
criterion is also not met: the mean count per target is `N₃ ≤ 560` against
`B² ≥ 2.5·10⁵` in the 20-bit cells (ratio ≤ `2·10⁻³`) and `32` against
`1.5·10⁶` at 28 bits.

Scope: `m = 3` only; the sizes (20–28 bit) say nothing about the
multiplicative-subgroup bases at `p ≈ 2^131` where `p − 1` has no divisor
near `2^t`.

### 5.1 ecbench rows past 28 bits

One new row on the registered `prime-kummer` ladder
(`research/pkm_tower_ecbench_20261013/`, 20–28 bits): **30 bits**,
`icv1-fp30-t49927-d70f5de2` (`p = 1,000,574,977`, `#E = 1,000,525,051` prime,
cofactor 1, `log₂ r = 29.90`, generated by the ladder's own
`examples/prime_tower_curves.rs --bits 30 --two-adicity 12 --seed 20261013
--max-cofactor 4`), Kummer tower `t = 12` (4,184 points, 2,092 columns; the
ladder's `t` rule gives 12 and 12 is the largest the curve's 2-adicity
allows), same arms, 8 targets × 3 rounds + warm-up. Session
`ECBS1hfb8f08d66f01` (spec `ECS1h5e999ae301b7`), `pkm_ext/`:
**160 records, 160 verified, 120 replays reproduced, 0 problems** (`ecbench
verify --replay-all`). Operation counts only: the host was loaded (≈ 100) and
the run is L0 (`--cpus none --allow-busy`), so no wall-time claim.

| bits | log₂ r | rho (negation) `S` | IC + `rootfind-mitm` `S` | ÷ rho, w = 0 | w = 0.1 | w = 1 |
|--:|--:|--:|--:|--:|--:|--:|
| 28 (prior) | 26.20 | 0.936 | 131.6 | 140.6 [113.0, 182.3] | 158.7ᵃ | 238.7ᵃ |
| **30 (new)** | 29.90 | 0.921 | 146.4 | **158.9 [121.6, 221.4]** | **174.9** | **247.1** |

ᵃ the 28-bit end-point of the frontier campaign's PKM row (a different
session of the same ladder, `research/ic_frontier_campaign_20261015/README.md`;
its w = 0 value there is 141.3).
The 30-bit band (`pkm_ext/tables/p30.band.txt`: `rho-neg` 0.921 / 1.012 / 1.830;
`ic-rootfind` 146.4 / 177.0 / 452.2; `ic-mitm` 144.7 / 175.6 / 454.2;
`ic-abscissa` control 146.6 / 179.7 / 477.5 at w = 0 / 0.1 / 1) leaves the arm
order unchanged and the tower base equal to the plain base of the same size
within 1.2% (`ic-rootfind` / `ic-abscissa` = 0.999 at w = 0).

**Distance to rho: 159×** at 30 bits (up from 140.6× at 28), the same direction
and about the same size as the `r^{1/6}` pair-table floor predicts
(`0.048` bits of ratio per bit of `log₂ r` between the two cells, against 0.17
predicted and the ladder's `0.26 ± 0.28` fit; these three numbers are not
distinguishable at two points). This row is a **measurement, not an
improvement**: the gap grew. Extrapolating the `r^{1/6}` law to `log₂ r = 130.2`
gives about `×2^{16.7}` more than at 30 bits, i.e. `~10^7×` rho
(extrapolation with the exponent it rests on, not a measurement).

**Not done:** the 32-bit row. The exhaustive point count of the curve
generator (`prime_tower_curves`, `O(p)`) had not finished a 32-bit curve after
about two hours on the loaded host; no 32-bit spec exists. The generator
needs a faster point counter (e.g. BSGS/Schoof–Elkies–Atkin) to go further;
that is the next step for this row.

## 6. What this program does and does not conclude

**Measured.** (1) Plain-Macaulay `D` at `m = 3` on random binary curves is
`4, 5, 6` for `ℓ = 1, 2, 3`, equal to the semi-regular reference in every
cell, and identical to the Koblitz `b = 1` value on the same `(V, x_R)` in
52 of 52 paired draws: **no scaling deviation from random `b`.** (2) The
first fall degree stays 2–3 while `D` grows: the first-fall-degree
assumption does not predict `D` here. (3) No structured base on four
20–28-bit prime curves has more decompositions than a random base of the
same size (all within the sampling interval of expectation): **the
combined-system falsifier is null.** (4) The PKM Kummer row at 30 bits is
159× the matched rho (w = 0), 175× (w = 0.1), 247× (w = 1): **the gap grew**
from 141× at 28 bits.

**Derived.** The gates of §2–3: for ECC2-131, `m ≤ 3` is closed by linear
algebra at any `D`; at `m = 4–6` the solving degree must be `≤ 1–5`
(model-dependent; `≤ 3` at the first usable arity `m = 4`, generous price).
For ECCp-131: PKM `D ≤ 3` at `m = 4, 5` (`≤ 4` at `m = 6`); small-abscissa
`δ ≥ 0.169` at `m = 4` against a lattice reach of `0.0125`, a 13.5× shortfall
in the exponent.

**Extrapolated (stated uncertainty, not measured).** At `n = 131` the plain
solving degree of the `m = 3` system is between `≈ 6–7` (plateau) and `≈ 39`
(semi-regular) depending on which hypothesis the `ℓ ≤ 5` data supports; the
PKM tower's matched-rho gap at `log₂ r ≈ 130` is of order `10⁷` if the
pair-table `r^{1/6}` law holds. Both are far outside the measured range.

**Not concluded.** No claim that either 131-bit challenge is or is not
index-calculus-breakable; no cost below rho for any arm; no statement about
`m ≥ 5` closure degrees (unmeasured). The model's constants (`κ_dec`,
`κ_LA`, `ω`, no fill-in) are assumptions; the gates are stated at two price
points so a reader can see how much they move (`D*` by up to 3).

**What would change the verdict** (each a falsifier registered here for the
next round):

1. a closure/F4 solving degree **≤ 3 at `m = 4`**, or **≤ 5 at `m = 6`
   (generous, `N = mℓ`)**, at `ℓ ≥ 5` on random binary curves, held constant over
   ≥ 2 sizes with ≥ 4 draws per cell (the single most favourable gate cell
   and the nearest prior measurement: closure degree 4 at `m = 4`, `ℓ ≤ 4`);
2. a prime-field PKM `D ≤ 3` at `N = m·t ≥ 20` (the measured plateau is 7);
3. a prime-field small-root method reaching `δ ≥ 0.169` on `S₅`
   (`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4 item 1);
4. a combined-system count (Falsifier B) with `N/E ≥ 1.1` on a structured
   base at `B ≥ 2^10` and an interval excluding 1.

**Next improvement to try** (CLAUDE.md: name the next step): measure the
`m = 4` chained system on random binary curves at `ℓ = 5, 6` with the
closure instrument (`backlog/XP-20261018-m4-solving-degree-equivariant/`)
and extend the `prime_tower_curves` generator with a sub-`O(p)` point
counter so the `prime-kummer` ladder can pass 32 bits.

## 7. Not done in this change

* Falsifier A for `n ≥ 13` (no draw finished; §4.2) and `m ≥ 4` on random curves.
* The 32-bit prime-kummer row (§5.1).
* Registering `icv1-fp30-t49927-d70f5de2` and ECCp-131
  (`icv1-fp131-t29529377007934231835-9681b699`, from `scripts/curve_id.py prime`)
  in `docs/curves/registry.json`: the registry digest is pinned by
  `covers.json`, `cover-links.yaml` and the lab browser, whose regeneration did
  not complete here (disk full). Until then these two slugs are unregistered
  (AGENTS.md §11).
* `cargo test --release --lib`: 4,443 passed, 14 failed. Nine of the 14
  (`pollard_collab::cairn`, `pollard_collab::net`) fail with `Operation not
  permitted` on a local socket bind in this sandbox; the other five
  (`ecbench::gpu` ×2, `jv_cover` ×2, `koblitz_groebner::checkpoint`) are
  time-budget/process-spawn tests on a host at load 90–170. None touches a
  file of this change; I did not run them on the base commit.

## 8. Reproduce

```sh
cargo build --release --example certicom131_ic_gates --example certicom131_fppr_dreg \
  --example certicom131_prime_combined_count --example certicom131_report
./target/release/examples/certicom131_ic_gates --json research/certicom131_ic_program_20261020/gates.json
F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 \
  ./target/release/examples/certicom131_fppr_dreg --cells 11:3:7 --unsat 4 --seed 20261020
./target/release/examples/certicom131_prime_combined_count \
  --curves research/pkm_tower_ecbench_20261013/curves.jsonl --targets 8 --max-b 2048
./target/release/examples/certicom131_report research/certicom131_ic_program_20261020
```
