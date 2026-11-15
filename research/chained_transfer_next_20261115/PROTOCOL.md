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
