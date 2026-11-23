<!-- Written by the main session from the validator's returned text, verbatim apart from this line: the validator's harness blocked it from writing this file. -->
# Validator report VAL-20261103-c31c76: plain JV sieve route vs matched negation-map rho at p = 1511

Verdict: passed. Gate-admissible: VALIDATION.json written, and
`python3 scripts/validation_gate.py evidence research/jv_sieve_negrho_20261102`
returns `valid`.

- Snapshot: main aa1c89ab2a. The study files are clean against it.
- Scope: weak class only, p = 1511 (l = 2^61.4). Accounting, not an improvement.
- Checker: check/src/main.rs, Rust, independent of fit/. Output: check_output.txt and
  recompute.json. Both rerun byte-identically from replay/.

## What was checked
- Inputs:
  - Cover rows are byte-identical to V2's raw/ic_1511_sparse.json (16 rows, seeds
    17-32, solved, correct, sparse).
  - MANIFEST.sha256 passes sha256sum -c.
  - graph.png is in the artifact store; its sha matches .artifacts.json.
- Freeze:
  - PROTOCOL.md, run.sh, MANIFEST and fit/Cargo.toml equal FROZEN.sha.
  - The binary jv_cover_rho_frozen (2a61a521) is present and matches.
  - The tool hash 99503e22 -> 788e3993 is a graph-only edit: the git diff
    73582ce306..30abe2bac8 touches only the SVG offset and two labels.
  - The timestamps (19:01:32Z / 19:04:03Z / T0 19:06:47Z) are clock-only. Git first
    holds these files at 73582ce306 (19:37:29Z), while the runs were in progress.
  - The design was already committed in LEADS.md at 3764c93e98 (18:50:54Z).
- Receipts (142 rho runs):
  - correct, not capped, negation, batch 16, r 256, dp 16, c_add_e 331.
  - muls_tuned = 24 mul + 18 sq + 62 inv, exactly.
  - muls_harness = 55 mul + 55 sq + 166 inv, exactly.
  - s and s+ (+ fp_adds + six native counters) to 1e-12. steps = walk_adds.
  - The file name matches (p, seed, run).
- Identity: l and c_add_e of all 16 new runs equal V2's row of the same seed.
- Budget: both rounds complete in 2571 s; no job over 5400 s.
- Control 3 (producer replay):
  - new_p1451_s1_run0 equals #3278's record in every field except wall_ms.
  - The s1 cut is that file's seed-1 row.
  - The all-seeds copy equals the #3278 file.
- Control 2: matched mean 0.8056, sd 0.4834, se 0.1209; theory 0.88796; z = -0.682.
  #3278 pooled z = +1.150.
- Test 1 (homogeneity, steps/sqrt l), validator / producer:
  - |diff| 0.1268, perm p 0.281 / 0.282.
  - KS D 0.2778, perm p 0.186 / 0.182.
  - KW H 1.924, perm p 0.170 / 0.169.
  - Result: homogeneous, so the reference is the pooled 142 runs.
- Validator replays (main build d21002a3, nonce draws):
  - B rho p=1511 seed 23 run 0: identical except wall_ms (621 s vs 1004 s).
  - #3278 p=1451 seed 1 run 6: identical except wall_ms (320 s vs 432 s).
  - Cover p=1511 seeds 21 and 32: identical to the V2 rows.
  - One build therefore reproduces both arms.

## Recomputation
fit.json: all 89 deterministic numbers (80 ratio cells, 5 reference means, z, 3 test
statistics) agree to 1.4e-15 relative. Primary set (seeds 17-24, pooled reference):
- S/rho U1: 0.006474 [0.005456, 0.008020] (x154.5 below rho).
- S/rho U2: 0.01556 [0.01311, 0.01927].
- S+/rho U1: 0.02685 [0.02428, 0.02976].
- S+/rho U2: 0.06452 [0.05835, 0.07151].
- S+/(rho S+) U1 / U2: 0.02261 / 0.04447.
- Band A U1 at w = 0 / 0.1 / 1: 0.00647 / 0.10710 / 0.12142.
- Band B U1 at w = 0 / 0.1 / 1: 0.00647 / 0.00934 / 0.01067.
- Band A U2 at w = 0 / 0.1 / 1: 0.01556 / 0.11653 / 0.12256.
- Band B U2 at w = 0 / 0.1 / 1: 0.01556 / 0.01193 / 0.01103.
- Matched-only reference: S/rho U1 0.00738 [0.00561, 0.01022].
- Every cell is below rho, for both seed sets and both references.
- At w = 1, layer A stays below parity unless the unemitted F4/descent/LA additions
  exceed k = 69 per unemitted multiplication.

## §5a counter list (mean per run)
- Cover (seeds 17-24): sieve_adds 5.61e9, sieve_lookups 2.94e8, enum_adds 4.72e9.
  F4/descent/LA additions have no counter; the layer-A proxy is 1.56e9.
- Rho (142 runs): canon_cmp 1.44e9, hash 5.75e9, dp_check 1.44e9, scalar_ops 4.33e9,
  table_ops 2.2e4, cycle_cmp 1.14e10, fp_adds 6.49e10.
- Byte counters: none on either arm.

## VALIDATION.json
- kind: negation.
- Protocol-named: pooled-if-homogeneous.
- Same unit: U1, F_p multiplications / (331 sqrt l) on both arms.
- Same binary or process: examples/jv_cover --exp sieve and --exp rhostrong; one build
  reproduces both arms on the replayed sample.
- gae_band (layer A, U1), in GAE per sqrt l:
  - IC: 0.005943 / 0.7089 / 7.034.
  - Rho: 0.9180 / 6.619 / 57.93.
- order_holds_across_band: true.
- What it admits: the plain sieve route at p = 1511 (V2 instances, seeds 17-24)
  against the matched rho. It does not admit study A's line-search arms or LEADS.md's
  combined numbers.

## Limitations
- The replay is sampled: 1/16 new rho runs, 1/126 #3278 runs, 2/16 cover rows.
- Homogeneity has low power; it is "not rejected". The matched-only reference gives
  ratios about 14% higher, still below rho everywhere.
- strongest_available rests on the generic-instance argument: alpha is uniform in
  F_{q^3} minus F_q and no subfield structure is imposed. j-invariants were not checked
  per instance.
- Cover F4/descent/LA additions are unemitted (an implementation gap). Layer A is an
  upper-bound proxy; layer B is the emitted lower bound.
- Memory is not reported for either arm.
- The run was local, not on isolab-eks.
- LEADS.md's "Combined" H S/rho U1 = 0.0042 (x240) is a product over different seed
  sets. Dividing directly on seeds 17-24 gives 0.00454 (x220); over 16 seeds, 0.00422.
  U S+/rho is robust (0.0141 vs 0.0142).
