# Prime31 signed W8 public-polynomial RR9 gradient gate — frozen protocol (2026-11-19 UTC)

Status: **prospective**; the frozen design predates the new source-specific CI experiment. Original curve only; no deliberately weak curve.

## Hypothesis and scope
Evaluate whether the public-target-conditioned Riemann–Roch norm gives a practical non-generic F4 front end with a cheap exact Jacobian. Implement an inversion-free *algebraic predicate and differential*, **not** an unknown-index finder. Reuse the native `prime31-algebraic-ic` Rust crate at the original pinned EC arithmetic source `1df714645aa683cf975bc76fcacb9842cb4780fd`, p=2,147,483,647, a=847,514,846, b=2,087,444,724, subgroup n=2,147,446,831, cofactor 1.

For public R=(r,s) and 7 coefficients z=(a0,..,a3,v0,v1,v2), set A(X)=sum(a_i X^i), V(X)=X^3+sum(v_i X^i),
M_R,z(X)=(X^2+rX+r^2+a_curve)V(X)^2-(X-r)A(X)^2-2sA(X)V(X).
For each one of 8 independently weighted signed slots, use the **eight public unsigned x-coordinates** x_(j,b) and compute H_j(R,z)=product_(b=0..7) M_R,z(x_(j,b)).
All x-coordinate inputs are from the public original-curve factor base and the scalar-weight map. The candidate API receives **only R, z and public support**. Inversions and EC sums in H and its Jacobian must be zero. The derivative is computed analytically and with prefix/suffix products, and replayed by a separate truncated-dual-number implementation.

## Registered experiment
- Old weights (1,3,5,7,11,13,17,19) and ternary (1,3,9,27,81,243,729,2187); B8 first canonical rational-x points; verify exactly 64 distinct unsigned support x values per arm.
- For each arm, 16 deterministic *planted grader-only* signed 8-point witnesses; derive R by true EC group sum, solve the 7-coefficient chart from the eight signed leaves **outside the candidate API**, and require H_j=0 for all 8 slots. Record Jacobian rank over F_p, not an inferred generic rank.
- Apply the predetermined z_0+1 mutation to each chart; require at least one H_j nonzero on each negative.
- Independently compute H and all 8×7 partials as coefficients of epsilon in F_p[epsilon]/epsilon^2 by direct truncated polynomial products, with no reuse of the analytic-gradient subroutine. Record exact agreement.
- Count candidate polynomial invocations and elapsed time separately from support generation and planted chart interpolation. No F4 or unknown-index extraction is run in this protocol.
- Derive and test the universal support-count inequality for any subgroup order n and a fixed m-slot B-support representation: uniform hit probability <= min(1,(2B)^m/n). A negative scaling result **does not prove a lower bound for arbitrary non-generic ECDLP algorithms**.
- Preflight mathematical properties with separately authored Python original-curve affine arithmetic and finite-field elimination. Run source-bound Rust 1.100.0 native 40-library-tests and dedicated example tests in GitHub Actions. Retain raw JSON artifact. Independent signed PCC validation remains pending.

## Falsifiers and no-go claims
- Any planted H_j != 0, any derivative mismatch, duplicate support x, corrupted group identity, unexpected chart inconsistency, or mutated chart with all eight H_j zero fails this experiment.
- The existence of a full-rank Jacobian **does not construct a chart from a public target**, bound F4 complexity, or show a faster ECDLP solver. No generic 4+4 lookup is used by the public polynomial evaluator, but planted fixtures **do** use secret witness indices to construct comparison data.
- There is no comparison to optimized shared BSGS or rho in this algebraic-component gate. The prior complete generic-grader 1024-target ternary Rust batch took 9.128 s versus old 11.516 s, but this is **not** the non-generic frontier. Frontiers require full rank31 and 1024 unknown logs with actual **public-only** unknown-index chart extraction, all costs, and same runner.
- The next pass requires an actual degree-bounded polynomial-system solver of H_0=...=H_7=0 whose inputs exclude any witness, chart, generic sum-table, target discrete log, or oracle answer; run the frozen 46/18 B8 public panel and independent replay, then four-window full batch.

## Source and prior-lead audit (2026-11-19)
- isogeny-labs/crypto: #4692 (RR9/F4 proposal, no source-verified public chart solver), #4728 (existing norm/RR9 and generic B8 windowed control), #4770 (same-runner ternary generic-grader 1.262x), #4532 (matched shared BSGS 170 ms / 1024 targets), #4714 (M6 incomplete relative to BSGS), #4722 (formal support bounds), #4720 (signed pair projector).
- isogeny-labs/crypto-autoresearcher: SA-04 and SA-18 norm/determinant negative screening; SA-19 Edwards-coordinate transfer obstruction.
- isogeny-labs/cryptanalysis: binary Semaev/SAT methods do not establish a comparable prime31 unknown-index extraction.
- Published: Petit–Kosters–Messeng (PKC 2016, doi:10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala (FFA 2018, doi:10.1016/j.ffa.2018.01.009).
- Shared research-KB MCP and EKS are inaccessible from the current connector runtime; primary GitHub sources and publisher records were used instead. No novelty clearance or signed validation claimed.

The purpose of this experiment is to move from informal polynomial proposals to an auditable **non-generic algebraic front-end**, while expressly not confusing that front-end with the still-missing cheaper complete prime-field ECDLP solver.
