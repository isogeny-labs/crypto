# Frozen protocol: W8 ternary superincreasing signed weights on original Prime31

Registered: 2026-11-18 UTC, before source-specific GitHub Actions runs. Stacked draft [#4770](https://github.com/isogeny-labs/crypto/pull/4770) on [#4728](https://github.com/isogeny-labs/crypto/pull/4728). All results are producer-side [pcc:unverified]; the generic oracle is a **grader**, NOT a non-generic ECDLP extractor.

## Exact mathematical prediction (no independence assumption)

Let each of eight slots j choose one of B labelled base symbols and a sign ±, and map the resulting word to its integer formal coefficient vector c in Z^B. For ternary weights w_j=3^j (j=0..7), the map is **injective** for every B>=1: if two words differ, take the largest slot j at which their signed labelled choices differ. In a coordinate changed by that slot, the coefficient difference has magnitude at least 3^j, while all lower slots combined can change that coordinate by at most 2*(1+3+...+3^(j-1))=3^j-1. Contradiction. Distinct **formal** vectors need not have distinct EC sums, because the base points satisfy dependencies modulo group order n.

Thus for B=8 there are exactly 16^8=4,294,967,296 distinct signed formal vectors; the older W8 weight schedule (1,3,5,7,11,13,17,19) has 3,240,125,537 by #4722's exact Rust/Python census. The formal support expands by 32.5556%, but both counts EXCEED n=2,147,446,831, so this **does not imply any nontrivial bound or advantage** in natural relation yield. For B4 the respective counts are 8^8=16,777,216 versus 5,491,889 (factor 3.0549), again only *formal* counts. Any efficiency gain must be measured on public residuals and charged fully.

## Immutable curve and implementation

p=2,147,483,647=2^31-1; a=847,514,846; b=2,087,444,724; n=2,147,446,831 prime; cofactor=1; G=(625037715,2100627251). Reuse the original Rust PRIME31 Arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`, canonical first 31 rational-x points, windows (0..7),(8..15),(16..23),(23..30), and original incremental rank-31 RREF / shifted 1,024-log recovery.

Two arms in **one identical Rust source**: OLD=(1,3,5,7,11,13,17,19) and TERNARY=(1,3,9,27,81,243,729,2187). All other EC operations, group hash maps, factor-base points and Rust implementation are identical. The 4+4 sum tables are explicitly built by generic group operations and charged in setup. Both candidate arms use only public targets/shifted public points at their lookup interface; the test secret is used only to generate inputs and verify a recovered log, never passed to `Oracle::find`.

## Controls and registered design

- Assert 64 weighted slot x-abscissae are distinct per window; reject any collision instead of silently filtering.
- Log each half-table size/collision (directly in reference preflight; Rust checks at least 64,000 distinct, and exact 65,536 on TERNARY).
- Verify group addition identities for all accepted rows and public recovered logs, rank 31/31, 1,024/1,024 targets or record a failure with all work charged. No scalar leaks into candidate relation test.
- Deterministically derive each public target scalar from (master seed, target index) with a **separate** derivation for each offset attempt (target index, attempt); this fixes identical public target panels across weights regardless of differing success counts.
- Execute source-specific OLD,TERNARY,TERNARY,OLD (ABBA) complete 1,024-target runs, with one release binary, identical compiler/host. Count EC additions and total field multiplies/inversions, train/query counts, time per phase and wall. Report both time orderings rather than a cherry-picked speedup.
- Run 40 unchanged library regression tests and 3 isolated Rust tests, then a separately implemented Python BigInt verifier for EVERY accepted training relation, recovered base log, and 1,024 target from EVERY arm; use no Python solver.
- Compare natural hit yield and fully paid cost. 1,024 complete correctness is the correctness gate; a speed advantage against old W8 is an implementation improvement, not a non-generic prime-field ECDLP breakthrough.

## Pre-registration observations, separate from scored run

Before creating this protocol, a **development-only** independent Python original-curve affine group preflight checked exactly 65,536 distinct four-slot EC sums in each half of ALL FOUR B8 windows for ternary weights; old weights had 64,737 distinct sums per half. On the first 256 independent public Q points from the archived original-window corpus, on *window zero only*, exact generic-grader membership was OLD 201/256, TERNARY 221/256, paired discordance OLD-only 28 vs TERNARY-only 48, both 173, neither 7; paired two-sided exact binomial p=0.02863. This is **exploratory**, conditioned on an older corpus, one window, and an independent Python grader, not source-bound Rust validation and not an independent held-out result. The ABBA Rust screen uses new deterministic per-target derivations as its scored panel.

## Existing leads / negative controls checked

- #4722: exact old signed formal count; #4728: verified generic 4+4 31-rank/1024-target control, but old complete time 14.098 s; #4714: earlier six-slot M6 and matched shared-BSGS loss; #4692: existing draft public-target-only RR9/F4 unknown-index extraction, unchanged here; #4720: signed-pair projector.
- crypto-autoresearcher `experiments/semaev-alternatives-20261105/SUMMARY.md`: SA-04 RR norm and SA-18 determinant method negative, SA-19 twisted-Edwards limited scope; do not rebrand formal coordinate changes as a new algorithm.
- PKM 2016 (DOI 10.1007/978-3-662-49387-8_1) and APS 2018 (DOI 10.1016/j.ffa.2018.01.009) are relevant prior prime-field index-calculus methods.
- Shared research-KB connector unavailable in this execution; cross-repo GitHub source, prior result and primary publication review is bounded, not exhaustive.

## Go / no-go

If ternary has higher natural hit probability and lower fully charged 1,024-target Rust cost on ABBA, retain it as a candidate **weight-choice control for #4692**, BUT re-evaluate the public-only algebraic selector's polynomial degree, coefficient height/bit complexity, multiplicity and runtime: 3^j weights multiply larger points and may make unknown-index equations harder. If no fully paid improvement, retire ternary retuning as a speed lever. In neither case may a generic 4+4 grader's speed stand in for actual non-generic F4 extraction, or the old ~144 ms shared BSGS from a **different runner** stand in for a same-runner comparison. A true solver breakthrough still requires generic-table-free public-only relations, rank 31, 1,024 blinded logs and same-runner optimized index calculus, shared BSGS and rho.
