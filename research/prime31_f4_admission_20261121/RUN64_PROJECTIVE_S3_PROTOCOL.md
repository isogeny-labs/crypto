# Prime31 run64 — projective S3 evaluation versus batched affine extraction (frozen protocol)

Date: 2026-11-22 UTC. Owner: existing draft PR #4901, unchanged original Rust crate `research/prime31_ic_20261115/native`.

## Exact, limited question
Run63 priced normalizing every public Q+kG into affine x before any algebraic work. For a fixed public factor-base pair (x_i,x_j), can an Semaev S3 **zero test** instead be evaluated directly on the project's Jacobian state, and is that cheaper on the same host? This is a necessary-component ablation, **not** blind relation extraction or a complete ECDLP solver.

Curve: original prime-field cofactor-1 `PRIME31`, p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251). No weak-curve substitution.

S3(x_i,x_j,x)=A_ij*x^2 + B_ij*x + C_ij, with
- A_ij=(x_i-x_j)^2,
- B_ij=-2[(x_i+x_j)(x_i*x_j+a)+2*b],
- C_ij=(x_i*x_j-a)^2-4*b*(x_i+x_j).
For a Jacobian point `(X:Y:Z)`, `x=X/Z^2`, so `Z^4*S3(x_i,x_j,x)=A_ij*X^2+B_ij*X*Z^2+C_ij*Z^4`.
Zero testing is equivalent when Z != 0. Infinity must be excluded; S3 zero does **not** prove unique sign orientation or factor-base membership beyond the particular pair.

## Reuse / prior-art audit
- Reuse `prime31_algebraic_ic::{check_curve,Arithmetic,Base,PRIME31}`; compare every tested group endpoint with the original Rust arithmetic. Reuse the already source-validated fixed-G Jacobian kernel and batch-256 normalization design from run63; do not create a separate generic EC or F4 backend.
- Pair coordinates are drawn from the **actual public Dickson depth-10 factor base** (not privately known scalars). Use public group-law sums/negations as positive correctness controls, and mismatches/zero-relation negatives.
- S3 follows Igor Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004), https://eprint.iacr.org/2004/031; prime-field one-relation prior art: Amadori–Pintore–Sala (2018), https://doi.org/10.1016/j.ffa.2018.01.009.
- Existing repository owner lanes: #4391 (Jacobian/batch inversion already), #4809/#4846 (S3 rootfinding), #4692/#4812/#4822 (blind RR9/Horner F3), #4891 (small-B F4). No algorithm novelty claim from changing an evaluation chart. Research-KB MCP unavailable in this GitHub-only runtime; reviewed reachable PR reports.

## Frozen benchmark and verification
- Source-specific GitHub Actions on Ubuntu runner, Rust toolchain 1.100.0, `--locked --release` in the original slim crate. Run 250000 +G steps per rep; 5 reps; rotate arm order. No scalar leak or precomputed group-sum table in any timed arm.
- Seven timed arms: (0) shift-only; (1–3) shift+homogeneous S3 K in {1,4,16} public coefficient pairs; (4–6) shift+batch-256 affine X materialization+affine S3 K in {1,4,16}. Both S3 arms must consume every output, including negatives, with black_box/digest.
- Verify 40 seed chains × 192 increments against original Rust group law, including O,+G,-G,order-wrap, plus all benchmark final endpoints. Independently check affine-vs-homogeneous S3 residue identity and 16 actual `P_i+P_j` controls. Fail closed on any discrepancy. Include at least two distinct source-specific CI executions before asserting timing stability.
- Report per-arm five raw ns/query, within-run paired medians and relative costs; use no timing claim until CI has actually executed. Count memory footprint only if measured. Any cost projection to 4699949 synthetic run59 probes and historical 170.113ms shared BSGS is **cross-host sensitivity**, never a solver win. The K=1/4/16 tests are fixed-pair S3 polynomial evaluations, **not an unknown-index Semaev oracle** and not a faithful proxy for F4 root extraction.

## Decision gates / proposed next experiments
1. Determine whether homogeneous evaluation is actually faster at K=1, and measure crossover by K=4/16. A null or loss is a useful falsifier of projective-only extraction optimization.
2. If a meaningful K advantage survives independent run repeat, insert this exact homogeneous evaluation into the existing #4809 S3 rootfinding path without table enumeration; require public-only root+sign recovery, failed probes and independent group checks.
3. Benchmark full 1/32/128/1024 unknown Q, target-log recovery and natural relation-rank completion including setup, failures, GPU/CPU time, memory and same-host strongest optimized IC, shared BSGS and rho. Do **not** promote this microbenchmark to complete-solver evidence.
