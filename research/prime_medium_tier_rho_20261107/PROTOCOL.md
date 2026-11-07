# Prime medium-tier rho reference bound: preregistered protocol

**Status: draft, frozen by the main session before launch** (`protocol.json`
`freeze` block; `experiment_protocol.py freeze --filed-before-affected-data`).
Written 2026-11-07, before any measured session of this study existed. Results
go into `README.md`, `sessions/` and `reports/`, never back into this file;
a change after freeze is a dated amendment at the end.

Lead: `LEAD-RHO-matched-rho-calibration` (crypto-autoresearcher
`knowledge/leads/ecdlp/generic/LEAD-RHO-matched-rho-calibration.yaml`, ledger
refs `H-ECDLP-ff38dc`, `H-CRYPTO-a405d4`, `H-ICEX-835c4f`, `H-ECDLP-f3d2fd`).
Every number quoted from the repositories below is `[pcc:unverified]`.

## 1. Question

Every prime-field IC gap is divided by a matched `rho.negation` reference. The
only sealed prime rho bounds are **toy tier** (`docs/bounds/records/calibration-prime-rho-neg-v2.json`,
`ECBND1h67e22de83dca`: mean 1.453 [1.184, 1.772] x the floor `sqrt(pi/4)`,
alpha 0.398 [0.315, 0.475], six curves at log2 r 15.7-25.3, 24 runs each)
[pcc:unverified]. The sealed frontier has a medium-tier domain for Koblitz
curves only (and two prime *oracle* domains, `static_dh:*`, whose rho rows are
references beside Cheon's method on r +- 1 shaped curves, 8 runs per size).

1. What do `rho.negation`, `rho.plain` and `bsgs.negation` cost, as sealed
   `ecbench.bound/v1` records (constant and exponent with intervals, per
   phase, with the §5a band), on generic prime-order curves at
   log2 r = 32-49 (the medium tier: field bits 33-49)?
2. Is the toy excess of `rho.negation` over its floor finite-size set-up (it
   decays with `N = sqrt(r)` toward 1) or a persistent walk shortfall?

## 2. Curves (frozen inputs)

Source: the harness's own deterministic constructor
`prime_orbit_index_calculus::generate_instance(generic, bits, seed)`
(`CurveSpec::PrimeOrbitShape {shape: "generic"}`: a random prime `p` in
`[2^(bits-1), 2^bits)`, random `a, b`, certified group order with cofactor 1,
anomalous curves skipped). `prime_search` (`find_prime_order_curve`, the
source of the toy `icv1-fp*` ladder) refuses `bits > 32`, and the bench roster
stops at 20 bits, so this is the in-harness generator that reaches the medium
tier. `specs/curve-generation.json` is the plan-only spec; `ecbench plan`
resolved it, and the measured specs give the ten curves explicitly as
`prime_explicit` (as the toy calibration did), so neither planning nor any
audit repeats the search.

- Sizes: field bits `b in {33, 37, 41, 45, 49}`, so `log2 r in [b-1, b)`:
  the five sizes are `log2 r ~ 32, 36, 40, 44, 48`. Bits 32 would put the
  smallest size in the toy tier (`toy <= 32` field bits) and a fit that spans
  tiers is refused. The Koblitz medium ladder actually sealed in
  `docs/bounds/records/` sits at log2 r 27.8, 32.1, 36.6, 39.0, 44.3, 44.5 and
  47.2 (calibration and pair-claw records), so "32-48" spans it.
- Two curves per size (the curve effect is estimated, §8 negative control).
- Seeds are derived, not chosen: `seed(tag) = int(SHA-256("prime_medium_tier_rho_20261107/v1/" + tag)[0:12 hex], 16)`
  with `tag = curve/bits=B/index=J`, `J in {1, 2}`; the same rule gives the two
  target seeds (`target_seed/block=A|B`) and one measurement seed per job
  (`measurement_seed/job=J1..J6`). The derivation is in `seeds.json`.

The resolved curves (slug, EC1, p, a, b, r, generator) are in `curves.json`
and in the table of `README.md`. None of them is in `docs/curves/registry.json`
yet; the PR that lands this protocol registers them
(`python3 scripts/build_curve_registry.py` harvests the committed
`curves.json` / specs) before any prose cites a slug (AGENTS.md §11).

## 3. Arms

| arm | role | method | why |
|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | the matched prime rho of every prime IC gap; the toy bound's method id, so toy and medium are one instrument |
| `rho-neg-aa` | control | `rho.negation` | A/A: identical method, identical seeds (ecbench shares one algorithm seed per workload and round across arms). Counts must match `rho-neg` exactly (determinism); its wall time is the noise floor the EKS gate admits |
| `rho-plain` | baseline | `rho.plain` | positive control for the detector: theory says `sqrt(2) = 1.414` times `rho.negation` |
| `bsgs-neg` | baseline | `bsgs.negation` | exact-analysis constant `S = 1.0` (`methods::expected_s`): calibrates the harness at medium sizes |
| `rho-neg-v2` | candidate | `rho.negation_v2` | the `_v2` accounting the e2e frontier reads new work against (Brent cycle detection, lost-walk counters charged); secondary, reported only |

All parameters at their registered defaults (written out by the harness
before hashing; `rho.*` `cap_multiple = 64`). One binary for every arm and job.

## 4. Workloads, rounds, order, seeds

- `targets_per_curve` 8 (planted, `uniform_scalar_sha256_v1`), one target per
  workload, cold, every phase charged. Measurement: `rounds: 5`, `warmup: 1`,
  `order: alternate`, `isolation_required: L2`, `timeout_seconds: 300` per
  execution.
- Per size and arm: 2 curves x 8 targets x 5 measured rounds = **80 verified
  runs** (the lead asks for >= 24); 400 per arm over the medium tier.
- Jobs (one ecbench session each; every EKS job is capped at 3600 s, and the
  pod replays every run, which doubles the solve time):

| job | curves | targets | target seed | measurement seed |
|:--|:--|:--|:--|:--|
| J1 | the six curves at 33, 37 and 41 bits | 8 | block A | J1 |
| J2 | both 45-bit curves | 8 | block A | J2 |
| J3 | 49-bit curve 1 | 4 | block A | J3 |
| J4 | 49-bit curve 1 | 4 | block B | J4 |
| J5 | 49-bit curve 2 | 4 | block A | J5 |
| J6 | 49-bit curve 2 | 4 | block B | J6 |

  A 49-bit curve's eight targets are two four-target blocks with different
  target seeds, so the two sessions hold different workloads (the bound fit
  refuses two sessions that share a workload and an algorithm seed).

## 5. Metrics

Primary (counted, `ecbench.gae`, valid at any isolation level):
`S = total_gae / sqrt(r)` per run; per size the mean `S`, `S / floor`
(`floor = sqrt(pi/4) = 0.8862`, `A = 2`) and `R = S / declared S`; per arm and
tier the bound's constant (`constant.ratio_to_floor`, cluster bootstrap over
the sizes' workloads, 2 000 resamples, the harness's fixed seed) and exponent
(`fit.alpha` with interval; 10 curves = 10 size rows, so `level: exponent`).

§5a band: every arm's unpriced counters (`cost.unpriced` and every
`*_uncharged` counter with its count) are listed, and each bound is fitted at
`--weight 0`, `0.1` and `1` GAE per unpriced native operation, the three
records for every arm. The rho reference's own unpriced work is part of what
is being sealed: on the toy ladder it was 0.88-1.31 per `sqrt(r)`, of the
order of the charged work [pcc:unverified].

Secondary: L2 solve wall time per GAE (`ns_per_gae`), read only against the
session's A/A interval and only where the pod's L2 gate admitted every record;
peak RSS; the `field_ops` axes (`field_muls`, `field_sqrs`, `field_invs` per
`sqrt r`) which the prime generic walks count.

## 6. Analysis (frozen)

1. Per job: the pod's `ecbench verify --replay-all` receipt is the replay
   certificate; locally `ecbench verify --dir S --replay-all --out S.audit.json`
   again on the downloaded session; `ecbench table --dir S`;
   `ecbench compare --dir S --a rho-neg --b X --save` for `X` in
   `rho-plain, bsgs-neg, rho-neg-v2, rho-neg-aa`.
2. Bounds (`fit.sh`, every figure computed by `ecbench`): for each arm,
   `ecbench bound fit --dir J1 ... --dir J6 --arm ARM --audit ... --tier medium
   [--weight w] --root . --out docs/bounds/records/prime-medium-<arm>[-w0.1|-w1].json`,
   then `bound check` on each and `frontier build`.
3. Finite-size law (`examples/prime_medium_tier_rho_rfit.rs`, the frozen
   native analysis; it reads sealed records only): for each of `rho.negation`,
   `rho.plain`, `bsgs.negation`, the per-size `R = mean S / declared S` of the
   committed toy record and the new medium record, `N = sqrt(r)`, weights
   `1/se^2` from each size's committed 95 % interval;
   - model A, the lead's form: `R(N) = 1 + c / N^beta`;
   - model B, free asymptote: `R(N) = R_inf + c / N^beta`;
   - `beta` profiled on `[0.10, 3.00]` step 0.002; parametric bootstrap, 2 000
     resamples, seed 20261107; chi-square goodness of fit for each model.
   `--seed 20261107 --resamples 2000`, inputs in this order: toy record, medium
   record.

## 7. Predictions (written before any data of this study)

Data already seen when writing them (in-sample, not sealed, heterogeneous
curves, `[pcc:unverified]`):

- toy sealed bounds above: `rho.negation` R = 2.22, 1.50, 1.42, 1.31, 1.22, 1.06
  at log2 r 15.7-25.3; `rho.plain` 2.50-1.47 x floor; `bsgs.negation`
  0.86-1.36 x floor;
- a pool of every committed `ecbench.record/v1` on prime curves with
  32-50 field bits (CM, volcano, conductor-gap and shaped curves included,
  cofactors included): `rho.negation` 1.062 x floor over 18 735 verified runs
  (per field size 0.96-1.18), `rho.plain` 1.49-1.50 x floor (n = 640),
  `bsgs.negation` 1.10-1.16 x floor (n = 823) (`sources/prior-pool.txt`);
- the Cheon medium-tier oracle-domain `rho.negation` references: 1.11-1.19 x
  floor, 8 runs per size, intervals about +-0.3.

| id | quantity | prediction |
|:--|:--|:--|
| P1 | `rho.negation` medium constant, ratio to floor, w = 0 | 1.06, inside [1.00, 1.10] |
| P2 | `rho.plain` medium `R` (S / 1.2533) | 1.05, inside [1.00, 1.10]; ratio to the negation floor about 1.49 |
| P3 | `bsgs.negation` medium `S` | 1.00 inside its 95 % interval (ratio to floor 1.13) |
| P4 | paired `rho-plain / rho-neg` S ratio (positive control) | 1.41, interval excluding 1.2 from below |
| P5 | model A on `rho.negation`, toy + medium | `beta` about 1 (0.6-1.4); medium `R` predicted by A below 1.01; A may be rejected by chi-square if the walk's own excess (a few percent for an r-adding walk) is real |
| P6 | model B `R_inf` on `rho.negation` | inside [1.00, 1.10] |
| P7 | A/A | every paired count identical (ratio exactly 1); wall A/A admitted |
| P8 | `rho.negation_v2 / rho.negation` paired S | 1.00, interval inside [0.97, 1.05] |
| P9 | curve effect within a size | none after Holm over five sizes |

Consequence stated in advance, derived and not measured: if P1 holds, a prime
IC gap normalised to a *toy* rho at 1.2-1.5 x floor understates the distance
to a medium-tier rho by that toy factor (the lead's "gaps grow"); gaps
measured against a same-session matched rho are unchanged.

## 8. Controls

- **A/A** `rho-neg-aa`: same method, same seeds; determinism of counts and the
  wall-time noise floor (EKS admission requires `aa_pairs >= 5` and an
  admitted A/A interval). Count noise is read from rounds (independent
  algorithm seeds) and targets, through the cluster bootstrap.
- **Positive** `rho-plain` against `rho-neg`: a known `sqrt(2)` effect through
  the same harness, workloads and seeds. Expected detection: paired ratio
  interval lower end above 1.2. A miss makes the study inconclusive.
  `bsgs-neg` is a second positive control on the absolute scale (exact
  constant 1.0).
- **Negative** (null population): random generic prime-order curves from the
  generator, two per size. Under the null, rho's constant does not depend on
  the curve; per size, the two curves' mean `S` differ by
  `z = (m1 - m2) / sqrt(se1^2 + se2^2)`, two-sided, Holm over the five sizes
  at 0.05. A detected curve effect is reported and the size's interval is
  read with it; it does not change the decision rule.

## 9. Decision rule (primary: `rho.negation`, w = 0)

`M` = the medium bound's `constant.ratio_to_floor` (value, 95 % interval).

| verdict | condition |
|:--|:--|
| `setup_excess` (the toy excess is set-up; the lead's "within 1.10x") | `M <= 1.10` and `M.ci95_hi < 1.30` |
| `within_1.10_strict` (reported in addition) | `M.ci95_hi <= 1.10` |
| `walk_shortfall` (reopen `LEAD-RHO-negation-map-fruitless-cycles`) | `M >= 1.30` and `M.ci95_lo > 1.10` |
| `intermediate_excess` | `M.ci95_lo > 1.10` and `M < 1.30` |
| `unresolved` | otherwise |
| `inconclusive` | positive control P4 fails, or any audit fails |

The R(N) fit (P5, P6) is reported beside the verdict: model B's `R_inf`
interval and model A's chi-square say whether the asymptote is 1. It does not
override the table. `rho.plain` and `bsgs.negation` are reported against their
declared constants (`R` interval contains 1 or not, Holm over the three
secondary methods with `rho.negation_v2`). The band: the verdict is stated at
w = 0 (the lead's quantity); the w = 0.1 and w = 1 constants are reported, and
any IC gap rescaled with this bound uses the same weight on both sides.

Power (heuristic, from the prior pool's per-run SD 0.55 x floor): 400 runs give
a standard error of about 0.028 on `M` (interval about +-0.055 before the
cluster widening), so `setup_excess` is decidable when the truth is near 1.06
and `walk_shortfall` when it is near 1.3; `within_1.10_strict` will usually not
be reached at a true 1.06, which is why it is a separate, secondary line.

## 10. Stopping rules, failures, inadmissible

- Fixed N: the six jobs, once each. No job is extended or repeated because of
  its numbers.
- A session that is complete with a passing audit is admitted for the
  **counts** even when the pod's L2 gate fails (exit 42/43): levels gate wall
  time, never counts. Its wall time is not reported.
- An incomplete session (Spot interruption, launcher failure, timeout of the
  job) is kept as evidence, is inadmissible to the bound by the harness's own
  rule, and is replaced by **one** rerun of the identical spec under a new job
  name, recorded as an amendment with the failed session's identity. A second
  failure of the same job stops the study at the sizes that completed; the
  bound is then fitted on what completed and says so.
- A run that does not verify or is censored (`exhausted`, `timeout`) stays a
  row; the bound treats it by the harness's rules.
- Inadmissible: changing curves, seeds, targets, rounds, arms, the unit, the
  accounting, the weights or the thresholds after seeing any result; dropping
  or rerunning individual executions; pooling toy and medium in one bound;
  reading the R(N) extrapolation beyond 49 bits as a measurement; quoting
  local or non-L2 wall time.

## 11. Execution

Iso Lab on EKS Spot (AGENTS.md §9a, `isolab-eks`), `c8a.2xlarge`, four CPUs,
L2 pool, `seconds: 3600`, `memory: 4Gi`, zero measurement retries, no
checkpoint (`policy: none`; a lost job is an incomplete session, §10).
Requests: `requests/J1.request.json` ... `J6.request.json`
(`isolab.eks.request/v1`, validated with `remote/validate-request`). The image
must be built from the commit that lands this protocol (GitHub
`isolab-experiment.yml` commit build, or the AWS bridge with that image's
`job.build`), because `rho.negation_v2` and the current audit must be in the
binary. Before launch: `scripts/checkpoint.sh init` + push, and one
`bench.record/v1` per job (`tools/benchrec.py init`). The pre-flight runtime
estimate is in `README.md` ("Compute estimate").

## Amendments

None.
