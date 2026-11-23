# experiment.protocol/v1 (preregistered, NOT yet run) — lazy orbit IC column sweep vs matched rho

Lead: research/ecbench_lazy_orbit_members_20261018 ("fewer columns under the lazy oracle").
Author: parallel-sweep investigator session, 2026-11-16. Frozen before any gated data.
Class: **engineering** (a constant/factor-base size moves; algorithm + exponent unchanged).
Status: design only. To be RUN ON ISOLAB (native L2 + replay + A/A admission; perf-ipc host for w_cal).
Nothing here is a result; all prior numbers quoted are [pcc:unverified] re-derivations from committed records.

## 1. Question
Can the lazy orbit-member oracle, at a tuned (smaller) factor-base column count `c`,
drive the n=31 (and n=17, n=29) index-calculus arm **defensibly** below its matched
strong-rho reference — i.e. survive BOTH (a) a paired CI that excludes 1 and
(b) the AGENTS.md §5a pricing band, order-preserving at every weight?

Motivation from the committed c=2 n=31 data (re-derived here, [pcc:unverified]):
- lazy/rho-strong = 0.985, cluster-bootstrap 95% CI [0.908, 1.062] — **includes 1**.
- §5a band (uncharged native op priced 0/0.1/1 GAE, applied identically to both arms):
  lazy/rho-strong = 0.985 (w=0) -> 1.028 (w=0.1) -> 1.286 (w=1). **Order flips inside the band.**
  Driver: lazy IC carries 1411 uncharged native ops/run (all in `factor_base`: point lifts,
  `lift_field_squares`, `min_key_field_squares`) vs rho-strong's ~749; the lazy oracle
  change is entirely in `oracle_setup` and does nothing to this burden.
- Fewer columns shrinks `factor_base` (and its 1411 uncharged ops) but raises trial/relation
  count. The tradeoff — smaller band penalty vs larger w=0 base cost — is the empirical unknown.

## 2. Arms (per curve, all `*_full` methods; only factor_base `columns=` and oracle id vary)
For each curve n in {17 (a=1), 29 (a=1), 31 (a=0)}:
- `rho-frob`          : rho.signed_frobenius_full                (reference)
- `rho-frob-aa`       : rho.signed_frobenius_full                (A/A control, derived seed)
- `rho-strong`        : rho.signed_frobenius_strong_full         (**matched-rho parity target**)
- for c in {1, 2, 3}:
  - `ic-orbit-c<c>-renew`      : ic.pipeline_full, factor_base=compact-orbit-scan:columns=<c>,
                                  oracle=mitm-frobenius-counted:m=<c>, targets=stride-renew   (baseline)
  - `ic-orbit-c<c>-renew-lazy` : same, oracle=mitm-frobenius-lazy-counted:m=<c>               (candidate)
Rationale: n=31's committed cell is already c=2, so "fewer columns" there means c=1; n=17/n=29
committed baselines are c=1, so the sweep also probes c=2,3 upward. The full {1,2,3} x {eager,lazy}
cross isolates the column effect on BOTH the w=0 cost and the uncharged-op footprint.

## 3. Counted unit and pricing (mandatory, the part the lead omitted)
- Primary counted unit: `ecbench.gae` (group-addition equivalents), paired by target.
- **§5a band (required):** enumerate every uncharged native counter for EVERY arm
  (rho: canonicalisations/partition_hashes/table_inserts/table_queries; IC: all `*_uncharged`
  counters, field squares/mults included; exclude byte-floor counters). Price at
  w ∈ {0, 0.1, 1} GAE/op, applied identically to all arms. Report the arm order at each weight.
- **Calibrated weight (2026-11-12 rule):** measure `w_cal` = (native mulmod time)/(group-add time)
  on the isolab host with `perf-ipc` on an isolated PMU host; report order at w ∈ {0, w_cal, 10·w_cal}.
  (Cannot be measured in the current dev box — no PMU; this is a core reason the run must be gated.)
- Report each arm's cost with interval across the band; never compare calibrated vs uncalibrated sides.

## 4. Falsification criteria (decision rules, frozen)
- **H2 (engineering win over IC baseline, per curve, per c):** lazy cheaper than eager iff the
  paired `eager/lazy` cluster-bootstrap 95% CI lower bound > 1 at w=0. (Re-derived prior: holds at
  c=2 n=31, ratio 1.240, and stays >1 across the whole band — direction-robust. This is the claim
  that actually stands; it needs no rho comparison.)
- **H3 (defensible parity/sub-rho, per curve, the lead's target):** candidate `lazy` at the chosen c
  is "defensibly below matched strong rho" iff ALL of:
  (i) paired `lazy / rho-strong` 95% CI excludes 1;
  (ii) order `lazy < rho-strong` holds at EVERY band weight {0, 0.1, 1};
  (iii) order holds at {0, w_cal, 10·w_cal}.
  If the order flips at any band weight -> report **unresolved at that boundary**, no parity claim
  (§5a). Failing (i) -> null. This is the criterion the committed c=2 n=31 cell FAILS on (i) and (ii).
- **H1 (same algorithm) gate:** per paired target, trials/relations/matrix_rank/recovered-log and
  `pair_table_entries` equal across eager and lazy; `rho-frob-aa/rho-frob` CI contains 1. Any failure
  voids the curve. (Re-derived prior: 0/640 H1 mismatches, A/A exact.)

## 5. Power / sizing
Committed cells used 32 targets x 3 rounds (96 pairs) and the rho-variance-dominated CI was
[0.908, 1.062] (half-width ~0.077). To make H3(i) decidable, size to **64 public targets x 3 rounds**
per cell (≈ halve CI half-width to ~0.055) ; n=37/41-style reductions not needed (n≤31 here).

## 6. Seeds and order (frozen, distinct from the 2026-10-18 claim)
- Fresh public target seeds, one per curve, NOT reused from the claim (claim used 2026101810xx):
  n17 target_seed=202611160017, n29=202611160029, n31=202611160031; target_kind=public.
- measurement.seed = 202611162000 + n; order=alternate (interleaved arms); warmup=1; rounds=3.
- A/A arm uses a derived seed per the harness A/A contract; isolation_required=L2.

## 7. What cannot be claimed regardless of outcome
No exponent change, no statement about ECC2K-130 or deployed curves. A positive H3 is a per-curve,
per-host, fully-priced parity/sub-rho *distance* result on cold public Koblitz targets, never a
general "IC beats rho". Prices remain one host's microbenchmark band + its measured w_cal.

## 8. Expected outcome (pre-registered honest prior, [pcc:unverified], NOT a result)
Most likely: fewer columns reduces the factor_base uncharged op count (helping the band at w≥0.1)
but raises the w=0 base cost (hurting H3(i)); a column count that satisfies H3 (i)+(ii)+(iii)
simultaneously at n=31 is judged UNLIKELY, because the band obstruction is structural (IC does ~2x
rho's uncharged native work) rather than a tuning artifact. The run's main value is (a) a measured
w_cal that the crude band cannot substitute for, and (b) a tightened CI that converts "0.985 [±]"
into a decided null-or-win. Worth gated time only as a bounded, calibrated measurement — not as an
expected rho-beating headline.
