# EXP-R31-APS-PUBLIC-Y: prospective direct-public-ordinate S3 evaluation

Date 2026-11-16 UTC. Frozen *before* modifying Rust code or running the following benchmark. This is a bounded **existing index-calculus** engineering experiment, not a non-generic or asymptotic novelty claim. [pcc:unverified]

## Prior measured source result and why change direction

The exact-source [four-bit public-jump result](APS_WINDOW4_PUBLIC_JUMPS_RESULT_20261116.md) completed 120/120 full cold logarithms; producer window4 APS reduced cold time only 1.2445% versus the binary-Jacobian incumbent, **failing** its preregistered ≥5% threshold. The public-source phase is 36.9% of window4 wall, so further window width tuning alone has limited leverage. The separate prime31 native S3 implementation, `algebraic_mode_factorized_index_variant`, already implements a **factorized algebraic two-root formula** and batch inversion across pair denominators. It currently re-extracts one Fp square root `y_i = sqrt(f(x_i))` for each first/second partition point, even though the prior public walk generation has already provided the correct `y_i` ordinate and checked curve membership.

For distinct abscissae `x,z`, the two unordered roots of the Semaev `S3(x,z,w)` quadratic are
`w_±=((y_x±y_z)/(x-z))² - x-z`.
Changing either square-root sign swaps those roots, so reusing *the already public, validated* `pt.y` without its `pow(y, (p+1)/4)` recomputation yields the identical unordered root pair, leaves the original-curve signed relation recovery unchanged and performs no new group arithmetic in root discovery. It remains generic MITM-equivalent. Test this rather than reimplementing a second algebraic S3 pair oracle.

Checked prior: PKM 2016 DOI 10.1007/978-3-662-49387-8_1 and APS 2018 DOI 10.1016/j.ffa.2018.01.009, the existing `APS_JOINT_SHAMIR_BATCH_RESULT_20261116.md`, `APS_WINDOW4_PUBLIC_JUMPS_RESULT_20261116.md`, previously merged #4391 and #4349, current #4451 (S9 zero-fibre), `M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`, norm-branch-rank pilot, and autotearcher negative-lead atlas references. Research KB/Conductor/EKS inaccessible in this GitHub runtime; actual source and primary publications are the fallback. No absence/novelty claim.

## Frozen implementation and benchmark

Reuse `native/examples/prime31_aps_one_relation.rs` and `PRIME31` exactly; add an **opt-in** `use_public_ordinate` S3 variant, while keeping current sqrt-lift incumbent unchanged. Require `y²==x³+ax+b` as a full field check for every reused ordinate; do not merely trust the source label. Count actual Fp square-root exponentiations as 0 in the new arm, and keep distinct numerator/denominator/infinity checks as before. Keep all pair-root enumeration, packed lookup, verification, label generation, identity/collision handling, and scalar recovery unchanged. Unit controls compare complete source/S3 outputs for a short partition, sign-symmetric root sets, and original-curve scalar replay.

New 24 distinct hidden-scalar public targets, fixed `target_seed=0x59334150534f5231` and `rho_seed=0x593352484f533231`, per-target five arms in rotating order:
0. old **window4 APS with sqrt re-lifts** (the exact-source winner of the prior window4 experiment),
1. new **window4 APS with public ordinate reuse**,
2. identical public-y A/A repeat,
3. existing unfolded rho32 (32-lane, 32 jumps, DP bits 4, 2m cap),
4. existing unfolded rho64 (same but 64 jumps).

All costs cold; charge all fresh public-source jump construction, operand checks, field arithmetic, S3 pair queries and misses, signed verification, and scalar replay. For every case require all 72 APS recoveries and **identical S3 root and signed witness transcripts**; the only permitted changed fields are instrumented sqrt counts, S3 oracle timings, cold wall, source/verification arithmetic counters and the opt-in flag. Independently check every recovered `[k]G=Q`.

Pre-registered scoped engineering gate: total candidate/old ratio ≤0.95, at least 18/24 paired candidate wins, A/A drift ≤5%, all 72 APS complete and transcript-identical. Include correct rho successes and censored caps; no fastest-frontier claim from a nominal win on a small 24-Q sample. Report all five totals and per-target records; retain losing/negative outcomes. Exact-source CI is **producer measurement**; no PCC-signed validation, isolated EKS Spot timing, external PKM/APS or separate tuned Wiener–Zuccherato rho is implied.

If this also fails ≥5% whole-call gate, stop microtuning fixed-S3 arithmetic without a new cost mechanism. Return to the genuine nonenumerative S9/K961-S5 target-dependent f-and-sign extractor, reusing #4228/#4349 Rust certified roots and the B4/B8/Dickson31 controls. No weak-curve substitution or misleading new-nongeneric claim.
