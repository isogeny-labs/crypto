# R31-S3-XFIBRE — precompute square roots once per x-fibre, then solve S3 quadratics
Date: 2026-11-16 UTC. Standing: **prospective** Rust-only development experiment, no measurement at registration. No novel non-generic claim. Existing #4391 unequal 384/384/16384 published APS control is frozen and unchanged.

## Previous leads and literature reviewed before this experiment
- Source-exact #4391 [APS_UNEQUAL_PROFILE_384_FRESH_REPORT_20261116.md](APS_UNEQUAL_PROFILE_384_FRESH_REPORT_20261116.md): 24 fresh complete prime31 scalars, old 1536-equal 1111.688ms, optimized 384/384/16384 278.384ms, exact source-run 43817946682 [pcc:unverified]. The existing algorithm uses one Tonelli square root **per visited pair** of the first two input lists, except diagonal x.
- Previous #4383 T1024 pilot reversed on 24 fresh cases, so this study requires a new fixed untouched 24-Q panel, alternated order and A/A. #4188 root index is generic MITM, #4373 chained S3 and anchored m6 F4 are censored, #4228/#4349 quotient/certified work remains separate; #4391 symmetric m8 B4/B8 exhaustive 26,400 checks validate GIVEN f, not target-conditioned root discovery.
- Published foundational references: Petit–Kosters–Messeng (PKC 2016 DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala (FFA 2018 DOI 10.1016/j.ffa.2018.01.009); Semaev (ePrint 2004/031). These establish the polynomial/one-relation methods. No priority claim for discriminant factorization.
- Sources checked (KB): unavailable: no research-KB MCP tool loaded and no repository checkout to run wrapper. Current GitHub primary sources and primary literature metadata/abstracts used as bounded fallback.

## Exact mathematical hypothesis

For E: y²=x³+Ax+B and S3(X,Y,Z)=aZ²+bZ+c where
a=(X−Y)²,
b=−2((X+Y)(XY+A)+2B),
c=(XY−A)²−4B(X+Y),

the **polynomial identity in Z[X,Y,A,B]** is

    b²−4ac = 16 (X³ + A X + B)(Y³ + A Y + B).

Thus, for any two *rational abscissae* x,y on E, compute arbitrary roots r_x²=x³+Ax+B and r_y²=y³+Ay+B **once per first/second list point**, *using only x and finite-field operations* (not stored source ordinates or EC addition). Then +4 r_x r_y is a valid square root of every pair discriminant. Both ± roots of S3 are obtained using the identical already-present batch field inversions. This replaces O(number of pair trials) finite-field square-root exponentiations by at most |A|+|B|, with the exact same candidate x roots and full signed ECDLP validation.

This is an arithmetic optimization of a *generic MITM-equivalent* published APS one-relation implementation. Known rational x-fibres themselves are public; this is NOT a new asymptotic or genuinely non-generic attack. It must be judged as a complete same-algorithm internal cost experiment, separate from the non-generic IC frontier.

## Frozen correctness and performance admission

**Fixture:** ordinary cofactor-one E/F_(2147483647), A=847514846, B=2087444724, n=2147446831 prime, same generator and same `[384,384,16384]` walk-labelled source as existing #4391. Preserve all source seeds, public labelled lists and original per-pair S3 code.

**Implementation:** add an opt-in S3 x-fibre mode to existing Rust `prime31_aps_one_relation.rs`. Do not copy/vend any F4 or source implementation. A parallel unit test must, across the frozen public point sources and diagonal exception controls, check discriminant factorization, old/new complete signed x roots, and exact candidate order. The x-fibre precomputation is charged inside the measured algebraic phase; count exponentiations and pair multiplications, don't count a field sqrt as a saved EC addition.

**Panel:** 24 *new* deterministic public Q generated from fixed caller-only `0x5846_4942_5245_3234` seed, not used in tuning. For each, alternate `legacy`, `x_fibre`, `x_fibre_repeat` execution order. Each arm cold-builds all three public labelled lists, charges all failed rounds (cap 6), independently returns and verifies the target scalar. Record every Q, caller-only k, exact attempts, root evaluation counts and times. No hidden generic control is allowed to provide an answer.

**Prerecorded provisional internal improvement gate:** 24/24 valid returned logs in each arm, same source/algebraic pair-trial counts and same exact scalar per Q, candidate total <=80% of legacy full cold wall, candidate A/A drift <=5%, and >=18/24 paired full cold wins. This is only a development gate; no statistical significance, fastest-external superiority, independent PCC or general security claim follows.

**Stop/failure:** if the product-root discriminant identity fails for any case, if diagonal branches differ, or any recovered scalar differs, record the failed exact case and do not promote a gain. If candidate fails the timing gate, freeze a negative result; do not select a new profile from the confirmation panel.

## Distinct scientific followup

The **non-generic frontier** is not solved by this shortcut. It still requires target-conditioned algebraic selected-root recovery on the symmetric eight-summand quotient (`f(T)|H(T)^8`), without enumerating candidate f or precomputing group sums, plus full cold PKM/APS/F4/CAS comparisons. This optimization is only a stronger complete-Rust cost control for evaluating that genuinely different future algorithm.
