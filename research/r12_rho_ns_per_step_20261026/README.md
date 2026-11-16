# R12: ecbench rho trend versus fold-backend rho speed (2026-10-26)

Register item R12 of `docs/audits/REPAIR_FRAMEWORK_20261016.md`: the audit saw
the N83 headline rho at about 599 ns/step against about 240 ns/step for the
in-repo fold-backend rho on the same CPU, and the ecbench rho trend had never
been reconciled with that. This note measures the gap on this machine, explains
what it is, and lists what it touches. It is a documented explanation, not a
rebased trend (see "What was not done").

## 1. What the ecbench rho is

| Name | Where | Field kernel | Walk |
|---|---|---|---|
| `rho.signed_frobenius_strong` (the "strong reference") | `src/cryptanalysis/ecbench/methods.rs` -> `koblitz_strong_rho::StrongRho` (u64) / `WideStrongRho` (u128, `WideGf2`) | `Gf2` / `WideGf2` | 32 lockstep lanes, batched inversion, distinguished points, signed-Frobenius orbit (A = 2n) |
| `rho.signed_frobenius` ("tuned") | `ecbench::methods` | scalar `Gf2` | one walk, one inversion per step, look-ahead, cycle escape |
| `rho.negation`, `rho.plain` | `ecbench::methods` | prime or generic group | one walk, one inversion per step |
| N83 worker `wide` | `examples/n83_rho_optimization_eval.rs` | `WideGf2` (`sparse_xor_fold_loop` reduction): same code as ecbench's wide strong rho | strong |
| N83 worker `fold` | same example | `Gf2_128` from `semaev_decomp` (CLMUL/PMULL tail folds) | strong, same trajectory (counters equal) |

"Fold backend" exists only as `FoldField` in that example, at m = 83. No
ecbench method uses it.

The ecbench "trend" has two forms: the per-curve ratio table (`ecbench table`,
`compare`) and `ecbench trend` (a WLS fit of `ln(ratio)` against `ln r`,
`ln n` from `compare --save` files; `src/cryptanalysis/ecbench/trend.rs`). Both
take their rho cost from whichever rho arm the session's spec names, in
`ecbench.gae` (counted) and, for wall comparisons, in ns per GAE.

## 2. Measurements (this machine, paired)

Host: Apple arm64 (`host.json`: aarch64, pmull), macOS, one process, **L0**
(`--cpus none --allow-busy`): the host was not quiet or pinned, so these are
descriptive, not admitted wall claims. No Linux PMU or PSI exists here. Binaries
built from `a559bc4a01` (see "Environment").

### 2a. ecbench rho, small Koblitz sizes (`spec.json`, `data/ecbench_records.jsonl.gz`)

ecbench session `ECBS1h6b04e7f6ec21`, 378/378 records verified. Curves
a = 0, n = 41/53/61, 6 public targets each, 6 measured rounds after 1 warm-up
(`order: alternate`, even rounds so the order is balanced), three arms
interleaved: `rho-strong`, `rho-tuned`, and `rho-strong-aa` (an A/A control of
the strong arm). ns/step = sum of `solve_wall_ns` / sum of walk steps
(`walk_steps` for strong, `steps` for tuned). These include the solve's own
setup (jump table etc.); at 10^5 steps that is a visible share, and the same for
both arms.

| n | strong ns/step | A/A strong ns/step | tuned ns/step | tuned / strong (ratio of means) | cells (curve, target) where tuned is not slower |
|---:|---:|---:|---:|---:|---:|
| 41 | 95.0 | 89.1 (A/A 0.94) | 519.9 | **5.48** | 0 / 6 |
| 53 | 92.2 | 92.3 (A/A 1.00) | 861.5 | **9.34** | 0 / 6 |
| 61 | 102.5 | 109.6 (A/A 1.07) | 979.7 | **9.56** | 0 / 6 |

The unbatched tuned rho is 5.5 to 9.6 times slower per step than the strong
rho at these sizes, and loses in every cell. Its 520 to 980 ns/step is the same
size as the audit's 599 ns/step "rho baseline". The A/A control stays within
+/-7 %, so the gap is far outside noise. Different walks, so per-step cost is
compared, not time to solution.

### 2b. N83 worker, `wide` (what ecbench's wide strong rho runs) versus `fold` (`n83_abba.sh`, `data/n83/`)

Two public targets of the legacy N83 fixture (`research/n83_evaluation_20261012/
optimization/fixtures/training_{1,2}.jsonl`, walk seed 202610128311), lanes 32,
dp_bits 4, E0. Run order ABBA / BAAB per target, so 4 runs per arm per target.
Wide and fold walk the same trajectory (identical `group_additions` in every
run); ns/step = `online_interval_ns` / `group_additions`.

| target | steps | wide ns/step (range) | fold ns/step (range) | wide / fold |
|---|---:|---:|---:|---:|
| training_1 | 16,892,303 | 217.7 (198 to 237) | 201.7 (184 to 225) | **1.08** |
| training_2 | 6,561,878 | 197.8 (196 to 200) | 185.4 (181 to 193) | **1.07** |

Fold is faster in 2 of 2 targets and in most pairs, but the ranges overlap on
training_1 (the host was busy), so 1.07 to 1.08 is a small, weakly supported
gap on arm64. For reference, the frozen 2026-10-12 x86 kernel panel
(`research/n83_evaluation_20261012/optimization/records/panel/kernel`, nine
paired targets) gives wide 435 and fold 333 ns/step (1.31), and the held-out
default/selected process-cost ratio there is 1.76 (CI 1.54 to 2.03). Those are
other hardware and are quoted, not re-measured.

### 2c. What is not reproduced

The audit's 599 versus 240 ns/step (2.5x) was not reproduced. On this arm64
host the fold-versus-wide gap is 1.07 to 1.08, not 2.5. The 599 ns/step
implementation is not identified in the register beyond "the rho baseline of
`autolab/protocol.json`" (a `koblitz_rho_fixture` `signed_frobenius`/`packed`
config); I did not run that fixture here, so "the 599 ns/step rho is the
unbatched tuned/fixture rho" is a **hypothesis** supported by the size match in
2a, not a measurement. The x86 PCLMUL fold path (`clmul_walk`) does not exist on
arm64, which may account for part of the difference.

## 3. Explanation of the gap (status: DERIVED from 2a and 2b, hypothesis flagged)

1. Two different things were called "rho speed". Within the strong algorithm the
   field backend matters by 1.07 to 1.76x depending on hardware (2b). Between
   the unbatched per-step-inversion rho and the lockstep batched-inversion
   strong rho the per-step cost differs by 5.5 to 9.6x at n = 41 to 61 (2a).
   The audit's 2.5x sits between them.
2. `ecbench` never used the fold backend. Every ecbench rho cost is the
   `Gf2`/`WideGf2` strong rho (or a tuned or generic per-step-inversion rho).

## 4. Affected comparisons (MEASURED-UNMATCHED)

Counted GAE does not depend on ns/step: the charged unit is the group addition
whatever the field kernel costs. So the label applies to **wall-clock** IC-versus-rho
statements (ecbench `ns_per_gae` ratios, `compare` `wall` fields, and any figure
turned into seconds) whose rho arm is not `rho.signed_frobenius_strong`, or, for
the strong rho, was priced by a backend slower than the fastest in-repo one. For
a counted-GAE ratio against `rho.negation`, `rho.plain` or `rho.signed_frobenius`
the rho is a different, weaker algorithm than the strong reference of
`docs/ic/boundary_targets.json`, so those rows are also not matched to the
strongest rho (AGENTS.md §5a).

`data/affected_specs.tsv` lists, for every committed `ecbench.spec` that has an
`ic.*` arm and a rho arm, the study directory, spec count and the rho arms. 745
specs in 68 directories; 37 directories pair IC only with a non-strong rho.
Directories with only non-strong rho arms (the most exposed):

bitop_ladder_20261019, certicom131_ic_program_20261020,
conductor_gap_cm_orbits_20261015, conductor_gap_cm_pairs_20261010,
cyclotomic_falsifier_20261015, drastic_prime_conductor_gap_20261012,
ecbench_conductor_gap_20261012, ecbench_factor_bases_20261013,
ecbench_rootfinding_{alloc_20261014, arity_20261013, batchtable_20261013,
lanes_20261013, tuning_20261013}, eccp131_bsgs_ic_20261020,
eks_crypto_validation_20261019, fiber_ic_scaling_20261011,
full_volcano_graphwide_ic_20261017, gaudry_quartic_d4_{char2_20261015,
cost_20261014, ecbench_20261013}, generator_effect_ic_20261019,
ic_linalg_pipeline_20261019, ic_next10_20261014, ic_oracle_setup_blocked_20261020,
ic_planner_validation_20261020, ic_techniques_20261012,
isogeny-volcano-cost-v2_20261017, isogeny-volcano-cost_20261017,
isogeny_conductor_gap_round{2_20261011, 3_20261011}, pkm_isogeny_tower_ecbench_20261015,
pkm_p224_kummer_shape_20261022, pkm_supersingular_aux_20261015,
pkm_tower_ecbench_20261013, sparse_la_timing_20261018, twist_index_calculus_20261020,
volcano_ecdlp_census_20261017.

The other 31 directories also carry a strong-rho arm and are affected only in
their wall comparisons if the strong rho ran a slower field backend than the
fastest available (m = 83, wide versus fold).

This list is of specs, not of claims: whether a given row quoted a wall ratio
was not checked per row (see below).

## 5. What was not done

- The trend was not rebased. That needs the ecbench rho arm replaced by the
  fastest rho (and wall ratios re-formed) over about 745 specs; no fold backend
  exists in ecbench for any size, and an L2 host is needed to admit wall time.
  Doing it is a campaign, not a repair.
- The x86 PCLMUL/fold path, the audit's 599/240 pair, and the
  `koblitz_rho_fixture` baseline were not run.
- No row-level check of which claims in the 68 directories quote a wall ratio.
- Negation-map rho on prime curves (`rho.negation`) was not compared with a faster
  in-repo prime rho; `src/ecc` Pollard rho is a toy (docs/perf/RESULTS.md: 8.5 ns/step).

## Environment

- `origin/main` (`baa1951357`) did not compile at measurement time: unclosed
  delimiters in `src/cryptanalysis/binary_semaev_chain_sat.rs`,
  `ecbench/methods.rs` and `ecbench/workload.rs` (merge damage). Measured at
  `a559bc4a01` (main, 2026-10-26 12:00), which compiles. This PR changes no source file.
- GitHub Actions did not run (billing); AWS credentials rejected, so no S3 upload.

## Reproduce

`python3 analyze.py` rebuilds `results.json` and the graph from `data/`. To
re-measure: build `ecbench` and the `n83_rho_optimization_eval` example at a
compiling commit, then
`ecbench run --spec spec.json --out <dir> --cpus none --lock <file> --allow-busy`
and `./n83_abba.sh` (edit the paths in the script).

## Capsule restoration provenance (2026-10-28)

The committed `data/session.json` originally lacked the session's `spec.json`
and `plan.json`, so the repository-wide evidence audit could not check this
otherwise complete capsule. The restored `data/spec.json` is a byte-for-byte
copy of this study's checked-in `spec.json` (raw SHA-256
`a4d1fbabd5072bfcbfc127aff263d67be7129ce5474fb7e35eec26b6d0f883c4`).
Expanding it through the repository's deterministic `Spec::from_json` ->
`spec::plan` -> `runner::PlanDoc` serialization at
`f6ef59053567470f313b80f6438fd6524a746d01` reproduces the session's pinned
spec SHA-256
`68278bcbc7c4c17e893f17ab16743be91f091ab60f9a6659cd064d0c068993d6`
and plan SHA-256
`77763dc5f2786e77c9deb2ed4beb18e5c0167ad211ca987fe170924d57cded28`.

The repository audit script now accepts the capsule's existing compressed
`ecbench_records.jsonl.gz` artifact name and materializes `records.jsonl`
only in its temporary audit copy. The committed artifact manifest and ignore
file retain their original bytes. Decompression produces 378 records with
the session-pinned SHA-256
`bed8044b3395f97b543a2451724d3c94b3475e30fc8cd73a7e372ecee5443807`.
This restoration reran no trial and changed no measured record.
