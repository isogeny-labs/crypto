# Prime31 run 55 — Dickson two-summand F4 yield ceiling: frozen protocol

Date: 2026-11-22 UTC (2026-11-21 PST). Scope: **the existing original-Rust Dickson two-summand prime-field index-calculus lane only**, not the separate seven/eight-leaf y-coset solver, RR9/F3 or any claimed novel solver.

## Existing implementations / research ownership
- Original source of the native two-summand tower F4 and prime31 curve arithmetic: `research/prime31_ic_20261115/native/lib.rs` and `src/cryptanalysis/f4_fp_tower.rs`, reused unchanged.
- Current Prime31 public-only RR9/F4 owner #4692, 14-variable Horner owner #4812, four-variable quotient owner #4822. None of their predicates is replaced with a two-summand oracle.
- The active signed25/coset F4 candidate and cold-build CI belong to #4901; this is a separate negative-yield control checked into the same ongoing PR.
- Literature: Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, Finite Fields Appl. 51 (2018), 168–182, DOI 10.1016/j.ffa.2018.01.009. Its one-relation optimization means this full-rank lower-gate is **not universal across all prime-field index calculus**.
- KB MCP unavailable in this runtime; direct GitHub files/PRs and the primary publication listing were consulted.

## Registered question
Even with a hypothetical instantaneous zero-error original tower-F4 oracle, can a random-shift, two-summand, fully rank-complete index-calculus pipeline using the largest available Dickson factor base (depth=10) plausibly beat the existing optimized **shared** Prime31 negation-BSGS baseline (historical 170.113 ms/1024 targets from #4793/#4532)?

## Deterministic execution and outputs
- Fixed E/Fp: p=2147483647, A=847514846, B=2087444724, #E=n=2147446831, G=(625037715,2100627251).
- Reuse original Rust `Base::build(PRIME31,10,...)`, which selects seed 3 and 509 y-canonical base points from 1024 Dickson roots. Include both signs; 1018 signed points.
- Enumerate **unordered** signed-point pairs, including same-point doubles; represent infinity explicitly. Check every point is on curve and compute the number of distinct group sums (not just a cardinality upper bound).
- Independent verifier: hand-written C++ group law with modular exponentiation, reading an independently generated base.csv, and frozen run50 `adaptive4096.csv` SHA256 `07dfd381c1d069f4610e907ca3e7f9922ebb599ed473a18dca8e010652bb6e6a`. This is reference enumeration **for the coverage audit only**, not offered as an index-calculus or ECDLP algorithm.
- Rust CI should build only the preexisting *small* `research/prime31_ic_20261115/native/Cargo.toml`, not the huge root Cargo target that timed out after 18 minutes in Actions #44989107767. Run release example with 80-second hard timeout.
- Exact result must be labelled [pcc:unverified] until original Rust and independent enumerations agree on all counts. Do not claim new discrete logs or F4 win.

## Statistical / mathematical acceptance gate
For uniform independent relation targets R in E(Fp), success probability is **exactly** the number of distinct pair sums divided by n. Expected number of nontrivial unordered-pair relation representations per R equals (unordered pairs minus inverse-to-identity pairs)/n, regardless of collisions.

Using Wald's identity on an independent uniform random-shift stream, the expected number of oracle calls to *accumulate* at least 509 nontrivial relation occurrences cannot be below `509*n/(pairs-identity_pairs)`. This assumes the algorithm insists on enough equations for full rank and counts **every** occurrence as potentially useful (overoptimistic). It does not prohibit single-relation tricks, adapted target selection, larger supports, or a different index calculus.

The historical 170.113-ms/1024-target shared-BSGS run is from a different execution; dividing it by this call requirement yields **a sensitivity bound** for the two-summand oracle, **not** a same-runner measured algorithm comparison. Remaining setup, collection, LA and descent costs make the actual ceiling stricter.

## Proposed next experiment after this gate
Focus resources on the existing PR #4822 four-variable RR9/Krylov sparse operator and #4812 14-variable Horner F3 with public-target-only blind root extraction (register costs incl. negative probes) rather than attempting millions of two-summand F4 invocations. Run independent source-bound Rust arithmetic CI and restore the blocked #4692 checkpoint before any large compute.
