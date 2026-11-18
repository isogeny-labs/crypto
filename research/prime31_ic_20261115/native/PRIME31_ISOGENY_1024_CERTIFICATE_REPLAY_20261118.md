# Prime31 3-isogeny: 1,024 public-target certificate transport and complete accounting gate

2026-11-18 UTC. **Validation:** producer-authored native Rust plus separately authored Python integer arithmetic; individual CI outcomes must be checked separately. **[pcc:unverified]**, not an independently signed PCC. **Not** a novel relation extractor, new cold complete solver or new end-to-end speed advantage.

## Registered source, invariant curve and target

The existing source curve is fixed; no weak substitution:
```
p  = 2147483647 (2^31 - 1)
n  = 2147446831 (prime, full E(Fp) group order, cofactor 1)
E  : y^2 = x^3 + 847514846 x + 2087444724
G  = (625037715, 2100627251)
E' : y^2 = x^3 + 323891735 x + 1985248359
G' = (1767020261, 634125456)
```
Degree-three normalized Vélu map, from PR #4671:
```
u = 533591203; tau = 1555600864; v = 259447350
h = x-u
X = x + 2*tau/h + 4*v/h^2
Y = y*(1 - 2*tau/h^2 - 8*v/h^3)
```
The kernel is rational as a subgroup but the nontrivial kernel points are not Fp-rational; `h != 0` for affine Fp points. The map is scalar blind. The normalized dual and maximal-order CM property are established separately in #4671 and #4660.

## Exact algebraic implication: transport preserves all relations

Let `B=(P_0,...,P_30)` be any factor base in E(Fp). Because n is prime and coprime to 3, the degree-three separable isogeny `phi` restricts to an **isomorphism of rational groups** `E(Fp) -> E'(Fp)`. Therefore for every public `Q`, shift `a`, and signed integer vector `c`:
```
sum_j c_j P_j = Q + [a]G
    iff
sum_j c_j phi(P_j) = phi(Q) + [a]G'.
```
The proof is linearity of the group homomorphism in the forward direction, and injectivity in the reverse direction. Since the coefficient vectors are unchanged, the relation matrix over Z/nZ and its rank are identical (with the same source-label convention). Reusing a **transported** basis and source witness oracle therefore cannot improve algebraic coverage or useful matrix rank through CM alone. Any solver benefit from E' must arise from the *cost* of discovering those relations in E' coordinates, or a **new native E' factor base**, not from a higher relation yield for an unchanged transported base. This rules out an invalid speedup inference without ruling out a true target-coordinate extraction improvement.

We separately construct **first 31 rational x-coordinates native to E'** and compare them with the 31 source first-x points transported through phi; report their exact x-coordinate set overlap. Native-E' factor bases and transported-source bases are not interchangeable controls.

## Frozen 1,024-target replay protocol

Re-use, do not regenerate, `DICKSON31_COMPLETE_RANK_1024_RAW_20261117.json` from #4623. Source-bound native Rust example:
`research/prime31_ic_20261115/native/examples/prime31_isogeny_1024_certificate_replay.rs`

- Import the original `PRIME31`, `Curve`, `Arithmetic`, `Point` and `check_curve` from the Rust development adapter pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`.
- Read the 31 stored relation rows, independently perform modular 31x31 Gaussian elimination, require full rank and check each recovered factor-base logarithm by scalar multiplication on original E and on E' (using original Rust point arithmetic).
- Read all **1,024 frozen public points**, perform scalar-blind Vélu transport to E', and compare **per-point inversions** against **Montgomery batch inversion**, retaining both Fp operation counts and wall-clock diagnostics.
- For each frozen signed eight-token witness, recover `k` from **witness coefficient logs and public shift**, verify the original E relation, the transported E' signed relation, original scalar equality `[k]G=Q`, and E' equality `[k]G'=phi(Q)`. The old known-answer scalar is used only **after** new witness-based recovery, for an assertion.
- Preserve the 951/70/3 shift histogram, require zero generic fallbacks and fail on missing/duplicate/bad points, wrong rank, wrong witnesses, or batch-vs-scalar discrepancy. This validates archived relations; it does **not** run a fresh F4, RR9 or Semaev extractor and does **not** measure cold collection and new full solve on E'.
- Separately authored `verify_prime31_isogeny_1024.py` uses its **own Python group addition, modular inversion, double-and-add, and Vélu formulas**, checks all 31 basis equations, 62 basis scalar images, all 1,024 source and 1,024 target scalar identities, 2,048 original/image signed relations, one mutated wrong-log negative control, and confirms the native-E' first-x overlap claim. This is producer-independent implementation arithmetic, **not independent signed PCC**.

Exact-source workflow:
`.github/workflows/prime31-3isogeny-1024-replay.yml`.
Run on Ubuntu with Rust 1.100.0; checkout *pinned* original Rust source, overlay only new candidate and frozen JSON, cargo lib regression + focused example test + binary readback + independent Python replay. Preserve raw JSON artifacts.

**Measurements:** Read actual successful workflow's JSON and report them here before any comparison. No runtime or speedup claim has been prespecified. Inversion costs must include the exponentiation multiplications used by this native adapter. A/A host noise is not established for one run.

## Prior-lead and duplication audit (source checked, no novelty clearance implied)

| Source | Evidence status / consequence |
|---|---|
| [#4623](https://github.com/isogeny-labs/crypto/pull/4623) native complete 1,024-target Dickson31 Rust IC-style control | Frozen exact signed witness/rank31/log transcript; **generic** 4A/4A meet-in-the-middle, not evidence of a nongeneric extraction advantage |
| [#4635](https://github.com/isogeny-labs/crypto/pull/4635), [#4647](https://github.com/isogeny-labs/crypto/pull/4647), [#4652](https://github.com/isogeny-labs/crypto/pull/4652) | First-x, negation pairing and canonical-y B31 controls already investigated; do not duplicate simple base reranking |
| [#4660](https://github.com/isogeny-labs/crypto/pull/4660), [#4671](https://github.com/isogeny-labs/crypto/pull/4671) | Source CM conductor three, exact 3-isogeny to maximal CM order conductor one; a changed endomorphism norm floor does **not** imply a 9x discrete-log speedup |
| [#4604](https://github.com/isogeny-labs/crypto/pull/4604), original `prime31_m8_balanced_f4_input.rs` | Natural B8 S4/S3 F4: 16 public masks hit 15,000 reducer cap, no complete relations; **censored**, not a proved impossibility |
| [#4678](https://github.com/isogeny-labs/crypto/pull/4678) | Active APS/Split4/RR9 coefficient reduction and positive-control certificates; avoid duplicating its exact component work |
| [#4682](https://github.com/isogeny-labs/crypto/pull/4682), [#4675](https://github.com/isogeny-labs/crypto/pull/4675) | Semaev exact and shared-coefficient membership trials tracked by other concurrent PRs, not new benchmark results |
| [#4681](https://github.com/isogeny-labs/crypto/pull/4681) | Cold ecbench parity protocol with rho; draft has no successful full-cost comparison yet |
| [#4668](https://github.com/isogeny-labs/crypto/pull/4668) | Mathematical claimed exact subexponential Boolean tensor counter is an **unverified hand proof**, not runnable ECDLP or prime31 cost evidence |

Relevant primary work: Petit--Kosters--Messeng prime-field rational-map index calculus; Amadori--Pintore--Sala one-relation prime-field method (2018); Semaev summation polynomials; [prime-field Groebner assumption caveat](https://www.degruyterbrill.com/document/doi/10.1515/jmc-2019-0029/html). The direct local `research-kb` query service was unavailable in this connector execution; this is a bounded source, PR and GitHub lead review, **not exhaustive novelty clearance**.

## Experiment queue: prioritize actual non-generic extraction

1. **P0: E' native-first-x31 vs transported-B31 algebraic RR9/S9 or balanced S4/S3 extraction.** Import current Rust F4 / quotient / root-finding engines; no 4A point-sum precomputation in the candidate oracle. Freeze seeded 32 positive and 32 natural unplanted points, same original Q and phi(Q), exact root and sign verification; count censored F4 separately from negative completed ideals. Record new independent relation rows per field operation and failure. A pure transport of the generic 4A/4A oracle is a forbidden “nongeneric” claim.
2. **P1: E and E' original Rust complete 1,024-target matched cold benchmark.** Once a genuine candidate relation extractor exists, charge kernel/setup, map 1024 Q, base selection, every relation attempt and failure, full-rank mod-n LA, individual target descent, witness verification, and cold vs amortized costs. Use identical blinded original public Q and confirm every final `[k]G=Q` on **original E**. Compare strongest same-host native index-calculus frontier, shared-precompute BSGS and negation-map rho under the measured operation pricing model.
3. **P1: solve algorithmic coordinate effect rather than CM-norm effect.** Test the E'-native first-x and special polynomial factor bases, E-native matched factor bases and E' transported first-x. Use equality of transmitted signed-witness coverage as a metamorphic property, not a performance extrapolation.
4. **P2: capped F4 diagnostic ablation** on identical positive and natural masks for both curves, isolating field arithmetic, quotient dimension, degree fall, matrix size, exact univariate roots, and certificate completeness. Stop/mark censored if inherited matrix cap is exceeded. No expensive paid compute without explicit budget approval and preregistration.

**Acceptance:** A complete *non-generic* prime-field solver must derive its own public-target-conditioned signed relations algebraically without 4A table, reach true full modular rank, recover and independently verify at least 1,024 blinded original E logs, and beat a same-host strongest admitted IC baseline with all costs included (plus BSGS/rho separately). Do not promote claims from this replay alone. Keep signed PCC validation outstanding.
