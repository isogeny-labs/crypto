# Prime31: explicit 3-isogeny into a maximal-CM curve

Date: 2026-11-18 UTC. Standing: producer-side exact calculation; [pcc:unverified] until signed PCC. No new solver or performance claim.

## Inherited original curve, unchanged

Field p=2147483647=2^31−1. E: y²=x³+847514846x+2087444724. Prime full-group order n=2147446831, cofactor 1, G=(625037715,2100627251). Source geometric End(E)=Z[π] conductor 3, fundamental D_K=−803827011, as proved in merged PR #4660. All native original Rust group arithmetic is preserved. This is not a weak substituted curve.

## New explicit scalar-blind transfer

The rational kernel polynomial of order 3 has x=u=533591203, for which v=u³+au+b=259447350 is a nonsquare modulo p. The subgroup {O,T,−T} is stable under Frobenius despite T not being Fp-rational. Put tau=3u²+a=1555600864 and h=x-u. Classical normalized Vélu gives

x(phi(P)) = x + 2 tau/h + 4v/h²,
y(phi(P)) = y (1 − 2tau/h² − 8v/h³).

The rational point O maps to O; no rational affine point has x=u. Codomain coefficients follow a'=a−10tau, b'=b−14(u tau+2v):

E': y²=x³+323891735 x+1985248359, with G'=phi(G)=(1767020261,634125456).

The map is rational over Fp, separable of degree 3, a group homomorphism by classical Vélu theory. Since 3 is coprime to the prime n, it restricts to an isomorphism on E(Fp), so phi([k]G)=[k]G' for every scalar k. Evaluating phi requires only public point coordinates, not k. Hence the source ECDLP is equivalent to an E' ECDLP with the map overhead; this is a representation mechanism, not a faster DLP algorithm.

## Maximal endomorphism order of E'

The 3-division polynomial of E' splits completely, with four distinct rational x-roots 1890447397, 1697269373, 546710038 and 160540486. An exact integer-polynomial check matches all five coefficients of psi3(E') to three times the product of these linear factors. Thus Frobenius fixes all four lines in E'[3], hence acts as a scalar on E'[3]. Since trace=36817 is 1 modulo 3, that scalar is −1. Consequently pi+[1] kills E'[3] and divides by [3] in End(E'); (pi+1)/3 is an actual endomorphism, so End(E') is the maximal quadratic order O_K with conductor 1. [pcc:unverified]

The minimal nonscalar endomorphism degree on E' is (1+803827011)/4=200956753, compared with 1808610775 on E. The 9-fold difference in degree floors is not an ECDLP or scalar multiplication speedup.

The rational x=546710038 defines the normalized dual kernel. Its codomain has coefficients a''=81a=2076709469, b''=729b=1328781720 modulo p. Therefore phi_dual(phi(P))=(9x([3]P),27y([3]P)) on the scaled original model. This identity gives a strong actual-point correctness test.

## Implementation and validation

- Native Rust: research/prime31_ic_20261115/native/examples/prime31_velu_3isogeny_gate.rs; imports the original PRIME31, Curve, Arithmetic and check_curve. Only map formulas and finite-field polynomial checks are new. It preserves group law and verifies transport, correct destination, dual composition, original group order, all four 3-division roots and mutation controls.
- Independent Python big-integer readback: research/prime31_ic_20261115/native/verify_prime31_velu.py. Its field and EC formulas are separately written and check the Rust JSON readout, all sixteen public test points and each dual identity. Independent authored producer-side cross-check is not signed PCC.
- CI workflow: .github/workflows/prime31-3isogeny-gate.yml. Sparse checkout overlays only the candidate onto original Rust commit 1df714645aa683cf975bc76fcacb9842cb4780fd, compiles under Rust 1.100.0, executes native library tests, candidate focused tests and Python cross-check. Read its actual Actions status before claiming pass.
- Separately, a producer-local exact Python implementation checked the mathematical identities on 80 synthetic public points before the Rust check-in. No native Rust full-cost benchmark or ECDLP recovery was performed during this work. [pcc:unverified]

## Prior-lead audit and duplication checks

- Source conductor determination and 3-division factorization: crypto PR #4660 (merged); this study moves to a distinct maximal-order isogenous curve, rather than repeating the source certificate.
- Complete existing native Rust 31-column generic-equivalent prime31 solver: crypto PR #4623; reuse the logarithm recovery engine but not its generic 4A group-sum table.
- Factor-base and group-sum engineering: crypto #4635 (first rational x), #4647 (negation quotient) and #4652 (canonical y null). A map does not turn their enumeration into non-generic index calculus.
- Algebraic S4/S3, RR9, quotient and F4: crypto PRs #4604 and #4228 and original prime31_m8_balanced_f4_input.rs. Original public-query balanced S4/S3 F4 hit a 15,000 dense-reducer cap and extracted zero natural relations; do not rerun unchanged cap as an alleged result.
- Concurrent prime-field subexponential proof and fresh oracle: #4665 and #4668; these claims have unsigned status and their work owners are separate.
- Matched frontier: #4596 and historical shared-BSGS 170.113ms / 1024 targets. That number is an unmatched older run, not a current benchmark reading.
- Cross-curve background: docs/endomorphism-rules.md and docs/ic/EXPERIMENTS_ISOGENY_CONDUCTOR_GAP_20261007.md require explicit rational maps and charged composed evaluation; minimal endomorphism degree is not loop evaluation cost.
- External literature: classical Vélu isogenies; Sutherland, Isogeny Volcanoes, https://arxiv.org/abs/1208.5370; Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018, DOI 10.1016/j.ffa.2018.01.009. No claim that normalized degree-three Vélu formulas are new.

Research KB MCP and its wrapper unavailable in this connector session; searched named GitHub files, PRs and literature instead. This is a bounded review, not exhaustive novelty clearance.

## Prioritized falsifiable next experiments

1. P0: run original Rust target-conditioned balanced S4/S3/paired-S3 quotient and RR9 on E and E', on exactly the same 1024 public scalar-blinded points mapped without revealing logarithms. Require actual signed point relations on public residuals without generic 4A enumeration, with positive, negative and repeated-coordinate controls; censored F4 calls are indeterminate, not no-relation proofs. First ensure E' polynomial factor base is valid and preserve equal rank targets.
2. P1: when an algebraic extractor produces full-rank rows, route them into unchanged native Rust rank-31 and 1024-target log recovery, including E-to-E' map, both factor bases, relation collection, all failures, LA, individual recovery, return verification. Compare same-host optimized BSGS, negation-map rho and prior best verified native IC on exactly matching inputs, with cold one-target and amortized 1024-target intervals separately.
3. P2: compare quotient Hilbert functions, F4 reducer caps, natural-positive and certified-negative decision rates, and degree falls on E vs E'; include a randomly chosen same-isogeny-class control. A codomain with simpler arithmetic but equal generic search is not an algebraic ECDLP advance.
4. P3: if the maximal CM order suggests short ideal-class paths, measure their explicitly factored isogeny-chain evaluation including forward/inverse coordinate conversion, not just geometric degree.

## Explicit status

Rust implementation/check-in: completed, native CI status separately verified. Ordinary cofactor-one p=2^31−1: unchanged. Prior leads audited: bounded. Novel non-generic relation extraction and cheaper complete solver: NOT ESTABLISHED; no 1024-target timing has been run on E'. Claim standing: [pcc:unverified].

## Executed exact-source native Rust gate (2026-11-18T05:37Z)

- [Successful GitHub Actions run #44245421825](https://github.com/isogeny-labs/crypto/actions/runs/44245421825): original pinned native Rust 40/40 library tests passed, new Vélu/CM tests 2/2 passed, Rust JSON emitted, separately written Python exact arithmetic replay passed 16/16 public-scalar and dual controls. [pcc:unverified]
- New native Rust JSON for the full control panel (not a DLP benchmark) charges 205 Fp multiplications, 0 inversions in setup certificate construction; 2,516 Fp multiplications and 37 inversions across 16 scalar transport/dual controls and the additional group-law sanity checks. These raw counters are **not a cold one-target or 1,024-target ECDLP runtime**. [pcc:unverified]
- Earlier source-specific CI failed Rust E0499 (nested mutable field-meter calls); commit db64c5a630f9b57ef9169a0e793296d5019396c2 split two statements into local variables; the registered exact-source CI then passed all focused checks. Retain this failure/fix history.
- Independent signed PCC remains absent; no leaderboard promotion or claim of a new complete speedup.
