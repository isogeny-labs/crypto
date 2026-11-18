# Prime31 fraction-free, two-inversion batch inverse — measured component and research triage

Date: 2026-11-18 UTC. State: **producer measurement + independent BigInt replay, [pcc:unverified]**. No new unknown ECDLP scalar is recovered and no non-generic relation extractor has been shown in this experiment.

## Exact curve and source

The unchanged ordinary cofactor-one benchmark is `p=2147483647`, `E: y²=x³+847514846x+2087444724`, `#E(Fp)=2147446831`, `G=(625037715,2100627251)`. Use degree-three Vélu map from merged #4671 to `E': y²=x³+323891735x+1985248359`, `G'=(1767020261,634125456)`.

Source: `research/prime31_ic_20261115/native/examples/prime31_frobenius_inverse.rs`, same Rust 1.100.0 locked crate and GitHub source-specific workflow as #4701. Both implementations process the **same 256 public multiples of G'**, without knowledge of the scalar. Known scalars enter only the caller's independent verification panel.

## Mathematical mechanism

For the publicly known destination abscissa X, the original inverse rational map yields the monic cubic
`F_X(T)=T(T-u)^2+2*tau*(T-u)+4*v-X(T-u)^2`.
The Frobenius polynomial `T^p-T` detects its unique linear factor. Because this 3-isogeny's nonzero kernel points are nonrational and Frobenius exchanges them, the inverse cubic has exactly one rational root for nonidentity Q.

Original implementation computes `gcd(F_X,T^p-T)` using divisions and separately inverts the ordinate denominator for each Q. The candidate uses **fraction-free pseudo-remainder Euclid**. A pseudo-remainder step multiplies A by `lc(B)` before canceling the shifted leading term, preserving the gcd up to an invertible scalar. The output linear polynomial need not be monic; its rational root is `x=-g[0]/g[1]`. Two standard Montgomery batch inversions recover all 256 rational roots and all 256 ordinates. This is classical finite-field algebra, not a new cryptanalytic algorithm.

## Frozen exact arithmetic results

| 256-point stage | Existing ordinary Euclid | Fraction-free batched Euclid |
|---|---:|---:|
| Measured Fp multiplications | 356,864 | **297,850** |
| Measured Fp inversions | 1,024 | **2** |
| Public points reconstructed / exactly equal | 256/256 | **256/256** |
| E original curve and dual-isogeny agreement | 256/256 | **256/256** |

Savings: 59,014 instrumented field multiplications (16.54%); 1,022 fewer inversions (99.80%). Counts charge exponentiation multiplications inside measured inversions, so do not add a second estimate for inverse cost.

Source-specific GitHub Actions #44267027887 passed **40/40 pinned Rust release library tests, 2/2 example tests, and independent Python BigInt replay** of all 256 public coordinates, exact polynomial pseudo-gcd roots, 256 Vélu round trips and 512 source/destination known-answer scalar checks. Independent replay is producer-independent code but is **not** signed independent PCC.

### Wall-time diagnostics (not yet an isolated speedup claim)

At #44267027887, original then candidate: 1.958401 ms versus 1.731094 ms. Candidate then original: 1.739230 ms versus 1.915181 ms. At previous #44266787518 the first candidate invocation had an inconsistent cold 2.892690 ms versus 1.957223 ms; reversed order was 1.680293 versus 1.885181 ms. Therefore single-pair timing is sensitive to warmup/host variance; **do not cherry-pick a wall-time advantage**.

A 12-pair alternating-order same-host diagnostic was subsequently added. Its initial source-specific run #44267163834 failed *at compile time* due to a `serde_json::json!` macro recursion limit from the expanded transcript; no scientific outcome was emitted. A crate recursion-limit fix was committed and the final 12-pair exact-source run is pending. The earlier verified mathematical/counter result is unaffected.

## Prior-lead audit

| Lead / repository evidence | Assessment / impact |
|---|---|
| Original Prime31 complete rank31 1,024-target Rust solver, crypto #4623 | Generic-equivalent four-sum extractor; correct control but not a non-generic advantage |
| First-x31 B31 and negation quotient, crypto #4635 and #4647 | Useful measured engineering controls; neither removes generic point-sum enumeration |
| Coordinate y-ranked control, crypto #4652 | Negative full-solver comparison on pinned corpus; do not repeat blind re-ranking |
| Curve exact CM conductor, crypto #4660 | Low-degree nonscalar endomorphisms mathematically excluded on original E |
| Degree-three CM-isogeny + archived rank31 replay, merged crypto #4671 | Pure transport preserves signed relations and matrix rank; inverse preprocessing alone cannot improve source relation coverage |
| Earlier inverse-Vélu and batch-inversion experiments, repository `research/isogeny_torsion_fibers_20261012` and `APS_JACBATCH_FRESH_REPORT_20261116.md` | Classical prior art; novelty only this exact Prime31 engineering variant |
| RR9 shared-coefficient extractor, crypto #4692 | **Highest priority**: no-enumeration B4/B8 exact-fibre tests followed by natural B31 extraction |
| K-sum table/multiple-lever controls, crypto #4711 | Validated engineering advances but reported rho still wins; not a novel prime-field extractor |
| Theoretical prime-field subexponential proposal, crypto #4698 | Paper-level unverified claim, not a measured complete Prime31 implementation |

Related literature: Amadori–Pintore–Sala (2018), DOI https://doi.org/10.1016/j.ffa.2018.01.009, prime-field one-relation variant; simultaneous inversion algorithm https://flintlib.org/doc/algorithms/arithmetic/batch-inversion.html. Queried accessible GitHub source files for S3 x-fibre, batched inversion, RR9 and inverse Vélu. Shared research-KB connector unavailable; absence in bounded search is **not** novelty certification.

## Experiment queue with falsifiable gates

1. **RR9 #4692 B4/B8 controls:** require exact public-residual signed-fibre completeness (including absent fibres, multiplicity collisions and exceptional points) with the candidate never enumerating EC signed group sums; independent grader may enumerate.
2. **B31 nonenumerative natural relation yield:** output *new* exact signed eight-leaf relations, verify each in original Rust EC group, record failed and censored attempts and all polynomial/F4/root costs. Reject candidate when it silently calls the generic four-sum table.
3. **Rank and whole solve:** reuse the existing Rust modular rank31 relation solve and individual-log recovery for 1,024 blinded original-E points, no scalar-knowledge in extractor and no undeclared fallback.
4. **Strict fully charged cost:** count curve setup, factor-base construction, F4 matrices, rootfinding, public-target inversion and transfer, all unsuccessful trials, linear algebra and all per-target verification. Compare same-host cold one-target and 1,024-target amortized costs against strongest measured native IC, shared BSGS and negation rho. Require independent verifier/pinned artifacts and signed PCC.
5. **Native E' factor base (secondary):** compare native first-x31 E' versus transported source base with exact coverage/rank; do not infer improvement from isogeny itself.

No paid EKS compute was launched. All work here was in the existing Rust and GitHub Actions development gate. Avoid claiming a faster complete solver until one of the nonenumerative relation branches passes items 1–4.
