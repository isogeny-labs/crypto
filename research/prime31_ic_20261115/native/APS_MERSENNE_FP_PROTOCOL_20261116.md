# EXP-R31-APS-FP-MERSENNE — same-source, fully cold prime31 field-arithmetic A/B

Date 2026-11-16 UTC. Standing: **preregistered, source-only until exact-head Rust CI**; [pcc:unverified]. The goal is a faster **complete** Rust implementation of the published APS-style one-relation index-calculus methodology on the same ordinary cofactor-one prime31 curve, not a novel non-generic ECDLP asymptotic attack. No Python/C++ implementation, external precomputed logs, paid compute or silently substituted baseline.

## Literature and previously searched leads

- APS 2018: A.G. Amadori, F. Pintore and M. Sala, *On the discrete logarithm problem for prime-field elliptic curves*, FFA 51 (2018), 168–182, https://doi.org/10.1016/j.ffa.2018.01.009, one-relation methodology.
- PKM 2016: C. Petit, M. Kosters, A. Messeng, *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields*, PKC 2016, https://doi.org/10.1007/978-3-662-49387-8_1, compositional rational-map factor base.
- Existing repo: #4374 12/12 APS complete, #4383's 24-Q size confirmation (T1024 LOST to T1536), current #4391 walk source 24/24 old/new/repeat with measured same-run full cold original/walk **2.15564x**, merged #4373 m8 S3-chain censored at F4 cap; #4188 m4 natural-coverage ceiling, merged #4228 and open #4349 certified quotient lacking full 31-bit m8 target root extraction, #4077 whole-call feasibility gate, #4311 labelled selector alternatives, cryptanalysis #1536 and crypto-autoresearcher #3305. The preceding walk result is source-specific producer development evidence, not independent PCC or fastest external solver.
- KB MCP and repository wrapper cannot be invoked in this connector runtime. Review of GitHub primary files and published papers is bounded fallback. No novelty/absence claim.

## Mathematical reason and exact arithmetic bound

The prime is the Mersenne integer `p=2^31-1=2147483647`. For canonical residues `0<=x,y<p`, the product `t=x*y` fits in 62 bits. Since `2^31≡1 (mod p)`, a Mersenne fold

    z=(t & p)+(t >> 31)
    z=(z & p)+(z >> 31)
    if z>=p { z-=p }

returns the unique canonical residue. All intermediate additions fit in u64. This substitutes two shifts/masks and bounded adds for the existing Rust `(x as u128*y as u128) % (p as u128)`; correctness must be established by complete boundary and randomized old-versus-new Rust tests before end-to-end timing. Source-space group arithmetic is **not** rewritten and remains separately charged.

## Prespecified experiment

1. Add `mulp_mersenne` in the existing APS Rust example alongside untouched old `mulp` and typed compile-time `const FAST` selection for **S3 coefficients, field square roots and batched inversion only**; keep all source generation, relation-selection order, root validation, signed witness verification and scalar replay unchanged. Public target scalar must never enter the solver. Tests cover field boundaries, 100k deterministic random pairs, S3 equivalence on finite public x, and old/new equal solver outputs and query counts.
2. Add `--mersenne-confirm`: **24 fresh publicly constructed holdout Q** different from #4391's 24-Q panel, three full cold T1536 64-step walk arms per Q — old mod, Mersenne, Mersenne repeat — alternating A-B-B / B-B-A order. All arms use identical Q, public source labels, deterministic seed, 3-round cap, and full original Rust S3 recovery. Preserve failed rounds and exact known scalar, verify after completion only. Time `source`, `algebraic`, `total cold` and count pair roots, rank not applicable (one relation), group additions and all error paths.
3. Accept only if **72/72** complete correct unknown logs, all old/fast outputs and visited-pair counters equal, and FAST full-cold total below OLD with repeat stability. Report paired losses and wins, old/new oracle-only ratio and full-cold ratio separately. Reject unexplained output mismatch, compilation issues, missing arms, cap exhaustion or a favorable but censored trial. A/A and independent PCC are still absent; no leaderboard promotion.
4. Do not confuse this with the **non-generic m8 target-conditioned certified quotient** objective, which needs naturally extracted roots without explicitly enumerating K's 961 roots or the 961^4 Cartesian tensor. The m8 target remains prior-work-tracked and open.
5. The frontier denominator for algebraic relation finding remains published PKM/APS and same-ideal F4/quotient/external mature CAS. Rho is a separate complete cold ECDLP security control. A faster field kernel inside generic-equivalent S3 pair enumeration is not a new complexity claim.

Run same-host release Rust with current `cargo +1.100.0 ... --example prime31_aps_one_relation -- --mersenne-confirm` after exact-head focused tests. Capture raw outputs and source SHA. Code is **not implemented or timed** at preregistration.
