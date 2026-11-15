# Prime31 balanced m8: factorial root-multiplicity and symmetry-reduced oracle experiment

**2026-11-15. [pcc:unverified].** Exact finite-field producer-local check and mathematical consequence. No new relation oracle, Rust execution, F4 completion, lower cold-cost result or independent signature is claimed.

## Ordinary prime-order test

Field p=2,147,483,647; E: y^2=x^3+847514846x+2087444724, full prime order n=2,147,446,831, cofactor 1. G=(625037715,2100627251), public Q=(1547953785,1667575595). The native Rust depth-six Dickson base contains B=31 usable nonzero-y points, terminal 933677148; this is **not** the deliberately weak smooth-order curve.

Use the *first two previously recorded* natural-positive witnesses from M8_SPLIT_CONTROL.results.json, with the **same public masks** a=806206853, b=542783019 and group sum R=[a]G+[b]Q=(903384438,1336077056). Packed four-list codes are (18903234,258036944) and (37012034,107578577). They have **two distinct eight-element index sets**: {2,3,7,8,16,19,21,24} and {2,9,12,13,17,19,24,26}, with each recorded sign orientation retained.

For an ordering (P0,...,P7) of these eight signed points define U=P0+P1+P2, V=P3+P4+P5, W=P6+P7. The four balanced equations are

    S4(x0,x1,x2,x(U)) = 0
    S4(x3,x4,x5,x(V)) = 0
    S3(x6,x7,x(W)) = 0
    S4(x(U),x(V),x(W),x(R)) = 0.

Every permutation preserves U+V+W=R. An independently authored producer-local replay checked **all 8! = 40,320 permutations of each of the two witness sets**: no U,V,W or R was infinity and all **322,560 polynomial residuals were zero**. Because the leaf abscissae within each set and the two eight-element sets are distinct, this establishes **at least 80,640 different ordered rational solutions for the same public target**. These are not 80,640 independent relations. The reusable Rust reproduction is [prime31_m8_permutation_orbit_gate.rs](examples/prime31_m8_permutation_orbit_gate.rs); its exact-head compilation/execution is pending.

## Mathematical consequence and limits

Let R0 be the Dickson tower-coordinate ring over Fp with eight independent depth-six blocks and free u,v,w, and let I be the four balanced S4/S3 polynomials. Each ordered witness gives an Fp-valued evaluation homomorphism R0/I -> Fp. Distinct leaf-coordinate assignments distinguish the corresponding maximal ideals; they are pairwise comaximal. By the Chinese remainder theorem,

    R0/I --> (Fp)^(80640)

is surjective, hence **dim_Fp(R0/I) >= 80,640**, even when the left side is infinite-dimensional. This is a positive-target, unsaturated, labeled ideal statement, not a theorem that bounded-degree F4 must enumerate every root or allocate >=40,320 columns. It identifies an all-roots multiplicity burden; a first-root algorithm can potentially avoid that cost.

The displayed system has manifest relabeling symmetry: 3! within the first triple, 3! within the second, 2! within the pair and a swap of the two triples accompanied by u/v swap. Group order is **144**. For each distinct witness set, canonicalizing the triples and pair produces **280 orbits** of size 144; for the two witness sets together there are **560 such orbits**. These are duplicate *representations* of one signed group relation, not independent relation rows; **no 144x F4 speedup is asserted**. There can be additional x-only sign roots and repeated-index roots beyond this lower bound.

## Next discriminating Rust experiment

1. Compile and execute the checked-in exact-head Rust orbit control in the focused prime31 workflow. It imports the current Rust Arithmetic, Base and registered curve, and does not copy or replace the current packed F4 engine.
2. On the *same public targets*, compare the ordered balanced S4/S3 oracle against a **block-symmetric quotient**. Use elementary symmetric e1/e2/e3 for each triple and e1/e2 for the pair. Require the corresponding monic cubic/quadratic to divide the registered factor-base root polynomial, enforce cross-block distinctness and y-liftability, and resolve every accepted candidate to the full public point. This is a proposed engineering optimization, not a novel use of symmetric Semaev polynomials.
3. Freeze limits and run the existing 124 natural public masks plus held-out new masks, storing every failure, positive, timeout/censor, F4 input/solve/extraction cost, matrix and memory, deduplicated projective relation rows and **incremental rank**. Separate fixed base preparation from each target call.
4. Perform full cold scalar recovery with existing Rust linear algebra and final Q=[d]G checks, pricing algebraic preprocessing, saturation, repeated attempts, factor base, matrix, recovery and final verification against matched optimized rho **and** current index-calculus baseline. A stage-level speed improvement is not whole-solver improvement; only independently replayed results can be promoted.

## Previous work checked and novelty boundary

- [Petit–Kosters–Messeng PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) establishes prime-field algebraic index calculus. Symmetric Semaev/Gröbner methods are also prior art ([FPPR symmetry review](https://doi.org/10.1186/s40736-015-0012-6); [Amadori–Pintore–Sala](https://research.tue.nl/en/publications/on-the-discrete-logarithm-problem-for-prime-field-elliptic-curves/)). Do not claim novelty of the invariant-polynomial technique.
- [#4275](https://github.com/isogeny-labs/crypto/pull/4275) merged the dense-membership oracle comparison; [#4077](https://github.com/isogeny-labs/crypto/pull/4077) merged sparse PKM rank/cost gates. Reuse their baselines and gamma/delta criterion.
- [#4228](https://github.com/isogeny-labs/crypto/pull/4228) remains open on S3 quotients; [#3877](https://github.com/isogeny-labs/crypto/pull/3877) open on chained transfer; [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) open on auxiliary IC; [autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) open on the lead atlas. Avoid duplicating them.
- Research-KB MCP, Conductor, Rust toolchain and EKS admission were unavailable in this tool session. GitHub primary source and published literature fallback is not evidence of absent prior art.

## Reproduce and scope

    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_permutation_orbit_gate

Python finite-field producer-local replay tested **80,640 labeled solutions and 322,560 exact polynomial residuals**, 0 failures, and 560 disjoint block-orbits of 144. The two index sets have the same public masks and target. This is **not** a newly extracted relation, successful Rust CI, a signed PCC verdict or a cheaper complete prime-field ECDLP algorithm. [pcc:unverified]
