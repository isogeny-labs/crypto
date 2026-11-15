# Exact genus-seven hyperelliptic trace classification over F2

Status: complete finite census with two independent native arithmetic implementations; the geometric reduction to the finite search uses published characteristic-two hyperelliptic results. Independent mathematical/PCC review remains open. [pcc:unverified]

## Statement

Let E/F128 be ordinary, let t=129-#E(F128), and set A=Res_(F128/F2) E. A smooth genus-seven hyperelliptic curve Y/F2 with J_Y isogenous to A exists **if and only if**

`t ∈ {-15,-13,-9,-1,1,9,13,15}`.

For each of these eight traces, the census finds exactly one F2-isomorphism class. In the published n=7 W1 source family, `Tr(a)=1`; the q=2 W1 trace set is `{-17,-9,-1,7,15}`. Its genus-seven hyperelliptic isogeny-class screen passes at `{-9,-1,15}` and excludes `{-17,7}`. The trace `+9` comes from `a=0`, so it is a published W0 source with a direct genus-seven Hess descent witness. The `−9` W1 class remains an isogeny-class geometry lead, not a new GHS family or a demonstrated vulnerability. The `±13` hyperelliptic classes are outside the observed W1 source trace set.

The frozen 112-equation census paired each of 56 coefficient values with both `a=0` and `a=1`; it was not a 112-equation W1 census. Exactly 56 rows have `Tr(a)=1` and are W1. Among these, 42 pass the genus-seven hyperelliptic **isogeny-class existence** screen (21 with trace −9, 14 with trace −1, and 7 with trace +15), and 14 fail it (7 each with trace −17 and +7). These are equation counts in the fixed field representation, not counts of distinct isomorphism classes or curves with an explicit subgroup transfer.

## Reduction and proof boundary

The Weil polynomial of A is `X^14-tX^7+128`; its reciprocal L-polynomial is `1-tT^7+128T^14`. If J_Y~A, the point counts are `N_m=2^m+1` for 1≤m≤6 and `N_7=129-7t`. Ordinarity gives 2-rank seven. [Borodin–May, Lemmas 2.1–2.2](https://arxiv.org/pdf/2508.16886) then force the eight geometric Weierstrass points into one rational point and one degree-seven Frobenius orbit; the detailed parity argument is in `BRANCH_SUPPORT_BOUND.md`.

Move the unique rational branch point to infinity. The remaining orbit has one of 18 monic irreducible degree-seven polynomials v over F2. The involution `x↦x+1` acts without fixed septics and leaves nine support representatives. By [Borodin–May, Lemma 4.1](https://arxiv.org/pdf/2508.16886), every genus-seven model in this support class can be written `y²+v(x)y=u(x)` with `deg(u)=15`, leading coefficient one, after a possible shift `y↦y+x⁸` from the degree-16 presentation. The 15 lower binary coefficients give `9·2^15=294,912` equations. Isomorphic duplicates are deliberately retained.

The first Rust implementation enumerates all equations, tests `N_1,...,N_7` by the Artin–Schreier trace criterion, and checks the finite-branch smoothness equation from Borodin–May Lemma 4.1. The second enumerates every `(x,y)` over each F_(2^m), m≤7, and checks singularity directly at all roots of v in F128. Both obtain stage counts `[294912,147456,73728,36864,14336,5888,3328,1024]`, where the last entry has the required six counts, smoothness, and an integral t from `N_7`. The 1024 equations split into eight trace bins of 128 each, with the trace set in the statement. For each bin the producer checked exact equality of its candidate set with the 128-element orbit `u↦u+r²+vr` for `deg(r)≤7`. Distinct bins have distinct zeta functions. The nine-support completeness argument and this orbit check prove one isomorphism class for each surviving t. [pcc:unverified]

The finite search constitutes a reproducible exhaustive computational proof **conditional on the correctness of the two independent implementations**; it is not yet a Lean formalization of the geometry or a signed PCC certificate. The existing `TraceExceptions.lean` checks a separate five-value trace lemma only. The cited geometric lemmas and Tate's finite-field isogeny theorem are used at the paper-proof level.

The seven point counts determine a genus-seven zeta numerator by Newton identities for its first seven coefficients and the Weil functional equation for the remaining seven. Thus a witness with the displayed counts has the full polynomial `1-tT⁷+128T¹⁴`; the direct-pair replay checked all seven counts for every candidate that passed the earlier filters.

## Exact witnesses

Bits of each hexadecimal polynomial encode coefficients of x^i. All models are over F2, have `y²+v(x)y=u(x)`, and have the displayed `N_7`; their `N_m` for m=1,...,6 are 3,5,9,17,33,65.

| t | v | u | N7 | Paired-census source status |
|---:|:---|:---|---:|:---|
| −15 | `0xab` | `0x800b` | 234 | a=0 twist; not W1 |
| −13 | `0xb9` | `0xc020` | 220 | no |
| −9 | `0x8f` | `0xc07a` | 192 | W1; nonclassical coefficient slice |
| −1 | `0x91` | `0xc02d` | 136 | yes, shared |
| 1 | `0x91` | `0x800a` | 122 | a=0 twist; not W1 |
| 9 | `0x8f` | `0x802f` | 66 | W0; direct Hess genus-seven route |
| 13 | `0xb9` | `0x8051` | 38 | no |
| 15 | `0xab` | `0xc004` | 24 | yes, classical |

## Reproduction and limits

The frozen protocol and amendments are in `HYPERELLIPTIC_CENSUS_PROTOCOL.md`. Producer code: `src/hyp_census.rs`; independent replay: `src/hyp_replay.rs`. Final complete raw file: `reports/hyp-census-20261115T0855Z-result.txt`; replay: `reports/hyp-replay-20261115T0853Z/result.txt`. The first display-label error and interrupted orbit assertion are preserved in the protocol and raw files. Run the two programs with Rust 2021 `rustc -O`; their source SHA-256 values and checkpoint receipts are recorded in `reports/HYP_CENSUS_RUN.md`.

This classification concerns exact seven-dimensional hyperelliptic Jacobian isogeny classes over F2. It leaves nonhyperelliptic genus-seven targets, higher-genus covers, correspondence construction, subgroup action, and the full target-recovery cost open. Same Weil polynomial supplies an isogeny class, not an explicit transfer algorithm. No rho comparison is available for these geometry witnesses.

## Prior-work and proof map

The source **W0 and W1 families** and their genus-seven and genus-fourteen direct GHS curves are in [Menezes–Teske (2004/2006), Sections 5.1–5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf). The finite hyperelliptic enumeration method and smoothness criterion are in [Borodin–May (2025), Section 4](https://arxiv.org/pdf/2508.16886); their [code repository](https://github.com/bmatvey/char_2_hyperelliptics) gives a general enumeration implementation. The local research corpus already posed genus-n Jacobians in the Weil-restriction isogeny class as `LEAD-TRF-weil-restriction-jacobian-class`. An earlier internal study at `origin/main:research/ic_next10_20261014/10-torelli-toy/README.md` explicitly listed the +13 genus-seven witness `v=0xb9,u=0x8051`, so the ±13 carrier class is a rediscovery. Focused searches of the primary sources, local corpus, and exact polynomial/trace web queries found no prior statement of the **−9 F2 W1 hyperelliptic isogeny-class match**; this is a bounded search, not proof of publication priority. The predecessor's all-model enumeration may have traversed the ±9 equations without identifying the −9 W1 source trace. The +9 direct descent is already published W0. The −9 equation could be a new exact model or alternate coordinates for a previously known model. Its isogeny-class match is a geometry candidate within the existing W1 family, not a new eligible family or attack method.

| Statement | Lean declaration or evidence | Status |
|:---|:---|:---|
| q=2 seventh base-trace values | `TraceExceptions.q2_base_trace_values` | checked in Lean 4.35.0; no axioms |
| intersection of W1 and hyperelliptic trace lists, excluded list, witness count arithmetic, sample support-count arithmetic | `HyperellipticTraceLedger.w1_hyperelliptic_intersection`, `excluded_w1_traces`, `witness_seventh_counts`, `sample_support_counts_arithmetic` | checked in Lean 4.35.0; first two use only Lean standard `propext`, `Classical.choice`, `Quot.sound`; latter two have no axioms |
| general even-q branch-support count | `BRANCH_SUPPORT_BOUND.md` direct affine-orbit proof | paper-level; no Lean formalization of affine action or Hilbert 90 |
| completeness of branch-support reduction and existence/uniqueness of hyperelliptic classes | Borodin–May lemmas plus paper argument and two finite native runs | not formalized in Lean; geometry and native census are the missing bridge |
| isogeny-class match from equal Weil polynomial | Tate's finite-field isogeny theorem | paper-level application; no Lean instance connecting these models to Jacobians |
| nonzero subgroup-preserving transfer and attack cost | none | open |

The `lean-instance-gate` path referenced in the repository AGENTS.md is absent from this checkout and the listed host skills. We inspected the available `Init` theorem context manually; it contains no ready-made characteristic-two hyperelliptic Jacobian/isogeny structure for this geometry, so the full formalization remains open. `lean4checker` is not installed. The two standalone Lean files were compiled with `lean`; no `sorry`, `admit`, `native_decide`, or custom axioms occur.

The targeted repository proof gate passed: `tools/lean/proof-gate.sh research/ghs_n7_prym_trace_20261115/HyperellipticTraceLedger.lean research/ghs_n7_prym_trace_20261115/TraceExceptions.lean` returned `PASS 4 thm 0 trusted` and `PASS 1 thm 0 trusted`.
