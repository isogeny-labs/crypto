# PROTOCOL: does tuned double-large-prime beat the Koblitz compact-orbit baseline? (frozen before data)

Version 1, 2026-10-23. Branch `claude/ic-koblitz-dlp-tuning-20261023`. Written under
AGENTS.md ("Success criterion", "Implementation language: no Python", §§1–5a, §12) and the
`ecbench-measure` and `controlled-experiment` skills. Public synthetic Koblitz instances, operation
counts only, no key-recovery, sub-rho or deployed-curve claim.

## 1. Question and claim classes

**Question.** On the compact-orbit factor base, does the double-large-prime oracle
`mitm-frobenius:m=3,large_primes=2,lp_domain_factor=c`, with `(K, c)` tuned on a train panel,
lower the end-to-end cold one-target `S` below the current Koblitz baseline `ic-compact`
(`compact-orbit-scan:columns=Kb` + `mitm-frobenius-counted:m=2`, the Phase 1 frozen `Kb`) at
n = 37, 43 (primary) and n = 41, 47 (supplementary), at every weight of the AGENTS.md §5a band?

| sentence this study may publish | class | what establishes it |
|:--|:--|:--|
| "arm X has paired `S` ratio R to the baseline on curve C at weight w, 95% CI [lo, hi]" | comparative, statistical | this protocol: held-out targets, paired, A/A and positive control |
| "all runs verified; replays identical" | exact computation | `ecbench verify --replay-all` |
| "the integration note's 23.8x LP effect at n = 41 is detectable by this instrument" | positive control | section 7 |
| anything about n > 47, other curve families, wall time, or asymptotics | out of scope | not established |

**Boundary and reference (AGENTS.md §1).** Reference: matched Pollard rho with the signed
Frobenius fold (`rho.signed_frobenius`, arm `rho-frob`); the Phase 1 ratios of the baseline to it at
w = 0 are 5.2 (n = 37), 15.0 (n = 41), 7.4 (n = 43), 9.9 (n = 47)
(`research/ic_frontier_campaign_20261015/phase1`). `rho.signed_frobenius_strong` is run as a second
reference. Floor: the generic `√(π/2A)` ecbench records per run. The gap to rho-frob is reported
at each weight as the distance remaining, never as a pass/fail gate.

**Unit and priced work.** `S` = total GAE / `√r`, cold, every phase, ecbench's counted unit.
Per AGENTS.md §5a every arm, rho included, is also priced at w = 0, 0.1, 1 GAE per unpriced
native operation (`ecbench band`'s `priced_S(w)`); an order is a ranking only if it holds at all
three weights. Each arm's unpriced counters and counts are listed in the report.

## 2. Hypothesis, success and stop conditions (AGENTS.md §4)

- **Prior (stated before data):** a null. The 2026-10-12 best double-LP cell on this pipeline was
  17.7x the counted rho at n = 41 (`research/ic_techniques_20261012/double_large_prime`), against a
  baseline of 15.0x at that size. Dry runs (section 11) suggest that small `K` with a large `c` can
  be competitive at n = 43 and n = 47, so the null is not assumed.
- **Success (an improvement in the sense of the success criterion), per curve:** the selected
  double-LP arm `dlp-selected` has a held-out paired ratio `ΣS_dlp / ΣS_base-frozen` whose
  95% interval lies entirely below 1 at each of w = 0, 0.1, 1, with every run verified, both A/A
  ratios exactly 1.000, and `ecbench verify --replay-all` OK. The class is *engineering*
  (AGENTS.md §3), not an advance, unless the ratio to the floor falls with n; no exponent is claimed.
- **Established improvement** needs success on *both* primary curves (n = 37 and n = 43). Success on
  exactly one of the four curves is reported as suggestive and as one of four tests (at 95% a
  null produces one or more false positives among four independent curves with probability about 18%).
- **Equal-tuning clause.** If a re-tuned baseline arm `base-retuned` (section 5) exists on a curve and
  beats `base-frozen` there, success on that curve additionally requires beating `base-retuned`
  by the same rule; otherwise the double-LP arm has only been shown to beat an under-tuned baseline.
- **Regression** on a curve: the lower CI bound is above 1 at all three weights. **Unresolved
  inside the band**: anything else (a tie, or an order that changes inside the band).
- **Stop conditions.** The study ends when the holdout is run. A null, an unresolved result or a
  regression is reported with its numbers and its distance to rho. Total budget: 12 core-hours at
  one core; the run is stopped and reported partial if the train panel alone exceeds 6.

**Inadmissible (changing any of these after seeing results is a new, labelled protocol version):**
the arms, grids, targets, seeds, selection objective, weights, resample count, the verdict rule,
the baseline's `Kb`; choosing targets or seeds; dropping unverified runs from a ratio; combining
sessions with different binaries; wall time as a headline.

## 3. Instrument and isolation

- One binary for every session: `target/release/ecbench` built from this branch's base
  `origin/main` a53a7e2bfa20080a424b9a2484a16932b6e3c116 (SHA-256 recorded in each session, and
  `eaba5eb0db487c36752194d41e5d23c27ec2171cefc81b2e6d73ec811558ff57` at freeze). Paired analysis
  uses `examples/ic_paired_band.rs` (new, on this branch; it calls the repository's
  `band::priced_s` and `stats::cluster_bootstrap_ci`).
- Shared 4-core VM: `isolation_required: L0`, `RAYON_NUM_THREADS=1`, `taskset -c 3`,
  `--cpus none --allow-busy`, own lock file. Counts are deterministic given the seeds and stand at L0;
  no wall-clock claim is made. Wall times in records are practicality notes only.
- `ecbench compare --weight` is not in this binary (not on `origin/main`). Weighted `S` therefore
  comes from `ecbench band` (per-arm means) and from `ic_paired_band` (paired ratios with the same
  statistic as `ecbench compare`: ratio of sums over (workload, round) pairs, two-stage cluster
  bootstrap over workloads, 4000 resamples, seed 20261002). `ecbench compare` (recorded `S`,
  weight 0) is run as well and must agree with `ic_paired_band` at w = 0 on every ratio and interval.
- Task cards (`experiment-director` / `executor-mechanical`, AGENTS.md "Model tiers") are not used:
  this study runs inside a delegated subagent that cannot dispatch them. The commands are in
  section 9 and the reasoning-tier assessment is in `REPORT.md`.

## 4. Design

**Curves** (Koblitz, one per size: the repository's constructor finds a usable prime-order
subgroup, among n = 29..61 tried, only for K_0 at n = 31, 37, 41, 53, 59, 61 and K_1 at n = 29, 43, 47, 59, 61, so there is no
second curve at a size): n = 37 (K_0, r = 2^27.78), n = 41 (K_0, r = 2^39.00), n = 43 (K_1,
r = 2^32.11), n = 47 (K_1, r = 2^36.64).

**Train and holdout are separated by target.** The same four curves; public targets drawn by
`hash_to_subgroup_v1` from disjoint target seeds. Parameters are tuned per curve on train targets
and evaluated once on holdout targets. (No fresh curve exists at these sizes; this is the protocol's
main limit: the selection is tuned to the curve, not to the family, and a family claim is out of scope.)

| panel | target seed | algorithm seed | targets per curve | rounds |
|:--|--:|--:|--:|--:|
| train | 20261023001 | 20261023002 | 48 (n = 37), 32 (41), 32 (43), 24 (47) | 1 |
| holdout | 20261023101 | 20261023102 | 96 | 1 |
| positive control | 20261023201 | 20261023202 | 8 (n = 41) | 1 |

Warm-up 0 (counts are deterministic; there is no wall claim). The algorithm seed is one per
(workload, round) shared by every arm (`spec.rs`), so paired arms and A/A arms see the same seeds.

**Train grid.** Double-LP arm `dlp-k{K}-c{c}`: `factor_base = compact-orbit-scan:columns=K,raw_x_cap=1000000`,
`oracle = mitm-frobenius:m=3,large_primes=2,lp_domain_factor=c`, `max_trials = 50000000`, with
K in {1, 2, 3, 4, 6, 8, 12, 16}, c in {1, 2, 5, 10, 20, 40, 80, 160, 320}, restricted to K·c ≤ 640
(60 cells; the requested grid c in {2, 5, 10, 20} is contained). The grid is wider than asked
because the dry runs of section 11 put the minimum at the small-K, large-c edge. Baseline arms
`base-k{K}`: columns in {Kb/4, Kb/2, Kb, 2Kb, 4Kb}, oracle `mitm-frobenius-counted:m=2`
(unchanged from Phase 1), Kb = 8 (n = 37), 128 (41), 16 (43), 64 (47). Arm `rho-frob` on every
train spec.

## 5. Selection (frozen by hash before the holdout)

For each curve, over arms whose train runs all verified, the objective is

`minimax(arm) = max over w in {0, 0.1, 1} of  mean priced S(arm, w) / mean priced S(base-frozen, w)`

(`ic_paired_band --objective-base base-k{Kb}`; equal pair counts, so a ratio of means is the
paired ratio of sums). The selected double-LP cell is the dlp arm with the smallest `minimax`
(ties: lowest K, then lowest c); the selected `base-retuned` is the baseline cell with the smallest
`minimax` (equal to 1 for `base-frozen` itself; it is a separate arm only if it differs from Kb and
its `minimax` is below 1). One cell per curve, tuned for the whole band, not per weight.
**Edge rule:** if the selected dlp cell has c = 320 or K = 16 or K·c > 320, the grid is extended on
that side (c doubled with K·c ≤ 1280, K to 24 and 32), run on the *same train targets* and
the selection redone, at most twice, each time by a dated amendment to this file committed before the
holdout; a selection at c = 1 is extended to c = 0.5 and 0.25 the same way; K = 1 is a hard edge.
The result is written by `select.sh` to `selection.json`; its SHA-256 goes into `SELECTION.sha256`
and the holdout specs are generated from it by `make_specs.sh holdout`, then committed, **before**
any holdout session starts.

## 6. Holdout arms (per curve, same targets and algorithm seeds)

`rho-frob` (reference), `rho-strong` (second reference), `base-frozen` (ic-compact at Kb),
`dlp-selected`, `base-retuned` (only if section 5 makes one), and two A/A controls that repeat an
arm exactly: `base-frozen-aa` and `dlp-selected-aa`.

## 7. Controls

- **A/A (negative control, same instrument).** `base-frozen` vs `base-frozen-aa` and `dlp-selected`
  vs `dlp-selected-aa` must give ratio 1.000 (|R − 1| < 1e-9) with a degenerate interval at every
  weight on every curve. Counts are deterministic, so any other value is an instrument failure and
  voids the curve.
- **Positive control (detector power).** `specs/poscontrol-n41.json`: at fixed base (10 columns,
  n = 41) `dlp-k10-c10` against the same oracle with `large_primes=0` (`mitm-frobenius:m=3`), on the
  same code path. The integration note measured 5.25·10^7 vs 2.21·10^6 GAE (23.8x). The detector
  passes if the paired ratio dlp / plain at w = 0 has its 95% interval entirely below 0.2. If it
  fails, every "no improvement" is reported invalid, not null. Run first.
- **Replay.** `ecbench verify --replay-all` on every session (train, holdout, positive control).
  A failure is no result.

## 8. Decision rule and reporting

Per holdout curve and weight: the paired ratio `ΣS_dlp-selected / ΣS_base-frozen` with its 95%
cluster interval (`ic_paired_band`), and the same against `base-retuned` and, for context, the
ratios of both arms to `rho-frob` and `rho-strong` (the distance remaining). The verdict wording is
fixed by section 2: "improvement", "regression", "unresolved inside the band". Per-curve rows only,
never a pooled ratio across sizes (a pooled ratio is not a scaling claim). The report lists each
arm's unpriced counters with counts, flags any arm that names a counter without a count as a lower
bound that cannot be ranked, labels the class (AGENTS.md §3), and states what is not shown.
The train table (all 60 + 5 cells) is reported in full, with the winner's-curse gap between each
selected cell's train and holdout ratio.

**Power.** Resampling inference rather than a rank test; the smallest ratio resolved is set by the
across-target spread of the per-target ratio. With 96 holdout pairs per curve and the per-target
spreads seen in the dry runs (a factor of about 2 either way), a true ratio of 0.9 or below is
resolved; smaller true effects report "unresolved", and the interval widths are reported so the
reader sees the resolution.

## 9. Commands (frozen)

```text
cargo build --release --bin ecbench && cargo build --release --example ic_paired_band
research/ic_koblitz_dlp_tuning_20261023/make_specs.sh train            # specs/*.json (hashes in section 12)
export RAYON_NUM_THREADS=1
# positive control first, then train n=37, 41, 43, 47, then, after selection, holdout n=37, 43, 41, 47
taskset -c 3 target/release/ecbench run --wait --spec SPEC --out sessions/NAME --cpus none --allow-busy --lock LOCK --quiet
target/release/ecbench verify --dir sessions/NAME --replay-all --out audits/NAME.audit.json
target/release/ecbench band --dir sessions/NAME --weights 0,0.1,1 --json bands/NAME.band.json
target/release/examples/ic_paired_band --objective-base ARM --pair A:B --out OUT.json sessions/NAME
target/release/ecbench compare --dir sessions/NAME --a ARM_A --b ARM_B --save
```

## 10. Pre-flight runtime estimate (controlled-experiment §3)

Measured on this host on scratch dry runs (section 11), one core, solve wall per run: n = 37
cells 0.2–1.3 s, n = 43 0.01–1.7 s, n = 47 0.15–21.7 s (the largest cells are K = 24, c = 40 and
K = 16, c = 40; the product bound K·c ≤ 640 keeps the grid below that), `rho-frob` 0.006–0.015 s.
Estimate (sum over 65 cells per target, then times targets): train n = 37 about 0.5 h, n = 41
about 0.9 h, n = 43 about 0.5 h, n = 47 about 1.0 h (total about 3 core-hours); holdout (6 to 7 arms,
96 targets, four curves) about 1 core-hour; positive control and audits under 0.5 h. About 5
core-hours against the 12-hour cap. The actual run times go next to these in `REPORT.md`.

## 11. Dry runs and what they disclosed (scratch, not evidence)

Before this protocol was frozen, four scratch sessions (seeds 777001 to 777010, 1 to 2 targets,
10 to 16 cells, not committed as evidence; their band tables are in `dry_run/`) were run at
n = 31, 37, 43, 47 to time the instrument and test the specs. Three of them are on holdout curves
(n = 37, 43, 47), with seeds disjoint from section 4. They disclosed: (a) every cell verified and
the runs are seconds, so the grid can be wide and the holdout large; (b) the best double-LP cells were at the smallest K of each dry grid, and at
n = 47 the cells K = 2, c = 40 and K = 1, c = 80 (one target) were the best tried, which is why the grid
was widened to K = 1 and c up to 320, beyond c in {2, 5, 10, 20}; (c) at n = 47 one 2-target cell (K = 4, c = 10) was below the
baseline at w = 0 on those two targets, an observation of two targets that this protocol treats as a
reason not to assume the null, nothing more; (d) the baseline's ratio to `rho-frob` at w = 0 on the
dry targets was 5.7 (n = 37), 13.5 (n = 47), in line with Phase 1.

## 12. Frozen inputs

SHA-256 of the train and positive-control specs at freeze (`sha256sum specs/*.json`) are recorded
in `SPEC_HASHES.txt`, committed together with this file. This file's own hash is recorded in each
result file's header by `REPORT.md`.

## Amendment 1 (2026-10-23, before any train or holdout data)

**Finding.** The first positive-control session (`sessions/poscontrol-n41`, binary
`eaba5eb0…`, kept as evidence) verified 24 of 24 runs and gave the expected effect, but
`ecbench verify --replay-all` FAILED: every `dlp-k10-c10` record "differs in phase counts"
(8 of 8; `audits/poscontrol-n41.audit.json`). The cause is a wall-clock counter,
`lp_filter_wall_ns`, which `ic_framework::large_prime::relation_counters` wrote into the relations
phase's replayed `native` map. It is not priced (no `_uncharged` suffix, not in `total_gae`), so no
`S` changes; but a double-LP record cannot replay identically while it is there, and this protocol
requires `--replay-all` OK.

**Change.** One instrument edit, `src/cryptanalysis/ic_framework/large_prime.rs`: the collector no
longer emits `lp_filter_wall_ns` (the field stays on the collector). The binary is rebuilt from this
branch (`origin/main` a53a7e2bf + this edit + the `ic_paired_band` example) and its SHA-256 is
recorded in every session. All sessions are run with that one binary; the positive control is
re-run as `sessions/poscontrol-n41-r2`. The pre-fix session is not used for any inference except to
check that the fix changed no count: its per-record `total_gae` and every native counter other than
`lp_filter_wall_ns` must equal the r2 session's, record for record (same spec, seeds), reported in
`REPORT.md`. Nothing else in the protocol changes. `run_session.sh` (run, audit, band) is added to
the frozen commands.
