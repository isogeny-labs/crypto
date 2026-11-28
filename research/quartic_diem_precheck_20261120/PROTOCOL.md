# Pre-check for lead #1: linear-algebra scaling of Diem's line index calculus on prime-order plane quartics

Status: FROZEN before any data (sha256 in `frozen.sha256`). Every number this
study produces is `[pcc:unverified]`.

Parent: `research/hec_breakthrough_search_20261120/REPORT.md`, rank #1 and
"Recommended next step (referee)": "Cheapest decisive pre-check first: a single
p=32771 run to measure the LA scaling exponent - if LA already scales worse than
q^{1.4}, stop before the ladder."

## 1. Question

On generic smooth plane quartics over prime fields `F_p` whose `Pic^0` has PRIME
order (`h = 1`), how do (a) the counted linear-algebra work and (b) the total counted
cost of Diem's double-large-prime line index calculus scale with `p`, between
`p = 1031` and `p = 32771`? Is the lead's premise (an IC cost exponent clearly
below the rho exponent 1.5, theory ~1) visible in the lab's instrument, or does LA
already scale worse than `p^1.4`?

This is a pre-check of the scaling only. It does NOT compare against rho (see
section 7) and cannot produce a below-rho claim.

## 2. Instrument (fixed)

- Binary: `genus_ladder` built from `src/bin/genus_ladder.rs` at the commit that
  last changed the quartic path (`ca72267899`, 2026-11-18 16:44 UTC; no later commit
  touches `src/bin/genus_ladder.rs`, `src/cryptanalysis/quartic_ladder.rs`,
  `src/cryptanalysis/plane_quartic_pic.rs`, `src/cryptanalysis/chained_transfer/ic_control.rs`
  or `src/cryptanalysis/genus_ladder.rs`; HEAD `c03241fae8`, clean tree). The
  existing `target/release/genus_ladder` (built 2026-11-18 16:45) is copied, not
  rebuilt, to the session scratchpad as `genus_ladder.qdiem`; sha256
  `493ee2942473b8708a0b52f40ab10095c4da83ab0f0755ed3e5dbc02771b6921`, recorded in
  `binary.sha256`. No Rust source is modified.
- Curves: `quartic_ladder::make_instance(q, s, s, max_h)` — a random smooth plane
  quartic with a rational point from curve seed `s`, exact `#Pic^0` by
  baby-step giant-step (uncharged set-up), admitted iff `#Pic^0 = h r`, `r` prime,
  `h <= max_h`. With `--max-h 1` only PRIME-order Jacobians are admitted
  (`ic_control::split_order`). Curve seeds are scanned 1, 2, 3, ... per prime and the
  first `C` admitted are used (`--seed-search 600`).
- Arms (both are always run by the binary on each quartic):
  - `Q_diem_2lp` (PRIMARY, the lead's algorithm): factor base of
    `nb = ceil(qc sqrt(p))` random rational points, `qc = 3.0` (binary default, same as
    `genus_ladder_20261114`), lines through base pairs, up to two large primes, cycle
    construction on the large-prime graph, decomposition of `[a]G + [b]T` by conics
    (descent is charged inside the relation phase and its group operations).
  - `Q_diem_full` (secondary, reported only): factor base = all rational points.
- Linear algebra in both arms: build rows from the 2LP graph, peel weight-one
  columns, then SPARSE scalar Wiedemann on `A^T D A` with Berlekamp-Massey
  (`ic_control::wiedemann`; a dense projection on retry attempts), every `mod r`
  multiplication and addition counted into `la_mul`, `la_add`, plus DSU work into
  `la_add`. It is NOT block Lanczos; the lane's block Lanczos is not wired into this
  binary (disclosed; a block-Lanczos solver would change the constant, not the
  `N^2 w` shape of a Krylov solve).
- Rho is skipped (`--rho-cap 1000`, i.e. quartic cap 10 expected steps): this
  pre-check does not need it, and the quartic negation rho at `p >= 16411` is
  out of budget on this shared host.
- Hyperelliptic cells are suppressed by `--genera 2 --arms - --curve-seeds P=1,...`:
  one genus-2 seed is examined per prime (instance search only) and no hyperelliptic
  arm runs (`-` is not an arm letter). Those rows are ignored.

Command (one session, written to `raw/run.jsonl`):

```sh
genus_ladder.qdiem run --primes 1031,2053,4099,8209,16411,32771 --genera 2 --arms - \
  --curve-seeds 1031=1,2053=1,4099=1,8209=1,16411=1,32771=1 \
  --quartic --qc 3.0 --max-h 1 --curves C --seeds S --seed-search 600 \
  --rho-cap 1000 --threads 2 --out raw/run.jsonl
```

with `C = 4`, `S = 2` unless the budget rule (section 5) lowers them.

## 3. Metrics (counted units only; wall time is recorded and never used)

Per run of an arm (one curve, one solver seed):

- **LA-count** (primary for the LA rule): `la_mul` (multiplications modulo `r`).
  Sensitivity: `la_mul + la_add`.
- **Total counted cost** (primary for the total rule): FPM mid =
  `cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul` (the `genus_ladder_20261114`
  primary reading, `I = 10`, `kappa = g^2 = 9`). Sensitivity: FPM low
  (`I = 1, kappa = 1`), FPM high (`I = 30, kappa = 9`), and
  `GAE_w = group_ops + w (rel_mul + la_mul)` at `w in {0, 0.1, 1}`.
- Descriptors: `fb_cols`, `core_rows`, `core_cols`, `relations`, `attempts`,
  LA share of FPM mid.

Only runs with `certified = true` and `ok = true` enter the fit. An uncertified or
exhausted run is reported and counted; if any prime has fewer than 2 certified
curves for the primary arm, that prime is dropped from the fit and disclosed.

## 4. Fit and intervals (fixed before data)

- Unit of analysis: the curve. For each (prime, curve) take the geometric mean of
  the metric over its solver seeds.
- Point estimate: OLS slope of `ln(metric)` on `ln p` over all curve-level points
  (primary arm `Q_diem_2lp`).
- Interval (primary): 95% percentile interval from 4000 cluster-bootstrap
  replicates that resample curves with replacement within each prime (numpy
  `default_rng(20261120)`).
- Interval (secondary, reported): OLS t-interval on the per-prime means of
  `ln(metric)` (df = #primes - 2).
- Also reported: local slopes between consecutive primes and between the top two
  primes (16411 -> 32771), the LA share and its trend.

## 5. Budget rule (fixed before data)

Total compute must stay under about 90 minutes on 2 threads (a genus-4 search
owns the host's other cores). A PILOT runs first: the same command with
`--primes 1031,32771 --curves 1 --seeds 1` into `raw/pilot.jsonl`. Pilot rows are
used ONLY to estimate wall time (search and per-run) and are NOT in the fit.
From the pilot's wall time per admitted curve search and per run at 32771, the
grid is estimated assuming cost ~ p^1.5 below 32771; the first of these that is
estimated under 70 minutes is run:

1. `C = 4, S = 2`, all six primes;
2. `C = 4, S = 1`;
3. `C = 3, S = 1`;
4. `C = 3, S = 1`, primes 1031 ... 16411 only (32771 infeasible; disclosed, and the
   verdict then reads "inconclusive at 32771" if the rules are not already met).

If the session is cut at the budget, complete primes are analysed and the missing
cells listed; nothing is rerun with other parameters under this protocol.

## 6. Frozen kill rule

Lead #1 is STOPPED if EITHER

- (K1) the point estimate of the LA-count (`la_mul`) exponent of `Q_diem_2lp`
  exceeds 1.40, OR
- (K2) the upper end of the primary 95% interval of the total-cost (FPM mid)
  exponent of `Q_diem_2lp` is >= 1.40.

Otherwise the verdict is PROCEED-TO-LADDER (the full destination measurement with
the L10-corrected matched rho), which is NOT a success claim. The sensitivity
readings (la_mul + la_add, FPM low/high, GAE band) are reported; if any of them
would flip the verdict, that is stated as a caveat, but the verdict follows K1/K2 on
the primary metrics.

## 7. What this does not do

- No rho comparison: the L10 correction (lab quartic negation-walk rho priced
  ~1.73x high; `<rho>` class walk ~0.447 of plain negation;
  `research/g2plus_leads_20261116/L10_c34_picard_matched_rho/ASSESSMENT.md`,
  `problems/quartic-matched-rho-overpricing.md`) is not applied because no rho
  is measured; any IC/rho statement is out of scope.
- No validation gate (no replay certificate, no VALIDATION.json): pre-check only;
  no leaderboard or `benchmarks/` row.
- Small prime range (1031 ... 32771, a factor ~32): fitted slopes are local and
  can be bent by set-up terms (`genus_ladder_20261114` found the quartic 2LP
  slope 0.69 at small `p`, set-up dominated).
- Wall time is not a metric (shared host, other jobs running).
