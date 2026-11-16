# mllc: learnability control for LEAD-EXO-ml-learned-models (prepared, not run)

Status 2026-11-07: protocol drafted, generator and harness written, development check at N ~ 2^12 done.
**No scientific data exist. Nothing was launched.** All numbers here are `[pcc:unverified]`.

## Goal
Prepare the "learnability control on toy prime-order groups" of
`knowledge/leads/ecdlp/exotic/LEAD-EXO-ml-learned-models.yaml`: predict a bit of k from the coordinates of
Q = kP at N ~ 2^12..2^24, with positive controls of graded strength, a random-relabelling null, a preregistered
NULL / SIGNAL / UNDERPOWERED rule and stated minimum detectable effects. The round-2 memo item D4 predicts a
planted homomorphic leak is not learnable unless its multiplicative height is small, so the design measures a
height ladder instead of assuming the control is valid.

## Sources checked
- research-kb: NOT available in this session (`scripts/research-kb` has no executable). Not queried.
- `tools/lead_atlas.py --match` (crypto-autoresearcher sparse checkout): fails (`ModuleNotFoundError: orchestration`;
  `build_frontier_map.py` needs a package that is not in the sparse checkout). No match output; not recorded as "no hits".
- Read: `LEAD-EXO-ml-learned-models.yaml`, `LEAD-EXO-weak-class-instrument-calibration.yaml`,
  `LEAD-EXO-coordinate-statistic-leakage.yaml`, memo `docs/reports/leads-round2-20261104/new-leads-memo.md` item D4
  (lines 235-271), `research/ec_hardness_predictor_20261010/README.md`.
- Existing generators: the 40+ `research/ml_dataset_*` directories are relation/PDP and algebraic-target datasets, not
  (curve point -> log bit) data; none was reusable for this task. EC arithmetic was written fresh (exact, validated).
  Reused ideas from the calibration lead: planted control needed per instrument; a calibrated gate (the I1 A/A gate
  was miscalibrated there), so here the gate (PC-A) is explicit and the relabelling null is checked for bias.
- Conductor not installed (noted, not installed).

## What was done
1. `PROTOCOL.md` (draft freeze): arms, sizes, model classes, statistic, MDE table, decision rule, written predictions.
2. `mllc_ecgen.py`: prime-order curve search (p ~ 2^m, exact Legendre-sum point count, `#E` prime and != p, `N*P = O`),
   table-based exact Q = kP for arrays of k, on-curve and double-and-add cross-checks, F_p^* safe-prime group.
3. `mllc_harness.py`: arms, three models (sklearn logreg / HistGradientBoosting / MLP), binomial statistics with Wilson CI,
   relabelling null, `decide()`, `ladder()`.
4. `mllc_job.sh`: the full-ladder job (one process per size).
5. Development checks (this host, not claims): `dev/dev_m12.{json,log}`.

## Results (development check, N ~ 2^12, 3 curves, null R = 5; evidence class: measured, development only)
Generator validated at m = 12 and m = 24 (N = 16,147,123, curve built in about 50 s). Pooled over 3 curves, test n = 746 per
curve, MDE per curve 0.085, pooled 0.049 at alpha_cell = 4.2e-4 (derived).
- Curve arm: accuracy 0.47-0.52 for all three models and both labels; relabelling null means 0.487-0.522.
  Decisions: logreg NULL, mlp NULL, hgb UNDERPOWERED (hgb failed the gate: PC-A at delta = 0.10 reads 0.527 pooled).
- PC-A (explicit planted bit on curve features): pooled delta = 0.2 detected by all models; delta = 0.1 by logreg and mlp; 0.05
  by logreg; 0.02 by none (consistent with MDE).
- PC-B (Z/N affine): c = 1 accuracy 1.00 (all models); c = 2 0.81-1.0; c = 3 0.55-0.66; 5/3 chance; **random c is learned at
  N ~ 2^12 by hgb (0.61-0.73) and mlp (0.62-0.66), not by logreg**. So HD4's "random c reads chance" does not hold at 2^12
  (heights are ~sqrt(N) ~ 60 there); whether it holds at >= 2^16 is exactly what the ladder measures. Do not read any 2^12
  conclusion as evidence about 2^16-2^24.
- PC-D (Legendre in F_p^*): chance (0.51-0.52).
The harness is therefore able to separate "detects planted leak, curve at chance" (logreg, mlp) from "gate failed" (hgb), as designed.
Not measured: any size above 2^12 (the ladder), the per-fit scaling beyond one timing probe.

## Decisions made (and why)
- PC-A plus PC-B instead of one control: PC-A validates the pipeline on the curve's own feature layout (known advantage); PC-B
  is the lead's "homomorphic leak" and tests HD4. A NULL on the curve is scoped by both.
- Gate uses delta = 0.10 (largest advantage worth ruling out); failing it gives UNDERPOWERED rather than NULL.
- No hyperparameter tuning (would need its own split and multiplicity budget).
- N = 2^20/2^24 use 7e5 sampled points (4% of the group at 2^24), 3 curves per size; samples >> group are impossible at 2^24
  within budget, so memoriser-type leaks are out of scope (stated in PROTOCOL.md section 3).
- Frozen `ec_hardness_predictor` phases 0-3 were NOT executed here: that plan needs isogeny-class data and tooling
  (its README: "plan frozen, nothing run"), a separate job.

## Open threads
- Freeze: record sha256 of PROTOCOL.md / mllc_harness.py in the task card and `experiment.protocol/v1`.
- Image with python3.13, numpy 2.5.3, scipy 1.18.1, scikit-learn 1.9.1 (see Ready-to-run); resume checkpoint; benchmark record if
  the ladder is treated as a benchmark (it is a statistical experiment, not a timing one; director decides); pcc claim ids.
- Add the PC-B height computation (reduced a/b representative) to the output for the ladder plot; ladder currently labels arms by c.
- research-visuals report/graph/PDF after the run.
- The mlp (sklearn, 20 epochs) may be too weak to separate "unlearnable" from "undertrained" at large N; a longer-trained or
  torch variant is a possible follow-up if PC-B random-c reads chance at 2^16 and the mlp curve is not yet at its capacity.

## Errors
- First dev run hung without output under host load (CPU oversubscription by HistGradientBoosting threads); fixed by capping
  OMP/BLAS threads to 2 in the harness. The killed run left no data.
- A prediction in my first PROTOCOL draft (c = 2 gives ~0.75) was wrong (measured 0.81-1.0) and is rewritten before any
  scientific data. `NULL_BIAS` and `ladder()` were added after the dev run; the dev JSON predates `ladder()` (analysis re-done offline from it).

## Ready-to-run
Artifacts: `mllc_job.sh OUTDIR` (runs `python3 mllc_harness.py --sizes {12,16,20,24}` as four parallel processes,
logs versions and sha256 of sources and outputs). Estimate [derived from one 1e5-sample timing probe on a loaded host:
logreg 0.3 s, hgb 0.8 s, mlp 11 s per fit]: per 2^20 / 2^24 curve about 45 main + 100 null fits, mlp about 35 fits x ~55 s,
about 40 min per curve, so about 2 h each for sizes 20 and 24 (the 2^24 curve search adds about 50 s per curve) and minutes for 12 and 16;
wall about 2-2.5 h with 8 vCPU, 16 GiB RAM (about 2 GB per process), no GPU, no checkpoints needed (rerun a preempted size whole).
Cost: a few CPU-hours; spot suitable.

EKS command route (`launch-remote-command`, `isolab.eks.command-request/v1`): the request validator needs a digest-pinned
image from the isolab repository, `command` array (<= 256 strings), `cpu` in {2..28 even}, `memory`, `seconds`, and
`provenance.source_revision` = 40-hex commit. Job shape fits: `cpu: 8, memory: "16Gi", seconds: 14400, isolation_profile: "standard"`,
`command: ["bash","<repo path>/research/ml_learnability_control_20261107/mllc_job.sh","/results"]`. **Not ready**: the known
image (`Dockerfile.ecbench`, aws-cli base) has no evidence of Python, numpy or scikit-learn, and the sources must be in the image or
fetched by the command (committed to the revision named in `source_revision`). Needs either a per-run image with the libraries and this
directory, or a network route to PyPI (unverified). No request JSON was written because image digest and commit SHA do not exist yet;
it cannot be validated with `run-command`'s validator without them. Not launched.

## Shared files edited
None. New files only, all under this directory.
