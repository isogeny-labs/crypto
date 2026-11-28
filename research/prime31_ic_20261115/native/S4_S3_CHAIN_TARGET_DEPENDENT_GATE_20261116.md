# EXP-R31-SPLIT: native Rust S4 resultant versus chained S3 prime31 gate

Date: 2026-11-16 UTC (2026-11-15 Pacific). **Standing: proposed/committed source, [pcc:unverified].** No successful exact-head Rust execution, extracted arbitrary m8 relation, complete held-out scalar recovery, or lower complete ECDLP cost is asserted in this text. The new exact-head CI outputs must be checked separately.

## Scientific question

On the established **ordinary cofactor-one** curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831` prime, `G=(625037715,2100627251)`, can preserving a rational intermediate pair abscissa reduce the **F4 computational cost of the fixed three-point S4 blocks** compared with the materialized high-degree resultant? In particular, can the reformulation allow a genuinely target-dependent m8 polynomial system to run within existing Rust `TowerRing` constraints?

The original m8 input from #4188 is
```
 S4(x0,x1,x2,u)=0
 S4(x3,x4,x5,v)=0
 S3(x6,x7,w)=0
 S4(u,v,w,x_R)=0
```
plus eight Dickson-fibre membership conditions and subsequent full-y signed point checks. The fixed pair-S3 preprocessing already gives **961 finite w values** (930 if excluding duplicate-index diagonal) over this B31 fibre. The original all-eight Dickson-tower m8 native F4 stopped at the 15k dense-reducer budget on **16/16** natural targets with 0 completed roots. Certified finite-pair elimination followed by the original two S4 equations still stopped with degree ten on all 8 sampled w branches. Minimal single-S4 three-tower tests also stopped at degree ten: 14,129 partial F4 multiply-adds. These are capped diagnostic counters, not solver costs.

## New algebraic presentation

For the regular affine branch and any three rational curve points whose initial pair sum is not infinity, write the one-variable auxiliary pair abscissa `h=x(P0+P1)`. Replace a single S4 by
```
 S3(x0,x1,h)=0
 S3(h,x2,u)=0.
```
Eliminating `h` gives the usual resultant S4 condition **over the algebraic closure**, apart from leading-coefficient/identity saturation subtleties. However, a rational solution of the S4 projection does not in general imply that every resultant witness variable is rational. Hence finite-rational equivalence, nonreduced multiplicities, signs, vertical lines, repeated indices, identity pair intermediates and artificial Fp roots require explicit tests; a matched known regular positive is **not** an equivalence proof.

A single-S4 three-tower ablation compares the inherited original resultant to all three pairings (0,1), (0,2), and (1,2) using two S3s; the three actual signed positive witnesses for the frozen m8 record are checked before and after F4. The output reports input normal-form terms, reached degree, F4 muladds, matrix sizes and censored status. These are *fixed-ideal* experiments, not m8 oracles.

**Target-dependent integration**: After selecting an algebraically certified fixed-pair `w`, the remaining six Dickson point leaves need only the four ordinary auxiliary coordinates `u,v,h_L,h_R`. The full system is
```
 S3(x0,x1,hL)=0
 S3(hL,x2,u)=0
 S3(x3,x4,hR)=0
 S3(hR,x5,v)=0
 S4(u,v,w,x_R)=0.
```
This fits **36 quadratic tower variables + four free variables**, the existing native Rust F4 `n_free<=4` limit. It is a *target-dependent polynomial ideal*, not a generic EC pair-sum lookup. It is still only one conditional branch until the full 930 (or 961 with diagonal exceptions) w-values and exceptional cases are covered. A fixed positive control is used only to validate F4 soundness; seven already-registered public-Q w-controls are run without positive labels inside F4. The same-host original S4/S4/target ideal is also executed for the positive w-value for a fair *capped stage* comparison.

The code **reuses** the current `prime31_m8_balanced_f4_input.rs` polynomial and S3/S4 constructors, `Arithmetic`, `Base`, original `f4_fp_tower.rs::f4_tower`, and existing frozen natural-positive/public-Q controls. It uses no C++ or Python implementation. No secret scalar is supplied to F4; signed EC additions appear only in fixture construction and after-the-fact witness checks, not discovery.

## Why this is a discriminating experiment

- **A**: If the split-S3 F4 presentations also oversize (possibly earlier than the materialized S4), preserve that negative result. Extra free dimensions and denser eliminations are genuine counterarguments to splitting.
- **B**: If one or more split presentations complete under the same caps with exact comparable rational solutions, pursue certified rational-root projection/saturation and full branch coverage. Require complete root extraction and actual natural positive useful-rank before calling it an oracle improvement.
- **C**: If both presentations censor, stop varying only the w-root or unrelated towers; prioritize #4228/#4349 certified quotient/subresultant root work or an independently checked high-performance external CAS. No ratio of partial F4 engine clocks to complete results is admissible.

The bounded development limits stay `max_degree=12`, `max_nnz=15000`, `max_dense=15000`, `budget=5s` per call. The new mode explicitly calls `capped_f4` and reports **undetermined** upon oversize, timeout or incomplete pairs. It does not claim completed Gröbner bases, full ECDLP costs or root counts from partial bases. The 15k reduced-reducer cap is NOT a cap on entire Macaulay matrix.

## Reproduction (Rust only)

```sh
cargo +1.100.0 test --locked --release --lib --manifest-path research/prime31_ic_20261115/native/Cargo.toml
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_balanced_f4_input -- --split-s4-f4
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_balanced_f4_input -- --m8-s3chain-f4
```

New focused GitHub Actions development workflow `.github/workflows/prime31-s4-split-20261116.yml` runs the same release-profile Rust calls on the branch/PR. The source was edited through a GitHub-only connector; this environment has neither a Rust toolchain nor authenticated EKS/Conductor/KB admission. The workflow's **actual conclusion and raw stdout must be inspected before updating numerical findings**.

## Prior leads and frontier comparison

1. **Already searched original fixed-pair and triple elimination:** #4188 merged into main 2026-11-16 UTC; exact pair H2 degree 961 / H2-distinct 930, triple K3 17,980 distinct rational roots, target-independent S4 and S3 F4 failures; `native/PAIR_S3_ELIMINATION_POLYNOMIAL_20261115.md`, `native/S4_TRIPLE_QUADRATIC_ELIMINATION_20261115.md`, `native/M8_NATURAL_F4_CI_OUTCOME_20261115.md`. This attempt changes **F4 ideal presentation**, not point support enumeration and not the finite-field root census.
2. **Prior candidate IC implementation and cost gates:** #4275 dense-versus-tower, #4077 whole-call rank/cost rule, #4228 (now merged) certified quotient and ideal membership, #4349 (open) compact certified root projection; its wide-prime F4 contract is not yet demonstrated. The earlier native complete S3 root-index+LA 46.2s scalar recovery remains *generic meet-in-the-middle-equivalent* and is NOT an IC frontier champion.
3. **Cross-repo:** `isogeny-labs/cryptanalysis` #1536 Rust prime-field auxiliary IC; `isogeny-labs/crypto-autoresearcher` #3305 Semaev lead atlas; #3877 transfer thread. These have no independently qualified cheaper same-target prime31 non-generic complete solve to import.
4. **Published:** Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1 (prime-field rational-map/summation polynomial IC); Amadori–Pintore–Sala, FFA 2018, DOI 10.1016/j.ffa.2018.01.009 (one-factor-base-relation alternative). Rewriting S4 as chained S3 is classical resultant algebra and no novelty is claimed. Journal metadata/abstracts and the published PKM paper were consulted; full APS text was not authenticated here. KB MCP and repo wrapper unavailable: **bounded** review, not exhaustive novelty search.

**Proper frontier-first protocol**: compare complete, exact-output target-dependent algebraic queries on matched inputs against PKM-style, APS-style, native dense/tower F4 and the certified quotient worker; external msolve/Singular for mature Gröbner engine claims. Do not invent APS runtime numbers if that arm is not implemented. Then require actual natural-positive signed m8 rows, enough independent rank to solve 31 base logs plus unknown Q, twelve held-out cold unknown target recoveries, complete setup/failures/linear algebra/verification, and finally matched optimized Rust negation-rho as a *separate* whole-ECDLP security control. Any independent PCC endorsement requires a separate verifier.


## Actual Rust development execution — negative F4 presentation result

**Exact PR-source job:** [GitHub Actions 43783635698](https://github.com/isogeny-labs/crypto/actions/runs/43783635698), job `133026303660`, head `b4b19f5afbc498cb50d54ccde4b061c95e13acee`, checked-out PR merge `4991909c` over base `92b7b35f`; completed **success** 2026-11-16 01:01 UTC. This qualified **source-specific development execution** (not independent scientific PCC): 40/40 Rust release library tests passed and both new Rust modes compiled and ran. Exact stdout and F4 counters are frozen in [S4_S3_CHAIN_CI_EVIDENCE_20261116.json](S4_S3_CHAIN_CI_EVIDENCE_20261116.json). No raw stage time is called cold IC or used as a completed-ideal speed ratio.

| Frozen original-curve Rust ideal | Status | F4 degree reached | Partial F4 muladds | Reported attempted/peak dense reducer |
|---|---|---:|---:|---:|
| Single S4 resultant, 3 towers + u | **oversize / undetermined** | 10 | 14,129 | 2,571 previously accepted |
| Two S3s with pair (0,1), 3 towers + u,h | **oversize / undetermined** | 6 | 30,993 | 30,969 attempted |
| Two S3s with pair (0,2) | **oversize / undetermined** | 6 | 30,993 | 30,966 attempted |
| Two S3s with pair (1,2) | **oversize / undetermined** | 6 | 30,993 | 30,964 attempted |
| Complete conditional m8 old two S4s + target S4, fixed positive w | **oversize / undetermined** | 10 | 56,211 | 10,229 previously accepted |
| Conditional m8 four S3s + target S4, fixed positive w | **oversize / undetermined** | 5 | 41,032 | 17,433 attempted |
| Same conditional m8 S3 chain, seven public-Q w controls | **7/7 oversize / undetermined** | 5 | 41,032 **each** | 17,433 attempted **each** |

All 3 single-side chained presentations retained the original signed witness and all 8 conditional m8 branches executed without invalid known-positive refutation. **No chained arm completed the ideal, no roots were extracted, and no independent relation rows or scalar were recovered.** The caps and internal F4 clocks are stage diagnostics only. The original S4 single-side and full conditional m8 controls **also failed** to complete, so neither 30,993 versus 14,129 nor 41,032 versus 56,211 is a valid completed-work speed ratio.

**Decision:** This particular low-degree S3 chain does **not** fix the early F4 matrix blockage. Its three single-side variants consistently hit a larger attempted reducer around 30,965 entries at degree six, while the full conditional m8 S3 presentation also crosses 15k (17,433) by degree five. Changing the order of the initial pair did not give a gate breakthrough on this fixture. Increasing budget or merely cycling all w-roots before changing the fixed-ideal engine would repeat known censorship; do not claim an IC gain.

**Next discriminating mathematical/engineering gate:** move *target-conditioned* elimination into the **certified quotient/subresultant/selected-root** engine from the now-merged #4228 and open #4349, but first establish the missing supported 31-bit field route. Preserve exact original-ideal certificates, exceptional saturation, full rational root lifting and independent rank. Compare completed calls at identical inputs against PKM/APS algebraic IC frontier, then the complete cold-target cost independently. This experiment has closed the straightforward low-degree-vs-resultant presentation hypothesis at the registered cap, not ruled out other presentations or stronger solvers.
