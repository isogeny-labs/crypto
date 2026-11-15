# Red-team review of the frozen O4 protocol (2026-11-15)

Reviewer: Red Team (core tier, Opus 5.5). Standing of everything below:
`[pcc:unverified]`. No pcc claim exists for any number in this file. Each
number is one of three things: a reading of committed code, red-team
arithmetic, or a recorded value from an earlier run. Each says which.
Nothing here edits the protocol, runs the experiment or decides a status;
the Coordinator and the Validator decide.

## What was reviewed

- Frozen evidence: commit `f619a7a76b` ("chained transfer: survey open cases,
  freeze an O4 protocol"). `PROTOCOL.md` sha256 `fa7edef8…b898b` matches
  `frozen.sha256`. `NOTE.md` (sha256 `e97a070e…52af6`) is **not** covered by
  `frozen.sha256`.
- Code, read at HEAD `6f1bd2d9c2`. The files below are byte-equal to
  `f619a7a76b`, except `ic_boundary.rs`, whose diff does not touch the rho
  walk shape: `src/cryptanalysis/bielliptic_quartic.rs`,
  `research/bielliptic_quartic_20261005/README.md`,
  `src/cryptanalysis/chained_transfer/{ic_control,negation_remeasure,full_jacobian_ic,ec}.rs`,
  `src/cryptanalysis/ic_boundary.rs` (`rho_jumps_for`, `WalkShape`).
- Rules: AGENTS.md "The rule: boundary, table, ratio" (§1, §2, §5, §5a),
  "IC measurement rules", "Model tiers", "Preserve scope and report evidence";
  the O3 record (`research/full_jacobian_ic_20261113/ERRATA.md`,
  `research/validation_20261114/REPORT.md`,
  `research/full_jacobian_ic_rp_20261114/PROTOCOL.md` Amendment 1);
  `research/chained_transfer_proof_20261112/PROOF.md` §2 and §5.
- Red-team arithmetic: three stdlib Python scripts under the session
  scratchpad (factor-base size by `p mod 4` with arbitrary `(a, b)`, **not**
  the protocol's selection; slope and interval model; coupon-collector
  simulation). No protocol instance was selected, no arm was run.

## Bottom line

**Amend before any code is written.** Six findings (R1 to R6) decide what code
gets written or make the frozen rule unreadable. Under the rule as frozen:

- the `closed` branch is either unreachable (if the code uses
  `negation_remeasure::classify`, as the protocol says), or
- it is reached only by ranking across a band in which the order flips, which
  AGENTS.md §5a forbids.

The rest can be fixed in the same amendment. Commit the amendment under a new
hash in its own commit before any code or data, as the O3 rerun did with
Amendment 1.

## Findings, ranked by severity

Labels: **wrong** (contradicted by code, rule or arithmetic), **unsupported**
(no evidence either way), **unpriced** (a cost is missing or assumed),
**unfrozen** (left for the coder to decide).

### Critical

**R1. The decision rule ranks across a band in which the order flips (wrong).**
`PROTOCOL.md` "Decision rule" and "Pricing and calibrated reading".

The fault, in four parts:

- "closed" needs `above` only at `w = 0.1`, `w = 1` and calibrated factor 1.
- It lets the verdict stand when Q is `below` at `w = 0` ("the 0 GAE price
  cannot rank it").
- It says classes come from `negation_remeasure::classify`. That function
  (`negation_remeasure.rs`, `classify(arm: &[f64; 3], reference: &[f64; 3], …)`)
  returns `above` only when the arm is above at all three prices, and
  `unresolved` otherwise.
- AGENTS.md §5a: "Rank only where the conclusion holds at every weight in the
  band. If the order of two arms changes inside the band, report them as
  unresolved at that boundary, never as a win for either side." The
  calibrated reading is ranked only "when it holds at all three" of `0`,
  `w_cal` and `10 w_cal`.

The sibling O3 rule required `above` "at the band and at the calibrated
reading", all three prices (`research/full_jacobian_ic_20261113/PROTOCOL.md`,
line 118).

Arithmetic `[pcc:unverified]`. Q's group operations are the anchor plus the
target shifts.

- At `p ≡ 3 (mod 4)` the factor base is all of `E \ {O} / ±` (R2). The first
  multiple `G` is a column and `T` is a column at shift 0, so Q does about 1
  group operation.
- At `p ≡ 1 (mod 4)` about half the folded classes are columns, so Q does
  about 2 to 4.
- The negation rho's `total_ops` include set-up. The O3 run recorded 64.0 at
  `(p, r) = (211, 109)` and 161.3 at `(12841, 3169)`
  (`research/full_jacobian_ic_20261113/full_jacobian.json`, `rho_E.negation`,
  `[pcc:unverified]`).

So Q is `below` rho at `w = 0` and at calibrated factor 0 at every prime, and
`above` at `w ≥ 0.1`. Two consequences follow:

- Coded with `classify` on the band triple, every cell is `unresolved`, so
  P(closed) is about 0, against G9's 0.75.
- Coded as the prose says, the verdict ranks a pair whose order changes inside
  the band.

The root cause is the unit. Everything Q does is "native" (MulP, MulR, InvR),
so at `w = 0` it is nearly free. This is the O3 unit defect in mirror image.
Cheapest test: none needed; it follows from `classify`'s code and the counts
above.

Amendment (replace the first paragraph of "Pricing and calibrated reading"
and the three bullets of "Decision rule"):

> **Primary unit: one `F_p` or `F_r` multiplication or squaring**, as in
> `research/full_jacobian_ic_rp_20261114/PROTOCOL.md` Amendment 1, items 1
> to 4, with `w_inv = 10`.
>
> - Base of Q and D: `MulP + MulR + w_inv·(InvP + InvR)`, plus D's elliptic
>   additions by `full_jacobian_ic::ec_add_tally`.
> - Base of rho: `ec_add_tally` of every group operation in `total_ops`.
> - Everything else is native on every arm, priced at 0, 0.1 and 1 unit and
>   at 0, 1 and 10 `w_cal`, with `w_cal = time(class) / time(one F_p
>   multiplication)`. Native here means Canon, Hash, Lookup, Insert, Draw,
>   Word, and the `F_p` and `F_r` additions.
>
> Cells are classed by `negation_remeasure::classify` on the full
> three-price vectors, band and calibrated, with no price left out.
>
> - "closed by data" needs `above` in both vectors at every stage-1 cell,
>   plus the slope condition.
> - "advantage seen" needs `below` in both vectors at some cell.
> - Anything else is unresolved.
>
> The GAE reading of the frozen text is kept as a secondary
> `verdict_gae`. It is reported and not predicted.

### High

**R2. A hidden size parameter: the factor base is `p/2` or `p/4` by `p mod 4`
(wrong; G2 is wrong).** `PROTOCOL.md` "Instance selection", G2.

The rational points of `v^4 = f(x)` behave differently by residue
`[pcc:unverified, red-team argument]`:

- `p ≡ 3 (mod 4)`: `v ↦ v^4` has image the squares. Every `x` with `f(x)` a
  nonzero square gives exactly one folded column, so `n = (r − 1)/2`.
- `p ≡ 1 (mod 4)`: only `x` with `f(x)` a fourth power contribute, so
  `n ≈ p/4`.

Red-team check with arbitrary `(a, b)` and prime `#E`, not the protocol's
selection:

- `n/p` = 0.469, 0.526 (211); 0.535, 0.544 (419); 0.487, 0.500 (1619);
  0.502, 0.503 (3203). There `n = (r − 1)/2` exactly.
- `n/p` = 0.267, 0.269 (821); 0.246, 0.247 (6421); 0.244, 0.256 (12841).

The stage-1 residues are 3, 3, 1, 3, 3, 1, 1. So:

- G2 (`n/p` in `[0.15, 0.30]`, P = 0.8) fails at 4 of 7 primes, by arithmetic
  and not by chance.
- With the `≡ 3` primes at the low end, any `n^k` cost has its fitted
  exponent biased by `−0.145 k` (red-team OLS on the 7 stage-1 `ln p`).
- The unmodelled jump alone widens the 95% interval of an `n^2` slope to
  about ±0.47 (df = 5).
- At `p ≡ 3 (mod 4)` arm Q is a different algorithm. The linear algebra
  computes the log of every element of `E/±`, the individual log is a table
  lookup, and arm D accepts every trial.

This is the O3 defect (a cost-driving size not bound to `p`) in a new
variable. Lesson `L-12d575623c2f` asks for a selection-only pass before
freezing, and that pass would have shown it.

Amendment (to "Instance selection"):

> Every prime is `≡ 1 (mod 4)`. Stage 1: `229, 421, 821, 1621, 3209, 6421,
> 12841`. Stage 2: `25601, 51217, 102409`. These are the next primes `≡ 1 mod
> 4` at or above the frozen list, red-team computed.
>
> - A prime hop goes to the next prime `≡ 1 (mod 4)`.
> - The selection asserts `p % 4 == 1` and reports `n/p`.
> - A `--select-only` pass is run and its table is frozen with the amendment.
> - G2 becomes "`n/p` in `[0.22, 0.30]`".

If the `≡ 3` regime is wanted, it is a second series with its own fit, at least
four primes and its own predictions. It is never pooled.

**R3. The linear-algebra algorithm is not frozen, and a switch defeats the
slope rule (unfrozen).** `PROTOCOL.md` arm Q: "counted Wiedemann … or a
counted dense elimination at `p <= 3203` if that is the code that exists".

- The existing `RowReduction` (`bielliptic_quartic.rs`, `insert`) is
  Gauss–Jordan with back-reduction of every pivot, so it costs `O(n^3)`.
- Red-team model, combining R2's `n` with rho ≈ `45 + 0.886 √p + 2^dp`
  (`rho_jumps_for` gives 4 jumps with 6-bit coefficients for `r < 2^15`):
  - dense up to 3203 and Wiedemann above: `s = 0.84`, 95% interval
    `[−0.96, 2.64]`. The `closed` slope condition (lower end > 0.2) fails for
    an accounting reason.
  - Wiedemann throughout: `s = 1.49`, interval `[1.01, 1.96]`.
- The dense matrix is also infeasible at stage 2: about 5 GB at `n ≈ 25,600`
  and 21 GB at `n ≈ 51,200`.
- G4 (MulR slope in `[1.7, 2.2]`) is 1.71 under Wiedemann throughout and about
  2.56 under dense, so its score depends on this choice.

Amendment:

> One LA for every prime and both arms: `ic_control::peel`, then the counted
> `ic_control::wiedemann`, then counted back-substitution of every peeled
> column, so every factor-base column has a log.
>
> - Dense Gauss–Jordan is run only at `p ≤ 421`, uncounted, as a
>   cross-check of the Wiedemann solution.
> - `peel` needs a target column. Q and D have none, so a counted copy with
>   no `tcol` is written, with a unit test.

**R4. Phases of Q are unlisted, and the cold interval is not stated
(unpriced).** `PROTOCOL.md` prices MulP for "relation phase, root finding,
section reconstruction" and says rho's "set-up separated". It is silent on
these:

| # | Q phase in the code (reading of `bielliptic_quartic.rs`) | size | protocol |
|---|---|---|---|
| a | `Cover::new` square-root table, which replaces root finding by a lookup | `p` MulP, `p` entries | not listed; root finding then costs 0 MulP per pair |
| b | `rational_points` (fourth-power table plus `rhs` per `x`) | about `9p` to `10p` MulP | not listed |
| c | `group_order` by enumeration (called in `prepare`) | about `5p` MulP | not listed; rho gets `r` as input |
| d | `contains_quartic` check inside `image` | about 9 MulP per point | not listed |
| e | `recover` fibre witness: re-enumerates every rational point per target | `O(p)` | not listed |
| f | `prepare` replay of every factor-base log | `n` scalar multiplications, about `1.5 n log2 r` group ops | not listed; it decides G7 |
| g | final `[t]G == T` | one scalar multiplication | rho's is outside its count (`rho_full_ec` closure), so it must be symmetric |

The owner rule "Cold interval governs the primary comparison" (AGENTS.md "IC
measurement rules", commit `0b9d7b861e`, not an ancestor of `f619a7a76b`) puts
every target-independent set-up, in full, on the one target. The three
instances of a prime share `E`, so a `Cover`, point list or relation set
reused across the 12 runs of a prime would amortise it.

Amendment (new subsection "Phases and the cold interval"):

> Every Q and D run is cold. It is built from `(p, a, b, r, G, T)` alone and
> shares no state with another run. The order `r` is an input to every arm.
>
> - Tables built in the run are charged: the square-root table at `p` MulP, or
>   root finding by counted Tonelli–Shanks.
> - Point enumeration and the factor base are charged in MulP, Canon and
>   Insert.
> - Verification-only work is outside the primary total on every arm and is
>   listed as a sensitivity with its count: `verify_relation`, factor-log
>   replay, fibre witness, `[t]G == T`.
> - The rho total is `RhoMean::arm()` (`total_ops`, set-up and walk). The
>   set-up is reported beside it, not removed from it.
> - Q and D carry `lower_bound: true` with a `lower_bound_note` naming the
>   uncounted work: line de-duplication set, allocation, sorting.

**R5. The stopping rule needs a rank test that is not priced, and G3 is
likely wrong (unpriced, wrong).** `PROTOCOL.md`: "Collection stops when the
system has full column rank". G3: rows from `1.0 n` to `1.4 n`, P = 0.75.

- Wiedemann gives no rank. Tracking rank with `RowReduction` is the `O(n^3)`
  elimination of R3, and it would be uncounted unless stated.
- Full rank needs every column touched. With weight-4 rows on random columns,
  that takes about `n (ln n + 0.577) / 4` rows. Red-team simulation: 1.35 n
  (`n = 105`), 1.5 to 1.6 n (`n ≈ 220`), 1.81 n (800), 2.02 n (1600),
  2.06 n (3200).
- So G3 likely fails at 6 of 7 primes `[pcc:unverified, model]`.

Cheapest test, disclosed as a pre-freeze smoke: collect in seed-permuted order
at `p = 229` and `251` with the existing gated module, and count the rows
needed for full rank.

Amendment:

> Collect until every column has weight ≥ 1 and rows ≥ `n + 20`. Then run the
> counted solve. If it returns `None`, collect `0.1 n` more rows and retry.
> Every attempt is counted and the rows used are recorded.

G3 is re-predicted as "rows / n in `[(ln n)/4, (ln n)/4 + 0.6]`".

**R6. The copied arithmetic overflows at stage 2 (wrong).** In
`bielliptic_quartic.rs`, `Cover::section`, `6 * m * m * t * t % self.p` is
computed in `u64` with no reduction in between. It overflows for
`p ≥ 41,874` (`(2^64/6)^{1/4}`).

- `Cargo.toml` sets no `overflow-checks`, so release builds wrap silently.
- At 51203 and 102407, `divide_root` then fails, and `?` aborts the
  collection.
- The `p = 53` positive control cannot see this.
- G10 (P = 0.8 and 0.6 at those primes) is inconsistent with "copies, not
  relaxes".
- `square_roots: Vec<Vec<u64>>` of `p` entries also contradicts the
  "`p < 2^31`" domain.

Amendment:

> Every product is reduced mod `p` before the next multiplication, or computed
> in `u128`. A unit test compares section polynomials with a `u128` reference
> on 10^4 random lines at the largest stage-2 prime. The stated domain is
> `p ≤ 2^20`.

### Medium

**R7. The controls cannot catch scale defects (unsupported).**

- Positive control: only `p = 53`. That prime is `≡ 1 (mod 4)`, has
  `n = 11 ≤ 16` (Wiedemann's dense-projection branch), and uses the
  exhaustive pair order: 79 sections from all 990 pairs.
- It exercises none of the scale paths: overflow, the `≡ 3` regime, sparse
  projection, peeling and back-substitution, early stop.
- "Using the new uncapped module" needs an exhaustive mode the protocol does
  not name.
- Negative control: it does not say which row is tampered. A redundant row is
  caught only if the residual is checked over every row.
- `verify_relation` is taken out of the cost, and the protocol does not say it
  still runs. A "0 bad relations" line would then repeat O3 erratum 4.

Amendment:

> (i) The `p = 53` replay runs an `--exhaustive` mode named here.
>
> (ii) At the first instance of every prime, the logs of 32 columns, chosen
> by a fixed seed, are checked against an independent, uncounted BSGS on `E`.
>
> (iii) Tamper control: one independent row chosen by a fixed seed. The
> solver's residual `A x = c` covers every collected row
> (`ic_control::wiedemann` already does this; state it). A tampered
> certificate must be rejected by `verify_relation` at the largest prime.
>
> (iv) `verify_relation` runs on every row, uncounted, and its failure count
> is reported as a measured value.

**R8. Seeds and aggregation are not fixed (unfrozen; O3 erratum 6 class).**

- `seed0` is never given a value.
- `x = (seed * 2246822519 + 7) mod r` does not say whether the product wraps
  in `u64` or is taken in `u128`.
- The 16 negation-rho and 16 plain-rho seeds per instance are not listed.
- `s` names four things: the scan index, the shift, the slope and the IC seed
  index.
- The pair-permutation RNG, D's draws and Wiedemann's `rng` seed are not
  given.
- Stage-2 instance and seed indices are not given.
- `R(p)` is not defined as a ratio of means, a mean of ratios or a
  per-instance value.
- The fit could run over 7 per-prime points (df 5, `t = 2.571`) or over 21
  instance points (df 19). That choice changes the interval.

Amendment:

> **Seeds**
>
> - `seed0 = 0x5EED_0415`
> - `seed_k = seed0 ^ (k · 0x1000)`
> - `x_k = (seed_k as u128 · 2246822519 + 7) mod r`
> - IC seed `(arm, k, j) = base_arm + j + 10 k`, with `base_Q = 700`,
>   `base_D = 500`, `j ∈ 0..4`; stage 2: `k = 0`, `j ∈ {0, 1}`
> - rho seed `(k, j) = 1000 + j + 100 k`, `j ∈ 0..16` (stage 2: `0..8`)
> - one `StdRng::seed_from_u64(IC seed)` per run, consumed in this order:
>   permutation, draws, Wiedemann
>
> **Aggregation**
>
> - `R(p)` = mean Q total over its `3 × 4` runs divided by mean rho total over
>   its `3 × 16` runs
> - OLS over the 7 per-prime points, `t95(5)`
> - the binary prints every seed, and a unit test checks them against these
>   formulas

**R9. The verdict label overreaches the scope (unsupported).**

- `NOTE.md` defines O4 as "non-hyperelliptic covers with `g ≤ d² − d + 1`".
- The relations measured are line sections of weight 4 > `g` with principal
  class (`K − 4R_∞`). That is the "relations of weight `> g`" item of
  PROOF.md §5, not the T6 item. `NOTE.md`'s own O5 remark files weight above
  `g` under O3.
- For `d = 2`, T6's conclusion seems to follow without Castelnuovo–Severi
  `[pcc:unverified, red-team argument]`:
  - Suppose `ℓ(c^*Q) ≥ 2`. Then there is a map `C → P^1` of degree ≤ 2.
    The curve `v^4 = x^3 + a x + b` is a smooth plane quartic, so it is not
    hyperelliptic, and no such map exists.
  - The weight-3 class `c^*Q + R_∞` is rigid too. Its two finite points lie on
    `X = x0 Z`, which misses `R_∞ = (1:0:0)`.
- Arm Q uses the full factor base, `n = Θ(p)`, which is weaker than the
  best-known plane-quartic IC. The README itself lists "a smaller factor base
  with large-prime recombination" as unimplemented.

So a `closed` result would speak to this instantiation, not to O4. Cheapest
test: a written proof check by the Validator or `crypto-lean`; no run.

Amendment:

> The verdict values are `Q_above_rho` (family `v^4 = x^3 + ax + b`, full
> factor base, sizes run), `unresolved` and `Q_below_rho`. No outcome changes
> the status of any PROOF.md §5 item. A closure claim goes to
> `red-team-breakthrough` and `validator-breakthrough` (AGENTS.md "Model
> tiers").

**R10. The floor is not named, and it decides the band verdict before the run
(unpriced boundary, AGENTS.md §1).**

- Q must list `#C(F_p) ≈ p` points and solve `n ≥ p/4` unknowns. That is a
  floor of `Ω(p)` field operations, against rho's `O(√p)`.
- At `w = 0.1`, the enumeration alone (about `10p` MulP, so about `p` GAE)
  exceeds rho's 64 to 161 GAE at every stage-1 prime.
- So G5 (P = 0.97) and the "advantage" branch (G9: 0.03) are fixed by
  arithmetic, and P(advantage) is about 0.
- The measurement can tell constants and the LA exponent. It cannot test
  whether the cover's relations carry extra information.
- If `≡ 3 (mod 4)` primes are kept, there is a trivial reference row: walking
  `G, 2G, …` builds the same full log table in about `r/2` additions, with no
  linear algebra.

Amendment:

> New section "Boundary": the floor, its crossover with rho (below
> `p ≈ 13` at `n = p/4`), and, for the `≡ 3` series only, the exhaustive-table
> context row.

**R11. Linear-algebra additions and field inversions are not classed
(unpriced; O3 erratum 5 class).**

- `ic_control::mat_vec` and `mat_t_vec` count `mul` only for coefficients
  other than ±1. Folded quartic rows are almost all ±1.
- So Wiedemann's Krylov work is mostly `nat.add`: about `8 nr` additions
  against `nr` multiplications per iteration, by my reading.
- The protocol's classes (MulP, MulR, InvR, Canon, Lookup/Insert, Draw) price
  none of those additions.
- There is no InvP class. About 5 to 6 Fermat inversions per secant pair (secant
  normalisation, `line`, `1/b`, `1/(2c2)`, `section_point`) count either as 1
  op or as about `1.5 log2 p` MulP; the protocol does not say which.

Amendment:

> Classes AddP and AddR are native on every arm (priced at the band and
> `w_cal`). InvP counts each `pow(·, p−2)` as one inversion, priced at
> `w_inv = 10` in the field unit. The inversion count is reported.

**R12. The baseline clause relabels a control as the baseline (wrong
scope).**

- The protocol: "Arm Q is called an improvement of the index-calculus
  baseline only if its total is below arm D's at all three prices"
  `[pcc:unverified]`. D is a control defined in this protocol, not the lab's
  current IC baseline.
- At `p ≡ 3 (mod 4)` D accepts every trial.
- So Q below D is at most a relation-source component result (AGENTS.md §2,
  "component comparison").

Amendment:

> "Q below D at every price is reported as a relation-source component
> comparison, not as an improvement of the index-calculus baseline
> `[pcc:unverified]`."

**R13. The calibration denominator favours Q (unpriced reference cost).**

- `w_cal = time(op) / time(E addition)`, where the addition is `ec.rs`'s:
  `F_{p^2}` container and Fermat inversion.
- Amendment 1 of the O3 rerun records Fermat inversion at 14 to 30
  multiplications, against 7 to 15 for extended gcd.
- A slow denominator makes MulP look cheap and rho look expensive. That is
  conservative for "closed" and anti-conservative for "advantage".

Amendment:

> Record `t_add` for `ec.rs` and for a plain `u64` affine addition (the
> `ic_boundary::invmod` path). Report the calibrated class under both. An
> advantage needs both.

### Low

**R14. The rho reference is set-up dominated at these sizes.**

- Recorded walk share: 11.9 of 64.0 operations at `r = 109`, and 85.5 of
  161.3 at `r = 3169` (O3 json).
- Modelled rho exponent over stage 1: about 0.22, not 0.5. The slope `s` is
  then `e_Q − e_rho`.
- The walk shape switches from 4 to 8 jumps at `r ≥ 2^15`, between 25601 and
  51203, which puts a step into the stage-2 fit.

Amendment:

> Report Q's own fitted exponent, rho's walk-only and total exponents, and a
> phase-share table. Run the slope rule on a two-phase toy first (lesson
> `L-eda09df2bc11`).

**R15. j = 0 is not excluded.** `a_s = 0` with `p ≡ 1 (mod 3)` can give prime
`#E` with `j = 0`, and the matched rho would then need the order-6 fold.
Amendment: the selection rejects `a = 0` and `b = 0`.

**R16. `NOTE.md` is not hashed.** It carries the O4 definition and the
rationale. Amendment: add it to `frozen.sha256`, or state that it is not
frozen.

**R17. `--threads 2` during timing.** Amendment:
`measure_class_times` and `t_add` run with no other arm running; the host and
its isolation are recorded.

## The seven questions asked

1. **Is the size bound to `p`?** `r` is bound by Hasse, and `r ≥ 0.85 p`
   holds for `p ≥ 165` (checked). But `n` is not bound uniformly (R2), and rho
   is set-up dominated (R14).
2. **Is every arm in one unit, with set-up and LA charged?** No.
   - The unit is GAE, with Q's whole cost native (R1).
   - LA additions and inversions are unclassed (R11).
   - Set-up phases a to g are unlisted (R4).
   - The calibrated reading uses factor 1 only.
   - The band and calibrated readings are applied to all arms, but with
     `w = 0` and factors 0 and 10 dropped from the verdict.
3. **Can the `closed` branch be reached?**
   - Under `classify`: no (R1).
   - Under the prose, the slope condition holds with Wiedemann throughout
     (model `[1.01, 1.96]`, half-width about 0.47 with the residue jump, about
     0.11 without it) and fails with mixed LA (R3).
4. **Can the controls fail?** The negative control can, if the residual check
   covers every row. The positive control can fail only at `p = 53`, so it
   cannot see R2, R3, R5 or R6 (R7). The A/A check is a determinism check, not
   a noise floor.
5. **Is there an asymmetry?**
   - Unlisted set-up (R4); a rank test outside the clock (R5); additions
     unpriced (R11); the calibration denominator (R13).
   - Possible amortisation across the 12 runs of a prime (R4).
   - `verify_relation` possibly off (R7).
6. **Does the code support the plan?** Mostly, with four gaps: the overflow at
   `p ≥ 41,874` (R6); the dense LA unusable at stage 2 (R3); `peel` needing a
   target column (R3); and the `p = 53` counts needing an exhaustive mode (R7).
7. **Are the predictions consistent?** No:
   - G2 is wrong at 4 of 7 primes (R2).
   - G3 is likely wrong at 6 of 7 (R5).
   - G4 depends on the unfrozen LA (R3).
   - G7: Q's group operations are about 1 to 4 at every prime, so it is below
     rho at `w = 0` at the small primes too.
   - G9: P(closed) is about 0 under `classify`, and P(advantage) is about 0
     under the floor (R10).
   - G10 is inconsistent with the copied arithmetic (R6).
   - G11 needs a mode that is not specified (R7).

## Not checked

- Whether line sections at scale are close to uniform over columns. R5 uses a
  random-row model.
- Wall-time feasibility of stage 2 with Wiedemann. The estimate is minutes
  per solve at `n ≈ 51k`, not measured.
- The research-kb and lead-atlas checks that `NOTE.md` says were not run.
  This review makes no novelty statement.
