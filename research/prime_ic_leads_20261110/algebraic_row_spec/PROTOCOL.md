# DRAFT protocol: first counted prime-field algebraic-track row (pkm-semaev, isogeny tower)

**DRAFT. Not frozen. No data has been taken.** Machine-readable form:
[`protocol.draft.json`](protocol.draft.json) (`experiment.protocol/v1`, `freeze.status = "draft"`);
`experiment_protocol.py validate` reports OK, 0 findings
([`protocol_validate.out`](protocol_validate.out)). Fields marked `FILL-AT-FREEZE` (instance type, build
commit, binary and image digests, analysis script hash) are set, and the record frozen, before the
first execution. Labels: *measured* (path, `[pcc:unverified]`), *derived* (arithmetic in README
section 6), *plan*. The instrument is [`spec.json`](spec.json); this record names what it is for and
what counts.

## 1. Question

What does `pkm-semaev` on `prime-isogeny:t=T` cost on the Table B curves fp16, fp18, fp20, cold whole `[pcc:unverified]`
pipeline, in `S = GAE/sqrt(r)` across the AGENTS.md 5a band at w = 0, 0.1, 1, against matched
`rho.negation` and the generic-track incumbent on the same targets? This establishes the algebraic
track's own baseline (DECISIONS.md R1 Option C, D1, D2, D-EKS) and reports its distance to rho. It
tests no new method and licenses no improvement, parity, scaling or novelty claim.

## 2. Hypotheses and pre-stated readings

Written 2026-11-11, before any data from this spec. Data seen is listed per prediction; both
anchors are the k = 20 miniature (different curve from every Table B curve), so the predictions are
out of sample for the three curves.

**H1, distance.** On every curve the cheapest `iso` arm costs at least 1e4 times `rho-neg` at
w = 0.1. Mechanism: F4 multiply-adds per trial are 10^6 to 10^8 at t = 4..6 (Q-D table, measured
[pcc:unverified], `research/curve25519_algebraic_ladder_20261102/README.md`) against about one GAE of
walking per trial; at w = 0.1 a multiply-add is a tenth of a group addition. Anchor measured: k = 20
isogeny t = 6, S = 1.9e8 at w = 0.1 against rho 1.84 (e2e, 2 targets).
- *Reading if the lower 95% bound is above 1e4 on every curve*: recorded as the track's baseline
  distance; no further consequence.
- *If a curve falls below 1e4*: the F4-per-trial cost on that curve is below the anchors; the cost
  model of README section 6 is withdrawn and re-fitted from this row's records before any larger
  curve is planned. No claim.
- *If any paired ratio is below 1*: a parity candidate. It must pass the validation gate in full
  (replay certificate, independent validator, matched rho across the band) and a second host
  class before it is called anything; it is not announced from this row.

**H2, t.** Within t = 4..6 the cost at w = 0.1 rises with t: `iso-t6 / iso-t4` paired ratio of summed `[pcc:unverified]`
S above 5 (lower 95% bound) on every curve. Mechanism: hit probability per trial is about `2c^2/N`,
relations needed about `0.7c`, so trials fall as `1/c` while `c` grows 1.4x to 2x per step; F4 work per
trial grows about 7.5x (t = 4 to 5) and 14x (t = 5 to 6) in the Q-D table (derived).
- *If true*: the `2^t ~ N^(1/3)` rule is not cost-optimal here; the track's best configuration on
  these curves is the smallest `t` that verifies; the stretch cells (section 8) stay unrun.
- *If not (ratio below 1 on some curve)*: larger bases pay; the t = 7 stretch cells become the
  primary cells of an amendment, and the `2^t ~ N^(1/3)` rule is the planning rule again.

**H3, w = 0.** `iso-t6 / matched inc` ratio of summed S at w = 0 within [2, 10] on fp20. Role:
calibration, not decisive. Anchor measured: 42.4 / 7.92 = 5.4 at k = 20.
- *If outside*: the one-GAE-per-trial walking model (README section 6, step 3) is off by more than
  2x, which moves every `S(0)` entry of the model, not the band-priced ones.

**H4, exactness.** F4 and grid arms emit identical streams (`targets_tried`, `relations_found`,
`matrix_rank` equal for every (target, t)), `pkm_crosscheck_disagreements` is 0.
- *If violated*: every `iso-*` record of the session is void, a defect report replaces the row.

What none of H1-H4 can show: parity, scaling, an exponent, behaviour at m = 3 (section 7 gate),
behaviour on any other curve, or anything about the Kummer form (unavailable on these curves).

## 3. Arms

As in [`README.md`](README.md) section 3 and `spec.json`: `rho-neg` (reference), `rho-neg-aa`
(A/A), `inc-n20/n32/n51` (baseline; generic track; the arm matching the curve is its incumbent),
`iso-t4/t5/t6-m2` (candidates; algebraic track), `iso-t5-m2-aa` (A/A), `iso-t{4,5,6}-m2-grid`
(`engine=exhaustive`), `iso-t{4,5}-m2-xchk` (`crosscheck=1`). One ecbench session, interleaved
(`order: alternate`), one thread, one binary, one set of targets; algorithm seeds shared across arms.
The Table B recipe is re-measured here, not copied: its S need not equal the leaderboard's
(cross-track review note in `reference_arm/PROTOCOL.md`); the comparison is inside the session.

## 4. Panels, targets, seeds

Curves: `prime_roster` bits 16, 18, 20 (README section 2). 8 hashed public targets per curve
(`hash_to_subgroup_v1`), indices 0-7. `target_seed` 1504454584 and `measurement.seed` 976050496 are
fixed by a stated rule (first 8 bytes of `sha256("prime-ic-algebraic-row-20261111|<name>")` modulo
2^31) before data; the protocol records them as an explicit list. The auxiliary-curve search seed
is 0 (the default; a seed is not a design choice).

## 5. Pricing band

For every record, `priced_gae(w) = G - C + w (U + n_C)`, `S(w) = priced_gae(w)/sqrt(r)`
(`src/cryptanalysis/ecbench/band.rs`), w in {0, 0.1, 1}, default (post 2026-11-08) reading with table
work priced. U includes, for the F4 arms, the F4 multiply-adds and the `pkm_*`, `sqrt`, `row_ops`,
`legendre`, `target_guard`, `velu` counters; for rho its canonicalisations; for inc its counted
inserts and listed counters. Rho is not exempt. An arm that names an unpriced counter without
its count is a lower bound and is not ranked. Byte counters are listed, not charged. The grid
arms' counter is a different unit from F4's, so grid arms are read descriptively and never ranked
against F4 arms as if the units matched.

## 6. Controls

- **A/A**: `rho-neg-aa/rho-neg` and `iso-t5-m2-aa/iso-t5-m2`, counted ratio exactly 1.000 (the counts `[pcc:unverified]`
  are deterministic; a ratio other than 1 is a defect).
- **Negative control**: `iso-t*-m2-grid` (identical streams; the null population is the non-
  decomposing trial points, more than 1000 per execution, derived in README section 6) and
  `iso-t{4,5}-m2-xchk` (zero disagreements).
- **Exactness**: `ecbench verify --replay-all` identical in the job and on an independent host class.
- **Positive control: none.** ecbench has no way to plant `R = P_i +- P_j` into an `ic.pipeline` trial and no
  `oracle-check` for `pkm-semaev`; the draft says so in `controls.positive` rather than claiming one.
  The row is admitted without it and reports that. A public target's recovery (`[d]G = Q`) is the
  end-to-end exactness evidence; it cannot show that a missed decomposition would have been seen.
  (The unit test in `methods.rs` that `pkm-semaev` solves a planted scalar end to end is code
  reading, not a run here.)

## 7. Budget and stop rules

Derived expectation 4.2 h single-thread for the run, 12.7 h with both replays, 25.4 pod
vCPU-hours (README section 6). Stop rules:
1. **Fixed design**: 336 executions; stop when each has a record.
2. **Ceiling**: 46 pod vCPU-hours (Job 1 deadline 54,000 s, Job 2 28,800 s, 2 vCPU each), inside the
   approved D-EKS allowance of about 49 CPU-hours; USD 10 for builds, idle and retries (plan).
   On a deadline the session is kept as interrupted evidence, unfinished executions are reported
   censored, and nothing re-runs with a larger limit. An infrastructure (Spot) loss allows one retry
   from scratch, as a new Job under the same frozen request in a new name recorded as a retry;
   counted values stay valid, wall time does not.
3. **Per-execution timeout** 7,200 s: censored, recorded, never re-run longer.
4. **m = 3 gate** for `spec-m3.json`: preflight 2 targets of `iso-t4-m3-chain`; if F4 multiply-adds
   per target exceed 3e11 (about 400 s at 7.5e8/s, derived rate), do not run it and state the m = 3
   cell unmeasured. The preflight is not data and is not reported as a row.
5. **Invalidation**: a C1 mismatch, a run that does not verify, or a non-identical replay voids every
   `iso-*` record of the session.

## 8. Stretch cells and amendments (not in this freeze)

Only by a written amendment before their data: `iso-t7-m2` on fp18 (about 23 min per target, derived)
and fp20 (about 1.5 h per target, derived: `T = 5,638`, 7.46e8 per trial, 7.5e8/s), 4 targets each,
if H2 fails; hybrid guessing `guess=G`; other memberships; larger curves. None is run, tuned or
promoted on the strength of this row's results without an amendment; amendments carry a
timestamp earlier than their data.

## 9. Decision rules

**Admission** (all must hold, else the verdict is `not_established` and nothing below is a result): `[pcc:unverified]`
every `iso-*` run verifies; H4 holds; replay identical in the job and on the independent host class;
both A/A counted ratios exactly 1.000.

**C2, distance remaining.** Per curve and weight: `sum S(cheapest iso arm) / sum S(rho-neg)` over `[pcc:unverified]`
matched targets, 95% cluster-bootstrap interval over the 8 targets, 4000 resamples. The cheapest
arm is the one with the smallest `sum S` at w = 0.1 (fixed by this rule; all arms are reported). It is
a distance (owner criterion 2026-10-15), not a pass/fail gate, and no threshold is attached.
**C3, context**: `ecbench band --ranks` order of that arm against the matching `inc-n*` arm:
cheaper at every weight, or unresolved inside the band. Tracks may be ranked on the same instances
(AGENTS.md "Tracks of index calculus"); neither ratchets the other. **C4, context**: F4 over grid S
per weight, the cost an algebraic engine has to beat on the same system.
Multiplicity: C2, C3, C4 and the `iso-t6/iso-t4` comparison form one family, Holm-corrected;
the three weights are read jointly as an order across the band, not as three tests.

## 10. Inadmissible

Changing curves, t, targets, seeds, unit, band or accounting after seeing data; quoting S(0) alone `[pcc:unverified]`
for an arm with unpriced solver work; calling any outcome an improvement or regression of the generic
track, or the reverse; pricing F4 by wall time inside S; choosing the reported arm per curve by any
rule other than section 9; dropping censored executions; running `spec-m3.json` after its gate failed;
re-running with a longer timeout; calling the result parity, a new algorithm or a security statement.

## 11. Analysis, export and freeze

`analysis.sh` (to be written and hashed before freeze): `ecbench compare --weight W`, `ecbench band
--ranks`, `ecbench bound fit` (secondary only, three sizes), `experiment_protocol.py blind` with labels
masked until the decision table is written. Then `from-ecbench` and `check-export --verify-files`,
`research-visuals` outputs (S per arm per curve and weight with intervals, F4 multiply-adds per trial
against t, distance to rho against log2 r), the benchmark record and `VALIDATION.json` of README
section 10. Freeze: `experiment_protocol.py freeze protocol.json --commit auto
--filed-before-affected-data`.
