# Item 14 protocol: the first prime-field arity m = 4 cost cells

Frozen 2026-10-18, before any measured card run. Source design: section "14"
of `research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places and loosened nowhere; every deviation and every filled-in gap is listed
at the end. The pre-freeze probe runs (made while building and calibrating the
instrument, none of them kept as data) are disclosed in the last section.

Class: **first measurement of a primitive (the m = 4 decomposition oracle) and
of the end-to-end cost it implies**, at four prime-field sizes. It touches the
exponent question only through the rule below. The success criterion
(CLAUDE.md, "Success criterion") applies: the first measured m = 4 cells become
the baseline; every row reports its gap to the matched Pollard rho as the
distance remaining, not as a pass/fail gate.

## Question

What does one m = 4 (S5) decomposition attempt cost at a small-x base, with each
available oracle, and where does the measured cost put the m = 4 route against
rho?

Setting. A prime-field curve `y^2 = x^3 + a x + b` of prime order `n`. The base
is `F = {P : x(P) in [1, B]}`, both signs. An m = 4 attempt takes a target `R`
and decides whether `R = e1 P1 + e2 P2 + e3 P3 + e4 P4` with `P_i in F`, and if
so returns the points. By Semaev's construction this holds iff
`S5(x1, x2, x3, x4, xR) = 0` for the abscissae `x_i in V`, the set of abscissae
of `F` (`|V| = b_x`, `|F| = 2 b_x`). A full index calculus (IC) collects
`b_x + 1` independent relations `sum s_i l_{x_i} - b d = a (mod n)` from targets
`R = a G + b Q` and solves for the logarithm `d` of `Q`.

## What already exists, and what is reused (read before designing)

* **No m = 4 decomposition cost cell with the small-x base exists in the
  repository.** `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (2026-10-08)
  derives the table (section 2.4: `B* = (p T_dec)^(1/5)`, "the first arity that
  could beat rho is m = 4") and measures only Probe A (preprocessing rho) and
  Probes B and C (the `S3` lattice, m = 2). The framework's prime oracles
  (`subtract`, `mitm`, `rootfind-subtract` in
  `src/cryptanalysis/ic_framework/plugins.rs`) take `m <= 3`.
* **Arm (c), the PKM tower oracle, is already measured at m = 4 by committed
  results, and is reused, not re-run.** `research/notes/index-calculus/
  RESEARCH_PKM_TOWER_ORACLE.md` rounds 2, 3 and 6: Kummer family, `m = 4`,
  `p1 = 2013265921`, tower heights `t = 2, 3, 4` (`N = m t = 8, 12, 16`), F4 over
  `F_p` on the tower ring, refuting systems, with the engine's multiply-add
  counts and solving degrees. The rows used are
  `research/pkm_tower_round2_20260925/runs/M4-kummer-m4-p1.jsonl` (`N = 8, 12`),
  `research/pkm_tower_round3_20260925/runs/M4b-kummer-m4-p1-N16.jsonl`
  (`N = 16`, the 55-minute run) and, for the later engine at `N = 12` only,
  `research/pkm_tower_round6_20260930/bench/rows/m4-N12-ab0-B.jsonl`. The
  `reuse` subcommand copies the fields into `raw/` with the source path, and the
  card binds the three files by sha256. **No arm-(c) cell is re-measured.**
  Their `p1` (31 bits) is not one of this item's four sizes; see Deviation 3.
* **Arm (d), the Coppersmith lattice, has committed measurements at `S3`
  only** (`examples/prime_ecdlp_breakthrough_probe.rs`, Probes B and C; table in
  `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` sections 3.2 and 3.3): the
  Howgrave-Graham criterion holds up to `delta = 0.10` at every level 1..3 and
  never beyond, the heuristic criterion fades at 0.14 to 0.16. Those are reused
  as the measured side of the control; the probe cannot be re-run here (it
  depends on the `crypto_lib` crate, which does not compile at this branch base,
  and on big-integer LLL).
* **Arms (a) and (b) have no committed counterpart.** Arm (a) is new code
  because the framework's pair-table oracle stops at m = 3; arm (b) is new
  because the committed F4 (`f4_fp`, `f4_fp_tower`) cannot be built (library
  does not compile) and is not specialised to the small-x membership polynomial.

## Arms

All arms see the same curve, the same base and the same targets (paired).

* **(a) Enumeration / meet in the middle over a triple table.** A table of all
  sums of three signed base points (`i <= j <= k`, `|F|^3 / 6` group additions,
  one insert each), keyed by the point. One attempt probes `R - P` for each of
  the `|F|` signed base points, stopping at the first hit; a miss costs exactly
  `|F|` group additions and `|F|` lookups. This is the generic reference: no
  fold, no batching of inversions.
* **(b) F4 on S5 restricted by the base membership polynomial.** `S5` is
  `Res_X(S4(x1,x2,x3,X), S3(x4,x5,X))` evaluated numerically and interpolated
  exactly on a `9^5` grid (degree 8 in each variable), specialised at `x5 = xR`,
  reduced modulo `L(x_i) = prod_{x in V} (X - x)` (degree `b_x`), and handed to
  an F4 (grevlex, normal selection, Gebauer-Moller criteria, no signatures) with
  `{S5(., xR), L(x1), L(x2), L(x3), L(x4)}`. The ideal is radical and
  zero-dimensional, so the quotient dimension equals the number of ordered
  tuples `(x1..x4) in V^4` that are roots, and `1 in I` (dimension 0) is the
  refutation. The native unit is F4's elimination multiply-add.
* **(c) The PKM tower oracle.** Reused from the committed results above.
* **(d) Coppersmith / Jochemsz-May lattice.** The determinant condition of the
  level-`l` lattice for a polynomial of per-variable degree `d` in `m`
  variables: monomials `alpha in [0, d l]^m`, rows `x^(alpha - d k) f^k p^(l - k)`
  with `k = min(l, floor(alpha_i / d))`; the condition
  `sum_alpha |alpha| log X < sum_alpha k_alpha log p` gives
  `delta_max(l) = sum k / sum |alpha|`. It is computed exactly for `S3` (`d = 2`),
  `S4` (`d = 4`) and `S5` (`d = 8`) and compared with the measured `S3` reach.
  No LLL is run at `m = 4`: the smallest lattice has dimension 6,561.
* **Reference: matched Pollard rho**, measured in the same unit (below).

## Cells, controls, targets

`log2 p` in {16, 20, 24, 28}. Instances (found by `find-curve --seed 20261018`,
frozen in the example, recounted by `selftest`, each of prime order `n`,
`p = 3 mod 4`, `p` of exactly `log2 p` bits):

| log2 p | p | a | b | n | G |
|--:|--:|--:|--:|--:|--|
| 16 | 48479 | 25392 | 20877 | 48383 | (3, 19437) |
| 20 | 661603 | 422772 | 346249 | 660917 | (9, 268107) |
| 24 | 10539323 | 7836717 | 8828358 | 10542893 | (3, 2623236) |
| 28 | 248685287 | 49054657 | 177851102 | 248685841 | (1, 65256793) |

**Base size ("B at the section 2.4 optimum").** With `T_dec = |F|` for the
reference arm (a), the fixed point of `B = (p T_dec)^(1/5)` is `|F| = p^(1/4)`.
Because the number of abscissae on the curve below an x-bound fluctuates by
tens of percent at these sizes (a bound `floor(p^(1/4))` gave 6, 9 and 23 valid
abscissae at the first three cells in a probe), the base is defined by its size: the
first `b_x = ceil(p^(1/4) / 2)` abscissae in increasing order that lie on the
curve, so `|F| = 2 b_x` signed points and the x-bound is whatever that takes.
Design cells: `b_x = 8, 15, 29, 63` (`|F| = 16, 30, 58, 126`; x-bound
`B = 17, 35, 66, 122`). The same base is used for every arm of a cell.

**Targets.** 16 random targets per cell, `R = [k] G` with `k` uniform, an
abscissa not in `V` (a base point is a 1-decomposition), the same 16 for every
arm of a cell (seed 20261018). Controls:

* **Positive control** (every arm that runs): `R` planted as a signed sum of
  four base points (distinct abscissae; and, in `selftest`, with a repeated
  point), which must be found; the decomposition is verified by group addition.
  Arm (b): the quotient dimension equals the brute-force root count over
  `V^4` (24 for one planted tuple, 12 with a repeated point) and every element of
  the computed Groebner basis vanishes at the planted point.
* **Negative control / null**: targets that no oracle decomposes. Arm (a) is
  cross-checked on every random and planted target against an independent
  implementation (a pair table: `R - (P_i + P_j)` against all pair sums) which
  must agree on decomposable or not (AGENTS.md section 6: a new oracle is
  cross-checked on every input). Arm (b) is cross-checked against the
  brute-force grid root count wherever `b_x <= 12`.
* **Instrument checks** (`selftest`, card 1): the cell curves recounted by a full
  character sum; `n G = O`; `S3` vanishes on sums; the interpolated `S5` equals
  the resultant at off-variety points and vanishes on planted 4-sums; full IC
  and rho recover planted logarithms; the `S3` reach reproduces 1/18, 1/10,
  0.119 (the committed note's values).
* **Rho** is its own reference (below); its positive control is that every run
  recovers the planted logarithm.

## Unit, accounting, pricing

* **GAE.** One group addition (affine, doubling and `P + (-P)` included) is one
  group-addition equivalent. The instrument also tallies the field
  multiplications, squarings and inversions inside them.
* **Native operations**, counted by name and priced at `w` GAE each with
  `w in {0, 0.1, 1}` (AGENTS.md section 5a): table lookups and inserts, Legendre
  symbols and square roots (base construction), modular multiply-adds of the
  relation matrix (`rowop`), F4 elimination multiply-adds (`madd`), the
  field multiplications that build and specialise `S5` and that evaluate the
  basis at grid points (`build`), and for rho the distinguished-point tests and
  its table probes and inserts. Every arm lists every counter; none is dropped.
  Arm (b)'s `madd` is elimination only (symbolic preprocessing, matrix
  construction and hash maps are not counted), so it is a lower bound on F4's
  native work; arm (c)'s committed multiply-add is the same kind of count.
* **T_dec(arm, cell; w)** = mean over the cell's 16 random targets of
  `gadd + w * native` of one oracle call (online only). Precomputation (the
  table, `S5`) is reported separately as setup.
* **S** = total GAE (every phase: base, table or `S5`, jump table and start,
  relation phase, matrix, back-substitution, verification) `+ w *` native, and
  `S / sqrt(n)` in the framework's convention. Cold, one thread.
* **Matched rho**: Pollard rho with a 16-jump r-adding walk (jump scalars with
  four random bits set, built from the doubling chains of `G` and `Q`, counted),
  van Oorschot-Wiener distinguished points (`theta = floor(log2 p / 4)` bits),
  one thread, 16 independent logarithms per cell, the logarithm verified by a
  counted scalar multiplication. Its steps, DP tests and table operations are
  counted in the same unit, the jump-table setup included and reported apart.
  Gap to rho = `S(w) / rho(w)` at each `w`.
* **Extrapolation vs measurement.** Rows built from measured components rather
  than measured end to end ("oracle-phase cost" rows) are labelled MODEL and
  the formula is printed beside them; they are lower bounds.
* Wall time is a practicality note and never a metric.

## Decision rule (frozen; applied mechanically to the summary by the assess stage)

Design rule: *record the first measured m = 4 cells; ADVANCES if any oracle's
T_dec grows slower than p^(1/4).* Operationalised, tightened only:

1. **Arm (a) (the reference).** `e_a(w)` is the OLS slope of `log2 T_dec` on
   `log2 p` over the four design cells, with a 95% t interval (2 degrees of
   freedom). By construction (`T_dec = |F| = p^(1/4)`) the expected slope is 1/4;
   the reference defines the threshold and cannot ADVANCE against itself.
2. **Arm (b).** Its p-exponent cannot be fitted over the four cells (the engine
   does not finish above `b_x = 8`). `gamma_b(w)` is the OLS slope of
   `log2 T_dec` on `log2 |F|` over the 16-bit ladder cells that finished
   (`b_x = 3..8`), 95% t interval (4 degrees of freedom). With `|F| = p^(1/4)`
   the p-exponent is `gamma_b / 4`, so "slower than p^(1/4)" is
   `gamma_b < 1`. The p-ladder at `b_x = 5, 6` on the other three curves checks
   that `T_dec` depends on `|F|` and not on `p`.
3. **Arm (c).** `gamma_c` from the committed `|V| = 4, 8, 16` rows, the same
   mapping `e_c = gamma_c / 4`.
4. **Arm (d).** `delta_max` against the `delta = 1/4` that `B = p^(1/4)` needs.
   If `p^(delta_max) < 2` at every design cell the lattice reaches no base of
   two abscissae and the arm is BLOCKED (reach, a mathematical limit of the
   determinant condition, not a tool limit); the committed `S3` measurements
   give the empirical slack.
5. **Verdict per arm, at the level "primitive" (the oracle exponent):**
   ADVANCES iff the upper 95% bound of the exponent is `< 0.25` at both
   `w = 0.1` and `w = 1` (and at `w = 0` where defined; an arm with no group
   operation has `T_dec(w=0) = 0`, the exponent is undefined there and the arm is
   reported "unresolved at w = 0" in the band). MATCHES iff the interval contains
   0.25. REGRESSES iff the lower bound is `> 0.25` at both weights.
   INCONCLUSIVE if fewer than three finished points, or the band disagrees.
   BLOCKED with the exact cap that stopped it.
6. **Item verdict**: ADVANCES iff some arm ADVANCES at the primitive level;
   otherwise the strongest statement the arms give. An end-to-end (constant or
   exponent) claim needs a measured `S(w)` below the best measured `S(w)` of
   another arm at every `w` of the band, on the same instances; the first
   measured cells are the baseline. Ranking between two arms is reported only
   where it holds at all three weights; otherwise "unresolved at the boundary".
7. **Reporting.** For every row: `S(w)`, `rho(w)`, `S/rho` at the three weights,
   and the distance remaining. Not beating rho is never a failure.

**Predictions (written before the cards run; pre-freeze probes already saw the
sign of them).** (a): a miss costs exactly `|F|` GAE and `|F|` lookups; the table
holds about `|F|^3 / 6` entries; per-attempt hit probability about 5%; `S/rho`
grows by about `p^(1/4)`. (b): F4 refutes (dimension 0) the typical target and
returns 24 for a planted tuple; elimination multiply-adds grow like `b_x^12`; the
engine finishes at `b_x <= 8`. (c): as committed. (d): `p^(delta_max) < 2` at every
cell.

## Budget and card plan

One thread, `prlimit --as=4294967296` on every run, host shared (4 vCPU, at most
two lanes). Each card builds its own binary
(`rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native`)
and finishes in under 8 minutes (estimates from the probes below). At most 2
CPU-hours in total; the plan is about 50 CPU-minutes.

| card | content | est. |
|--|--|--:|
| 01 | build; `selftest --deep`; `reach`; `reuse`; rho x4 cells x16; arm (a) T_dec at the four cells (16 random + 4 planted, cross-check) and at the 16-bit ladder `b_x = 3..7`; full IC arm (a) x16 runs at the four cells and the ladder | 2 min |
| 02 | arm (b) ladder at 16 bits, `b_x = 3, 4, 5, 6, 7` (16 random + 2 planted, brute-force check) | 5 min |
| 03 to 06 | arm (b) at the 16-bit design cell `b_x = 8`, target indices 0-4, 5-9, 10-14, 15-17 (16 random + 2 planted) | 5 min each |
| 07 | arm (b) p-ladder `b_x = 5, 6` at 20, 24 and 28 bits | 3 min |
| 08 | arm (b) at the design cells 20, 24, 28 bits (`b_x = 15, 29, 63`), 16 targets, caps 3e9 multiply-adds, 2.5e7 nonzeros, 60 s | 6 min |
| 09 to 11 | full IC arm (b) at 16 bits: `b_x = 3` x16 and `b_x = 4` runs 0-4; `b_x = 4` runs 5-10; `b_x = 4` runs 11-15 | 5, 4, 3 min |

Arm (b) at the design cells 20, 24, 28 is run to a deterministic cap, not
to completion: a probe found `b_x = 15` stopped by the nonzero cap at step 21
(degree 39) with 4.5e8 multiply-adds, after 5 to 7 s, identical on two targets.
The caps are counts, with the wall-clock limit only a safety net.

## Deviations from the design, and filled-in gaps

1. **Language and library.** The instrument is a standalone `std`-only Rust
   file (`examples/blindspots_14_prime_m4.rs`, built by `rustc` in each card),
   because the `crypto_lib` library does not compile at this branch base (25
   errors in `ecbench`/`gaudry_quartic`, recorded by items 01, 02, 05 and 07).
   The framework's `run_pipeline`, `Calibration` and the committed F4 are
   therefore not used; the arithmetic, the pair-table cross-check, the
   incremental Gauss matrix, F4 and rho are written here and cross-checked
   against each other and against brute force. The GAE unit is the framework's;
   no calibration pins apply, which is why the 0, 0.1, 1 band is the only pricing.
2. **B per arm.** The design says "B at the section 2.4 optimum
   `(p T_dec)^(1/5)`" for each oracle. That is circular for arms whose `T_dec`
   explodes in `B`. The base size is the arm-(a) fixed point `|F| = p^(1/4)` for
   all arms at the design cells (and defined by size, not x-bound; see Cells),
   and arm (b) is additionally measured on a ladder of smaller bases down to
   `b_x = 3`, where its own full IC can complete. Each arm's own optimum can be
   read from the ladder; it is not run as a separate cell.
3. **Arm (c) is reused, at its own sizes.** The committed rows are at
   `p1 = 2013265921` and `|V| = 4, 8, 16`, not at the four cells; the PKM base
   needs `2^t | p - 1` and its size is `2^t`, so it cannot be set to
   `p^(1/4) = 15, 28, 57, 125` abscissae (`t = 5..7` is beyond the committed
   reach: `N = 16` took 55 minutes and 8 GB, and the width grew 8x, the
   multiply-adds 316x, from `N = 12`). Its per-attempt multiply-adds are assumed
   independent of `p` (the system is generic in `p`; the committed rows at
   other primes agree on degree). The model row for arm (c) uses
   `p1` and an upper bound on the hit probability, hence a lower bound on cost.
   The round-2/3 engine's counts are used for the exponent; the round-6 `N = 12`
   row (exit on, 5.14e10 instead of 8.55e10) is shown but not fitted.
4. **Arm (d) is a computation plus reused measurements, not a lattice run.**
   The determinant condition is a sufficient condition (it can understate the
   empirical reach: at level 1 the committed `S3` probe met the Howgrave-Graham
   criterion up to `delta = 0.10` against the determinant's 1/18). It is
   therefore reported as the predicted reach with the committed empirical slack
   (up to 1.8x at level 1, 1.0x at levels 2-3), not as a proof of a bound.
5. **Arm (b) is capped at the design cells 20, 24, 28.** It runs to a count cap
   (3e9 multiply-adds or 2.5e7 nonzeros), so its `T_dec` there is a lower bound
   (status `budget-*`) and no p-exponent can be fitted over the four cells; the
   ladder replaces it (Decision rule 2). 16 targets are run at each capped cell
   (they cost 5 to 7 s each); the stop is identical across targets in the probe.
6. **Full IC to completion.** Required for `log2 p <= 20`. Arm (a) is run to
   completion at all four cells (16 runs each; this is more than required).
   Arm (b) completes at the ladder bases `b_x = 3, 4` (16 runs each); at the
   design cells it would need about `(b_x + 1) / 0.056` attempts at 2.7e10
   multiply-adds each at `b_x = 8` (about 160 attempts of 52 s, 2.4 CPU-hours),
   beyond the 45-minute job cap and the 2-hour item budget, so it is BLOCKED there and appears as MODEL
   rows only. Arms (c) and (d) cannot complete at any cell.
7. **The membership polynomial** is `prod (X - x)` over the abscissae that lie
   on the curve (degree `b_x`), not over all integers `1..B`; this is the smaller
   (tighter) ideal, equivalent for the roots.
8. **F4 counts are elimination multiply-adds only**, the unit of the committed
   PKM rows too. The cost of building `S5` (about 14.5 million field
   multiplications by estimate, 200 per resultant evaluation plus the
   interpolation) and of specialising and reducing it per target is counted under
   `build` and is below 0.3% of the smallest finished F4 run.
9. **F4 is a plain implementation** (no signatures, no F5 criteria, dense
   accumulator elimination). The committed F4s are faster per system at larger
   sizes; the comparison with them is in operation counts only where the same
   system was run, which it was not. The result is the cost of this engine.
10. **All four variables, no symmetry.** `S5` is a symmetric polynomial and each
    root appears 24 times (12 with a repeated point). No symmetrised or
    equivariant reduction is used (that is a separate lever).
11. **Target walk.** Full-IC targets are `R = R_prev + T_j` for a uniform `j`
    of a 16-jump table (one counted addition per target, jump table and two
    start scalar multiplications counted in setup), not an independent scalar
    multiplication each, so consecutive targets are correlated.
12. **Cells for arm (b) beyond the design.** The 16-bit ladder (`b_x = 3..7`) and
    the p-ladder (`b_x = 5, 6` at 20, 24, 28 bits) are added to separate the
    dependence on `|F|` from the dependence on `p`; they are extra cells and
    change no required cell.
13. **Conductor.** `conductor` is not installed in this container (`which
    conductor` is empty and `.conductor/` does not exist), so no task was
    obtained; the hooks warn and let edits through (CLAUDE.md). Edits are
    confined to `research/blindspots_20261018/14-prime-m4-cells/` and
    `examples/blindspots_14_prime_m4.rs`.
14. **Statistics.** Four design cells give two degrees of freedom for the arm-(a)
    exponent, so its interval is wide by construction; the rule uses the interval,
    not the point estimate. Bootstrap over targets is not used: arm (a)'s miss
    cost is a constant and arm (b)'s multiply-adds agree to five digits across
    targets in the probes.

## Pre-freeze probe runs (disclosure; none kept as data)

Made while building the instrument, on the shared host, before this protocol was
committed:

* `find-curve` for the four sizes (28 bits took 293 s: a search for prime order).
* `selftest` repeatedly. It exposed two instrument bugs fixed before freezing:
  the first rho jump table (an arithmetic progression `T_j = T_0 + j U`) made
  different orderings of the same jumps collide trivially, so rho took 100x its
  expected steps; replaced by the sparse-scalar table. The first full-IC target
  walk chose jumps by the point's abscissa, which cycles; replaced by a uniform
  choice. Also a test prime that was 1 mod 4.
* A first cell definition by x-bound `floor(p^(1/4))` (6, 9, 23 valid
  abscissae at the first three cells) replaced by the size rule above.
* Arm (b), 16 bits, one target each: `b_x = 7`: 6.32e9 multiply-adds, 12.4 s;
  `b_x = 8`: 2.75e10, 52.4 s, 28 steps, maximum degree 29. Selftest values:
  `b_x = 4`: 1.3e7, `b_x = 5`: 1.55e8, `b_x = 6`: 1.16e9.
* Capped design cells, 1 to 2 targets each: the stop and counts quoted above.
* Full IC, arm (a), 16 runs per cell once: ok in 16 of 16 at all four cells
  (after the target-walk fix); mean GAE 3.9e3, 1.7e4, 7.9e4, 5.7e5; rho mean GAE
  462, 1.4e3, 4.3e3, 1.8e4 (16 runs). Full IC, arm (b), 3 runs each: `b_x = 3`
  ok 3 of 3 (2.3k to 3.8k attempts, 1.4e9 to 2.4e9 multiply-adds), `b_x = 4`
  ok 3 of 3 (0.75k to 1.5k attempts, 1.0e10 to 2.0e10).

The cards re-run all of these from the committed code; the probe numbers are
not used in any table.

## Addendum A: final card split (written before any measured card run; changes no cell, rule or threshold)

The card plan above was an estimate. After the instrument was final
(`examples/blindspots_14_prime_m4.rs`, sha256
`f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376`, `rustfmt` and
`clippy-driver` clean, `selftest` passing) the cards were split by measured time
so that each finishes well under 8 minutes on the shared host. Every card builds
its own binary, runs only that binary, and writes only under `raw/`.

| card | content |
|--|--|
| 01 | `selftest --deep`; `reach`; `reuse` (the three PKM files, bound by sha256); rho x16 at the four sizes; arm (a) T_dec (16 random + 4 planted, pair-table cross-check) at the four design cells and the 16-bit ladder `b_x = 3..7`; full IC arm (a), 16 runs, at the same nine bases |
| 02 | arm (b), 16-bit ladder `b_x = 3, 4, 5, 6` (16 random + 2 planted, brute-force root count) and `b_x = 7` targets 0-8 |
| 03 | arm (b) `b_x = 7` targets 9-17; p-ladder `b_x = 5, 6` on the 20-, 24- and 28-bit curves |
| 04 to 07 | arm (b) at the 16-bit design cell `b_x = 8`, target indices 0-4, 5-9, 10-14, 15-17 (16 and 17 are the planted controls) |
| 08 | arm (b) at the 20- and 24-bit design cells (`b_x = 15, 29`), cap 3e9 multiply-adds or 2.5e7 nonzeros (60 s safety), 16 targets |
| 09 | arm (b) at the 28-bit design cell (`b_x = 63`), same cap; full IC arm (b), `b_x = 3`, 16 runs |
| 10 to 12 | full IC arm (b), `b_x = 4`, runs 0-5, 6-10, 11-15 |

The target indices of a split cell are the indices of the same deterministic
sequence (seed 20261018), so the union of the cards is the 16 random plus 2 planted
targets of Cells, controls and targets. The assess stage runs
`<bin> summarize --dir research/blindspots_20261018/14-prime-m4-cells/raw`, which
prints the tables and the exponent fits that Decision rules 1 to 3 name (numbers
only, no verdict), and applies the rule text of this protocol to them. Arm (a)'s
fitted slope in that output is a check on the base sizes, not an arm verdict
(Decision rule 1). The summary's MODEL rows multiply the arm-(a) full-IC attempt
counts by arm (b)'s and arm (c)'s measured per-attempt native work; they are lower
bounds on those arms' oracle phase, not end-to-end measurements, and carry no
ranking by themselves.
