# Prime31 W8 block-scale: exact autocorrelation identity and controlled experiment (2026-11-19)

**Status: SOURCE-SPECIFIC RUST CI PASSED.** Pre-data protocol pinned at `25a649d0`; original-source validation and complete 5-arm generic-grader data recorded in [Actions #44543792457](https://github.com/isogeny-labs/crypto/actions/runs/44543792457) (PR run [#44543805506](https://github.com/isogeny-labs/crypto/actions/runs/44543805506) also passed). [Raw 3-file artifact](https://github.com/isogeny-labs/crypto/actions/runs/44543792457/artifacts/14539671492), [durable source-linked summary](W8_BLOCK_SCALE_CI_SUMMARY_20261119.json). Independent validation: original Rust library **40/40**, focused Rust tests **4/4**, independently recomputed **512 public targets**, all 2560 signed ordered-witness multiplicity cells via summary reconstruction and **20/20 affine witness certificates**. Signed third-party PCC pending. The experiment does *not* demonstrate a faster complete non-generic ECDLP solver.

## Mathematical result

Let G be the original odd-prime-order Prime31 EC group, n=2147446831. Fix an eight-point factor-base window and define l(U) as the number of signed four-slot words, weights (1,2,4,8), whose group sum equals U. There are 16^4=65536 words. For c=2^k (k>=4), set

    m_c(R) = sum_{U in G} l(U) l(c^{-1}(R-U)).

Then m_c counts **all** signed ordered eight-slot witnesses with weights (1,2,4,8,2^k,2^(k+1),2^(k+2),2^(k+3)). The four-slot membership problem is identical for the two halves; only the scalar-[c] join varies. Because n is odd, multiplying by c is bijective: all right-half group collision classes, point multiplicities and support cardinality equal the left half for every k. All five schedules are superincreasing, hence have no formal signed-integer coefficient collisions.

Define C(Delta) = sum_U l(U)l(U+Delta). Expanding pairs of witnesses with U1+cV1=U2+cV2 and grouping by Delta=U1-U2=c(V2-V1) proves the **exact finite-group second-moment identity**

    sum_R m_c(R)^2 = sum_Delta C(Delta) C(c^{-1}Delta).

Also sum_R m_c(R) = (16^4)^2 = 16^8, giving population mean exactly 16^8/n ~= 2.000034289 for every k. Therefore k affects target-hit concentration through *cross-block difference autocorrelation*, not through within-block collisions. This is a useful mechanism to optimize before expensive public-only F4.

Two exact redundancies: l(-U)=l(U) by flipping every signed leaf, so m_{-c}(R)=m_c(R) pointwise; negative block scalings should not be separately tested. Swapping U and V gives m_{c^{-1}}(c^{-1}R)=m_c(R), so c and its inverse have identical full-population multiplicity histograms (not necessarily identical outcomes on one fixed R sample). Neither is a novel ECDLP complexity theorem.

## Experiment and falsifier

The [frozen protocol](W8_BLOCK_SCALE_PROTOCOL_20261119.md) preregisters k=4..8, the same original ordinary cofactor-one p=2^31-1 curve, four eight-point windows, 128 never-before-used deterministic public Q per window, all 65,536 words per half, 512 paired Q total, and complete multiplicity counting. The same original-Rust four-slot S is reused; its scaled counterpart advances one doubling per arm. Table setup, full negative probes, affine witness replay and arithmetic counters are charged. The primary comparison is k=5 versus k=4. A higher k costs an additional x-only doubling-chain relation and must earn any added yield in *actual* F4 cost before promotion.

The grader explicitly uses generic EC half-sum tables; it is a controlled feasibility experiment, not an admissible non-generic extractor or complete ECDLP speedup.

## Prior-lead and literature audit

- [RR9/F4 extractor owner #4692](https://github.com/isogeny-labs/crypto/pull/4692): do not duplicate the public-only remainder-tower implementation.
- [Held-out full fibres #4807](https://github.com/isogeny-labs/crypto/pull/4807), [formal signatures #4805](https://github.com/isogeny-labs/crypto/pull/4805), [unweighted B8 cancellation #4803](https://github.com/isogeny-labs/crypto/pull/4803).
- [Full-rank exterior #4779](https://github.com/isogeny-labs/crypto/pull/4779) and [all-sign interpolation obstruction #4801](https://github.com/isogeny-labs/crypto/pull/4801).
- [Ternary complete generic control #4770](https://github.com/isogeny-labs/crypto/pull/4770) and [rank31/1024 W8 generic control #4728](https://github.com/isogeny-labs/crypto/pull/4728).
- [Extractor budget #4793](https://github.com/isogeny-labs/crypto/pull/4793), [optimized shared BSGS #4532](https://github.com/isogeny-labs/crypto/pull/4532): 170.113 ms for 1024 logs including setup on the historical source-specific runner, not a same-runner comparison to this control.
- Previously recorded crypto-autoresearcher SA-04/18/19 norm and coordinate negatives; GitHub code search found no equivalent Prime31 blockscale k-sweep, but absence is not proof of novelty.
- Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009). Research-KB MCP not loaded; GitHub and public literature are the documented fallback.


## Original-curve measured five-arm results (512 held-out public targets; 19 Nov 2026)

| Scale c | x-only doubling links | Public target hits / 512 | All ordered witnesses | Maximum multiplicity |
|---|---:|---:|---:|---:|
| 2^4 = 16 | 4 | **452 (88.28%)** | 1052 | 6 |
| 2^5 = 32 | 5 | 440 (85.94%) | 1063 | 8 |
| 2^6 = 64 | 6 | 440 (85.94%) | 1026 | 8 |
| 2^7 = 128 | 7 | 444 (86.72%) | 1027 | 7 |
| 2^8 = 256 | 8 | 448 (87.50%) | 979 | 7 |

Prespecified paired k=4 versus k=5: **64 k=4-only hits** versus **52 k=5-only hits**, a descriptive exact two-sided paired binomial/McNemar p=**0.3071**. This held-out panel does not establish a population advantage for k=4; it does **not** supply evidence that extra chain depth improves yield. The simplest four-doubling candidate k=4 therefore remains preferred for the public-only algebraic extractor given equal or better observed yield and shorter polynomial chain. The samples cover one curve and four small factor-base windows; no scaling result across p is inferred.

Window 0/1/2/3 target hits for k4: **117/107/116/112**; k5: 110/109/111/110; k6: 114/111/112/103; k7: 112/111/111/110; k8: 113/105/118/112. The shared four-slot sumset has **65,536** unique EC points for windows 0,2,3 and **65,496** for window 1 (the previously certified 40 EC collisions). Multiplication by each 2^k preserved these counts exactly, confirming the bijection invariant for all arms. Every query tested the full left support with **no early exit**; total probe cost per arm was about **33.55 million original-affine EC additions**, with **8.64–8.66 seconds** summed probe time across four windows on this runner. These are **generic control** costs and not F4 or whole-solver costs. Cross-arm shared setup is charged and reported separately in the raw record; one should not interpret incremental k=5..8 table setup as independent cold setup.

Source-specific run used `082177565c87fac47bbd0c61ba032eb4c5274cf8` with pinned original arithmetic `1df714645aa683cf975bc76fcacb9842cb4780fd`. The raw `prime31-blockscale-full.json` SHA256 is `681efda3b729f103cea62e82072989ef6032bf3ea9cfeaedc1f339b79c69431b`. The separately run Python BigInt checker output SHA256 is `a8ad22d243fc4cd4db8f3689fda64d7ddb635a29496369417d367da79088a08b`; artifact archive SHA256 from GitHub `ceb8222388d2b0ae9c7d097ee85aeccd5641dfb0d668e1576ba71e33ec1ef352`. Neither an independent signed PCC validation nor full nongeneric extraction has been completed.

**New decision:** No further arbitrary right-block exponent tuning on this single curve before genuine B8 public-only F4. #4692 should start with k=4 and can use the precomputed C(Delta) identity to explain when a scale change might matter; use the frozen 46+/18− controls before comparing k and certainly before promoting any complete-solver claim.

## Next solver gate

Implement in **#4692** an actual public-target-only RR9/F4 witness extractor that never looks up a generic four-sum table. Reproduce the sealed 46-positive/18-negative B8 corpus, count failed queries, eliminate twist-only roots, replay all recovered signs and factor indices. Only then plug it into original Rust rank31/1024 whole-solver and compare setup+online+LA+verification against optimized BSGS, best IC, and rho on one runner. No solver win or signed PCC validation claimed by this study.