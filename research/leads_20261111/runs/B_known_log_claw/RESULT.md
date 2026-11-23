# LEAD-ICB-tableB-binary-known-log-claw: result (2026-11-12)

**BOUNDARY RESULT, NOT AN INDEX-CALCULUS IMPROVEMENT.** `ic.known_log_mitm2_v1` is a
BSGS-class time-memory table arm (a known-log base, a folded two-sum table of T entries, and
Q + [a]H queries). Under the owner ruling of 2026-11-08
(`research/curve25519_autolab_20261104/bsgs/README.md`), BSGS-class results are built and
reported as labelled boundary results outside the index-calculus ratchet. Every number below
is a boundary figure. None of them moves a Table B row, the IC leaderboard or `benchmarks/`.
All result statements are [pcc:unverified].

Lead text and preregistration: `research/leads_20261111/lane_B_binary_ic.md`, section 1. The
thresholds below are copied from that section and were not changed after the data came in.

## What was run

- Binary: `cargo build --release --bin ecbench` at HEAD `6b7fc4520e`, in the shared working
  tree. Another agent's uncommitted edit to `src/cryptanalysis/f4_fp.rs` was present; nothing
  under `src/cryptanalysis/ecbench/` or `ic_boundary` differed from HEAD. The binary used was a
  copy with sha256 `85318bc71e0b41071dae2ffabad0c502e6e407f418b25939afec6645b843f8ac`.
- Specs: `make_specs.py` writes `specs/*.json` (hashes are in `specs/SHA256SUMS`).
  - The curves are the five Table B random-binary instances as `binary_explicit`, taken
    verbatim from `docs/curves/registry.json`. ecbench reports all five as `registered=true`
    with the board's ec1 ids.
  - Table size per the lead: T near sqrt(r/2), with size s = round((r/2)^(1/4)) unsigned base
    points, so T is about s^2. `max_pair_work` = s(s+1). `max_trials` = min(r-1, ceil(50 r/(2s^2))).
  - Each main spec has three arms: `rho-neg` (`rho.negation`, the matched reference),
    `known-log` (base_seed 20261112) and `known-log-aa` (base_seed 20261113, the A/A control).
  - Each main spec has 16 public targets (`hash_to_subgroup_v1`, target_seed 2026111202),
    4 measured rounds and 1 warmup, alternating order.
- Commands: `run.sh`, reproduced exactly. In outline:
  - `ecbench run --spec specs/main-<c>.json --out sessions/main-<c> --cpus none --allow-busy --lock <private>`
  - `ecbench verify --dir sessions/<s> --replay-all --out audits/<s>.audit.json`
  - `ecbench compare --dir sessions/main-<c> --a rho-neg --b known-log --json`, and likewise for the A/A pair
  - `python3 analyse.py`, which writes `summary.json`
- Runs were single-threaded at L0 (shared, busy container, no CPU reservation). These are
  operation counts only. No wall-time figure is a result.

Sessions (`sessions/`; records sha256 are in the audit files):

| session | id | records | audit (`--replay-all`) |
|---|---|---|---|
| main-f2m15 | ECBS1ha08e1fd467d7 | 240, all verified | OK, 192 replays reproduced, 0 problems |
| main-f2m18 | ECBS1h8f1ac49b6b5d | 240, all verified | OK, 192/0 |
| main-f2m21 | ECBS1hd75c06521a06 | 240, all verified | OK, 192/0 |
| main-f2m24 | ECBS1he8167ec57648 | 240, all verified | OK, 192/0 |
| main-f2m27 | ECBS1hd6ffb26daf76 | 240, all verified | OK, 192/0 |
| control-positive-f2m15 (planted) | ECBS1hf2b51590d151 | 16, all verified, `matches_planted` true | OK, 8/0 |
| control-negative-f2m15 (max_trials = 1) | ECBS1hc1f3275493ec | 32, all `exhausted`, recovered = null | OK, 16/0 |

## Results [pcc:unverified] (BOUNDARY, BSGS-class)

S = counted GAE / sqrt(r), averaged over 64 measured (target, round) runs per arm. KL/rho is
`ecbench compare`'s ratio of totals, with a 95% interval over the 16 workloads (linearised t).
The w = 0.1 and w = 1 columns price every arm's `*_uncharged` counters at that weight (AGENTS.md
§5a). On the known-log arm these counters are table inserts, lookups, scalar additions and base
draws. On rho they are canonicalisations.

| curve | log2 r | s | T (table entries) | rho-neg S | known-log S | A/A S | KL/rho w=0 [95% CI] | w=0.1 | w=1 | w=1 vs w=0 | A/A / KL | board best (unknown-log) |
|---|--:|--:|--:|--:|--:|--:|---|--:|--:|--:|--:|--:|
| f2m15-t131 | 13.99 | 10 | 100 | 1.965 | 2.932 | 2.902 | 1.492 [1.29, 1.72] | 1.512 | 1.616 | +8.3% | 0.990 | 13.07x |
| f2m18-t393 | 15.00 | 11 | 121 | 1.717 | 2.780 | 2.651 | 1.619 [1.36, 1.93] | 1.654 | 1.832 | +13.1% | 0.954 | 10.62x |
| f2m21-t2591 | 20.00 | 27 | 729 | 1.158 | 2.014 | 1.985 | 1.740 [1.53, 1.98] | 1.766 | 1.891 | +8.7% | 0.985 | 17.99x |
| f2m24-tm3047 | 21.00 | 32 | 1024 | 1.183 | 2.086 | 2.081 | 1.763 [1.51, 2.06] | 1.798 | 1.964 | +11.4% | 0.997 | 23.03x |
| f2m27-t12543 | 24.41 | 58 | 3364 | 1.043 | 1.893 | 1.767 | 1.815 [1.53, 2.15] | 1.866 | 2.102 | **+15.8%** | 0.933 | 38.21x |

- The A/A arm (`known-log-aa`) against rho-neg reads 1.477, 1.544, 1.715, 1.759 and 1.694 at
  w = 0. Its w = 1 drift is +6.0%, +9.1%, +7.4%, +10.3% and +13.0%.
- Rho-neg is cheaper at every weight on every instance. The band order holds, so known-log is
  above rho-neg throughout. The distance to matched rho at w = 0 is 1.49x to 1.82x.
- Known-log phase split in S, from `summary.json`. Base generation is the known-log scalar
  multiplications. The table is pair construction.

  | curve | base generation | table | queries | internal recovery check |
  |---|--:|--:|--:|--:|
  | f2m15 | 1.20 | 0.86 | 0.72 | 0.14 |
  | f2m27 | 0.39 | 0.72 | 0.77 | 0.007 |

  At f2m27 the table and queries add up to 1.49, close to the derived floor 2*sqrt(1/2) = 1.414.
  The rest is base generation, which the lead's mechanism left out and which shrinks relatively
  as r grows.
- The ratio to rho rises with r (1.49 to 1.82) because rho-neg's S falls towards its floor
  0.886 faster than known-log's S falls towards 1.414. The asymptotic expectation is
  1.414 / 0.886 = 1.60 (derived, not measured).
- Memory: T entries, from 100 to 3,364, plus 2s signed base points. No bytes are charged, which
  matches the band rule.
- Against the board's unknown-log recipe on the same instances (13.07x down to 38.21x), the
  known-log arm is 7 to 21 times lower in ratio-to-rho. That comparison is across a scope
  boundary, from a BSGS-class table arm to the IC recipe, and is **not** an IC improvement.

## Verdict against the preregistered prediction and falsifiers

| preregistered item | outcome |
|---|---|
| Falsifier: any instance > 3.0x at w = 0 | not triggered (max 1.815) |
| Falsifier: any unverified logarithm | not triggered (1,200 main records verified, `matches_target` true; the harness checks [d]G = Q) |
| Falsifier: A/A arms differ by more than 10% | not triggered (point ratios 0.933 to 0.997; at f2m27 the gap is 6.7%, with a 95% CI of [0.83, 1.05]) |
| Falsifier: ratio < 1.4 that survives a counter audit | not triggered (min 1.477) |
| Prediction: ratio at w = 0 in [1.4, 2.4] on every instance | **held** on all five, on both seeds |
| Prediction: w = 0.1 and w = 1 within ±15% of w = 0 | w = 0.1 held everywhere (max +2.8%). At w = 1, 9 of 10 arm-instance cells held. **One missed: known-log on f2m27 is +15.8%.** The A/A seed on the same instance is +13.0%. The rho side had fewer unpriced counters than the "both sides double" assumption, so the band is wider than predicted. |
| Positive control: planted target recovered on f2m15 | passed (16/16 verified, `matches_planted` true) |
| Negative control: max_trials below T reports exhaustion, not a log | passed (32/32 `exhausted`, no recovered value). Interpretation: max_trials = 1, which is below T = 100. |
| Rank only where the order holds at all three weights | holds: rho-neg < known-log at w = 0, 0.1 and 1 on all five instances |

**Overall: HELD, as a labelled boundary result** [pcc:unverified]. No falsifier fired and the
main prediction held. One secondary sub-prediction, the ±15% band width at w = 1, was missed by
0.8 points on one arm-instance cell, and that miss is recorded. Status in the programme's terms:
this measures the generic table arm on the Table B binary instances against matched rho. It does
not count toward the IC ratchet and moves no Table B row (owner ruling 2026-11-08). The lead's
own risk (1), the scope ruling, is thereby settled as "reference measurement, not improvement".

## What is blocked or missing for the validation gate

- **L2 isolation and EKS.** These are L0 runs in a shared container (`--cpus none --allow-busy`).
  The default route (isolab-eks, AGENTS.md §9a) is blocked because this container's AWS identity
  is rejected. Operation counts do not need L2, but the gate's wall/A-A admission does.
- **Independent replay certificate.** `--replay-all` reproduced every deterministic run on this
  same host. The gate needs a receipt from a host of another class (`ecbench-independent-runner`).
- **VALIDATION.json, benchrec record and S3 raw data.** These need S3 (blocked). No
  `bench.record/v1` record was written, because `tools/benchrec.py` pushes to S3. No `benchmarks/`
  or leaderboard edit was made, as instructed. A boundary row would in any case belong in a
  BSGS-class boundary panel, not in IC tables A to C.
- **pcc claim.** None was filed, so every statement here stays [pcc:unverified].
- research-kb MCP and the sibling lead atlas (vault-encrypted) were not available or not
  touched, so nothing new was searched.

## Code changes

None. No source file was edited, and the default behaviour is unchanged trivially.
`git diff HEAD -- src/cryptanalysis/ecbench src/bin/ecbench.rs` is empty. Only files in this
directory were written.

## Next step

- If the boundary panel is wanted as evidence, re-run `specs/main-*.json` unchanged through
  isolab-eks at L2 with an independent replay, and file a pcc claim for the
  "1.49 to 1.82x matched rho-neg, w = 0" boundary figures.
- For the IC ratchet this lead is closed: the arm is a generic table. The board's binary rows
  stay at 10.6x to 38.2x until an unknown-log (relation-based) recipe improves.
- A cheap extra: base generation is 0.4 to 1.2 S of the known-log cost. Choosing s to minimise
  base + table + queries (rather than T = sqrt(r/2)) would lower boundary S slightly. That is a
  constant tweak to a generic arm, and the owner ruling leaves it outside the ratchet.
