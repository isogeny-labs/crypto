# Run66 protocol — three-summand support ceiling and public-target holdout (2026-11-22)

## Preexisting evidence and scope

Continuation of draft PR #4901, run55–65, on the **unchanged** ordinary prime-order E/F_(2^31-1): p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251). Reuse the original `prime31_algebraic_ic::Arithmetic` and `Base::build`; no new ECDLP backend. The previous frozen 509-point Dickson and 677-point Dickson+y^7 hybrid bases have exact 2-sum supports 518085 and 916469, with 13 and 88 independent free homogeneous equations, respectively.

Prior research: #4692 and #4822 (RR9 public-only root extraction), #4812 (14-var F3/Horner), #4809 (S3), #4891 (actual S4/F4 tiny-base benchmark), prior run65 small-shift correlations, and `prime31_anchor2_m6_support_validator.rs` (existing **generic** three-sum grader at a much smaller B31 base). Also compare Amadori–Pintore–Sala, Finite Fields Appl. 51 (2018), DOI 10.1016/j.ffa.2018.01.009; the one-relation idea is not novel. Research KB MCP is not available in this runtime; the repository and paper checks are not exhaustive.

## Mathematically guaranteed capacity

For a base with B canonical nonsymmetric points, allow both signs and repetition. The number of unordered signed triples is binomial(2B+2,3). The number of **distinct group targets** representable as sums of three signed base elements is at most min(n, binomial(2B+2,3)); equality is not assumed. For B=509 the bound is 176348140/2147446831 = 8.2120%; for B=677 it is 414635420/2147446831 = 19.3083%. This is an exact combinatorial upper bound; it is NOT a bound on all index calculus designs.

## Frozen comparator and validation procedure

1. Independent C++ affine EC implementation constructs the full unordered signed **pair** dictionary, 518671/917335 records respectively. Query for a three-sum witness by testing Q - signed_base[i] against the dictionary. This is **generic enumeration used only as an external ground-truth grader**, explicitly inadmissible as a non-generic relation extractor.
2. Public cohorts (neither candidate receives witnesses): 4096 SHA-256 seeded deterministic hash-to-curve x trials, using a deterministic parity choice, and the 4096 frozen W8 public targets from run55. The latter are constructed eight-leaf positives, not uniform randomness. Record both cohorts separately.
3. Independently replay every reported positive triple using Python big-integer EC law, check on-curve, point provenance and nG=O; check the Dickson triple-hit indices are a subset of the hybrid indices. Never equate failure to find a root with UNSAT outside this exhaustive generic dictionary.
4. Replicate on original Rust 1.100.0, without standalone EC arithmetic: parse the frozen 677-point fixture and 256-point prefixes of each public cohort; compare the prefix pair/triple counts to independently frozen C++ numbers. Rust validates every positive witness via `Arithmetic::add` and the genuine unchanged curve. No Rust speed comparison to C++ or BSGS is allowed.
5. Preserve failures, exact code, counts, sample widths, source commit and reproducibility receipts. Avoid claiming complete ECDLP, actual F4 roots, a relation rank, or an advantage from a generic precomputed dictionary.

## Success and next admissible experiment

Success of THIS gate: exact combinatorial ceiling, reproducible public-holdout coverage and independently verified original-Rust arithmetic agreement. A genuine attack requires a **nonenumerative** public-only S4/F3/RR9 root extractor with no generic pair dictionary or planted index, 46-positive/18-negative registered gate, actual rank and independent [k]G=Q recovery, and all-in same-host IC/BSGS/rho timings. Do not open a competing extraction implementation when #4692/#4812/#4822 own that work.


## Rust fixture amendment (recorded before native Rust CI, after independent C++/Python census)

The native Rust source uses **existing** \`Base::build(PRIME31,10)\` plus the frozen 168 public y^7/cubic-fiber points in \`RUN66_Y7_EXTRA168.csv\` to reconstruct the exact 677-point hybrid without introducing a new base builder. To avoid duplicating a 4096-point sensitive calibration fixture, native Rust tests two **additional**, independently frozen, public x-coordinate walks (256 x-lifted points per walk): start=17/step=1 and start=100000003/step=1000003, advancing until the next quadratic-residue RHS and alternating y parity. These are not the original SHA-256/W8 cohorts; their independently computed C++ prefix hit counts are B509: (0 pair,15 triple)/(1 pair,16 triple), B677: (0,43)/(1,41). The full 4096+4096 original cohorts remain validated by independent C++ and Python only. Do not describe the Rust walks as reproducing the full SHA-256 or W8 holdout.
