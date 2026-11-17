# Protocol: Frobenius-folded k-sum against the lazy orbit-member incumbent (2026-11-15)

Frozen before any study session. Class: **engineering** (one oracle replaced by another on the same
Frobenius-closed factor base; no exponent, ECC2K-130 or deployed-curve claim). Every number in this
directory is `[pcc:unverified]` until a pcc claim exists for it.

## 1. Question

Commit `71d56d2af0` added `ksum-frobenius:m=3|4|5`. Its sizing pilot
(`research/koblitz_ksum_fold_20261115/`) beat the K1 recipe (`koblitz-standard-subspace` +
`pdp3-koblitz:m=3,engine=rootfind-negfold`) on K1 n23, but the K1 recipe is not the strongest IC arm
on these curves. The strongest existing IC arm is the Frobenius-orbit pair table with the lazy member
index (`mitm-frobenius-lazy-counted:m=2` on `compact-orbit-scan`, `targets=stride-renew`;
`research/ecbench_lazy_orbit_members_20261018/`, `research/ecbench_lazy_columns2_20261018/`). Does the
folded k-sum beat **that** incumbent, and the K1 recipe, on the same targets in the same session?

## 2. Honest framing

With one orbit column and m = 3 this is a Frobenius-folded *generic* (square-root class) collision
method: a table of the signed 3-sums of a base of size F has about F^3/(6 * 2n) entries and a probe
hits with probability proportional to that over r. It has no subexponential component and cannot beat
rho asymptotically. The question is a constant at two small sizes. A win here is a measured
improvement over the current best IC arm on these curves, nothing more. Gap to the matched rho is
reported as distance remaining.

## 3. Curves

K1 (Koblitz a = 1): n = 23 `icv1-f2m23-tm5197-1f85e9e1` (log2 r 22.00) and n = 29
`icv1-f2m29-tm40309-30c52b96` (log2 r 15.37), checked with `ecbench plan`. (The lazy studies used a = 0
at n = 23, `t5197`; their frozen column count does not apply to a = 1, so the incumbent's columns
are sized below.)

## 4. Arms (identical set on both curves, specs from `make_specs.py`)

| arm | method | role |
|:--|:--|:--|
| `rho-strong` | `rho.signed_frobenius_strong` | reference |
| `rho-strong-aa` | the same method, second copy | A/A control |
| `rho-neg` | `rho.negation` | context |
| `lazy-c<c>` | `ic.pipeline`, `compact-orbit-scan:columns=c,raw_x_cap=1e8`, `mitm-frobenius-lazy-counted:m=2`, `targets=stride-renew` | incumbents: n23 c = 3, 4; n29 c = 1, 2 |
| `k1-l<d>` | `ic.pipeline`, `koblitz-standard-subspace:dimension=d`, `pdp3-koblitz:m=3,engine=rootfind-negfold`, walk | K1 recipe (context): n23 d = 9; n29 d = 10 |
| `kf3-c1g-sr` | `ic.pipeline`, `compact-orbit-scan:columns=1,raw_x_cap=1e8,generator_orbit=1`, `ksum-frobenius:m=3`, `stride-renew` | **candidate (registered)** |
| `kf3-c1g-w` | the same with `targets=walk` | secondary candidate (no verdict) |
| `rho-strong-full`, `lazy-c<c*>-full`, `kf3-c1g-sr-full` | `rho.signed_frobenius_strong_full`, `ic.pipeline_full` with the arms above (c* = 4 at n23, 1 at n29) | secondary: the lazy studies' own pricing |

**Primary pricing.** Every arm of the primary tests is a plain method (`ic.pipeline`, `rho.*`), so
each one's unpriced native work (field operations, canonicalisations, hash probes, table inserts,
Frobenius maps) is charged by the same rule: `ecbench compare --weight w` and `ecbench band` at
w = 0, 0.1, 1 GAE per unpriced native operation, table work priced (the default since 2026-11-08;
AGENTS.md section 5a). The lazy studies used the `*_full` methods (one host's microbenchmark prices from
`full_prices.json`) with `rho-frob` as the reference. That is a different instrument; it is reported
as the secondary read-out on the three `*_full` arms (`ecbench compare` at the recorded S), and
is not the test. The two readings are never mixed in one ratio.

## 5. Sizing pilot (disclosed; not study data)

Run with `BENCHREC_SKIP="sizing pilot, not data"`, new target seeds 20261115901-907, 16 public targets
(8 for the last), 2 rounds, local host, scratch only (sizing specs: `make_sizing.py`). It fixed:
incumbent columns (n23: lazy c = 1..6 measured, S(w=0) 4.83, 1.84, 1.25, 0.976, 1.010, 1.084, so c = 3, 4 kept;
n29: c = 1, 2 measured 1.55, 2.36, kept), K1 dimension (n23: 8, 9, 10 -> 9; n29: 8..12 -> 10, S(w=0) about 3158 with
`max_trials` 2e7, lower caps do not change the verified runs), and the candidate (m = 3 and 4, columns 1
and 2, with and without `generator_orbit`, walk and stride-renew: m = 3, columns = 1, `generator_orbit=1` and
`stride-renew` was best on both curves; `kf3-c1g-w` kept as the walk variant). Sizing S values are
tuning information on those seeds only.

## 6. Design

3 sessions (replicates) per curve, `target_seed` 20261115101-103 (n23) and 20261115201-203 (n29),
48 public targets, 3 measured rounds + 1 warm-up, `order = alternate` (3 measured rounds cannot be
balanced; S is a deterministic operation count, so arm order cannot move it; wall times are
descriptive only), measurement seed = target seed + 1000, `--cpus none --allow-busy`, L0, local host
(deviation from the isolab-eks default, as the sibling lazy studies; owner-authorised local route for
this agent task is assumed, disclosed). `ecbench verify --replay-all --exit-code` after every session.
Binary: built from the repository head with `cargo build --release --locked --bin ecbench`; commit and
sha256 in `registered-at.txt`.

## 7. Registered tests

Notation: `R_w(X vs Y) = cost_w(X) / cost_w(Y)`, paired by target (cluster bootstrap, 4000 resamples,
seed 20261002), via `ecbench compare --dir ... --a Y --b X --weight w --resamples 4000`; cost is the
total cost over the pooled pairs.

- **H0 (validity).** All records verified, all replays reproduce, `lower_bound = false`, and
  `R_w(rho-strong-aa vs rho-strong)` contains 1 at each w pooled per curve (else the curve is
  reported unreliable and no verdict is made).
- **H1 (candidate vs the strictest incumbent).** Per curve, per w in {0, 0.1, 1}: the strictest
  incumbent is whichever of the two registered `lazy-c` arms has the smaller pooled cost at that w.
  H1 **holds on that row** iff (a) the upper 95 % end of `R_w(kf3-c1g-sr vs strictest lazy)` pooled over
  the 3 replicates is < 1, (b) the same upper end is < 1 in each of the 3 replicates, and (c) the point
  estimate of R is below the lower 95 % end of the pooled A/A ratio (beyond noise). Otherwise it does
  not hold; a lower end > 1 is a regression. The unit of conclusion is the row (curve x w); no
  pooling across curves.
- **H2 (K1 recipe).** `R_w(kf3-c1g-sr vs k1)` reported per row with CI (context, no gate).
- **H3 (distance to rho).** `R_w(arm vs rho-strong)` for kf3-c1g-sr, the incumbents and K1 reported
  per row, and the `ecbench band --ranks` ordering against rho (a pair is ranked only where
  the order holds across the band). Distance remaining, not a gate.
- **H4 (secondary pricing).** The same H1 read on the `*_full` arms (`kf3-c1g-sr-full` vs
  `lazy-c*-full`, recorded S) pooled and per replicate; `kf3-c1g-w` vs the same lazy arm for
  reference. No verdict is transferred between pricings.

## 8. What cannot be claimed

No asymptotic statement; no claim on any curve other than these two; n = 23 and 29 are tiny and rho's
fixed set-up (lanes, tables) is a large share of its S, so the "gap to rho" is partly a set-up
artefact. The incumbent columns and the candidate configuration were chosen on sizing seeds
(disjoint from the study seeds); selecting the strictest incumbent per w on study data is
conservative (it favours the incumbent). Nothing here enters the leaderboard or `benchmarks/README.md`:
no `VALIDATION.json` (replay certificate, validator report, matched rho with the 5a band)
exists for this directory.
