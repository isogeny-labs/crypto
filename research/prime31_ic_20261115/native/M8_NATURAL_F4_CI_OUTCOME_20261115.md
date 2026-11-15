# EXP-R31-D: prime31 natural F4 result and fixed-S3 bottleneck (2026-11-15)

**Standing:** GitHub Actions producer/development execution, **[pcc:unverified]**, no signed independent PCC standing. Run [43719063036](https://github.com/isogeny-labs/crypto/actions/runs/43719063036) compiled and executed the first natural panel; run [43719826200](https://github.com/isogeny-labs/crypto/actions/runs/43719826200) compiled and executed the six-arm ablation and the independently replayed public-mask panel. These are CI *development* runs, not isolated ecbench/EKS measurements. This document does not claim a cheap complete ECDLP solver.

## Inputs, prior work, and reproducibility

Work strictly on the inherited ordinary, full-prime-order prime31 target: `p=2^31-1=2147483647`, `E:y²=x³+847514846x+2087444724`, `n=2147446831` prime, cofactor one, `G=(625037715,2100627251)`, public `Q=(1547953785,1667575595)`. The inherited depth-six Dickson fibre has 31 canonical curve points. No smooth-order source curve, lookup-DLP shortcut or C++ oracle replaces the non-generic test.

The frozen [16-mask panel](M8_NATURAL_F4_PANEL_20261115.json) is generated from public `R=[a]G+[b]Q` points using the previously registered deterministic seed, not from planted witness requests. A *separate* generic pair-table **test-only** diagnostic found 5 masks positive within the registered restricted split (six signed templates), 11 without a witness under **that split**. The remaining eleven are *not* known negative to the full m8 ideal.

The actual `--panel-f4` command invokes the shared **Rust** `src/cryptanalysis/f4_fp_tower.rs` engine through the existing `prime31_algebraic_ic` adapter. It constructs S4(x0,x1,x2,u), S4(x3,x4,x5,v), S3(x6,x7,w), S4(u,v,w,x(R)), imposes the inherited Dickson tower equations, and uses `max_degree=12`, `max_nnz=15000`, `max_dense=15000` and a 5-second cooperative per-query budget. The test-only split labels never condition F4 execution; positive certificates merely check F4 soundness and full-target point identities afterward.

Raw exactly-run stdout and source pins are committed as [16-mask evidence](M8_NATURAL_F4_16_CI_EVIDENCE_20261115.json) and [six-arm ablation evidence](M8_F4_MATRIX_ABLATION_CI_EVIDENCE_20261115.json). The full repo test suite, independent signature and whole-call benchmark remain separate.

## Actual measured **Rust** F4 result

Across all **16** frozen naturally sampled public masks: **16/16 returned `undetermined` due to `oversize`**, not due to elapsed-time timeout; 0 completed polynomial refutations and **0 newly algebraically extracted relations**. Every mask had the same F4 counters:

| F4 field | Every mask |
|---|---:|
| successful/attempted steps in report | 5 |
| degree reached | 6 |
| maximum columns | 429 |
| maximum NNZ | 11,446 |
| maximum reported dense reducer entries | **26,618** |
| elimination multiply-adds | 30,943 |
| reduced-reducer-block dense cap | **15,000** |
| basis elements in partial report | 24 |

The internal F4 clock recorded **1.52–2.14 ms** per mask; its sum was approximately **26.20 ms for all 16**. These numbers are **not** end-to-end process timings, cannot price root extraction or cold setup, and cannot be compared to rho. The significant result is the reproducible **same early resource stop for every public target**, including five independently witnessed positives, not a speed result. The F4 implementation explicitly treats max_dense as a cap on the reduced reducer block B', not the whole Macaulay matrix.

## Six-arm ablation — what caused the stop

On the same frozen public Q and source, the optional `--ablate-f4` probe runs the *actual* F4 on six **different reduced ideals**, purely to isolate solver work. **An ideal with constraints removed is NOT the m8 relation oracle.**

| Input ideal | Result | Steps | Reached degree | Max dense B' | Muladds |
|---|---|---:|---:|---:|---:|
| left S4 only | oversize | 2 | 10 | 2,571 (prior accepted steps) | 14,129 |
| right S4 only | oversize | 2 | 10 | 2,571 (prior accepted steps) | 14,129 |
| **pair S3 only** | **oversize** | **5** | **6** | **26,618** | **30,943** |
| target S4 only | basis without root extraction | 0 | 0 | 0 | 0 |
| three fixed polynomials | **oversize** | **5** | **6** | **26,618** | **30,943** |
| all four (actual m8 system) | **oversize** | **5** | **6** | **26,618** | **30,943** |

**Concrete diagnosis:** `S3(x6,x7,w)` together with the fixed Dickson fibre relations **already reaches the same measured degree-six 26,618-entry reduced-reducer-block stop as the full four-polynomial system**. For the 15k-cap pilot, the first bottleneck arises **before target-dependent filtering changes the measured F4 work**. The left/right S4-only controls also reach independent oversize stops, so removing S3 does not prove the remaining F4 solves. These are observations on this source/engine/cap, not universal Gröbner complexity lower bounds.

This sharpens experiment prioritization: indiscriminately raising the full-system cap, fitting an exponent from 16 censored calls or tuning the group-theoretic hit rate is not yet justified. First address the fixed S3 membership algebra and benchmark alternate equivalent presentations that preserve real signed witness extraction.

## Next experiments — prioritized, with falsifiers

**R31-D1 (fixed S3 preconditioning; highest priority):** reuse the existing **certified paired-S3 quotient/preprocessing from crypto PR #4228** on the exact same B=31 Dickson base. Include preprocessing cost in cold solves, prove equivalence of rational x solutions on small/known-positive controls, preserve repeated-x and infinity exception handling, then compare fixed-S3 F4 stop degrees, B' NNZ, accepted witness extraction and all resource caps. If the new encoding still exceeds cap at or before degree six, record a negative result. Do *not* replace the algebraic target oracle with a generic group hash join.

**R31-D2 (target-conditioned algebraic order):** compare the same complete four-polynomial ideal using incremental fixed-ideal preprocessing vs target equation insertion, and also constrained row/monomial selection. Demand exact equivalence and signed y-lift recovery; a cheaper unconstrained ideal is not a valid win. Register 15k, 30k and 60k dense-cap **development diagnostics only**, subject to resource controls, and stop if memory grows rapidly without target-dependent separation.

**R31-D3 (natural witness extraction and rank):** on the frozen 16 masks plus new held-out public masks, require actual finite-field roots, point-lift and sign verification, canonical relation normalization, independent rank increments, all failed/censored calls and no known-log inputs. The generic 4+4 labels remain a **validator**, never a source of F4 answers or a fallback.

**R31-D4 (full solver and matched controls):** only when the algebraic oracle yields useful independent rows, integrate it in the existing Rust scalar-recovery pipeline; charge factor base, symbolic S3 preprocessing, all calls, linear algebra, final verification, wall/memory and operations under the native ecbench protocol. Compare the best matched optimized rho, BSGS, previous Rust IC; public holdouts, signed PCC independent replay, then larger p before asymptotic claims.

## Prior-art and searched branches

Prior searches are recorded in the [frozen protocol](M8_NATURAL_F4_PANEL_PROTOCOL_20261115.md), with the primary source [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and [Diem, Compositio 2011](https://doi.org/10.1112/S0010437X10005075), plus merged **crypto #4077** (whole-call gamma/delta gate), **#4275** (small planted-positive dense vs tower; no simple enumeration superiority), **#4228** (paired S3), **#3877** (transfers), **cryptanalysis #1536** (auxiliary Rust IC), **crypto-autoresearcher #3305** (previously searched lead atlas) and earlier #4188 m3 projective rank dedup. This is not a claim to have exhausted all methods. Shared research-KB tools and Conductor/EKS checkpoint admission were unavailable in this execution environment; the GitHub Actions development runner is not a substitute for the formal budget and validation gates.

**Boundary:** A cheaper complete **non-generic** ECDLP solver on this ordinary prime31 curve is **not established** by this step. The new result is a reproducible diagnosis of the precise fixed S3 constraint that causes our bounded Rust F4 attempt to terminate before providing algebraic relations.


## Follow-up exact-head Rust test — unused tower blocks ruled out

**Run [43720135195](https://github.com/isogeny-labs/crypto/actions/runs/43720135195), head `fb8d840b1777558727659529ca8d70846ded4530`, development CI success, [pcc:unverified].** A further **minimal-pair** control reconstructs the same S3(x6,x7,w) polynomial with **only two 6-level Dickson tower blocks and one free variable**, rather than retaining all eight blocks of the original balanced S4/S3 system. Both algebraic inputs run through the **same existing Rust F4 engine** with the same frozen 15k caps.

The minimal pair produced exactly the same F4 structural counters as both `pair_s3_only` in the large ring and the full four-polynomial system: **oversize true, 5 steps, degree 6, 429 columns, 11,446 NNZ, 26,618 dense reducer entries and 30,943 elimination muladds**. The minimal pair's *internal* F4 elapsed clock was ~1.48 ms, not a cold solve time. All seven ablation arms, 16 natural-panel calls and the independent mask-certificate replay passed their focused CI development steps. The [machine-readable original CI output](M8_MINIMAL_PAIR_S3_CI_EVIDENCE_20261115.json) includes run/source pins.

**Resolution:** The recorded degree-six/26,618-entry threshold is reproduced by the **intrinsic paired S3 + two Dickson fibres subsystem**; carrying six unused fibres is **not** necessary for this stop. This closes the unused-variable hypothesis for this exact engine/cap/input. It neither proves a Gröbner lower bound nor claims all presentations of the same relation are expensive. Prioritize **certified quotient solving or elimination of this specific pair constraint** (#4228), with small-case solution-set equivalence and signed full-target replay, *before* changing unrelated factor-base partitions. If that reduces matrix work, reintegrate into the complete target-dependent ideal; never count the reduced pair ideal alone as an IC oracle.
