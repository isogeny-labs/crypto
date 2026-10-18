# Item 03 protocol: polynomial-method solving on descended Semaev systems

Frozen before any measured run. Source design: section "03" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18. Cards and their
timing plan are appended in a second commit that also precedes every measured
run (the section "Card plan", filled from development timings on draws that
share no target with the frozen draws).

Class: **solver-stage cost study** (counted bit operations of one parity-count
or enumeration call). It computes no `S`, no GAE and no rho ratio. The unit is
the same for every arm of this item (bit operations, defined below), so the
section 5a pricing band (0, 0.1, 1 GAE per unpriced native operation) scales
all arms by the same factor and cannot change an order between them; it matters
only for the one cross-method statement (the exponent against matched rho),
which is made on exponents and is price-independent (Rule, last paragraph).

## Question

What exponent `c` in `2^{cN}` do the polynomial-method solvers achieve on the
Boolean Weil-descended Semaev systems, against FES and F4 on the same inputs,
and is any of them a constant-factor or exponent improvement at the sizes the
descent uses?

## What already exists, and what is reused (read before designing)

* **No arm of this item is measured anywhere in the repository.** A search for
  `dinur`, `lokshtanov`, `parity-counting`, `polynomial method` in code,
  results and the knowledge directory finds only the literature survey
  `research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` (section
  3.2: the published exponents `2^{0.6943N}` (Dinur), `2^{0.8765N}` (Lokshtanov
  et al.) are quoted and judged "above the `2^l` enumerator", an analytic
  statement, no run) and the EXPERIMENTS file. Nothing is skipped as already
  measured.
* **FES (`src/cryptanalysis/mq_fes.rs`) has no committed timing on these
  systems** (its tests are correctness tests). Its documented cost per Gray
  step is two XORs of the packed equation vector. The library does not compile
  at this branch base (25 errors in `ecbench` and `gaudry_quartic`, recorded by
  the item 02, 05 and 07 protocols; `cargo build --release --lib` fails here
  too), so it cannot be linked. The instrument re-implements the same
  algorithm (Bitner-Ehrlich-Reingold Gray order, derivative table updated by
  one XOR from the pair table, two XORs per step) and checks it against naive
  evaluation (Controls).
* **F4 (`koblitz_groebner.rs`) cannot be linked either.** The F4 arm is the
  Boolean-ring F4 of item 05 (`examples/blindspots_05_elimination_orders.rs`,
  arm `grevlex`), committed and sha-bound; item 05 validated it against the
  committed closure degrees (84 of 84) and committed msolve readings (32 of 32)
  on the same `S_3` chain systems. It runs here on exactly the draws of this
  item (same file format, same builder). Its unit is `touches` (matrix entries
  written plus entries toggled by row operations), one entry = one bit
  operation.
* **msolve (arm e) is not buildable here**: not installed; no GMP, MPFR or
  FLINT headers; about 4 GB free disk. Committed msolve readings on `S_3`
  chains exist (`research/ic_next10_20261014/06-equivariant-f4-m4/`) but at
  other cells. Arm (e) is recorded BLOCKED. What would unblock it: a prebuilt
  msolve 0.6.5 binary on the host (then `msolve -g 2 -t 1` on the same ANF).
* **Descended systems.** The generator is the one the repository's solvers use
  (`koblitz_groebner::build_decomposition_system`, the symbolic `S_3` of the
  chained decomposition); the instrument rebuilds it exactly as item 05 does
  (same sparse irreducible, same bit layout) and checks the exact solution
  counts of the draws against brute force on toy cells.

## Notation and cells

Cell `(n, l, m)`: field degree `n`, factor-base subspace dimension `l`, `m`
summands, `S_3`-chain system with `m*l` leaf bits and `(m-2)*n` chain bits,
`n` equations per link, `m-1` links. `N` is the number of Boolean unknowns of
the system handed to the solver, `E` its number of equations, `d` its degree.

* **`S_3` family (m = 2, quadratic, d = 2): the six cells of the design.**
  `N = 2l` and `E = n = N + 1`, the descent regime `m*l ~ n` (the expected
  number of solutions of a random target is about 1/2; the cell `(31, 15, 2)`
  of item 05 is of this form). Cells `s3-12 .. s3-32`:
  `(13,6,2)`, `(17,8,2)`, `(21,10,2)`, `(25,12,2)`, `(29,14,2)`, `(33,16,2)`,
  `N` = 12, 16, 20, 24, 28, 32.
* **`S_4` family (m = 3, cubic, d = 3).** The repository's chained
  `S_4` system has `N = 3l + n` unknowns and `E = 2n` equations, so the design's
  `N = m*l` cannot be met literally; the cells are the nearest to the design's
  grid at `n = 3l + 1`: `(7,2,3)`, `(10,3,3)`, `(13,4,3)`, `(16,5,3)`, with
  `N` = 13, 19, 25, 31 (Deviation 3).
* **Draws.** Per cell `draws.tsv` (item 05 format, committed, sha-bound into
  every card) holds one random `l`-dimensional subspace base and eight
  unsatisfiable targets (`idx` 0 to 7, exact solution count 0) followed by four
  satisfiable targets (`idx` 8 to 11). Unsatisfiable targets carry the timing
  rows (a refutation scans everything: FES's full cost, the F4 refutation
  degree, Dinur's data-independent cost), the same stand items 05 and 07 take;
  satisfiable targets carry the parity controls.
* **Positive-control systems.** For each cell and each of eight seeds, a random
  dense system with the same `N`, `E`, `d` (every monomial of degree <= d with
  probability 1/2). The design's positive control; also the null against which
  "descended systems are easier" is judged.

## Unit: bit operations

One **bit operation** is one two-input Boolean gate (XOR or AND) on single
bits, or one single-bit counter update, or one single-bit test. No arm gets a
word-parallel discount: a XOR of two `k`-bit vectors costs `k`. The count is a
property of the algorithm's control flow, not of the host. Wall time and peak
memory are descriptive.

* **FES.** Per Gray step `2E` (two XORs of the `E`-bit equation vector; the
  derivative update is skipped on the `N` steps that first flip a bit) plus the
  zero test, charged as the number of equation bits inspected up to and
  including the first set bit (`E` if the vector is zero): the loop is exact,
  counted per step. Setup (`N` derivative vectors) is `N*E`. A full scan of a
  system is `2^N - 1` steps.
* **Dinur and Lokshtanov arms.** One call of the parity-count core, counting
  exactly the operations enumerated in "The instrument" below. The full
  decision procedure multiplies this by at least `N + 1` (Valiant-Vazirani
  guesses of the hash size) and by the variable-fixing search; those
  multipliers are **not applied**: every cost reported for an arm of the
  polynomial method is a lower bound on the cost of solving, which favours the
  candidate (Deviation 5).
* **F4.** `touches` of `b05` (matrix entries written plus entries toggled).

## The instrument

`examples/blindspots_03_polynomial_method.rs`, native Rust, standard library
only, compiled by each card with `rustc --edition 2021 -C opt-level=3
-C target-cpu=native` (Deviation 1). Modes: `gen`, `selftest`, `fes`, `poly`,
`model`, `asym`, `crossover`, `summarize`.

**System form.** The `E` equations are stored as one packed table: for every
monomial of degree `<= d` over `N` variables a 64-bit word whose bit `j` is the
coefficient in equation `j` (so `E <= 64`; the cells have `E <= 33`). The
variables are numbered `0 .. N-1`; a node of the recursion splits them into
`y` (the low `N - n1` indices) and `z` (the high `n1`).

**The core call `MPC(system, n1, w, plan)` = Dinur's `MultParityCount`
(Algorithm 2 of Dinur 2021, arXiv 2005.04800).** It returns the vector of
parities `V[y] = sum over z of F(y, z)`, `F = prod (1 + P_j)`, for every `y` of
Hamming weight `<= w`, in a canonical order. `plan = (n2, n3, ...)` is the
chain of splits of the interior nodes below this node (`n2 <= n1`, each `>= 1`);
an empty plan makes the node a **leaf** (brute force, Algorithm 3).

* *Leaf.* Evaluates all `E` polynomials at every point of
  `P = {y : |y| <= w} x {0,1}^{n1}` by one in-place Moebius (zeta) transform
  over the down-closed set `U = P u {monomials of degree <= d}`, then
  `F(p) = [all E bits are 0]`, then the parity over `z` for each `y`. Counted:
  `E * sum_{x in U} |x|` (one XOR per set bit of every element of `U`, each
  on an `E`-bit word), `(E - 1) * |P|` (the AND of the polynomial values), `|P|`
  (the parity accumulation).
* *Interior node.* `l = n2 + 2`, `t = 48 N + 1` repetitions (the paper's
  Lemma 3.1 constant). Each repetition draws a uniform `l x E` bit matrix `a`,
  forms `R_i = sum_j a_ij P_j` (charged densely: `M * l * E`, `M` = number of
  monomials of degree `<= d`), calls the child `MPC(R, n2, d*l - n2, plan[1:])`
  to get `G` on `W = {x : |x| <= d*l - n2}` (over the first `N - n2`
  variables), interpolates `G` by an in-place Moebius transform on `W`
  (`sum_{x in W} |x|`), evaluates it on
  `P2 = {y : |y| <= w} x {0,1}^{n1 - n2}` by one transform over `P2 u W`
  (`sum_{x in P2 u W} |x|`) and adds the result bits into a scoreboard (`|P2|`).
  After `t` repetitions each point votes by majority and the votes are summed
  over the `u` part (`|P2|`).
* *Top level (`ParityCount`, Algorithm 1).* `V = MPC(system, n1, N - n1, plan)`,
  parity `= xor of V` (`2^{N - n1}`).
* **`dinur`** = the best (lowest counted cost) schedule `(n1, n2, ..., n_L)`
  with `L` interior levels, `L` in {1, 2, 3}, `N - 1 >= n1 >= n2 >= ... >= n_L
  >= 1`, for the cell's `(N, E, d)`, with `t = 48N + 1`. The degenerate schedule
  `L = 0` is plain Moebius exhaustive search (`exh`, descriptive) and is **not**
  eligible as the polynomial method. Sensitivity row `dinur-t11`: the same with
  `t = 11N + 1`, which is the smallest odd `t` that a Chernoff bound with the
  union over all `2^N` points needs (descriptive; ADVANCES uses `t = 48N + 1`).
* **`lptwy`** = the single-level scheme of Lokshtanov, Paturi, Tamaki, Williams
  and Yu (SODA 2017) as described by Dinur 2021 section 2.4 and footnote 2:
  `L = 1`, `n2 = n1`, `l = n1 + 2`, and the interpolation degree is `d*l`
  (the cancellation of `n1` in the degree of `G` is **not** exploited, which
  is the difference from Bjorklund-Kaski-Williams and Dinur); `t = 48N + 1`.
  Searched over `n1` only.

**Real versus counted.** The paper's constants make a real run of `dinur` at
`N >= 20` infeasible (the cost is what the study measures). The instrument
therefore has two executions of the same control flow: a **real run** that
operates on the data, draws the random matrices, votes, and increments the
operation counter at each charged site, and a **counted model** that evaluates
the same charges in closed form (log2 domain, sums over Hamming-weight classes
`C(k,a) C(z,b)`). The model is validated against the real run by exact
equality (relative difference <= 1e-9) on more than 20 (system, schedule,
`t`) combinations, including both arms and `d` = 2 and 3 (Controls). Real runs
are done at `N <= 16` (quadratic) and `N = 13` (cubic); beyond that the model
is the measurement and is labelled `model` in every row (Deviation 2).

**Asymptotic exponent routine (`asym`).** The exponent of the schedule above
as `N -> infinity`, from the entropy form of every term (no polynomial
factors): with `f_d(k) = (1-k) H((d-1)k/(1-k))` (`H` binary entropy) the
exponent of a schedule `1 > k_1 >= ... >= k_L` is the maximum of `1 - k_1`,
`f_d(k_1)`, `f_d(k_i) + k_i - k_{i+1}`, `f_d(k_{i+1})` and the leaf term
`f_d(k_L) + k_L`; for `lptwy` the entropy argument is `d k/(1-k)`. The routine
finds the least `E` for which a feasible schedule with at most `L_max`
levels exists (greedy descent from `k_1 = 1 - E`), for `L_max` = 1, 2, 3, 5,
10, 20, 60.

## Arms

| arm | what | cells | how |
|---|---|---|---|
| `dinur` (a) | multiple parity-counting, best schedule `L` in 1..3 | all | model every cell; real at `s3-12`, `s3-16`, `s4-13` |
| `lptwy` (b) | single-level probabilistic polynomial, degree `d*l` | all | same |
| `fes` (c) | Gray-code FES, 2 XORs per step | `S_3` family | real, full scan of every draw and every random system |
| `f4` (d) | item 05 F4, grevlex | all cells within caps | `b05 run` on the same `draws.tsv` |
| msolve (e) | not buildable here | none | BLOCKED |
| `exh` | Moebius exhaustive search (the degenerate `dinur`) | all | model, descriptive |
| `fes-floor` | `2^N * (2E + 2)` | `S_4` family | the incumbent FES refuses cubic systems (its documented limit); the floor is what FES would cost if it extended to degree 3 at the quadratic per-step price, which no cubic enumerator achieves; used only as the generous comparator on the cubic cells |

## Controls

* **Instrument soundness (selftest, exact; any failure exits 2).**
  (i) The Gray-order FES finds exactly the solutions of naive evaluation on 40
  random quadratic systems, `N` = 6 to 14, `E` = 3 to 20, including systems with
  no solution; (ii) the draw builder reproduces the exact solution counts of
  brute force on toy `S_3` and `S_4` chains; (iii) the real `MPC` agrees with
  brute-force parity on random systems for every schedule class (`L` = 1, 2, 3,
  `lptwy`) and for `d` = 2 and 3, with the paper's `t = 48N + 1` on the small
  cases and `t` = 9 on the bigger ones (disclosed per line); (iv) the counted
  model equals the real operation counter (relative <= 1e-9) on all of those
  runs; (v) the zeta-transform kernel equals the definition on random down-sets.
* **Positive control 1 (the algorithm sees parity).** On the satisfiable draws
  (`idx` 8 to 11) of `s3-12`, `s3-16`, `s4-13` and on eight random dense systems
  of each of those cells' `(N, E, d)`, for each target and each hash size `k` =
  0 .. N (the Valiant-Vazirani step: `k` random affine equations added), the
  real `dinur` and `lptwy` calls (paper `t`) must return the parity of the
  number of solutions of the hashed system, which the instrument obtains by a
  full scan. Disagreement on any of at least 200 hashed systems whose true
  parity is odd, or any disagreement at all, fails the control (exit 2).
  The share of hashed systems with odd parity is printed (a control that never
  saw odd parity would prove nothing).
* **Positive control 2 (published exponents are reproduced).** The `asym`
  routine must give, for `d` = 2: Dinur's limit `E*(L_max = 60)` within
  `[0.6943 - 0.0005, 0.6943 + 0.002]` (Theorem 3.1: `tau(2) = log2(phi) =
  0.69424`); for `d` = 3: within `[0.8114 - 0.0005, 0.8114 + 0.003]`
  (Dinur's quoted 0.8114); for `lptwy`, `d` = 2: `0.8765 +- 0.004`
  (Lokshtanov et al.); and `E*(L_max)` must be non-increasing in `L_max`.
  Failure exits 2. (The Bjorklund-Kaski-Williams value 0.804 is **not** a
  target: their analysis uses a refinement this instrument does not
  implement; the instrument's `L = 1` value is printed.)
* **Negative control (the right null for "structure helps").** The eight
  random dense systems per cell (same `N`, `E`, `d`) against the eight
  descended draws: the counted cost of `dinur`, `lptwy` and of an FES full scan
  must be identical (the cost is a function of `(N, E, d)` only for those
  arms); a difference would mean the instrument depends on the data and is
  reported, not hidden. The descended systems get one data-dependent
  diagnostic, `affine_combos`: the dimension of the space of equation
  combinations with no monomial of degree >= 2 (the only way structure could
  reduce `N` before the polynomial method starts); the random systems give the
  null for it.
* **Sensitivity (can the instrument see an effect at the size the rule asks?).**
  The instrument reports `FES` and `dinur` for an `N = 16` system whose
  equation count is varied (E = 17, 33, 64): the FES count must scale with `E`
  by the ratio `(2E+2)/(2*17+2)` within 2% and the `dinur` count must move by
  less than 1% (the cost is nearly independent of `E`). This proves the
  counter responds to the quantity it should respond to and not to another.

## Metric, fit and decision rule (frozen)

Per (cell, arm): counted bit operations, `rho_A(N) = bitops(FES) / bitops(A)`
(larger is better for `A`), wall time, status. Costs of `fes`, `dinur`,
`lptwy` are exact and equal across the draws of a cell (negative control), so
their per-cell intervals are degenerate; the F4 touches vary by target and are
summarised by the geometric mean with the range of the eight targets.

* **Fitted exponent.** `c_A` = least-squares slope of `log2 bitops` against `N`
  over the six `S_3` cells (for `f4` over the cells it finished), with the
  jackknife range (leave-one-cell-out; this is a stability interval, not a
  sampling interval, the costs being deterministic). The local exponent between
  `N = 28` and `N = 32` is printed beside it.
* **Crossover.** `N*` = the least `N` at which the counted model of `dinur`
  (resp. `lptwy`) falls below the FES cost `2^N (2(N+1)+2)`; searched on a
  coarse schedule grid up to `N = 400`; "> 400" if none.
* **ADVANCES (constant)**: `rho_dinur(32) >= 1.5` or `rho_lptwy(32) >= 1.5` on
  `s3-32` (an arm beats FES at the largest `N`, with a 1.5 margin).
* **MATCHES**: neither arm, and some arm has `2/3 < rho(32) < 1.5`.
* **TRADES**: `rho(32) <= 2/3` for both arms, and an arm has `c_A` below
  `c_FES` by more than the larger of the two jackknife ranges (a better
  exponent in the measured range, a worse constant: the crossover `N*` is the
  distance remaining).
* **REGRESSES**: `rho(32) <= 2/3` and no arm has the lower fitted exponent.
* **INCONCLUSIVE**: a control failed (the run exits 2 first).
* **BLOCKED**: arm (e); also any cell an arm cannot run at the host's limits,
  named with the cap that stopped it.
* **The exponent statement is made whatever the verdict**, at level
  *exponent*: for each family the published asymptotic exponent `c_pub`
  (0.6943 for `d` = 2, 0.8114 for `d` = 3, from the instrument's `asym`) is set
  against the requirement for an index-calculus win. With `N = m l` and about
  `2^l` relations of cost `2^{cN}` each, against rho at `2^{N/2}`, the
  decomposition beats rho only if `c < 1/2 - 1/m`: 0 for `m` = 2, 1/6 for
  `m` = 3, 1/4 for `m` = 4. The distance remaining is `c_pub - (1/2 - 1/m)`
  (for the fitted `c_A` too), and the index-calculus exponent relative to
  rho is `2/m + 2c` (a value above 1 loses). These are printed by `summarize`
  and are not a pass/fail gate for this item's verdict.
* The F4 row is reported as `bitops(F4) / bitops(FES)` per cell with the target
  range; cells where F4 hit a cap are named with the cap.

## Budget and execution

One thread per job; every run wrapped in `prlimit --as=...` (4 GiB; the real
`dinur` runs allocate at most 2^24 entries). At most 2 CPU-hours of measured
runs; every card under 9 minutes; no job over 45 minutes. Build products
(`rustc` outputs of `b03` and `b05`) live under the shared `target/` in a
per-card directory that the card removes. Raw outputs and the receipt go to
`research/blindspots_20261018/03-polynomial-method/raw/`. After the cards run,
`b03 summarize --dir raw` applies the rule to the recorded lines and prints the
table, the exponent statement and the verdict line; the README, SVG/PNG graph
and PDF (research-visuals skill) are written by the assessing stage.

## Deviations from the frozen design

1. **No linking against the crate.** The library does not compile at this
   branch base. The instrument is std-only and re-implements FES and the system
   builder (checked in selftest); the F4 arm is item 05's committed Boolean F4,
   not `koblitz_groebner.rs`.
2. **Counted model beyond N = 16.** Bit operations of `dinur` and `lptwy` at
   the larger cells come from the closed-form count of the same control flow,
   validated by exact equality against the real counter on small runs, because
   the paper's constants make a real run the quantity being measured and
   infeasible. Rows carry `mode = model` or `real`.
3. **`S_4` grid.** `N = 3l + n` for the repository's chain, so the cubic
   cells are `N` = 13, 19, 25, 31 and not the design's grid.
4. **msolve (e)** BLOCKED (not installed, no libraries, disk).
5. **Core call only** for the polynomial-method arms (no Valiant-Vazirani
   multiplier): a lower bound that favours the candidate.
6. **FES on the cubic family** is the quadratic-price floor, not a run: the
   incumbent refuses cubic systems.
7. **Unsatisfiable targets carry the timing rows**; satisfiable targets only
   the parity controls.
8. **Tightenings** beyond the design: the 1.5 margin on ADVANCES; the 8
   random-dense null per cell; the sensitivity control; the published-exponent
   band; the `t = 11N+1` sensitivity row as descriptive only.

## Card plan (committed before any measured run)

Filled from development timings on this host (shared 4-vCPU, one lane at a
time on the instrument); the development runs used the committed draws and
the instrument at `9cb0ba30...` (sha256 of the source, bound in every card) and
are not measurements: the cards re-run everything and write `raw/`.
Binaries: `target/blindspots-03/bin/b03` and `.../b05`, built by card 01,
removed by card 09.

`draws.tsv` check (before the cards): 10 cells x 12 rows (6 `S_3`, 4 `S_4`),
`idx` 0-7 have `solutions` 0 and `idx` 8-11 have `solutions` > 0 in every cell;
the FES full scan reproduces `solutions` 0 on all 48 unsatisfiable `S_3`
targets (the `fes` mode exits 2 on a mismatch) and the parity of the recorded
`solutions` of the satisfiable targets equals the instrument's `k = 0` truth in
`s3-12`, `s3-16`, `s4-13`.

| card | content | development time | timeouts (sum) |
|---|---|---|---|
| 01 | build `b03`, `b05`; selftest, `asym`, `sens`; counted model rows (`poly`, no `--real`) for all 10 cells | 4 s build, < 2 s per cell | 375 s |
| 02 | FES full scans (8 descended + 8 random dense each) on `s3-12 .. s3-28`; `crossover` to N = 400 | < 11 s; 99 s | 340 s |
| 03 | FES full scans on `s3-32` | 160 s | 420 s |
| 04 | F4 (item 05 `grevlex`) on `s3-12 .. s3-24` (8 targets), `s3-28` (targets 0-1, 80 s cap per job) | < 100 s; cap | 480 s |
| 05 | F4 on `s3-32` (target 0, 90 s cap) and on the four `S_4` cells (8 targets) | cap; < 45 s | 410 s |
| 06 | positive control 1 (real `dinur`, `lptwy`, paper `t`, hash sizes k = 0..N) on `s3-16` | 230 s | 450 s |
| 07 | positive control 1 on `s4-13`; on `s3-12` seeds 1 to 9 | 40 s; 12 s per seed | 370 s |
| 08 | positive control 1 on `s3-12`, seeds 10 to 18 | 12 s per seed | 270 s |
| 09 | positive control 1 on `s3-12`, seeds 19 to 26; `b03 summarize --dir raw`; remove binaries | 12 s per seed | 285 s |

Estimated CPU total about 20 minutes against the 2 CPU-hour budget; every card
sums to under 9 minutes of timeouts. The F4 jobs carry the same caps and
status words as item 05 (`timelimit`, `memcap`, `colscap`); a capped cell is a
named row, not a failure of the card.

Details that tighten or clarify the frozen text (none loosens it):

9. **Positive control 1 sample.** The odd-parity requirement is read as 200
   hashed systems, each checked by both polynomial-method arms: `summarize`
   requires 400 odd real calls (it printed 200 in the first version). The
   instrument's `--real-rand 8` gives the protocol's eight random dense
   systems per cell on `s3-16` and `s4-13`; on `s3-12` the control is repeated
   over 26 seeds (`--seed`, which changes the random systems and the
   Valiant-Vazirani hash equations; the four satisfiable targets are the same)
   because the descended satisfiable targets have mostly an even number of
   solutions (the odd share is about 7%), and 200 odd hashed systems cannot be
   reached otherwise within the budget. `--no-model` suppresses the duplicate
   counted-model rows in those runs (the model rows come from card 01 only).
10. **Cell labels.** `s4-13` is the cell `c7-2-3` (`N = 3l + n = 13`); the
    cell `c13-4-3` has `N = 25`.
11. **F4 caps.** `s3-28` and `s3-32` are expected to hit the item 05 time cap
    (item 05 recorded the same for `c31-15-2`); they are reported with the cap
    named, and BLOCKED applies to F4 at those cells.
12. **No use of the F4 interior.** The F4 arm is the committed instrument
    unchanged (source sha256 `f21a761b...`).

## Repair pass (2026-10-18, experiment-director)

Written after the first run, before any revision card ran. Question, arms,
cells, metric and decision rule are unchanged.

**What the first run left.** Card 01 completed (receipt
`raw/receipt-card-01.json`, status completed, 16 commands, all exit 0): build,
`selftest` SELFTEST OK, `asym` ASYM OK, `sens`, and the counted-model rows of
all ten cells. Cards 02 to 09 were **refused at lint** (no command started, no
receipt, no output): each named `target/blindspots-03/bin/b03` or `b05`, which
existed once card 01 had built it, and `task_card.py` refuses an argv token that
names a repository file the card's binds do not pin. Card 01 itself now fails
`check-relay` for the same reason. Nothing from cards 02 to 09 was measured, so
the revision reruns all of them.

**Card rules the revision follows.** Every revision card builds its own binary
into its own directory `target/blindspots-03-r2/cNN/` (absent when the card is
linted), runs only that binary, and ends by removing the directory, so the card
lints before and after the run. No card reads a file produced by another card.
`b03 summarize --dir raw` reads the outputs of all cards, so it is not a card
here; the assessing stage runs it after the cards (it needs a binary built from
the instrument at the sha below). Each card's timeouts sum to at most 470 s.
Outputs and receipts carry the suffix `-r2`
(`raw/card-NN-r2-<job>.{stdout,stderr}.log`, `raw/receipt-card-NN-r2.json`), so
nothing from the first run is overwritten, and `summarize` (which reads every
`*.stdout.log`) takes the first-run card 01 rows and the `-r2` rows together.

**Deviation 13 (instrument fix proved by the first run): the sensitivity
bound.** The frozen text of the Sensitivity control requires the `dinur` count
to move by less than 1% across E = 17, 33, 64. The instrument gated at 5%
(`rel_d.abs() < 0.05`), so `sens` printed SENS OK on changes of 1.36% (E = 33)
and 3.99% (E = 64), which the frozen text rejects. Fix: the gate is now
`< 0.01` as frozen, and `summarize` no longer reads the line "SENS OK" but
evaluates the recorded `sensitivity` rows (at least 3 rows, every
`dinur_rel_change` below 0.01, every `pass` true), so the first-run rows in
`raw/card-01-sens.stdout.log` are judged by the frozen bound too. Consequence,
stated here and not hidden: on the first-run numbers the sensitivity control
**fails**, and by the frozen rule a failed control makes the verdict
INCONCLUSIVE whatever the other rows say. `summarize` therefore prints the
verdict INCONCLUSIVE and, as a descriptive line only, the rule branch that
would apply with that one control waived. The revision cards do not re-run
`sens` (deterministic; nothing would change); a replacement of the 1% bound is
a decision for the owner, not for this item.

**Deviation 14: `summarize` descriptive line.** The added line in Deviation 13
changes no verdict and no number.

**Deviation 15: why card 01 is not re-run.** The edits touch `mode_sens` and
`mode_summarize` only. `smoke-r2.sh` (committed beside this file) rebuilds the
instrument and checks that `selftest`, `asym` and the ten `poly` model outputs
are byte-identical to the card-01 logs, and that `sens` now exits 2 on the
first-run numbers. Instrument source sha256 before the fix `9cb0ba30...`, after
the fix `c4a2a1ac94687e6a8170d651e5ecca81c444d4656f293207437f8394a224647c`
(bound in every revision card). The item 05 instrument is unchanged
(`f21a761b...`).

**Revision card plan** (timeouts are sums of the per-command timeouts; the
measured development times are the first run's plan, re-measured on the draws
at the smallest draw count for panics, see the build result):

| card | content (arguments unchanged from the first plan) | timeouts |
|---|---|---|
| 02-r2 | build `b03`; FES full scans (8 descended + 8 random dense) on `c13-6-2 .. c29-14-2`; `crossover` | 420 s |
| 03-r2 | build `b03`; FES on `c33-16-2` | 470 s |
| 04-r2 | build `b05`; F4 on `c13-6-2`, `c17-8-2`, `c21-10-2`, `c25-12-2` (caps as planned) | 360 s |
| 05-r2 | build `b05`; F4 on `c29-14-2` (targets 0-1), `c7-2-3`, `c10-3-3`, `c13-4-3` | 380 s |
| 06-r2 | build `b05`; F4 on `c33-16-2` (target 0), `c16-5-3` | 360 s |
| 07-r2 | build `b03`; positive control 1 on `c17-8-2` (real `dinur`, `lptwy`, paper `t`) | 470 s |
| 08-r2 | build `b03`; positive control 1 on `c7-2-3`; `c13-6-2` seeds 1 to 8 | 410 s |
| 09-r2 | build `b03`; positive control 1 on `c13-6-2` seeds 9 to 17 | 340 s |
| 10-r2 | build `b03`; positive control 1 on `c13-6-2` seeds 18 to 26 | 340 s |

Cards run one at a time in this order (the F4 time caps are wall-clock caps, so
two lanes at once could change which targets reach a cap; bit-operation counts
do not depend on load).
