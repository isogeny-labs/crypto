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


## Exact first full Rust execution, source-pinned (2026-11-16 01:29 UTC)

[GitHub Actions run 43786504359](https://github.com/isogeny-labs/crypto/actions/runs/43786504359), job `133035251870`, source-report commit `90a0314`: **success**. 40 shared Rust library release tests, **2/2** APS example tests, actual `--one` binary completed. The original public Q `(1477799919,824532935)` returned `log_G Q=1038579171`, verified by a signed zero relation and `[k]G=Q` without supplying k to relation search. The first deterministic public-labelled partition sufficed, 1536 nonidentity points in each of three sets. The program's cold in-process wall was **327,525,503 ns (327.526 ms)**, with **135,786,811 ns** labelled-point construction and **191,380,563 ns** algebraic pair-root search plus candidate validation; run/host external process startup is excluded. There were **672,768** actually visited pair S3 quadratics/square roots, **438** row batch inversions, one abscissa match, one signed candidate, and **57** group additions in the post-discovery verifier. The algebraic relation used indices `[437,1247,879]`, signs `[-1,+1,-1]`, coefficients `A=1144570523,B=2011873483`, with `A+B*1038579171≡0 (mod 2147446831)` separately checked by integer arithmetic. No rejected identical abscissae or identity points in this run. Frozen machine-readable receipt: [APS_ONE_RELATION_PRIME31_RUST_CI_20261116.json](APS_ONE_RELATION_PRIME31_RUST_CI_20261116.json).

**Strict interpretation:** this is the first source-bound completed cold Rust scalar recovery by the APS-style **one-relation** route for this registered prime31 Q, with no factor-base-log matrix. It is **not** a new non-generic complexity result. It is **one** source-specific developer run, no A/A isolation or PCC, not comparable via a precise wall ratio to the previous 46.2s index/LA from a different host and source. The `--compare` code path is a same-input generic group-support control and must execute before any *matched* comparative verdict; it is not hidden fallback. Do not compare the 327ms development observation to partial/censored F4 engines and call it IC-frontier superiority.

**Next registered holdout:** `--holdouts` constructs 12 independent deterministic public Q=[k]G with k retained only in the *caller-side verifier*, not in the solver. It executes the same bounded 3-round labelled-list search, independently checks `k` after each algebraic solve, and prints all successes, misses, query counts and cold walls. The existing Rust CI now invokes `--compare` and `--holdouts`; their **actual source-specific execution receipts are pending**, not assumed to pass.


## Same-source complete Rust experiment and 12 held-outs, 2026-11-16 01:35 UTC

**[Actual source-specific completed GitHub Actions #43787167910](https://github.com/isogeny-labs/crypto/actions/runs/43787167910), job 133037288540, source-report head `6b2c38de`: PASS.** This CI executed the Rust `--one`, `--compare` and new `--holdouts` flags on the **same development runner**, in release mode. All 40 reused Rust library tests and two example tests passed. Full raw program outputs, target labels and individual cold durations are frozen in [APS_ONE_RELATION_PRIME31_MATCHED_CI_20261116.json](APS_ONE_RELATION_PRIME31_MATCHED_CI_20261116.json). Still **[pcc:unverified]**, not an isolated performance trial or signed scientific result.

| Full one-public-Q Rust development program / same source | Observed |
|---|---:|
| Algebraic `--one` complete unknown log `1038579171` | **267.922 ms** in-process cold wall |
| Algebraic `--compare` complete unknown log `1038579171` | **268.610 ms** in-process cold-to-verified-scalar wall |
| `--compare` source/labelled base construction | 98.874 ms |
| `--compare` S3 finite-field root query + signed verification | 169.421 ms |
| **Separate** generic signed group-addition join on the same labelled sources | 325.897 ms for join alone |
| Source construction + generic join, same code/runner | **424.771 ms**, excludes generic arm's separate process startup |
| Generic join actual EC additions | 1,345,017 |
| Algebraic side pair S3 quadratics actually visited | 672,768 |
| Equivalent recovered scalar with both modes | Yes |

The same-seed, same-point-set complete in-process **algebraic vs generic** cost ratio is **424.771/268.610 ≈ 1.581x**, but this is **one sequential development run** without A/A randomization, external mature implementations or signed PCC; it is only a relative engineering data point. It **does not** constitute a novel non-generic algorithm: the source searches the finite pair support, equivalent in reachability to a generic group meet-in-the-middle join, even if modular arithmetic happens to be faster than this particular affine group implementation.

**Twelve public held-out unknown logs:** `--holdouts` recovered **12/12** independently constructed `Q_i=[k_i]G`, checking `[k_i]G=Q_i` and exact recovered k only in the caller-side verifier after search. Every test succeeded in its **first** bounded round. Total of the **12 separate charged in-process cold solver walls** was **2,271.161215 ms**; 4,368,384 field S3 quadratic trials in total. Minimum per-target 101.793 ms, maximum 504.566 ms, all status/output cases retained verbatim in the JSON above. The 12-point study does **not** use one target's relation as a precomputed oracle for another; each trial reconstructs its own 4608 labelled points and source masks. Generated Q scalars appear in the output solely for independent known-answer verification.

**Decision:** This confirms correctness and removes conventional multi-relation factor-base LA for the published APS labelled one-relation method on the fixed ordinary prime31 curve, and supplies a measured Rust **complete solver control** for future non-generic m8 experiments. It does **not** establish a cheaper-than-frontier genuinely nonenumerative IC solver or asymptotic improvement. The next actual scientific advance requires a certified target-conditioned m8 rational-root mechanism that avoids enumerating finite S3 pair support. Charge all certificate/quotient work, exceptional branches, all missed natural targets and independent useful rank; compare completed equal-output calls with PKM/APS and a matched external CAS. Keep rho/BSGS separate as security/final full-cold controls.
