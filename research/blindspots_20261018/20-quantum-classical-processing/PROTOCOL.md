# Item 20 protocol: classical pre- and post-processing for quantum ECDLP

Frozen before any measured run. Source design: section "20" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18.

Class: **component comparison of resource-model arms** (constant level), with a
**native measurement** of the classical post-processing on toy group sizes. No
arm here is a cross-method row against rho, and **no claim is made about
hardware timelines**. Rho appears only as the matched reference for the
classical post-processing work (distance remaining, never a gate).

## Question

How much does the classical side change the quantum cost of ECDLP on P-256 and
secp256k1: Ekerå's post-processing (one run with a limited search, or several
runs with a lattice), tolerance to failed runs, and windowed scalar arithmetic?
The reported unit is **logical qubits x Toffoli gates**, against the committed
baseline estimate.

## What already exists (read before designing)

* `src/cryptanalysis/quantum_estimator.rs` is a **binary-field** estimator
  (Putranto et al. 2024: Karatsuba, Itoh-Tsujii, `2n PD + 2 PA`). It has **no
  prime-curve estimate**, so it cannot price P-256 or secp256k1. It is
  extended (below), not re-measured.
* The committed baseline estimate for 256-bit prime curves is the table in
  `research/notes/ecdlp-general/RESEARCH_ECDLP_STATE_OF_THE_ART.md`, section 5:
  Roetteler-Naehrig-Svore-Lauter (RNSL) 2017, **2,330 logical qubits and
  1.26 x 10^11 Toffoli**; and the provisional 2026 rows (~1,200-1,450 qubits,
  ~70-90 x 10^6 Toffoli; "treat exact figures as provisional").
* RNSL's closed forms (arXiv 1706.06752, abstract): at most `9n + 2 ceil(lg n)
  + 10` qubits and `448 n^3 lg n + 4090 n^3` Toffoli gates for an `n`-bit prime
  curve. At `n = 256` the qubit formula gives exactly 2,330 (matches the
  table) and the Toffoli formula gives 1.2875 x 10^11, **2.2% above** the
  table's 1.26 x 10^11 (the table value is RNSL's T-count 1.60 x 2^39 divided
  by 7, as read from Haner et al. Table 1). The formula is the baseline
  because it is a function of `n`; the 2.2% is reported, not hidden.
* Ekerå, "Revisiting Shor's quantum algorithm for computing general discrete
  logarithms" (arXiv 1905.09084 v5, 2026-03-17), read in full for the points
  below. Two exponent registers of `m + sigma` and `ell` qubits (`m` the bit
  length of the group order `r`, `ell ~ m/s` for tradeoff factor `s`, `sigma`
  a padding length); `m + sigma + ell` group operations per run, so `2m` for
  Shor (`s = 1`, `sigma = 0`) and `~(1 + 1/s) m` per run over `n >= s` runs
  with the lattice post-processing. A single run of the semi-classical-QFT
  version succeeds with probability "approximately 60% to 82%" (heuristic,
  depends on `r`); with a limited classical search it "heuristically exceeds
  99%". The paper's own example: `sigma = 7`, `B_eta = 0`, `B_Delta = 100`,
  `ell = m + nu`, `nu = -57` gives `2m - 50` quantum group operations and
  about `2^33` classical group operations (`4 sqrt(Bt)`,
  `Bt = ceil((B_Delta + 1/2) 2^-nu)`); `nu = -107` for a 114-bit short DLP
  (Pons 2020) gives `2m - 100 = 412` operations, "a relative reduction of
  ~20%". Ekerå-Håstad short logarithms: `~3m/2` operations per run for an
  `m`-bit logarithm, `(1/2 + 1/s) m` with tradeoffs (same paper, 1.2.3); a
  promise problem, not the generic ECDLP.
* Haner-Jaques-Naehrig-Roetteler-Soeken, "Improved quantum circuits for
  elliptic curve discrete logarithms" (arXiv 2001.09580), read for the
  windowing facts used here: a window of `w` bits needs a table look-up of
  `2^w` points (`2^(w-1)` with a sign qubit), costs `4 * 2^(w-1)` T gates per
  look-up, six look-ups per window if no extra registers are kept and two if
  two registers are kept, replaces a controlled addition of a classical point
  by an addition of two quantum points, "saves `w - 1` point additions", and
  the optimal window is `~ lg(A/24)` for a point addition of `A` T gates. Their
  256-bit results (Table 1): RNSL 2,338 qubits, T = 1.60 x 2^39; "Low W"
  window 19, 2,124 qubits, T = 1.72 x 2^32; "Low T" window 19, 2,619 qubits,
  T = 1.08 x 2^31; "Low D" window 15, 2,871 qubits. Their windowed signed
  point addition (window 8) costs `432 n^2 + 1.07 x 2^14` T for "Low T".
* Crate state: `cargo build --release --lib` fails at the branch base (25
  errors; items 01, 02, 03, 05, 07, 08, 10, 16 and 17 record the same), so no
  instrument can use the crate. `quantum_estimator.rs` itself has no crate
  dependencies and compiles on its own (checked by `rustc --test`).

Nothing in the repository measures the classical post-processing of a Shor
run, the success probability of one run, the tolerance of the lattice step to
failed runs, or a windowed prime-curve estimate. None of the cells below is
already measured by a committed result, so none is cited in place of a run.
The three numbers taken from the literature above are **calibration anchors
and positive controls**, not results of this item.

## Arms

All arms share one resource model (`prime_shor` in `quantum_estimator.rs`),
one group-order shape and one counted unit. The model is a function of `n`
only: **P-256 and secp256k1 have the same `n = 256` and the same top-bit shape
(group orders `2^256 - 2^224 + ...` and `2^256 - 4.3 x 10^38`), so the model
gives them one row, labelled with both names**; it has no `a = 0` versus
`a = -3` term, as RNSL's formula has none (reported as a limit). Rows for
`n = 384` and `n = 521` are scaling context, outside the question.

Per-addition price: `p(n) = (448 n^3 lg n + 4090 n^3) / (2n)` Toffoli, one
controlled addition of a classical point (RNSL's total divided over its `2n`
additions). **Qubits stay at `9n + 2 ceil(lg n) + 10` for every non-windowed
arm** (semi-classical QFT with control-qubit recycling is what both RNSL's
count and Ekerå's algorithm assume), so only Toffoli changes there.

| arm | what it is | per-run controlled additions | runs |
|---|---|---|---|
| **A0** | committed baseline: Shor, `sigma = 0`, `ell = m`, success taken as 1 | `2m` | 1 |
| **A0p** | A0 with its single-run success **measured** (`sigma = 0`, `ell = m`, search box `B_eta = 0, Bt = 0`, i.e. Shor's own post-processing) | `2m` | `1/p0` |
| **E1-0** | Ekerå single run, `sigma = 7`, `B_eta = 0`, `B_Delta = 100`, `ell = m` | `2m + 7` | `1/p1` |
| **E1-T** (tier `T`) | same, `ell = m + nu(T)`, `nu(T)` the most negative integer whose post-processing (formula below, priced at `c = 1`) fits the tier `T in {2^20, 2^30, 2^40, 2^50, 2^58}` group operations | `2m + 7 + nu(T)` | `1/p1` |
| **E2-s** | Ekerå lattice, `s in {2, 3}`, `sigma = 7`, `ell = ceil(m/s)`, runs added one at a time up to `s + 3`; lattice over every subset of size `>= s` | `m + 7 + ell` | `E[runs]` from the measured cumulative success |
| **W(X)** | windowed version of `X in {A0, E1-0, E1-2^40}`, window `w <= 24` chosen to minimise the Toffoli count at each band point | `ceil((m+7)/w) + ceil(ell/w)` windows | as `X` |
| **EH-m_d** | Ekerå-Håstad short logarithm, `m_d in {128, 160, 192}`: **promise problem** (the scalar is known to be below `2^m_d`); row for context, never ranked against A0 | `3 m_d / 2` | 1, success cited `>= 0.99`, not measured |

Window model, per window: `kappa * f * p(n) + mu * 2^w` Toffoli, and extra
qubits `2w` (address and unary-iteration ancillas) plus `2n` when `mu = 1`
(the two-register variant keeps the looked-up point). Classical
**pre-processing**: `windows * 2^w` classical point additions to build the
tables, reported next to the quantum saving.

**Band (the model's honest uncertainty; a verdict must hold at every point):**
`kappa in {0.75, 1, 1.5}` (an uncontrolled quantum-quantum addition against
RNSL's controlled classical-point addition: cheaper through losing the
control, dearer through two quantum operands); `mu in {1, 3, 6}`
(`2 * 2^(w-1)`, `6 * 2^(w-1)` and `6 * 2^w` look-up Toffoli per window:
signed with two kept registers, signed with six look-ups, unsigned with six);
`f in {1, 1/10, 1/62}` (arithmetic cheaper than RNSL's by a constant;
`1/62` is Haner et al.'s windowed `432 n^2` T point addition divided by 7,
against RNSL's `p(256)`). Ratios are always taken against A0 **at the same
`f`**, so an arithmetic improvement is never credited to windowing.

## Instrument (all native Rust, std only)

`examples/blindspots_20_quantum_classical.rs`, with the resource model in
`src/cryptanalysis/quantum_estimator.rs` (`prime_shor`, additive; the example
includes that file with `#[path]`). Built with a standalone `rustc` invocation
(the crate does not build); the card binds both files by sha256.

* **Exact quantum-measurement sampler.** For a cyclic group of prime order `r`
  (`m`-bit), log `d`, registers `m + sigma` and `ell` qubits, the joint
  distribution of the measured pair `(j, k)` is sampled **exactly** (not by
  Ekerå's histogram heuristic): with `M = 2^(m+sigma)` the marginal of `u = rj
  mod M` is `[rho F_hi(u) + (r - rho) F_lo(u)] / M^2`, `rho = M mod r`,
  `F_T(u) = sin^2(pi T u / M) / sin^2(pi u / M)`, the third-register value
  `e` given `j` has weight `n_hi(e) F_hi + n_lo(e) F_lo`, and `k` given
  `(e, j)` is the squared magnitude of a length-`2^ell` FFT. The brute-force
  2-D transform of every class on a tiny instance must agree with the
  factorised probabilities to 1e-9 (selftest gate).
* **Post-processing.** Single run: `z = round(r j / M)`, `kappa = round(r k /
  2^ell)`, search `eta in [-B_eta, B_eta]`, `t in [-Bt, Bt]` with `Bt =
  floor((B_Delta + 1/2) r / 2^ell)`, `d = (t - kappa) (z + eta)^-1 mod r`;
  the actual search is baby-step giant-step in a **counted** cyclic group
  (additions counted, hash probes counted separately as native work), the
  candidate is accepted only if `[d] g = x`. Success over many runs is also
  computed by a closed criterion (is the true `t` inside the box for some
  `eta`, with `z + eta` invertible); the two must agree on every verified
  sample. Several runs: the congruences `d (z_i + eta_i) + kappa_i = t_i mod
  r` form an integer hidden-number lattice (dimension `n + 1`); it is reduced
  by LLL (`delta = 0.99`, integer basis, floating Gram-Schmidt), Babai plus a
  `{-1,0,1}^(n+1)` neighbourhood is searched, candidates are checked against
  every run and then against `[d] g = x`. The lattice is tried on every
  subset of size `>= s` of the runs taken, so a bad run is tolerated at the
  cost of more reductions.
* **Toy group.** The cyclic group `Z_r^+` with a random non-zero generator
  `g`, `x = [d] g`, and every group operation going through a counter. The
  measurement distribution depends only on `(r, d)`, and the counted
  operations are group additions, so nothing about the curve is lost; no
  shortcut through the additive structure is taken (the search sees `g`, `x`
  and equality tests only). `r` is the **largest prime below `2^m`**
  (the shape of both curves' group orders); a sensitivity arm uses the
  smallest prime above `2^(m-1) + 1` (the other extreme of the 60-82% range, see Amendment 1).
  `d` is redrawn every 20 runs.
* **Model rows** (`--mode model`) print every arm above at every band point,
  with the mechanical verdict of the rule below.

## Unit

* **Quantum:** logical qubits x Toffoli gates, and each factor. The headline
  Toffoli figure is the **expected total** (per-run Toffoli x expected number
  of runs); the per-run Toffoli figure (the size of the single circuit) is
  printed beside it. Depth and wall clock are not modelled.
* **Classical post-processing:** counted group additions `G` (BSGS giant and
  baby steps, the two scalar multiplications and the verification, each by
  left-to-right double-and-add) and unpriced native work `N` (hash probes,
  lattice floating-point operations, candidate checks), priced at `c in {0,
  0.1, 1}` GAE per native operation (AGENTS.md 5a): `S(c) = G + c N`. The
  model formula for the single-run search is `G ~ (2 B_eta + 2) sqrt(2 Bt +
  1) + (2 B_eta + 3)(1.5 m)`, `N ~ G`; the toy runs measure `G` and `N`
  against it, and the ratio measured/formula is the extrapolation error that
  is reported with every `m = 256` figure.
* **Distance to matched rho:** `log2` of the classical post-processing work
  against `0.5 log2(pi r / 4)` (rho on the full group), reported as a
  difference in bits, never as pass or fail.

## Controls

1. **Instrument gate (selftest, before anything counts).** (a) RNSL qubit
   formula gives 2,330 at `n = 256`; Toffoli formula within 3% of 1.26e11.
   (b) Sampler agrees with brute force to 1e-9 on `(r, sigma, ell) = (29, 1,
   4)` and `(31, 2, 3)`. (c) LLL preserves the lattice determinant and returns
   a reduced basis on random bases; Babai recovers a planted close vector.
   (d) BSGS recovers every `t` in a box of 200 planted values with the counted
   cost inside `[0.5, 2]` times the formula. (e) Window model at `f = 1/62,
   kappa = 1, mu = 3`: the best window lies in `[15, 22]` (Haner et al. use 15
   to 19 at 256 bits) and the model's windowed total, in T gates (7 per
   Toffoli), lies within a factor 3 of Haner's Low-T/Low-W range
   `[1.08 x 2^31, 1.72 x 2^32]`. Any failure: the item's verdict is
   INCONCLUSIVE.
2. **Positive control (the instrument can see the effect).** At `m = 16` with
   `sigma = 7`, `B_eta = 0`, `B_Delta = 100`, the measured single-run success
   has a Wilson 95% lower bound `>= 0.97`; with `sigma = 0`, `B_eta = 0`,
   `Bt = 0` it lies in `[0.40, 0.90]` (the paper's 60-82% band, widened for a
   16-bit group); and success is non-decreasing in `sigma` and in `B_Delta`
   across the grid (allowing the sampling interval).
3. **Negative control (the right null).** The same post-processing applied to
   **uniformly random pairs `(j, k)`** with the same `d` and `r`: the success
   rate must equal the chance rate `(2 B_eta + 1)(2 Bt + 1)/r` (capped at 1)
   within 4 standard errors, in every cell. This is the null of a post-
   processing step that has been given no information; any excess in the real
   runs is therefore due to the measurement.
4. **Search/criterion agreement.** On the first 100 runs of every single-run
   cell, the actual BSGS search and the closed criterion give the same answer
   (zero mismatches).
5. **Sensitivity to `r`.** One cell family repeats the grid at the
   other-extreme prime (Amendment 1); the headline uses the curve-shaped prime.

## Cells (frozen)

Timing decides only how cells are split across cards, never the cells.

**S (single run, exact sampler).** Grid inside each cell: `B_eta in {0, 1,
2}`, `B_Delta in {0, 1, 2, 4, 10, 30, 100}`, giving the success rate with a
Wilson interval, the closed-criterion count, the null rate and chance, and the
counted cost of the actual search on 100 verified runs at
`(B_eta, B_Delta) = (0, 100)` and `(1, 30)`.

| cell group | m | sigma | nu (ell = m + nu) | prime | runs per cell |
|---|---|---|---|---|---|
| S12 | 12 | 0, 1, 2, 4, 7 | 0, -2 | curve-shaped | 4000 |
| S16 | 16 | 0, 2, 4, 7 | 0, -2, -4 | curve-shaped | 1500 |
| S20 | 20 | 0, 2, 4 | 0 | curve-shaped | 600 |
| S16w | 16 | 0, 7 | 0 | other-extreme | 1500 |

**L (several runs, lattice).** `s in {2, 3}`, `sigma in {4, 7}`, `ell =
ceil(m/s)`, `n_max = s + 3` runs, every subset of size `>= s` tried in order of
size, for `n = s .. s + 3`: success rate (Wilson), lattice reductions, native
operations, candidates checked, quantum group operations (`n (m + sigma +
ell)`), at `m in {12, 16, 20}`, 400 trials per cell.

**R (resource model).** `n in {256, 384, 521}` (256 labelled P-256 and
secp256k1), all arms and band points, with the success probabilities `p0`,
`p1` and the lattice cumulative successes **measured inside the card** at
`m = 16` (2000 runs per configuration, `nu in {0, -2, -4}` for `p1`, trials
300 for the lattice) and entered as Wilson **lower** bounds (the conservative
side for the quantum cost). `p1` is the minimum over the three `nu`.

## Decision rule (frozen)

For each non-promise arm and each band point let `rho` be the ratio of
`qubits x expected-total-Toffoli` to A0's at the same `f`.

* The classical post-processing is **capped** if its work at the `c = 1` price
  exceeds `2^40` GAE (a few hours of one workstation core at 10^7 point
  additions per second is `~2^35`; `2^40` is the stated outer edge).
* **ADVANCES (constant)** if `rho <= 0.95` at **every** band point and the
  arm is not capped.
* **TRADES (constant, quantum for classical)** if `rho <= 0.95` at every band
  point but the arm is capped; or if the Toffoli ratio is `< 1` and the qubit
  ratio `> 1` (or the reverse) at every band point and the product rule is
  not met.
* **REGRESSES** if `rho >= 1.05` at every band point.
* **MATCHES** if `rho` lies in `(0.95, 1.05)` at every band point.
* Otherwise **INCONCLUSIVE** (the band straddles).
* The same rule against A0p is **reported as a second column and never
  decisive**: an advance that appears only against A0p comes from correcting
  the baseline's assumed success of 1, which AGENTS.md calls an accounting
  change, not an improvement.
* The item's overall verdict is the best verdict among the arms E1-0, E1-2^40
  (and lower tiers), E2-s and W(...); it is INCONCLUSIVE if the selftest
  gate or any control fails, BLOCKED if no cell could run.

Every arm's row also carries: classical group operations and native work, the
`c in {0, 0.1, 1}` totals, and the distance in bits to matched rho.

## Budget

One thread per job, `prlimit --as=4294967296`. Estimated 15 CPU-minutes for
all cards (the exact sampler is `O(r + 2^ell log 2^ell)` per run); hard limit
2 CPU-hours and 8 minutes per card. The largest allocation is the marginal
table of `M = 2^(m+sigma) <= 2^24` doubles (128 MB). Disk: no per-item target
directory; the binary is built under `target/blindspots-20/` and removed at
the end of its card.

## Deviations from the frozen design (none loosens it)

1. **"Extend `quantum_estimator.rs`" is done additively** (`prime_shor`), but
   the example cannot use the crate (it does not build), so it includes that
   one dependency-free file with `#[path]` and is compiled with `rustc`. The
   file is checked by its own `rustc --test`.
2. **The committed baseline for prime curves is reconstructed**, because the
   committed estimator is binary-field only: RNSL's two closed forms, with the
   2.2% difference to the committed table value reported (above).
3. **Lattice in hidden-number form** (integer, from Ekerå's section 6.1
   congruence applied to every run) instead of the real-valued matrix (26);
   the same information, exact integer arithmetic, and the same dimension
   `n + 1`. The search over `eta_i` is not run in the lattice (`eta_i = 0`
   assumed; its failure is part of the measured success).
4. **Ekerå-Håstad short logarithms are modelled and cited, not simulated**:
   the toy group would need the unknown-order setting. They are a promise
   problem and carry no verdict.
5. **Success probabilities in the model come from `m = 16` toy groups**
   (lower confidence bounds), not from `m = 256`; the paper's claim that they
   tend to 1 with `m` is not used. The extrapolation is flagged on every row.
6. **P-256 and secp256k1 share one row** (the model has no curve term).
7. **Windowing uses a band, not a point**, because the quantum-quantum
   addition price against RNSL's controlled addition is not published as a
   separate figure.
8. **Pre-processing scope:** the classical pre-processing priced is the
   window tables (`windows * 2^w` additions); the classical computation of
   `x = [d] g` side data is zero.

## Outputs

`raw/receipt-card-NN.json`, `raw/*.jsonl` (one record per cell and grid point,
and the model rows), `raw/*.log`. The README, graph and PDF are the
assessment stage's.

## Amendments made after the first commit and before any measured run

The instrument was written and smoke-tested (selftest, and every mode and
cell at 3-40 runs) after this protocol was first committed. Nothing below
loosens a gate, a control or the decision rule; each item replaces the text
it names. No measured card existed when they were made.

1. **"Worst-shape" prime.** The smoke run shows the curve-shaped prime (just
   below `2^m`, the shape of both group orders) gives the *low* end of
   Ekerå's 60-82% single-run range (`0.58` at `m = 16`, `sigma = 0`), and the
   prime just above `2^(m-1)` the *high* end (`~0.8`). Cell family S16w and
   "Sensitivity to `r`" (control 5) are therefore the **other extreme of the
   range**, not the worst case; the headline stays on the curve-shaped prime.
2. **Search work formula.** The one in "Unit" is replaced by the code's
   `prime_shor::single_run_search_work` (worst case, every `eta` scanned, no
   early exit): with `W = 2 Bt + 1`, `q = ceil(sqrt(W))`, `G = (q - 1) + 1 +
   (2 B_eta + 1)(3 m + 1 + ceil(W/q)) + 1.5 m`, `N = q + (2 B_eta + 1)
   ceil(W/q)`. The measured mean includes the early exit at the right `eta`,
   so for `B_eta > 0` it is below the formula; both and their ratio are
   printed.
3. **Cell sizes.** S12: 10000 runs per cell. S16 is split in two invocations,
   `nu = 0` (all four `sigma`, 4000 runs; this one carries the positive-
   control gate, which needs `sigma = 0` and `sigma = 7` with `nu = 0` in the
   same invocation) and `nu in {-2, -4}` (2000 runs). S20: 500 runs per cell.
   The lattice cells at `m = 20` use `sigma = 4` only (`sigma = 7` would need
   a `2^27`-entry table; the cap is `2^24`); `m = 12, 16` use `sigma in
   {4, 7}`. 400 trials per lattice cell.
4. **Selftest gates as built:** the sampler is checked against brute force on
   `(r, sigma, ell) = (29, 1, 4), (31, 2, 3), (13, 0, 4)`; the BSGS gate
   compares the counted additions with the pure BSGS part of the formula (no
   per-`eta` scalar multiplications); a sixth gate runs the actual search and
   the closed criterion on 150 runs at `m = 10` and requires zero mismatches.
   A zero-probability `u` or class `e` (where an exact sampler has nothing to
   sample) is skipped, not renormalised.
5. **A0p** takes the **upper** Wilson bound of `p0` (the side that favours the
   baseline and so is the harder comparison for the candidate arms); the
   candidate arms take the lower bounds, as stated.
6. **Positive-control gate** is evaluated only for the curve-shaped prime at
   `m = 16` (it is printed as `"na"` elsewhere). The "monotone in `sigma` and
   `B_Delta`" gate allows two standard errors.
7. **Per-run lattice cost** columns are means over a whole trial up to its
   stopping run, so they repeat on every `runs` row of one cell.
