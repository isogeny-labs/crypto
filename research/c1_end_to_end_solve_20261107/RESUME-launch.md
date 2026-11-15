# Resume: crypto @ research/c1-end-to-end-solve-20261107

## Goal
First end-to-end index-calculus measurement (relations, linear algebra, discrete log) of the
twisted all-lift line u.F_q against the standard base F_q on a = 0 curves over F_(2^m)^4
(C1 family B), against the matched strong negation rho, in ecbench's counted unit, on isolab-eks.

## Lead / hypothesis
Per-trial yield of the twisted line is about 16x the standard base (PR #2704,
research/c1_twisted_line_ladder_20261027); per-trial Macaulay cost ratio 0.961. Hypothesis P1: the
end-to-end S ratio twisted/standard is below 1 (point value predicted in [0.1, 0.71]).

## Spec
research/c1_end_to_end_solve_20261107/{PROTOCOL.md, protocol.json, specs/c1-e2e-m5.json,
specs/c1-e2e-m7.json}. Code: src/cryptanalysis/ecbench/xline_bin.rs (bin-x-line base,
gaudry-x-bin oracle, f2m4_a0 curves), dispatch in methods.rs solve_ic_f2m4, CurveSpec::F2m4A0 in
workload.rs.

## State
Implemented and tested locally (xline_bin 3/3, workload/registry 17, f2m4 15, band/compare 23).
Dev smoke m=5 (target_seed 99, not evidence) verified and replayed. Protocol frozen at commit
234e6193833f9db0f3811e79cfae61c6f743dee8 (protocol_sha256 114ac2b9...). No measured data.
Route: GitHub isolab-experiment.yml on main (AWS bridge denied for 590183823895/adam).
Held: scripts/checkpoint.sh gate AccessDenied (no upload token); coordinator raising with owner.

## Next steps
1. Once the checkpoint gate passes: benchrec init for each dispatch.
2. Dispatch isolab-experiment.yml on main twice: source_revision=<pushed sha>,
   spec_path=research/c1_end_to_end_solve_20261107/specs/c1-e2e-m5-std.json and
   .../c1-e2e-m5-tw.json, instance_type=c8a.2xlarge, seconds=3600.
3. Fetch sessions into research/c1_end_to_end_solve_20261107/sessions/, run analyze.sh STD TW.

## Runs in flight
None.

## Stop and success conditions
PROTOCOL.md "Decision rule" and "Stopping rules"; budget USD 10 total (none spent).

## Open questions
Frobenius-folded rho for subfield (family A) curves is not implemented on this tower.
