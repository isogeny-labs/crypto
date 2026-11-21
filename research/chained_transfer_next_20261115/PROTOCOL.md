# Open case O4, the line-relation index calculus on a bielliptic plane quartic: protocol (2026-11-15)

Frozen by `frozen.sha256` before any measured run. Nothing was run before
freezing: no unit test, no selection, no timing. This text was written from
reading the files named under "What exists". Every prediction and every number
named here is `[pcc:unverified]`. Choice and reasons:
`research/chained_transfer_next_20261115/NOTE.md`.

## Question

Does the line-relation index calculus of `src/cryptanalysis/bielliptic_quartic.rs`
(curve `C: v^4 = x^3 + a x + b` covering `E: y^2 = x^3 + a x + b` by
`(x, v) -> (x, v^2)`, genus 3, degree 2, not hyperelliptic) cost less than the
matched negation-map rho on `E`, at any `p`, when every native operation of
every arm is priced at 0, 0.1 and 1 GAE (AGENTS.md §5a) and read at calibrated
weights; and how does the gap grow with `p`?

PROOF.md T5 to T7 do not apply (non-hyperelliptic; T6 needs `g > d^2 - d + 1 = 3`).
The relations used are line sections, of weight 4 and class `K`, projected to
`E` by the norm. The README of the diagnostic states that timing, calibrated
operation totals and a rho comparison are null, and the code refuses `p > 257`.

## Instance selection, bound to the size parameter

The instance is `E` itself, so the transferred order is `r = #E(F_p)`.
`r` is tied to `p` by Hasse: `|r - (p + 1)| <= 2 sqrt(p)`. The O3 defect (an
`r` chosen independently of `p` while rho costs `sqrt(r)`) cannot recur, and
the selection still asserts it.

For the prime with list index `i`, scan `s = 0, 1, ..., 3999`:
`a_s = mix64(0xA4B1 + 101 i + s) mod p`, `b_s = mix64(0xB7C3 + 101 i + s) mod p`
(`mix64` of `residual_walk`). The first `(a_s, b_s)` that passes every check is
the instance: nonsingular (`4a^3 + 27b^2 != 0 mod p`); `r = #E` prime (the
diagnostic's `solve` requires it); `r >= ceil(0.85 p)` (always true by Hasse
for `p >= 211`, recorded as a check, not a filter); at least 8 rational
points on `C`. If no `s` passes, the next prime above `p` is tried, at most 20
times; the prime used is the one recorded and the fit uses it. A prime that
runs out of hops is reported as not run and not replaced.

Requested primes. Stage 1: `211, 419, 821, 1619, 3203, 6421, 12841`. Stage 2:
`25601, 51203, 102407`, run in that order and not started after the wall-clock
budget of 1500 s (`--budget-s 1500`) has passed; a stage-2 prime that is not
started or not finished is reported as not run, never extrapolated.

DLP instances. Stage 1: 3 per prime, `seed = seed0 xor (k * 0x1000)`, generator
`G = E.point_from_seed(seed)`, `x = (seed * 2246822519 + 7) mod r`, `T = [x]G`.
Stage 2: 1 per prime. The solver never receives `x`.

## Arms and references

- **Q** (primary arm): the diagnostic's mechanism at scale. Factor base: the
  distinct non-identity images `c(R)` of rational points `R` of `C`, folded by
  `+-`. Rows: complete rational line sections found by secant pairs `(R1, R2)`
  in an order permuted by the IC seed; the residual quadratic must split over
  `F_p`; each section gives the row `sum_i c(R_i) = 0` on the folded columns.
  One anchor row fixes the scale with a known multiple of `G`. Linear algebra
  mod `r` by the counted Wiedemann of `ic_control` (after `peel`), or a counted
  dense elimination at `p <= 3203` if that is the code that exists. Individual
  log by a signed rational fibre of a shifted target `T + [s]G`, `s` up to 512.
  Collection stops when the system has full column rank; a pair-trial budget
  of `64 * columns` per run ends a run that cannot reach it (reported as a
  failed cell, not retried).
- **D** (control that separates the source from the linear algebra): the same
  columns, anchor, linear algebra and target stage, but the rows are found by
  elliptic additions: pick `P1, P2, P3` in the factor base at random and keep
  the row if `-(P1 + P2 + P3)` is in the factor base up to sign. It shows what
  the quartic source saves in group operations.
- **Matched reference (primary)**: negation-map rho on `E` in the same process
  (`rho_full_ec(e, gen, target, r, seed, negation = true)`), set-up separated,
  16 seeds per instance (stage 2: 8).
- **Context, not the matched reference**: plain rho (`negation = false`), 16
  seeds per instance.

Seeds. Stage 1: 4 IC seeds per instance for Q and D, `700 + s + 10 k` for Q and
`500 + s + 10 k` for D. Stage 2: 2 IC seeds. Every log is certified by
`[t]G == T` on `E`; an uncertified run is a failed cell and counts against
G1 below.

Controls.
- **Positive control** (must pass before any scale run): the diagnostic's own
  known-answer replay at `p = 53, a = 2, b = 1`: 45 rational points, 79
  certified sections, 11 folded columns, rank 11, target scalar 17
  (`bash tools/check_bielliptic_quartic.sh` and the `quartic-ic` `demo`), and
  the scalar replay over all 59 scalars with two generators, using the new
  uncapped module, not the old one. If the new module does not reproduce those
  counts, the run is not started.
- **Negative control**: flip one entry of one certified row (or one right-hand
  side) before linear algebra. The solve must then fail certification
  (`[t]G != T`) or fail the residual check. A solve that still certifies is a
  defect of the instrument and stops the run.
- **Source control**: arm D, above.
- **Noise floor**: a second pass of arm Q on the first instance of the first
  three primes with the same seeds, reported as A/A; counted operations must
  be identical, and wall time reported with its spread.

## Pricing and calibrated reading

1 GAE = one elliptic addition on `E`. The native counters of every arm, rho
included, are priced at 0, 0.1 and 1 GAE per operation (AGENTS.md §5a).
- Q and D: group operations (anchor, target shifts, D's relation additions)
  are the 0 GAE price; the native classes are `MulP` (relation phase, root
  finding, section reconstruction), `MulR` and `InvR` (linear algebra),
  `Canon` (folding an image to `+-`), `Lookup`/`Insert` (column map), `Draw`.
- Rho: `Canon`, `Hash`, `Lookup`, `Insert`, `Word`, as in
  `negation_remeasure::RhoFull::native`.
- The row certificate re-verification (`verify_relation`) is not needed to
  solve; the primary total leaves it out (this favours the IC arm: if the arm
  is above without it, it is above with it). It is added as a listed
  sensitivity, together with the linear-algebra additions.

Calibrated reading: `w_cal(class) = time(native op) / time(one group addition
of E)`, best of 25 batches of 20,000 in the same process
(`measure_class_times`), each class at 0, 1 and 10 `w_cal`. The host and its
isolation are recorded; if the run is not on an isolated host the calibrated
reading is labelled as taken on a shared VM. Order classes `above`, `below` or
`unresolved` by `negation_remeasure::classify`. The calibrated factor 1 is the
reading that decides the verdict.

The 0 GAE price has a known weakness here: arm Q's group operations are few
(anchor and shifts), so at 0 GAE it can sit below rho for reasons of
accounting. The rule below does not let a 0 GAE reading decide an advantage.

## Run

The binary does not exist yet; this command line is the frozen interface.

```
python3 tools/benchrec.py run <record> -- bash -c 'set -o pipefail; \
  ./target/release/quartic_bielliptic_ic --threads 2 --min-r-frac 0.85 \
  --max-prime-hops 20 --stage2 25601,51203,102407 --budget-s 1500 \
  --out-dir research/chained_transfer_next_20261115 \
  --protocol research/chained_transfer_next_20261115/PROTOCOL.md \
  2>&1 | tee research/chained_transfer_next_20261115/run.log'
```

One run. If it is killed before stage 1 completes, the rerun is the same
command under a new record, and both are reported. Before the run: fill
`.resume/RESUME.md` (`scripts/checkpoint.sh init`); the execution route is the
one the experiment director freezes in the task card (default `isolab-eks`);
this protocol does not choose it and does not authorise the run.

## Decision rule

Cells are the stage-1 primes. Stage-2 primes enter a second fit, reported
separately, and do not enter the verdict. Let `R(p)` be the ratio of arm Q's
total to the negation-rho total at `w = 0.1`, and `R_cal(p)` the same at the
calibrated factor 1. Let `s` and `s_cal` be the OLS slopes of `ln R` and
`ln R_cal` on `ln p`, with 95% `t` intervals (`fit_loglog`, `t95`).

- **O4 closed by data (loss grows)**: every stage-1 Q cell is certified and
  is `above` rho at `w = 0.1`, at `w = 1` and at the calibrated factor 1,
  **and** the lower end of the 95% interval of `s` and of `s_cal` is above 0.2.
  The label of the `w = 0` price is reported next to the verdict; if Q is
  `below` at `w = 0` the verdict reads "closed at the priced band and the
  calibrated reading; the 0 GAE price cannot rank it".
- **O4 open: advantage seen**: any cell with Q `below` rho at `w = 0.1`, at
  `w = 1` and at the calibrated factor 1 together. An accounting change is
  not an advantage: the cell is re-read with every native counter listed, and
  with D's total beside it, before the word is used.
- **Unresolved**: everything else, reported with the cells or slopes that block
  the rule. Missing or uncertified cells make the verdict unresolved, never
  closed.

The verdict is the binary's `verdict` field, computed by the code frozen here;
RESULTS.md quotes it exactly. The gap to rho is reported for every row as the
distance remaining, not as a pass or fail gate for the success criterion. Arm
Q is called an improvement of the index-calculus baseline only if its total
is below arm D's at all three prices and its logs are certified
`[pcc:unverified]`; nothing here would be called an improvement over rho
unless the advantage branch is met.

## Frozen predictions

G1. Every stage-1 Q and D run is certified (`[t]G == T`) and no instance is
skipped. P = 0.85. (New code at 12 times the old field gate: the risk is rank
or budget failure, not mathematics.)

G2. Folded columns `n` satisfy `n / p` in `[0.15, 0.30]` at every stage-1 prime.
P = 0.8. (The `p = 53` replay has 11 columns, 0.21.)

G3. Rows reach `1.0 n` to `1.4 n` at the point where the system first has full
column rank, for Q. P = 0.75.

G4. The slope of `ln(Q MulR count)` on `ln p` (stage 1) lies in `[1.7, 2.2]`.
P = 0.8. (Sparse rows of weight at most 4 and `n` columns: about `8 n^2`.)

G5. Every stage-1 Q cell is `above` rho at `w = 0.1` and at `w = 1`. P = 0.97.
At the calibrated factor 1, every cell is `above`. P = 0.95.

G6. `s` (stage 1, `w = 0.1`) lies in `[1.2, 1.8]` (P = 0.7), and its 95%
lower end is above 0.2 (P = 0.95). `s_cal` lies in `[1.2, 1.8]` (P = 0.6).

G7. Arm Q's group operations (anchor plus shifts) are at most 600 per instance
at every prime (P = 0.8), and below the negation rho's group operations at
`p >= 6421` (P = 0.6), so the `w = 0` price does not rank Q against rho at the
smaller primes and may favour it at the larger ones.

G8. Arm D's group operations exceed rho's at every cell (P = 0.97). Q's total
at `w = 0.1` lies between 0.3 and 3 times D's at every stage-1 prime
(P = 0.7), and Q is worse than D at `w = 1` at six or more of seven primes
(P = 0.8).

G9. Verdict "O4 closed by data" (with or without the 0 GAE rider): P = 0.75;
"unresolved": P = 0.22; "advantage seen": P = 0.03.

G10. Stage 2 completes `25601` (P = 0.9), `51203` (P = 0.8) and `102407`
(P = 0.6) within the budget rule.

G11. The positive control reproduces 45 points, 79 sections, 11 columns and
the scalar 17 (P = 0.97); the negative control fails certification
(P = 0.99); the A/A passes show identical counted operations (P = 0.98).

## Stop conditions

- The positive control does not reproduce, or the negative control still
  certifies: stop before any scale run; report the defect.
- Two consecutive uncertified Q runs at one prime: record the prime as a failed
  cell, continue with the next prime; the cell is not replaced or retried.
- The wall-clock budget passes: no new stage-2 prime is started.
- No prime within 20 hops has a passing instance: that prime is not run.
- Any request to change an arm, a seed list, the selection or the rule after
  data: stop and write an amendment with its own hash; the frozen rule is not
  edited.

## What exists and what must be written

Exists (names checked by reading, not by building):
- `src/cryptanalysis/bielliptic_quartic.rs`: `Cover::new` (gated at
  `MAX_PRIME = 257`), `rational_points`, `secant`, `line_relation`,
  `verify_relation`, `collect_relations` (gated at `MAX_PAIR_TRIALS = 65536`),
  `prepare`, `recover` (gated at `MAX_TARGET_SHIFTS = 512`), `group_order`.
  Not edited by this protocol; the new module copies, not relaxes, it.
- `src/bin/quartic_ic.rs` (`quartic-ic demo | collect | solve`),
  `tools/check_bielliptic_quartic.sh`, evidence under
  `research/bielliptic_quartic_20261005/`.
- `src/cryptanalysis/chained_transfer/ic_control.rs`: `Nat`, `Row`, `peel`,
  `wiedemann` (all `pub(crate)`); `negation_remeasure.rs`: `rho_full_ec`,
  `mean_rho`, `ClassTimes`, `measure_class_times`, `report_arm`, `classify`,
  `ratio3`, `ec_instance`; `full_jacobian_ic.rs`: `fit_loglog`, `t95`,
  `TalliedEc`; `ec.rs`: `Curve`, `point_from_seed`; `tools/benchrec.py`.

Must be written:
1. `src/cryptanalysis/chained_transfer/quartic_ic.rs`, registered in
   `chained_transfer/mod.rs`:
   - `QuarticCover::new(p, a, b)` without the 257 gate (`p < 2^31`), keeping
     `rational_points`, `secant`, `line_relation` semantics and adding mod-`p`
     multiplication counters; `QuarticCover::group_order` must stay fast
     enough at `p = 102407`.
   - `candidate_ab(p, s)`, `select_prime_order(p, i, min_r_frac)` returning
     the instance, the prime used and the passed-over primes.
   - `collect_rows_counted(cover, seed, max_trials)` returning rows over the
     folded columns and a counter record (`mul_p`, `add_p`, pair trials,
     non-split residuals, duplicates).
   - `solve_quartic_ic(inst, params)` (arm Q) and `solve_direct_ic(inst,
     params)` (arm D), each returning the log, a certified flag, the columns,
     rows, group operations and a `Nat`-style record by class.
   - Tests: `reproduces_p53_demo` (45, 79, 11, scalar 17), `tamper_fails`,
     `selection_binds_r_to_p`, `folding_is_sign_consistent`.
2. `src/bin/quartic_bielliptic_ic.rs`: the arms, rho references, pricing,
   `verdict`, `quartic_ic.json`, the table and SVG, following
   `src/bin/full_jacobian_ic.rs`.
3. A reviewer check that `ic_control::{Row, peel, wiedemann}` fit a folded
   sign row; if not, a counted copy with a unit test, not an edit of
   `ic_control.rs`.

## Not claimed

No result and no statement about curves outside the family
`v^4 = x^3 + a x + b`. A closed verdict here speaks to this family and to the
sizes run, not to the other non-hyperelliptic covers under O4.

## Amendment 1 (2026-11-15, before any code or data)

Frozen by the new `frozen.sha256`, which now covers `PROTOCOL.md` and
`NOTE.md` (finding R16). It implements the red-team review
`research/chained_transfer_next_20261115/REDTEAM.md` (findings R1 to R17)
before any code is written or any run is made. The sections above stay
unedited as the record. **Where they, or `NOTE.md`, disagree with this
amendment, this amendment governs.** Every number and every prediction below
is `[pcc:unverified]`: no pcc claim exists for any of them.

What was run before this freeze, and nothing else:

- The red-team's three stdlib Python scripts (REDTEAM.md, "What was
  reviewed"): factor-base size by `p mod 4` with arbitrary `(a, b)`, a slope
  model, a coupon-collector simulation. No protocol instance, no arm.
- A selection-only Python script of this amendment, in the session scratchpad
  (`select_only.py`, sha256 `371d3356…98d8b`, output sha256 `f8ed3787…7c1cb`).
  It applies the selection rule below to the new prime ladder and counts the
  folded columns `n`. It ran no arm, no rho, no linear algebra and no timing,
  and it used no repository code. Its table is item A4 below. Its first pass
  (before the `r != p` and embedding-degree checks of item A3 existed) picked
  an anomalous curve at `p = 229` (`u = 3`, `a = 149`, `b = 109`,
  `r = #E = 229`); that is why item A3 adds those checks. This is a selection
  rule changed after seeing selection output, disclosed here; no cost of any
  arm was seen.
- A three-line check that the same column count gives the known answer at
  `p = 53, a = 2, b = 1`: 45 rational points of `C` and 11 folded columns.

No code under `src/` was written or changed, and no `quartic_ic.json`,
`run.log` or benchmark record exists in this directory (checked 2026-11-15).

### Why

The red team found that the frozen rule cannot be read as written
`[pcc:unverified]`:

- R1: the decision rule ranks Q against rho across a price band in which the
  order flips (Q is nearly free at 0 GAE because all its work is native), and
  `negation_remeasure::classify`, which the protocol names, would class every
  cell `unresolved`.
- R2: the factor base is `(r - 1)/2` at `p ≡ 3 (mod 4)` and about `p/4` at
  `p ≡ 1 (mod 4)`; the frozen ladder mixes both residues, so a hidden size
  parameter enters the slope (the O3 defect in a new variable).
- R3 to R6: the linear algebra is not fixed, set-up phases and the cold
  interval are not priced, the stopping rule needs an unpriced rank test, and
  the copied section arithmetic overflows `u64` at `p ≥ 41,874`.
- R7 to R17: controls that cannot see scale defects, unfrozen seeds and
  aggregation, a verdict label wider than the measurement, an unnamed floor,
  unclassed additions and inversions, a control relabelled as the baseline, a
  calibration denominator that favours Q, a set-up-dominated rho, `j = 0`
  curves, an unhashed `NOTE.md`, and timing under a second thread.

### Summary of findings and their disposition

| finding | severity | disposition | item |
|---|---|---|---|
| R1 unit and band | Critical | adopted with two changes (Q's own group operations enter the base; a lower-bound arm cannot be classed `below`) | A1 |
| R2 residue of `p` | High | adopted; select-only table frozen | A2, A4 |
| R3 linear algebra | High | adopted with one change (two RNG streams, item A10) | A5 |
| R4 phases, cold interval | High | adopted | A6 |
| R5 stopping rule | High | adopted; G3 restated per prime | A7 |
| R6 overflow | High | adopted | A8 |
| R7 controls | Medium | adopted, with the tampered row and the BSGS failure action fixed | A9 |
| R8 seeds, aggregation | Medium | adopted with one change (collection and LA RNGs split) | A10 |
| R9 verdict scope | Medium | adopted; the T6-style argument is recorded, not adopted as a claim | A11 |
| R10 floor | Medium | adopted, with the floor restated in the field unit | A12 |
| R11 additions, inversions | Medium | adopted | A13 |
| R12 baseline clause | Medium | adopted verbatim | A14 |
| R13 calibration denominator | Medium | adopted with change (moved to the rho base, since the field unit no longer divides by an addition time) | A15 |
| R14 set-up-dominated rho | Low | adopted | A16 |
| R15 `j = 0` | Low | adopted, plus `r != p` and embedding degree `> 6` | A3 |
| R16 `NOTE.md` unhashed | Low | adopted | A17 |
| R17 timing under threads | Low | adopted | A18 |

No finding is rejected.

### Changes

**A1 (R1). Primary unit, base and decision rule.** Replaces the first
paragraph of "Pricing and calibrated reading" and the three bullets of
"Decision rule".

- **Primary unit: one `F_p` or `F_r` multiplication or squaring**, as in
  `research/full_jacobian_ic_rp_20261114/PROTOCOL.md` Amendment 1, items 1
  to 4, with the frozen inversion weight `w_inv = 10` (`--inv-weight 10`).
  Both moduli are below `2^20` here (A8), so one product costs the same `u64`
  code on both.
- Base of Q and D: `MulP + MulR + w_inv · (InvP + InvR)`, plus every elliptic
  addition or doubling the arm performs (Q: anchor and target shifts; D: the
  same plus its relation additions), each priced by
  `full_jacobian_ic::ec_add_tally`. Change from REDTEAM.md: the red team named
  only D's additions; Q's anchor and shifts are group operations too and are
  priced the same way, so no group operation of any arm is left out.
- Base of rho (negation, and plain as context): the `FieldOps` tally of
  `full_jacobian_ic::rho_full_ec_tallied` over the whole run (set-up and walk,
  every group operation in `total_ops`), as `muls + sqrs + w_inv · invs`
  (`ic_boundary::FieldOps`).
- Everything else is native on every arm, priced at 0, 0.1 and 1 unit per
  operation (the band) and at 0, 1 and 10 `w_cal` (the calibrated vector),
  with `w_cal(class) = time(class) / time(one F_p multiplication)`. Native
  classes: Q and D: `Canon`, `Lookup`, `Insert`, `Draw`, `AddP`, `AddR` (A13);
  rho: `canonicalisations`, `partition_hashes`, `dp_lookups`, `dp_inserts`,
  `word_ops`.
- Cells are classed by `negation_remeasure::classify` on the full
  three-price vectors, the band vector and the calibrated vector, with no price
  left out, and with the arm's `lower_bound` flag (A6: true for Q and D).
- `Q_above_rho` (the verdict that replaces "O4 closed by data", A11) needs
  `above` in both vectors at every stage-1 cell, every stage-1 Q run
  certified, the rho re-pricing condition of A15, and the slope condition of
  "Decision rule" (the lower end of the 95% interval of `s` and of `s_cal`
  above 0.2), with `R` and `s` now in the field unit (A10 defines them).
- `Q_below_rho` needs `below` in both vectors at some cell. Change from
  REDTEAM.md: `classify` never returns `below` for an arm with
  `lower_bound: true` (it returns `unresolved (lower bound)`; unit test
  `classify_never_calls_a_lower_bound_below`), and A6 makes Q a lower bound.
  So under this amendment `Q_below_rho` is unreachable. A cell whose counted
  totals are below rho in both vectors is reported as
  `unresolved (lower bound)` with `counted_below_rho: true`, and it requires a
  further amendment that counts the work listed in `lower_bound_note` before
  any reading. That is the conservative reading of a lower bound below a
  reference, and the red team's rule is kept for the case where that later
  amendment removes the flag.
- Anything else is `unresolved`, reported with the cells or slopes that block
  the rule.
- The GAE reading of the frozen text is kept as a secondary `verdict_gae`,
  reported, labelled, and not predicted. Change: it is computed with
  `classify` on the full GAE three-price vectors (0, 0.1, 1 GAE and 0, 1,
  10 `w_cal` against one `E` addition), not by the frozen prose, because the
  prose rule ranks across a flipping band, which AGENTS.md §5a forbids.
- The sensitivity at `w_inv = 0` and `w_inv = 30` is reported from the stored
  counts (base only, native at 0) and does not enter the verdict.

**A2 (R2). Prime ladder, all `p ≡ 1 (mod 4)`.** Replaces "Requested primes".

- Stage 1: `229, 421, 821, 1621, 3209, 6421, 12841`.
- Stage 2: `25601, 51217, 102409`, run in that order under the unchanged
  budget rule (`--budget-s 1500`).
- Each is the least prime `≡ 1 (mod 4)` at or above the frozen list
  (`211 → 229`, `419 → 421`, `1619 → 1621`, `3203 → 3209`, `51203 → 51217`,
  `102407 → 102409`; the others were already `≡ 1`). Recomputed for this
  amendment; it agrees with REDTEAM.md.
- A prime hop goes to the next prime `≡ 1 (mod 4)`.
- The selection asserts `p % 4 == 1` and reports `n` and `n/p`.
- No `p ≡ 3 (mod 4)` series is run. If it is wanted later, it is a second
  series with its own fit, at least four primes and its own predictions, never
  pooled with this one.

**A3 (R15, extended). Selection checks.** Added to the checks of "Instance
selection": `a != 0` (excludes `j = 0`), `b != 0` (excludes `j = 1728`),
`r != p` (excludes anomalous curves, whose discrete log falls to a
polynomial-time lift), and `r` does not divide `p^k - 1` for `k = 1..6`
(embedding degree above 6). The last two are this amendment's additions to
R15, made after the first selection pass picked `r = p` at `p = 229`
(disclosed above). Symbols are renamed (A10): the scan index is `u`, so
`a_u = mix64(0xA4B1 + 101 i + u) mod p` and
`b_u = mix64(0xB7C3 + 101 i + u) mod p`, `u = 0..3999`, with `i` the
position of the requested prime in the concatenated list, stage 1 then
stage 2 (`i = 0..9`). `mix64` is `residual_walk::mix64` with wrapping `u64`
arithmetic. The checks `r` prime, `r >= ceil(0.85 p)` and at least 8 rational
points on `C` stay.

**A4 (R2). Select-only table, frozen.** From the scratchpad script described
above, before any code. `n` is the number of folded columns, the count of `x`
with `f(x) = x^3 + a x + b` a nonzero fourth power; `#C(F_p) = 1 + 4n` here
because `r` is odd, so `f` has no root. No hop was needed at any prime.

| i | p | u | a | b | r = #E | r/p | #C(F_p) | n | n/p | (ln n)/4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 229 | 38 | 65 | 224 | 223 | 0.974 | 253 | 63 | 0.2751 | 1.04 |
| 1 | 421 | 5 | 139 | 134 | 457 | 1.086 | 485 | 121 | 0.2874 | 1.20 |
| 2 | 821 | 2 | 313 | 186 | 877 | 1.068 | 905 | 226 | 0.2753 | 1.36 |
| 3 | 1621 | 13 | 384 | 149 | 1627 | 1.004 | 1549 | 387 | 0.2387 | 1.49 |
| 4 | 3209 | 2 | 2442 | 2430 | 3119 | 0.972 | 3013 | 753 | 0.2347 | 1.66 |
| 5 | 6421 | 44 | 3361 | 1552 | 6521 | 1.016 | 6381 | 1595 | 0.2484 | 1.84 |
| 6 | 12841 | 22 | 3731 | 9102 | 12919 | 1.006 | 12829 | 3207 | 0.2497 | 2.02 |
| 7 | 25601 | 59 | 20573 | 16511 | 25423 | 0.993 | 25425 | 6356 | 0.2483 | 2.19 |
| 8 | 51217 | 2 | 8600 | 30914 | 51131 | 0.998 | 51533 | 12883 | 0.2515 | 2.37 |
| 9 | 102409 | 4 | 96014 | 95982 | 102359 | 1.000 | 102365 | 25591 | 0.2499 | 2.54 |

The binary's `--select-only` must reproduce `p, u, a, b, r, n` row for row
before any arm runs. A mismatch stops the run before any arm; the cause
(script or binary) is found and recorded in a further amendment, and this
table is not edited. Stage-1 `n/p` spans 0.2347 to 0.2874, a spread of about
±10% that enters any `n^k` slope; it is reported, not corrected.

**A5 (R3). One linear algebra for every prime and both arms.** Replaces "Linear
algebra mod `r` by the counted Wiedemann of `ic_control` (after `peel`), or a
counted dense elimination at `p <= 3203` if that is the code that exists".

- `peel`, then the counted Wiedemann of `ic_control`, then counted
  back-substitution of every peeled column, so every factor-base column has a
  log.
- `ic_control::peel` needs a target column; Q and D have none. A counted copy
  with no `tcol` is written in the new module, with a unit test; `ic_control.rs`
  is not edited.
- Dense Gauss–Jordan runs only at `p ≤ 421` (229 and 421), uncounted, as a
  cross-check of the Wiedemann logs. A disagreement is an instrument defect and
  stops the run.

**A6 (R4). New subsection "Phases and the cold interval".**

- Every Q and D run is cold: it is built from `(p, a, b, r, G, T)` alone and
  shares no state with another run. The three instances and the four seeds of
  a prime rebuild every table. The order `r` is an input to every arm (rho
  receives it already); `group_order` is not called inside an arm.
- Tables built in the run are charged: the square-root table at `p` MulP (one
  product per entry), or root finding by counted Tonelli–Shanks; the
  fourth-power table likewise.
- Point enumeration and the factor base are charged in MulP, Canon and Insert.
- Verification-only work is outside the primary total on every arm and is
  listed as a sensitivity with its count: `verify_relation`, the replay of
  factor-base logs (`prepare`), the fibre witness re-enumeration of `recover`,
  and the final `[t]G == T`. Rho's `[t]G == T` is already outside its count
  (`rho_full_ec` closure), so this is symmetric.
- The rho total is `RhoMean::arm()` (`total_ops`, set-up and walk) and its
  field tally (A1). The set-up is reported beside the total, not removed from
  it.
- Q and D carry `lower_bound: true` with a `lower_bound_note` naming the
  uncounted work: the line de-duplication set, allocation, sorting, and the
  `F_p` and `F_r` operations that the counters do not see (listed by the code
  review of A5 and A13).

**A7 (R5). Stopping rule.** Replaces "Collection stops when the system has
full column rank".

- Collect until every column has weight at least 1 and rows reach `n + 20`.
  Then run the counted solve (A5). If it returns `None` or its residual fails,
  collect `ceil(0.1 n)` more rows and retry.
- Every attempt is counted, and the rows used and the number of attempts are
  recorded.
- The pair-trial budget of `64 · n` (Q) and a draw budget of `64 · n`
  triples (D) cover all attempts of a run; exhausting it is a failed cell, not
  retried.
- No rank is tracked during collection.

**A8 (R6). Arithmetic domain.** Every product in the new module is reduced
mod `p` before the next multiplication, or computed in `u128`. The stated
domain is `p ≤ 2^20` (replaces "`p < 2^31`"). A unit test compares the
section polynomials with a `u128` reference on `10^4` random lines at
`p = 102409`.

**A9 (R7). Controls.**

- (i) The `p = 53` replay runs in an `--exhaustive` mode named here: every
  ordered pair of the existing pair order, as in `bielliptic_quartic.rs`, so
  the counts 45, 79, 11 and the scalar 17, and the scalar replay over all 59
  scalars with two generators, are comparable with the old module.
- (ii) At the first instance (`k = 0`) of every prime, the logs of 32
  columns, chosen by `StdRng::seed_from_u64(0xB565 ^ p)`, are checked against
  an independent, uncounted baby-step giant-step on `E`. A mismatch is an
  instrument defect and stops the run.
- (iii) Tamper control, at `k = 0`, seed `j = 0` of Q at every stage-1 prime,
  as an extra uncounted run: the row with index
  `mix64(0x7A3B ^ p) mod rows` of the collected system gets one entry changed
  before linear algebra. The solver's residual `A x = c` covers every collected
  row (as `ic_control::wiedemann` already does; the copy keeps it). The solve
  must then fail its residual or certification. In addition, one certificate
  is altered at `p = 12841` and at the largest stage-2 prime run, and
  `verify_relation` must reject it.
- (iv) `verify_relation` runs on every collected row of every run, uncounted,
  and its failure count is reported as a measured value.
- The A/A pass (Q, first instance of 229, 421, 821, same seeds) is renamed a
  determinism check: counted operations must be identical; wall time is
  reported with its spread and is not a noise floor of the counts.

**A10 (R8). Seeds, symbols and aggregation.**

- Symbols: `u` is the selection scan index (A3), `h = 0..512` the target
  shift (`T + [h]G`), `j` the IC or rho seed index, `k` the instance index;
  `s` and `s_cal` are only the slopes.
- `seed0 = 0x5EED_0415`; `seed_k = seed0 ^ (k · 0x1000)`;
  `G_k = E.point_from_seed(seed_k)`;
  `x_k = (seed_k as u128 · 2246822519 + 7) mod r`, and if `x_k = 0` the
  instance uses `x_k = 1` and prints that.
- Stage 1: `k = 0, 1, 2`; stage 2: `k = 0`.
- IC seed `(arm, k, j) = base_arm + j + 10 k`, `base_Q = 700`,
  `base_D = 500`, `j = 0..3` (stage 1), `j ∈ {0, 1}` (stage 2).
- Rho seed `(k, j) = 1000 + j + 100 k`, `j = 0..15` (stage 1), `j = 0..7`
  (stage 2); the same seeds for the negation and the plain rho.
- RNGs, change from REDTEAM.md: two streams per run, so that A7's retries
  do not interleave collection and linear algebra. Collection:
  `StdRng::seed_from_u64(IC seed)`, consumed by Q's pair draws or D's triple
  draws in collection order (retries continue the same stream). Linear algebra:
  `StdRng::seed_from_u64(IC seed ^ 0x4C41)`, consumed by Wiedemann across
  attempts.
- Q's pair order: the frozen "order permuted by the IC seed" cannot be
  materialised at stage 2 (about `p^2 / 2` pairs). Q draws two distinct affine
  rational points of `C` uniformly with the collection RNG; a line already
  seen is skipped and counted as a duplicate. D draws three columns uniformly
  and a sign for each uniformly. The `--exhaustive` mode of A9 (i) is the only
  mode with the old fixed order.
- `R(p)` is the mean Q total over its `3 × 4` runs divided by the mean
  negation-rho total over its `3 × 16` runs, at the price in question;
  `R_cal(p)` likewise at calibrated factor 1. `s` and `s_cal` are OLS slopes
  of `ln R` and `ln R_cal` on `ln p` over the 7 per-prime points, with
  `t95(5)`. Stage 2 is fitted separately (3 points, `t95(1)`), not in the
  verdict.
- The binary prints every seed, and a unit test checks them against these
  formulas.

**A11 (R9). Verdict values and scope.** The `verdict` values are
`Q_above_rho` (family `v^4 = x^3 + a x + b`, full factor base, the sizes run),
`unresolved` (including `unresolved (lower bound)`) and `Q_below_rho`
(unreachable under A1 while Q is a lower bound). "O4 closed by data" and
"O4 open: advantage seen" are no longer verdict values. No outcome changes the
status of any `PROOF.md` §5 item or of O4 in `NOTE.md`. A closure claim goes
to `red-team-breakthrough` and `validator-breakthrough` (AGENTS.md "Model
tiers"). The red team's argument that the weight-3 classes are rigid for
`d = 2` without Castelnuovo–Severi is recorded as a red-team argument
`[pcc:unverified]`, not adopted as a statement of this protocol; a written
check by the Validator or `crypto-lean` would decide it.

**A12 (R10). New section "Boundary".** `[pcc:unverified]` arithmetic, not a
measurement.

- Floor of Q and D: the factor base needs `f(x)` at every `x` (at least 2
  products each, `x (x^2 + a) + b`), so at least `2p` units, and a solve in
  `n ≈ p/4` unknowns. Rho: about `0.886 √r` walk additions with the negation
  map, about 13 units each at `w_inv = 10` (`2M + 1S + 1I`), so about
  `11.5 √p` units.
- Crossover of the enumeration floor with the rho walk: `2p = 11.5 √p` at
  `p ≈ 33`, below every ladder prime. With rho's set-up included (O3 record:
  64.0 operations at `r = 109`, 161.3 at `r = 3169`, about 830 and 2,100 units
  at 13 units each), the floor alone does not order the arms at `p = 229`
  (`2p = 458`) and exceeds the rho total by about an order of magnitude at
  `p = 12841` (`2p = 25,682`). The red team's crossover "below `p ≈ 13`" is the
  same comparison in GAE.
- So a `below` cell is excluded by arithmetic at the upper stage-1 primes,
  and at the lower ones the linear algebra decides it. The measurement can
  give constants, phase shares and the LA exponent; it cannot test whether the
  cover's relations carry information beyond a full-factor-base index
  calculus.
- The `p ≡ 3 (mod 4)` exhaustive-table context row is not needed, because no
  such series is run (A2).

**A13 (R11). Additions and inversions classed.** Classes `AddP` and `AddR` are
native on every arm (band and `w_cal`). `InvP` counts each `pow(·, p − 2)` as
one inversion and `InvR` each inversion mod `r`, both priced at `w_inv = 10`
in the base. The counted copy of the linear algebra records `InvR` separately
and does not also charge `pivot_inversion_steps(r)` multiplications; a unit
test checks that `MulR + pivot_inversion_steps(r) · InvR` of the copy equals
`Nat.mul` of `ic_control` on the same input. The inversion counts are reported.
Fermat inversion costs about `1.5 log2 p` products in time, so `w_inv = 10`
understates Q's inversions at the larger primes; this favours Q and is
conservative for `Q_above_rho`.

**A14 (R12). Baseline clause.** Replaces the sentence of "Decision rule" that
begins "Arm Q is called an improvement": "Q below D at every price is reported
as a relation-source component comparison, not as an improvement of the
index-calculus baseline `[pcc:unverified]`."

**A15 (R13). Calibration denominator, moved to the rho base.** In the field
unit, `w_cal` divides by the time of one `F_p` multiplication (the same `u64`
`a · b mod p` code on both sides), so the red team's concern, a slow `E`
addition in the denominator, no longer applies to `w_cal`. It applies instead
to the rho base, which counts each addition as `2M + 1S + 10` units while
`ec.rs` runs it in an `F_{p^2}` container with Fermat inversion. So:

- `t_add` is recorded for `ec.rs` and for a plain `u64` affine addition (the
  `ic_boundary::invmod` path), together with `t_mul`.
- Each cell is also classed with the rho base replaced by
  `total_ops · t_add / t_mul`, once for each `t_add`.
- A cell is `above` (or `below`) only if the class holds under the field-tally
  rho base and under both time re-pricings; otherwise it is `unresolved`.
  This covers the red team's "an advantage needs both" and applies the same
  test to `above`.

**A16 (R14). Exponents and phase shares.** Reported per stage: Q's own fitted
exponent (`ln` Q total on `ln p`), rho's walk-only and total exponents, and a
phase-share table (Q: enumeration and tables, relation collection, linear
algebra, individual log; rho: set-up, walk). The stage-2 rho series crosses
the walk-shape change of `ic_boundary::rho_jumps_for` (4 to 8 jumps at
`r ≥ 2^15`, so between 25601 and 51217); it is reported, not corrected. Before
the run, the slope rule is applied to a two-phase toy cost (a dominant
low-exponent phase plus a 1%-share steep phase) and to a fixed-cost case, as
lesson `L-eda09df2bc11` asks, in a unit test `slope_rule_two_phase_toy`.

**A17 (R16). `NOTE.md` is frozen.** `frozen.sha256` now holds
`sha256sum PROTOCOL.md NOTE.md`. The previous file held `PROTOCOL.md` only.
`NOTE.md` is not edited; where it states the O4 scope more widely than A11,
A11 governs.

**A18 (R17). Timing isolation.** `measure_class_times`, the `F_p`
multiplication time and both `t_add` run with no arm and no other thread
running; the host and its isolation are recorded. `--threads 2` remains for
the arms, whose counted operations do not depend on threads.

**A19. Run command.** Replaces the command of "Run"; the binary still does not
exist, and this is the frozen interface.

```
python3 tools/benchrec.py run <record> -- bash -c 'set -o pipefail; \
  ./target/release/quartic_bielliptic_ic --threads 2 --min-r-frac 0.85 \
  --max-prime-hops 20 --stage1 229,421,821,1621,3209,6421,12841 \
  --stage2 25601,51217,102409 --budget-s 1500 --inv-weight 10 \
  --seed0 0x5EED0415 \
  --out-dir research/chained_transfer_next_20261115 \
  --protocol research/chained_transfer_next_20261115/PROTOCOL.md \
  2>&1 | tee research/chained_transfer_next_20261115/run.log'
```

Before it, in order: the unit tests; `--exhaustive` at `p = 53` (A9 i);
`--select-only` against the A4 table. Any failure stops before the run.

### What stays

The question (narrowed in scope by A11), the family `v^4 = x^3 + a x + b`
and the cover, the arms Q and D and the matched negation rho with plain rho as
context, 3 instances and 4 IC seeds per stage-1 prime, 16 and 8 rho seeds, the
scan length of 4000, the 20-hop limit, the checks `r` prime,
`r ≥ ceil(0.85 p)` and at least 8 points, the target shifts up to 512, the
pair-trial budget of `64 · columns`, the stage-2 budget rule, the slope
threshold 0.2, one run with the stated rerun rule, the stop conditions, and
"Not claimed". The execution route is still the experiment director's task
card (default `isolab-eks`), and this amendment does not authorise the run.

### Predictions under the amendment

Restated where the amendment affects them; the others hold as frozen. All are
`[pcc:unverified]`.

- **G1** (unchanged text, P = 0.85): every stage-1 Q and D run is certified
  and no instance is skipped, now with A7's retries.
- **G2** (replaced). The old G2 (`n/p` in `[0.15, 0.30]`) is no longer a
  prediction: the A4 table gives stage-1 `n/p` from 0.2347 to 0.2874, inside
  the red team's restated band `[0.22, 0.30]`, so that band is disclosed as
  known before data. New G2: the binary's `--select-only` reproduces the A4
  table row for row. P = 0.9.
- **G3** (restated per R5). For Q, the mean over the 12 runs of a prime of
  `rows used / n` lies in `[(ln n)/4, (ln n)/4 + 0.6]` at every stage-1
  prime, that is `[1.04, 1.64]`, `[1.20, 1.80]`, `[1.36, 1.96]`,
  `[1.49, 2.09]`, `[1.66, 2.26]`, `[1.84, 2.44]`, `[2.02, 2.62]` for
  229 to 12841. P = 0.6. The formula is the red team's, set before the
  table; the numbers come from the A4 `n`.
- **G4** (band kept, P lowered to 0.75): the slope of `ln(Q MulR)` on `ln p`
  over stage 1 lies in `[1.7, 2.2]`. The red team's Wiedemann model gives
  1.71, near the lower edge, and the `n/p` spread of A4 adds noise.
- **G4b** (new, for A13): the slope of `ln(Q AddR)` on `ln p` over stage 1
  lies in `[1.9, 2.5]`. P = 0.7. Model: Krylov additions of about
  `2n` products of a matrix with about `n ln n` nonzero `±1` entries.
- **G5** (restated for A1 and A15): every stage-1 Q cell is `above` rho in
  the band vector and in the calibrated vector, and under both time
  re-pricings of A15. P = 0.93. Q's base includes MulR and the `2p`
  enumeration floor at price 0, so the 0-price entry is no longer the weak
  one.
- **G6** (band widened before data): `s` (band vector, `w = 0.1`) lies in
  `[1.2, 1.9]` (P = 0.7) and its 95% lower end is above 0.2 (P = 0.95);
  `s_cal` lies in `[1.2, 1.9]` (P = 0.6). The upper end moves from 1.8 to
  1.9 because the field unit charges the `n^2` linear-algebra products at every
  price; this was set from the red team's model, not from data.
- **G7** (restated; A6 moves the factor-log replay out of the total): Q's
  group operations (anchor plus shifts) are at most 40 per run at every prime
  and their mean per prime is at most 8 (P = 0.9); they are below the negation
  rho's group operations at every prime (P = 0.97).
- **G8** (restated in the field unit): D's group operations exceed rho's at
  every cell (P = 0.97). Q's total at native price 0.1 lies between 0.3 and 3
  times D's at every stage-1 prime (P = 0.8). Q's total exceeds D's in both
  vectors at six or more of the seven primes (P = 0.75).
- **G9** (restated for A1 and A11): `verdict = Q_above_rho`: P = 0.78;
  `unresolved` (any form): P = 0.22; `Q_below_rho`: P = 0, unreachable
  while Q is a lower bound. `counted_below_rho: true` at any cell: P = 0.02.
  `verdict_gae` is not predicted.
- **G10** (restated for A2 and A8): stage 2 completes `25601` (P = 0.9),
  `51217` (P = 0.8) and `102409` (P = 0.65) within the budget rule.
- **G11** (restated for A9): the `--exhaustive` replay gives 45 points, 79
  sections, 11 columns and the scalar 17 (P = 0.97); the tamper control fails
  its residual or certification at every stage-1 prime and the altered
  certificate is rejected (P = 0.99); the determinism check shows identical
  counts (P = 0.98).
- **G12** (new, A5 and A9): the dense cross-check at 229 and 421 agrees with
  the Wiedemann logs, and the 32-column BSGS check passes at every prime run
  (P = 0.95). `verify_relation` reports 0 failures on every run (P = 0.97).
- **G13** (new, A16): over stage 1, rho's total exponent (field unit) lies in
  `[0.1, 0.45]` (P = 0.7) and Q's own exponent in `[1.5, 2.2]` (P = 0.7).
- **G14** (new, A1): the cell classes at `w_inv = 0` and `w_inv = 30` (base
  only, native at 0) equal those at `w_inv = 10` at every stage-1 prime.
  P = 0.9.

Disclosure of bands set after looking at numbers: G2's band and G3's
per-prime values use the A4 selection table, which was computed before this
freeze; no arm cost, rho cost or timing of this protocol was seen. G5, G6,
G7, G10 and G13 use the red team's models and the O3 rho record
(`research/full_jacobian_ic_20261113/full_jacobian.json`) as quoted in
REDTEAM.md; no number from a run of this protocol exists.

## Amendment 2 (2026-11-18, before any data)

Written after the measurement code (PR #4319) and before any measured run.
No arm cost, rho cost, timing or verdict of this protocol has been seen. The
only outputs seen are the unit tests, `--select-only`, the `p = 53`
`--exhaustive` replay, and a smoke run at 229, 421 and 821. The smoke run had
one instance and two seeds per arm, used unit class times, and its numbers
were not read as data. It showed one fact used here: at 421, a tampered row
removed by peeling was caught only by the factor-base log replay (B1). The
code settled eight points where Amendment 1 is open (PROGRESS.md, "Choices
where the protocol is open"). This amendment freezes them. B4 changes the
stopping rule; the rest freeze readings already in the code.

**B1 (A9 iii). Tamper control, detection.** A changed entry in a row that
peeling removes is absorbed by the peeled column. The residual then holds,
and `[t]G == T` holds unless the target uses that column.

- The tamper is applied to the system of the clean run's final solve. That
  system goes through one solve, with a fresh linear-algebra stream and no retry. The
  changed entry is the first entry of row `mix64(0x7A3B ^ p) mod rows`,
  counted over relation rows; the anchor is row 0 and is not a candidate.
- Detected means any of the following, and the report names which: the
  solve fails, the residual fails, `[t]G == T` fails, or any factor-base log
  fails its replay `[x_c]G == P_c`.
- The control's prediction (G11) reads with this definition.

**B2 (A7). Row count.** "Rows reach `n + 20`" counts relation rows. The
anchor row is extra.

**B3 (A10). Target shifts.** `h = 0..511`: 512 shifts, the old module's
`MAX_TARGET_SHIFTS`. This replaces "`h = 0..512`". If no shift folds onto a
column, the run fails as `target_shifts_exhausted`.

**B4 (A7, change). Acceptance test.** A rank-deficient core can pass the
Wiedemann residual with a kernel-shifted solution. Under Amendment 1 a wrong
target log then ended the run as uncertified, and no retry was made.

- After a solve passes its residual, the target log is computed from it, and
  `[t]G == T` is checked with a counted scalar multiplication in the log
  phase. The shift search is counted as in A1.
- If the check fails, the attempt is a failed attempt (`wrong_log`), and the
  A7 retry rule applies: `ceil(0.1 n)` more rows, then solve again. Every
  attempt's linear algebra, shift search and check are charged. The budget
  covers all attempts.
- A run is certified when the accepted solve's target log also passes the
  uncounted independent check (`mul_independent`) of A6. A difference
  between the counted and the independent check stops the run as an
  instrument defect.
- Rho checks its collision answer the same way and is charged for it, so the
  acceptance test is not an extra cost Q alone pays.

**B5 (A10). Arm D, identity sums.** A triple with `P1 + P2 + P3 = O` is
discarded and counted as rejected; `O` is not a column.

**B6 (A1, A16). Calibrated weights.**

- Q's and D's class times are measured on their own code paths
  (`measure_quartic_class_times`); rho's come from `measure_class_times`.
- Every class time is divided by Q's `F_p` multiplication time, so all arms
  are in one unit.
- The calibration runs alone, before the arms (A17).

**B7 (A11, A16). Secondary verdict.** `verdict_gae` uses the §5a band and
the calibrated GAE vectors only. The slope condition applies to the primary
(field-unit) verdict alone. `verdict_gae` stays labelled secondary and is not
predicted.

**B8 (A5, A9). Where the controls run.**

- Dense cross-check: every run with a kept system at `p ≤ 421`.
- BSGS check: Q and D seed `j = 0` at `k = 0`, every prime run.
- Altered certificate: at `12841` and at every stage-2 prime run. This is a
  superset of "the largest".
- Determinism check: the four Q seeds at `k = 0` for 229, 421 and 821.

**What stays.** Everything else in the original protocol and Amendment 1,
including the predictions G1 to G14, except as restated below.

**Predictions under Amendment 2.**

- G11 reads with B1's definition of detected.
- **G15** (new, B4): over all stage-1 runs of Q and D, `wrong_log` attempts
  occur in at most 5% of runs. P = 0.85.
- **G16** (new, B4): every certified run has the counted and the
  independent check agree. P = 0.99.

The 2026-11-16 lead review classed O4 as closed by obstruction: the
line-section factor base grows like `p` against rho's `√p`. This amendment
does not change that expectation (G2 to G7). It makes the confirmatory run
unambiguous.
