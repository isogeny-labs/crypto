# Prime31 run62 — complete and Mersenne-folded mixed-Jacobian shift protocol
Frozen 2026-11-22 before candidate source and timing. Status: **registered**, not yet executed.

## Objective and gap
Run61 measured 26.943 ns per fixed-`G` Jacobian shift against 282.940 ns original Rust affine, on the unchanged cofactor-one ordinary curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. But run61's `Jac::add_fixed` **panics for H=0**, so it cannot be integrated into a *complete* ECDLP implementation without handling `O`, doubling and cancellation. Run61 also uses `a*b % P` and `(a+P-b)%P` in its specialized arithmetic.

The original Rust APS solver already implements Mersenne field reduction and standard Jacobian plus batch affine normalization (`research/prime31_ic_20261115/native/APS_JACBATCH_FRESH_REPORT_20261116.md`); this is **not a new group-law method**. Research question: can we make the faster run61 primitive mathematically complete **without erasing its speed advantage**, and is explicitly folded 31-bit arithmetic worthwhile at all? Falsifiable by same-process paired Rust benchmark.

## Defined candidate
`Fp31` canonical residues <p. For `t=a*b < p²<2^62`, set `s=(t & p) + (t >>31)`, then conditional subtract p (at most once). For `a+b <=2p-2`, conditional subtract p once. For `a-b`, conditionally add p. Implement **one shared source-linked Rust example**, three arms:
1. Existing original affine `Arithmetic::add` (reference / real-curve proof oracle).
2. Previous original run61 `Jac::add_fixed` (partial, use only safe nonexceptional fixed-G benchmark to compare; cannot be designated the solver).
3. Complete `Option<Jac>` with the same mixed-Jac formula plus identity, `H=0,R=0` doubling, `H=0,R!=0` cancellation. Fast Mersenne-folded add/sub/mul. Preserve full n-periodic behavior.

## Correctness gates
- p arithmetic compare fast primitives against independent `u128` modulus on **at least 100,000** deterministic pseudo-random and boundary operand pairs; test 0,1,p-1 and products near `(p-1)^2`.
- Reference group-law checks: `O+G=G`, `-G+G=O`, `G+G=2G`, `(n-1)G+G=O`; all checked against existing original `Arithmetic` (including infinity).
- Seeded **>=64 public initial scalars** including 0,1,2,n-1,n-2 plus randoms; **>=64 consecutive +G steps each**, with each transition replayed by original `Arithmetic::add`. Known k only constructs/grades the correctness fixtures, never enters an ECDLP algorithm.
- Do not claim verified if any mismatch/panic/timeout. Record source SHA and runner details.

## Performance gates
Same-process 5 repetitions of at least 250,000 repeated public `+G` steps per arm on an identical input chain, with each arm finishing at an equal independent checked endpoint. Arm order rotated or deliberately same as run61 to expose source-comparison limits. Print five raw ns/shift for reference, partial, complete folded. Verify endpoints via `Arithmetic::scalar`. Do not turn this into synthetic full solver timing. Benchmark reports assume zero exceptional events for the long chain; separate adversarial tests cover exceptional inputs. Report separate **complete / partial** slowdown and **original affine / complete** speedup. This is an arithmetic component benchmark only, no blind F4 or qualified index-calculus improvement.

## Admission and failure decision
Accept complete implementation as the new public-shift primitive only if all arithmetic and exceptional tests pass and complete median < original affine median. If complete median > 36.195 ns, the run59 4,699,949-query / 170.113-ms historical BSGS **cross-run sensitivity** is already exhausted by shifts alone (not universal lower bound); if <=36.195 ns, this is merely necessary, not sufficient, because factor-base, algebraic extraction, rank and target recovery remain unpaid.

## Prior leads and research classification
- #4391 (APS Jacobian batch and Mersenne) already has related engineering: not novel.
- #4901 runs55–61: two-summand support, 88 free rows, partial-rank quotient and exact support; prior 10.5x partial mixed component.
- #4692 / #4812 / #4822: owns blind public-only S3/RR9 selected root extraction; do **not** duplicate that effort or substitute precomputed group-pair lookup.
- One-relation prior art APS 2018 DOI:10.1016/j.ffa.2018.01.009, and prior negative prime-field CAS results #4373/#4383. Research-KB MCP not available here; no exhaustive novelty claim.
- Complete Prime31 1/32/128/1024-target solver, original-ideal root extraction, optimized BSGS/rho/IC matched comparisons and independent PCC remain the acceptance requirements for a cheaper non-generic ECDLP claim.
