# First counted prime-field algebraic-track row: a ready-to-submit specification (2026-11-11)

**Status: specification only. No data exists.** Nothing here was run: no `ecbench`, no benchmark, no
remote job, no `cargo`. A session with an EKS identity submits it as written (section 11); what
stops that from this container is in [`BLOCKERS.md`](BLOCKERS.md). This is lead 4 of
[`../README.md`](../README.md) section 7.

Labels. **measured** = read from a committed file (path given); every measured number quoted from the
repository is `[pcc:unverified]`, because none of the files cited carries a pcc claim id (checked:
`grep -c "pcc:"` is 0 in each). **derived** = computed here, arithmetic shown. **plan** = a choice
made here. No novelty is claimed. The `research-kb` MCP server was not available to this session, so
the "knowledge base first" step was not done; the atlas and KB were not queried.

Files: [`spec.json`](spec.json), [`spec-m3.json`](spec-m3.json) (gated, section 9),
[`protocol.draft.json`](protocol.draft.json) + [`PROTOCOL.md`](PROTOCOL.md),
[`BLOCKERS.md`](BLOCKERS.md), [`job-algebraic-row.sh`](job-algebraic-row.sh) and
[`make-request.sh`](make-request.sh) (the pod command and the request writer, untested beyond `bash -n`
and a `jq` dry run on dummy inputs), and the recorded checks
[`spec_check.out`](spec_check.out), [`protocol_validate.out`](protocol_validate.out).

## 1. The question and the row

**Question.** What does the lab's native prime-field algebraic oracle cost on Table B curves, whole
pipeline, in `S = GAE/sqrt(r)` across the AGENTS.md section 5a band, against matched
`rho.negation` and the generic-track incumbent on the same targets?

**The row.** `pkm-semaev` (`src/cryptanalysis/ic_framework/pkm_semaev.rs`): `S_(m+1)` together with the `[pcc:unverified]`
membership equations of PKM's isogeny tower (`Membership::Isogeny`), F4 over the tower quotient ring
(`f4_fp_tower`, native: no msolve), hits read off the counted grid and lifted. It is the first counted
row of the algebraic track (`research/prime_ic_program_20261029/DECISIONS.md` R1 Option C; the track's
baseline is empty). Per AGENTS.md "Tracks of index calculus" and D2 it is ranked against the generic
incumbent and matched rho on the same instances, in the same unit, across the band at w = 0 / 0.1 / 1.
The track ratchets only its own baseline: nothing here is an improvement or regression of the generic
track, and a generic-track gain would never move this row.

**Established if it completes (plan).** A cold, verified, replay-identical, band-priced cost for the `[pcc:unverified]`
algebraic track on three Table B curves, with its distance to matched rho reported as a distance.
**Not established.** Parity or any rho comparison beyond these curves; scaling (three sizes, one
generator and one auxiliary-curve seed each); an exponent; any statement about the Kummer form (not
available here, section 2); anything about m >= 3 unless the gated cell runs; wall-clock speed (a
command Job has no native L2 admission). The only evidence on record points the other way from
parity (section 7, H1): the smoke at k = 20 is about 10^8 times rho at w = 0.1. That would be the
number this row reports as its distance, with a measured replacement for the model in section 6.

## 2. Curves and the base: Kummer is not available, the isogeny tower is needed

Curves: the Table B prime roster rows, built by `{"kind":"prime_roster","bits":B}`
(`ic_boundary::roster_prime_instance`, from `bench_curves()` in `src/cryptanalysis/research_bench.rs`).
Facts below are measured from `docs/curves/registry.json` and `research_bench.rs` (curve parameters,
order) and derived with Python integer arithmetic (`v2`, primality of the orders by trial division):

| bits | slug | p | order N = r (prime, cofactor 1) | generator | p mod 8 | v2(p - 1) | N^(1/3) | log2 N^(1/3) | Kummer t max | Kummer base vs 2^t ~ N^(1/3) |
|--:|:--|--:|--:|:--|--:|--:|--:|--:|--:|:--|
| 16 | `icv1-fp16-t351-d433f24e` | 65353 | 65003 | (1, 15165) | 1 | **3** | 40.2 | 5.33 | 3 (8 abscissae) | 8 << 40 |
| 18 | `icv1-fp18-t175-45770ebc` | 261847 | 261673 | (2, 4) | 7 | **1** | 64.0 | 6.00 | 1 (2 abscissae) | 2 << 64 |
| 20 | `icv1-fp20-t1293-3c2999ba` | 1048291 | 1046999 | (1, 123004) | 3 | **1** | 101.5 | 6.67 | 1 (2 abscissae) | 2 << 101 |

`Membership::Kummer` needs `2^t | p - 1` (`prime-kummer`, `primitive_two_power_root`). So on fp18 and
fp20 the largest Kummer tower has `t = 1`, and on fp16 `t <= 3`; none reaches `2^t ~ N^(1/3)` (about
`2^5`, `2^6`, `2^7`). **The isogeny base `prime-isogeny:t=T` is needed on all three**: it puts no
condition on p (`pkm_semaev.rs`, `Membership::Isogeny`), builds `V = x(R' + <Q>)` from an auxiliary
curve with a point `Q` of order `2^t` (seeded search, charged to the factor-base phase; seed 0, the
default, so the arm text matches `e2e-iso-t6-k20`) and has `2^t` abscissae of which about half lie on
the curve.

**t grid (plan).** `t in {4, 5, 6}` on every curve. The rule `2^t ~ N^(1/3)` points to `t = 5, 6, 7`
(fp16, fp18, fp20); `t = 7` is not in `spec.json` because the cost model (section 6) puts it at
about 1.5 h per target on fp20 and about 23 min on fp18, for no prediction that needs it. It is
the first stretch cell if H2 fails (`PROTOCOL.md` section 8).

**Expected columns (derived from one curve).** The only measured column counts are the k = 20
miniature's, seed 0 (`research/curve25519_algebraic_ladder_20261102/README.md`, Q-D table, isogeny
form [pcc:unverified]): 9 / 18 / 25 / 65 columns at t = 4 / 5 / 6 / 7 (of 16 / 32 / 64 / 128
abscissae); the e2e record says "25 of 2^6 abscissae on the curve". Columns on the Table B curves
are **plan, not measured**: about 0.4 to 0.55 of `2^t`, so 6-9, 13-18, 25-35 for t = 4, 5, 6.
`ecbench` records the actual `columns` in each factor-base block.

## 3. Arms (14 in `spec.json`, one session, 336 executions)

`spec.json`: 3 curves x 8 public targets x 14 arms, `rounds` 1, `warmup` 0, `order: alternate`,
`isolation_required: L0`, `timeout_seconds: 7200` per execution, `target_kind: public`
(`hash_to_subgroup_v1`, one target per workload, never pooled). All arms see every curve; the seeds are
shared by arms through ecbench's per-(workload, round) algorithm seed.

| arm | role | method | track | purpose |
|:--|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | | matched rho, one target |
| `rho-neg-aa` | control | `rho.negation` | | A/A: counted ratio must be exactly 1.000 |
| `inc-n20`, `inc-n32`, `inc-n51` | baseline | `ic.pipeline`, `prime-abscissa:size=N`, `mitm:negation_folded=1` (walk targets, m = 2) | generic | the Table B recipe `mitm_m2_negfold_walk_balanced`, re-measured in ecbench: N = leaderboard column count (measured: 20, 32, 51 columns for fp16, fp18, fp20, `docs/ic/leaderboard.json`); the arm whose N matches a curve is that curve's incumbent, the others are context. Arm params are spec-wide, so each size runs on every curve |
| `iso-t4-m2`, `iso-t5-m2`, `iso-t6-m2` | candidate | `ic.pipeline`, `prime-isogeny:t=T`, `pkm-semaev:m=2,mode=full`, `max_trials=400000` | algebraic | the row |
| `iso-t5-m2-aa` | control | `iso-t5-m2` again | | A/A of the algebraic arm (t = 5 only: repeating t = 6 would double the dominant cost; counted replay identity covers the rest) |
| `iso-t4-m2-grid`, `iso-t5-m2-grid`, `iso-t6-m2-grid` | control | same, `engine=exhaustive` | | the counted V^2 grid: stream-identity check (C1) and the cost F4 has to beat (C4) |
| `iso-t4-m2-xchk`, `iso-t5-m2-xchk` | control | same, `crosscheck=1` | | every F4 refutation grid-checked; counter `pkm_crosscheck_disagreements` must be 0 (the check itself is uncharged) |

Not in the spec, and why. `mitm-prime-batch:m=2` (a layout variant of the same incumbent; the Table B
oracle name is the pair table `mitm`, so the incumbent is the leaderboard's recipe). `guess=G` hybrid
guessing and the `bits`/`digits` memberships (tuning knobs of the oracle; adding them after seeing
the first row is the track's next step, not this row). Kummer arms (section 2). A planted-decomposition
positive control (BLOCKERS item 6). The m = 3 chain is in [`spec-m3.json`](spec-m3.json) (section 9).

Syntax sources for the arms: `pkm-semaev:m=2,mode=full` and `m=3,mode=chain`, `engine=exhaustive`
and `crosscheck=1` are parameters read by `PkmSemaevOracle::read_params`; the arm syntax is that of
`research/curve25519_ic_ratchet_20261102/sessions/pkm-smoke-k20/spec.json` (the only pkm-semaev
smoke), `.../e2e-iso-t6-k20/spec.json` (isogeny) and `research/generator_effect_ic_20261019/e3b/
sessions/e3-c6/spec.json` (`engine=exhaustive`, `mitm:m=3,negation_folded=1`).

## 4. Targets and seeds

- `target_seed` = 1504454584, `measurement.seed` = 976050496: the first 8 bytes (big endian) of
  `sha256("prime-ic-algebraic-row-20261111|target_seed")` and `sha256("...|measurement_seed")`, modulo
  `2^31` (derived; fixed before any data, in `spec.json`). Do not change them.
- 8 targets per curve, 24 workloads. The ecbench skill's guidance is 8 or more per curve for a
  claim; this row reports a distance, not a claim.
- Workload ids and algorithm seeds depend on (curve, `target_seed`, index, `measurement.seed`, round),
  not on which spec holds the workload (`src/cryptanalysis/ecbench/spec.rs`, `plan`, code reading), so
  `spec-m3.json` reuses fp16 targets 0-3 of `spec.json` and any later split by curve keeps the same
  workloads.

## 5. What is measured

Cold whole-pipeline `S = priced_gae(w)/sqrt(r)`, `priced_gae(w) = G - C + w (U + n_C)`
(`src/cryptanalysis/ecbench/band.rs`), at w = 0, 0.1, 1, for every arm, rho included. `r = N` here
(cofactor 1), so `sqrt(r)` is 255, 512 and 1023 for fp16, fp18, fp20 (derived). Every phase is charged
(factor base including the auxiliary-curve search, oracle set-up, relations, linear algebra, verify);
the online/target-dependent split is recorded as the pipeline already does.

Unpriced native counters, listed per arm and priced alike at w (from the records of
`research/curve25519_algebraic_ladder_20261102/sessions/e2e-iso-t6-k20`, measured, [pcc:unverified]):
- `iso-*` F4 arms: `solver_F_p_multiply-adds_(elimination_only)_uncharged` (the dominant one),
  `sqrt_solves_uncharged`, `row_ops_uncharged`, `legendre_symbols_uncharged`,
  `target_guard_probes_uncharged`, `velu_evaluations_uncharged`, `pkm_grid_fp_mults_uncharged`,
  `pkm_system_fp_mults_uncharged`.
- `iso-*-grid`: the same list with the grid's `pkm_grid_fp_mults_uncharged` as the main entry. It is a
  different unit from F4's multiply-adds (docs/ic/SWEEPER.md says a grid rung "cannot be ranked" against
  F4 rungs); the grid arms are controls, read descriptively, priced in the band like every counter.
- `rho-neg`, `rho-neg-aa`: `canonicalisations_uncharged` (about one per step).
- `inc-n*`: the table oracle's counted per-step inserts, priced by default since 2026-11-08 when the
  record carries the `counted:` marker (AGENTS.md 5a); its other unpriced counters as listed by its records.
An arm that names a counter without a count is a lower bound and is not ranked. Byte-valued
counters are listed, not charged (`NON_OPERATION_COUNTERS`). The band is the only reading; no
calibration of F4 multiply-adds to GAE is pinned on these curves, and none is invented here.

Reported per curve (decision rule in `PROTOCOL.md`): per-arm S at the three weights, the paired
ratio of summed S against `rho-neg` and against the matching `inc-n*` with 95% cluster-bootstrap
intervals over the 8 targets, the `ecbench band --ranks` order, F4 over grid S, F4 multiply-adds per
trial, hit rate, relations, columns, peak RSS. Distance remaining is `S_arm / S_rho` and nothing else.

## 6. Budget (derived, with the arithmetic) and memory

**Anchors (measured, [pcc:unverified]).** `.../sessions/e2e-iso-t6-k20/records.jsonl`: curve
`p = 1048573`, group order 1,049,624 = 8 x 131,203, isogeny t = 6, m = 2, two targets:
`targets_tried` 11,692 and 14,932; F4 multiply-adds 6.16e11 and 7.78e11; F4 wall 823.5 s and 1022.4 s
(L0, one host); `relations_found` 15 and 18; columns 25; peak RSS 25,320 and 25,548 KiB.
Throughput: 6.16e11 / 823.5 = 7.5e8 multiply-adds per second (derived; second record 7.6e8).
Q-D table (same README): F4 multiply-adds per target 5.23e5, 3.96e6, 5.55e7, 7.46e8 at t = 4, 5, 6, 7
(isogeny form, k = 20).

**Model (derived; each step labelled).**
1. Hit probability per trial `h = 2c^2 / N` for `c` columns. Check on the anchor: `2 x 25^2 / 1,049,624
   = 1.19e-3`, against measured 15/11,692 = 1.28e-3 and 18/14,932 = 1.21e-3.
2. Relations needed `rho = 0.7 c` (15/25 and 18/25 = 0.6 and 0.72 on the anchor; two points).
3. Trials `T = rho / h = 0.35 N / c`.
4. F4 work per target `T x (multiply-adds per trial)`; seconds = that / 7.5e8.
5. Columns `c = 9, 18, 25` at t = 4, 5, 6 (anchor curve's, applied to the Table B curves: the
   largest assumption), F4 per trial from the Q-D table, with 5.49e7 at t = 6 (measured:
   6.16e11 / 11,217 solver calls).

| curve | t | c | T = 0.35 N/c | F4 mult-adds per target | seconds per target |
|--:|--:|--:|--:|--:|--:|
| fp16 | 4 / 5 / 6 | 9 / 18 / 25 | 2,528 / 1,264 / 910 | 1.3e9 / 5.0e9 / 5.0e10 | 1.8 / 6.7 / 66.6 |
| fp18 | 4 / 5 / 6 | 9 / 18 / 25 | 10,176 / 5,088 / 3,663 | 5.3e9 / 2.0e10 / 2.0e11 | 7.1 / 26.9 / 268 |
| fp20 | 4 / 5 / 6 | 9 / 18 / 25 | 40,717 / 20,358 / 14,658 | 2.1e10 / 8.1e10 / 8.1e11 | 28.4 / 107.5 / 1,073 |

The fp20 t = 6 cell reproduces the anchor (1,073 s derived against 824 and 1,022 s measured, on a
curve of nearly the same N), which is a consistency check of steps 1-4 at one point, not a
validation at the others.

Per target the F4 arms sum to `t4 + t5 + t6 + aa(t5) + xchk(t4) + xchk(t5)`: 90 s (fp16), 363 s
(fp18), 1,452 s (fp20). Grid arms, `rho`, `inc` arms are derived to cost seconds in total (grid:
`V^2/2` pairs per trial, about 10^4 multiplications per trial at t = 6). With 8 targets:
`8 x (90 + 363 + 1452) = 15,240 s = 4.23 h` of single-thread work for the run (the protocol's
`estimated_total_seconds` rounds this to 15,228 from unrounded inputs). In-job `--replay-all`
re-executes every deterministic run (about the same again, 4.2 h) and the independent replay on a
second host class once more (4.2 h): **12.7 h single-thread, 25.4 pod vCPU-hours at the
command-request minimum of 2 vCPUs per pod** (`remote/run-command`), against the approved D-EKS
allowance of about 49 CPU-hours (range 15-150, `DECISIONS.md`; whichever way that allowance counts,
single-thread or vCPU, this fits). Margin: x1.5 on the vCPU figure gives 38 vCPU-hours, still
inside. Deadlines: Job 1 (run + replay-all) 54,000 s (15 h, 1.77 x the derived 8.5 h); Job 2
(independent replay) 28,800 s. Ceiling: 46 pod vCPU-hours (2 x (54,000 + 28,800) / 3600), enforced by
the deadlines. Dollars (plan): c8a Spot is about 0.16 USD/h per 8 vCPU (arity launch README,
measured there); 2-vCPU pods for about 23 pod-hours are under USD 5; **ceiling USD 10** for build,
idle and retries, inside the USD 200 daily allowance (`COST_CONTROLS.md`).

**Dominant risk to this table.** Columns, trials and relations on fp16/fp18/fp20 are modelled from
one curve and two targets; a factor of 2 either way moves fp20 t = 6 between about 0.5 and 2 h per
target (4 to 16 h for 8 targets). The Job deadline bounds it; the per-execution timeout of 7,200 s
censors it.

**Memory.** Measured peak RSS is about 25 MB at t = 6 (anchor). The bits form at t = 9 was killed at
about 4.5 GB in the ladder, but that is another membership and t; the isogeny arms here stay at
t <= 6. Request 4Gi (pod); 8Gi for `spec-m3.json`, whose memory is unmeasured. p < 2^31 holds for
all three curves (engine limit).

## 7. Pre-stated readings (what each outcome changes)

Full text in `PROTOCOL.md`. In short, written before data:
- **H1 (distance)**: best `iso` arm >= 1e4 x rho at w = 0.1 on every curve. Data seen: the k = 20 e2e
  row, 1.9e8 / 1.84 at w = 0.1 (measured, [pcc:unverified]). *If true*, the row records the track's
  baseline distance and nothing more. *If a curve falls below 1e4*, the F4 cost model is wrong (per
  trial work lower than the anchors) and the extrapolation in section 6 is withdrawn before any larger
  curve is planned. *If any ratio is below 1*, it is a parity candidate and must pass the full
  validation gate before it is described as anything.
- **H2 (t)**: `iso-t6 / iso-t4` of summed S at w = 0.1 above 5 on every curve. *If true*, the
  `2^t ~ N^(1/3)` rule is not cost-optimal at these sizes and the track's best configuration is the
  smallest t that verifies. *If not*, larger bases pay and the t = 7 stretch cells become the primary.
- **H3 (w = 0)**: `iso-t6 / inc` ratio within [2, 10] on fp20 (calibrated on 5.4 at k = 20,
  measured: 42.4 / 7.92). Calibration, not decisive.
- **H4 (exactness)**: F4 and grid streams identical, zero crosscheck disagreements. *A mismatch voids
  every `iso-*` record of the session.*

## 8. Controls

A/A (`rho-neg-aa`, `iso-t5-m2-aa`), grid cross-check on every t (`*-grid`), refutation check `[pcc:unverified]`
(`*-xchk`), exact replay (`--replay-all`) and an independent replay on a second host class. **No
positive control** (a planted `R = P_i +- P_j` that the oracle must find): ecbench cannot plant
decompositions into `ic.pipeline` and there is no `oracle-check` for pkm-semaev; recorded as a
limitation in the protocol (`controls.positive.present: false`, with the reason) and BLOCKERS item 6.
Public targets are verified by `[d]G = Q`.

## 9. The m = 3 chain cell (gated)

`spec-m3.json`: fp16 only, 4 targets (fp16 targets 0-3 of `spec.json`), arms `rho-neg`, `rho-neg-aa`,
`inc-m3-n64` (Table B m = 3 recipe: 64 columns, `mitm:m=3,negation_folded=1`; measured S 23.88 on
fp16, `docs/ic/leaderboard.json`), `iso-t4-m3-chain` (`pkm-semaev:m=3,mode=chain`) and its grid
control. **Nothing measured bounds m = 3 at t = 4**: the only m = 3 chain data is t = 2 (3 columns,
S = 211 / 6.18e6 / 6.18e7 at w = 0 / 0.1 / 1, the smoke) and the multiples control of this lead
(`../devcheck/`, degree closures, not costs). Hit probability is much higher at m = 3 (derived,
not modelled further) but F4 per trial is unmeasured, so a gate: run 2 targets of `iso-t4-m3-chain`
first (preflight, not data); if the F4 multiply-adds per target exceed 3e11 (about 400 s at
7.5e8/s), do not run the cell and say the m = 3 chain is unmeasured. Timeout 1,800 s per execution,
8Gi.

## 10. Validation gate, replay certificate and validator (`docs/VALIDATION_GATE.md`)

No row reaches the leaderboard or `benchmarks/README.md` without these (plan; none exists):
1. **Replay certificate.** In-job `ecbench verify --dir session-row --replay-all --exit-code --out
   session-row.audit.json` (Job 1) must be `OK`, every replay `identical`; then an **independent**
   replay on a different `env_class_id` host (`ecbench host`) from Job 2
   (`job-algebraic-row.sh replay`), per `ecbench-independent-runner`. Pin the receipts' SHA-256
   in `replay-certificate.json` (`status: verified`, `independent: true`, `scalars_verified: true`,
   non-empty `runs`).
2. **Validator report** (`validator-report.json`: `status: pass`, `independent: true`, `raw_data` files,
   `headline` list of `{name, claimed, recomputed, rel_tol}`): a recompute of the headline S values
   from `records.jsonl` by someone other than the producer (form: `research/ic_validation_gate_20261024/B/
   validator-report.json`; the `audit-runs` and `sheriff` skills are the existing independent
   readers).
3. **Rho reference**: `rho-neg` of the same session, same binary, same unit, band at 0 / 0.1 / 1 in
   `VALIDATION.json.rho_reference.gae_band`, `kind: negation`, `order_holds_across_band` stated.
4. `VALIDATION.json` in the evidence directory; `python3 scripts/validation_gate.py evidence DIR`
   must say valid. A pre-gate or unvalidated row renders as such. The row goes in its own section
   outside tables A-C (AGENTS.md section 7b) with `python3 scripts/build_ic_leaderboard.py`; whether the
   result is "big" (AGENTS.md leaderboard rule) is the owner's call on the numbers, not this
   spec's.
5. pcc: every cited number carries a claim id and standing, or is labelled `[pcc:unverified]`
   (AGENTS.md "Sabotage is assumed"; `python3 tools/pcc/pcc.py canary` and `ledger verify` in CI).

## 11. Submission (isolab-eks, exact steps; none executed here)

Route (plan, **a decision for the main session/owner**): the audited `isolab_k8s_benchmark` /
`isolab.eks.request/v1` path requires L2, at least five measured rounds, a warmup and a matching A/A
(`taskq/deploy/eks-karpenter/HARNESS.md`), caps each Job at 3,600 s and the L2 pool at 8 vCPU /
32 GiB. This row is a count (operation counts need no isolation level, `ecbench-measure` section 2),
so on that route it would multiply the work by (5 + 1) and the fp20 t = 6 arms alone (about 18 min
each, derived) would not fit a 3,600 s Job per target. The spec therefore runs as an
`isolab.eks.command-request/v1` Job (`remote/launch-remote-command`; precedent
`research/arity_completion_20261104/launch/`): L0, wall time descriptive only, counts exact and
replayed. If the owner insists on the L2 path, only fp16 fits it (about 90 s per target for all F4
arms, 8 targets x 6 executions per arm) and the spec needs `rounds >= 5`, `warmup 1`,
`isolation_required: L2` and an A/A for every arm: a different, larger spec, not this one.

Prerequisites, in order (the hooks refuse the launch otherwise):
1. `conductor check --summary "first counted algebraic-track row" --scope path:research/prime_ic_leads_20261110/algebraic_row_spec`
   (Conductor is not installed here, BLOCKERS item 5).
2. Freeze: set the `FILL-AT-FREEZE` fields of `protocol.draft.json` (instance type, build commit,
   binary and image digests, analysis script hash), then
   `python3 ../crypto-autoresearcher/tools/experiment_protocol.py freeze protocol.json --commit auto --filed-before-affected-data` per the `experiment-protocol` skill
   (canonical checker at that path; the validated copy used here
   is `research/primefield-companion-schur-20261101/tools/experiment_protocol.py`, byte-identical to
   the canonical one I found, sha256 prefix `0f46a2b856bb`). Copy to `protocol.json` beside `PROTOCOL.md`.
3. Real linter: build `ecbench` at the frozen commit and run
   `ecbench plan --spec spec.json` (and `spec-m3.json`): every curve `registered=true`, every
   method resolves, the `prime-isogeny` towers build at each `(p, t)`, `columns` recorded. This
   container has no binary and may not run cargo; the job script repeats `ecbench plan` as a gate.
4. Resume checkpoint: `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (goal, this spec, run
   commands, stop conditions), `scripts/checkpoint.sh push`, `scripts/checkpoint.sh gate`.
5. Benchmark record: `python3 tools/benchrec.py init --slug prime-ic-algebraic-row --problem
   prime-ic-algebraic-track-row`, run launchers as `python3 tools/benchrec.py run RECORD -- ...`,
   `add-raw`, `metric` (unit `S`, `gae`, `fmul`, `count`), `close`, `report`, in the same PR
   (`benchmarks/progress/` regenerated; `bench-records` CI).
6. Commit and push the frozen protocol, spec, job script and request writer, then
   `python3 taskq/deploy/eks-karpenter/check-experiment-git --help` and keep its receipt
   (AGENTS.md section 9a).
7. Build identity: the deployed default image's ecbench predates the isogeny membership
   (BLOCKERS item 2). Build `taskq/deploy/eks-karpenter/build-bundle SOURCE_DIR COMMIT OUT` from a
   clean checkout at the frozen commit, publish and layer
   (`publish-image` / `oci-layer-publish.py`) with `spec.json`, `spec-m3.json` and
   `job-algebraic-row.sh` at `/opt/study/`, record the digest and the ecbench SHA-256 in `image.json`.
8. Pool check, then submit:

```bash
export AWS_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 \
       ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher
aws sts get-caller-identity            # account 104186811093, the executor's own identity
scripts/isolab_pool_check.sh --cpu 2 --memory 4Gi --pool cpu-spot --save "$RECEIPTS/pool.json"   # exit 0
research/prime_ic_leads_20261110/algebraic_row_spec/make-request.sh IMAGE.json "$PROTOCOL_SHA256" "$STAMP"
export ISOLAB_CLIENT_OUTPUT="$RECEIPTS/row"          # fresh, must not exist
taskq/deploy/eks-karpenter/remote/launch-remote-command research/prime_ic_leads_20261110/algebraic_row_spec/request-row.json
# status / receipts later: isolab.eks.command-status/v1 through the same launcher; Job 2 after Job 1:
REPLAY_INSTANCE=<other x86 family> ROW_RESULTS_PREFIX=s3://.../runs/<job>/<pod-uid>/ make-request.sh IMAGE.json "$PROTOCOL_SHA256" "$STAMP"
taskq/deploy/eks-karpenter/remote/launch-remote-command research/prime_ic_leads_20261110/algebraic_row_spec/request-replay.json
```
   (`validate-request` accepts only `isolab.eks.request/v1`; for a command request `run-command`
   validates server side, so there is no local validator, BLOCKERS item 7.) Under Claude Code the
   command goes through the task-card route (`experiment-director` freezes it,
   `python3 tools/task_card.py lint` must print `CARD OK`, `executor-mechanical` runs it unchanged),
   with the pool check as a card step. `wait_seconds` is at most 3,000, so the launcher returns
   before the Job ends: poll with the status request, never busy-poll, never resubmit under a new
   name (an ambiguous submission is reconciled against the original request).
9. Verify before anything is cited: request/image/binary identity, Job and Pod UIDs, S3 manifest
   hashes (`SHA256SUMS`), `plan.txt`, `session-row.audit.json` OK with every replay identical,
   Job 2's independent audit, A/A ratios exactly 1.000, C1. Then `ecbench compare`, `ecbench band
   --ranks` (flags confirmed with `--help` at run time), `experiment_protocol.py from-ecbench` and
   `check-export --verify-files`, reports with `research-visuals`, the PR (ready for review, auto-merge
   enabled), and a check-in on it until merged.

## 12. What was checked here

- `spec_check.out`: an ad hoc structural check (not the repository's linter) against the field
  names and rules read from `src/cryptanalysis/ecbench/spec.rs` (`deny_unknown_fields`, roles,
  arm-name charset, one reference, strings-only params): no errors; one warning (odd rounds with
  `order: alternate`, the same one the smoke spec carries). **The repository's linter is
  `ecbench plan --spec`; there is no standalone spec validator (searched `tools/`, `scripts/`,
  `docs/ecbench/`). It was not run** (no binary; cargo not allowed).
- `protocol_validate.out`: `experiment_protocol.py validate` on `protocol.draft.json`: OK, 0
  findings (draft status).
- The old reference-arm draft also validates with the same tool (sanity, not shipped).

## Owner decision needed before submission (added by the main session, 2026-11-11)

The spec is written as an `isolab.eks.command-request/v1` Job (counts-only row; wall time
descriptive, no native L2 or A/A admission), because the audited benchmark route caps a Job at
3,600 s and needs at least 5 rounds with a warmup. The owner chooses between this command-Job route
(D-ROUTE Option 0/1) and a larger L2 spec; nothing is submitted until then. The image also needs a
rebuild with the isogeny membership (`BLOCKERS.md`).
