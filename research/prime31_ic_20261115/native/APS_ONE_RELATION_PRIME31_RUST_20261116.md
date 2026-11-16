# EXP-R31-APS — Rust prime31 labelled one-relation S3, exact frontier and generic-equivalence control

**Date:** 2026-11-16 UTC / 2026-11-15 Pacific. **Standing:** [pcc:unverified]; source-only until exact-head Rust execution is checked. No novel complexity claim or faster complete prime-field ECDLP asserted.

## Scientific question and prior-lead audit

The previous *complete* Rust target-log solver in merged #4188 took 46.200742075 s cold on one public Q after producing 32 independent equations; it built a full S3 pair-root index and was mathematically generic pair-MITM-equivalent. New PR #4373 tested the balanced m8 S4 vs chained-S3 F4 formulation, with every tested branch censored at a 15k reduced-block cap and no extracted full relation. A qualitatively different published **single-relation** index-calculus lane is now worth implementing as an explicit performance/methodology control before further m8 solver rearrangements.

Checked the full accepted [Amadori–Pintore–Sala 2018 manuscript](https://pure.tue.nl/ws/portalfiles/portal/91158650/609.pdf), §§3–3.2, and [journal record](https://doi.org/10.1016/j.ffa.2018.01.009). The APS technique uses a *random labelled, target-dependent* factor base; one verified summation-polynomial zero relation can recover the target logarithm without a full factor-base-log linear system. Also checked [PKM 2016](https://doi.org/10.1007/978-3-662-49387-8_1), [McGuire–Mueller 2017](https://eprint.iacr.org/2017/1262) (direct polynomial evaluation/signed point-sum variants), [classical-ECDLP prior-art archive](../../classical_ecdlp_20261028/README.md), #4311's labelled one-relation and coordinate-join selector assessment, #4280's APS split-variable negative timing, #4077 rank/cost gate, merged #4228, pending #4349 certified quotient, cryptanalysis #1536 auxiliary Rust IC and crypto-autoresearcher #3305 Semaev-alternatives atlas. These are **not new inventions**. The shared research-KB/Conductor/EKS/cargo checkout were unavailable in the GitHub-connector runtime; primary GitHub source and paper are the bounded fallback, not an exhaustive novelty clearance.

## Precisely implemented Rust method

Fixture is the identical publicly specified ordinary prime-order curve, **not** a deliberately weak curve:

    p = 2147483647, E: y² = x³ + 847514846 x + 2087444724
    #E(Fp)=n=2147446831 prime, cofactor 1
    G=(625037715,2100627251)
    Q=(1477799919,824532935), a public point with scalar kept out of solver

Reuses `prime31_algebraic_ic::Arithmetic`, `check_curve`, `PRIME31` from the existing Rust crate. `native/examples/prime31_aps_one_relation.rs` uses only Rust, no Python or C++.

For each of three disjoint labelled lists, of size T=1536, deterministically sample public integer labels (a,b) and construct `R=aG+bQ`, discarding the identity and repeated x (counting both rejections). The input is not seeded from any relation witness. Discover a candidate for

    S3(x(R_i), x(R_j), x(R_k))=0

by evaluating each **ordinary S3 quadratic** over Fp, solving both finite rational roots with per-row batched denominator inversions, and intersecting the *third list's abscissae*. Candidate discovery does not invoke EC additions, target logs, or an EC sum lookup. **Only after** coordinate discovery, verify the eight signed combinations using actual EC arithmetic, require a nonzero aggregate target coefficient B, derive `k=-A/B (mod n)` from the labelled relation `A G+B Q=O`, and independently replay `[k]G=Q`. The single relation eliminates the 31/32-dimensional factor-base logarithm linear system completely.

The flag `--compare` additionally performs a separate **generic EC group-sum hash join** on the same lists for **classification and reproducibility**, never as a hidden fallback source of algebraic answers. `--one` executes only the algebraic arm and reports its complete Rust cold wall. Both preserve per-round statuses and all misses; three deterministic bounded rounds prevent a silently unbounded search. Unit tests check S3 field roots against independently computed signed EC sums and the modular scalar-recovery equation.

### Complexity and anticipated coverage, not a measured speed result

Raw signed triples per complete three-list draw: `2³ T³ = 28,991,029,248`; divided by n, expected raw signed relation **occurrences** under independently uniform public labels is about 13.50. Paired global-negation choices double-count x-only equivalences, and dependencies/collisions reduce distinct matches. This **does not** guarantee any given pseudorandom fixture or a usable B≠0 zero relation; the program records all unsuccessful rounds. Algebraic pair-root trials cost T² per round (up to 2,359,296 polynomial evaluations); source `aG+bQ` generation, candidate verification and modular inversion are charged for cold cost. The separate group join uses the same T² pairs and different operations, with identical labelled data.

**Important limitation:** Directly solving every S3 quadratic over the two finite enumerated lists is *generic meet-in-the-middle-equivalent*. This is **not a new non-generic ECDLP attack**, even though it exercises the single-relation index-calculus algebraic formulation and performs genuine target-log recovery. Its most valuable use is a same-curve complete **APS-style control** for judging whether a proposed F4/certified quotient mechanism contributes anything beyond generic support intersection.

## Rust-only repro and completed-cost acceptance

    cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --one
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --compare

The dedicated existing `.github/workflows/prime31-s4-split-20261116.yml` now requests the two example tests and one complete `--one` execution (the heavy `--compare` is a separately requested matched followup). Source-specific CI must report a *real* solution and replay before we claim completion. Distinguish source/point-generation, algebraic S3 search, signed verification, unused masks and arithmetic costs. A single non-isolated development run is [pcc:unverified] and cannot substantiate superiority over other completed algorithms.

## Research disposition and next precise non-generic experiment

1. **APS control:** establish exact-head Rust source/test and a verified public Q logarithm, separately time the same-input generic join; no novelty or advanced IC rank is credited.
2. **Non-generic candidate (highest priority):** port *supported* selected-root certified quotient from #4228/#4349 to the actual 31-bit finite-rational target-dependent m8 ideal. The existing certificate worker accepts only 3–8 ordinary variables with shape/degree caps, whereas the inherited eight-six-level tower has many more variables; a variable/ideal equivalence bridge or a new verified quotient shape is a **mathematical prerequisite**, not a mere Rust API call.
3. **IC frontier:** measure *completed equal root sets* against PKM compositional factor bases, APS labelled-base single relation, Rust dense/tower F4 and certified quotient on the same original ideal and frozen natural targets; external CAS comparison when available. Censored F4 never denotes UNSAT.
4. **Full non-generic cold solve:** independently useful signed rows, rank and held-out unknown target scalars on this curve, full preparation/failures/LA accounted, then optimized rho *separately* as the ECDLP security baseline. Independent PCC and multiple ordinary prime-order curves required for a generalized gain.

This is a **measured experiment protocol and executable Rust method**, not a claim that the user-specified cheaper complete non-generic/index-calculus frontier has been achieved.
