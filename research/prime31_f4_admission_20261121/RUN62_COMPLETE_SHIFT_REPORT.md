# Prime31 run62 — close the run61 fixed-G Jacobian completeness gap

**Date:** 2026-11-22 UTC. **Validation:** exact independent Python field/group tests PASS in local development environment; native source-linked Rust run62 has been committed and GitHub Actions CI is configured, but at report issuance no source-specific PASS/benchmark receipt was available. No qualified all-in solver speedup is claimed.

## Input scope and previous work

Unchanged **ordinary** ECDLP curve over p=2^31-1=2147483647: `E: y²=x³+847514846x+2087444724`, prime group order `n=2147446831`, cofactor 1, generator `G=(625037715,2100627251)`. The original `prime31_algebraic_ic::{Arithmetic,PRIME31}` Rust implementation remains the independent implementation oracle; this work supplies an **opt-in local arithmetic experiment**, not another ECDLP or EC backend.

- **Run61 #4901:** partial mixed-Jacobian/affine fixed-G 26.943ns versus original affine 282.940ns (10.501x, original Rust source-specific PASS #45017580013); it panics on H=0. The 4,699,949 run59 query model suggests shift-only 126.63ms versus 170.113ms historical 1024-target shared BSGS. The hosts/runs differ.
- **#4391 prior engineering:** `APS_JACBATCH_FRESH_REPORT_20261116.md` already implemented Jacobian batch normalization and Mersenne arithmetic; neither the group law nor Mersenne field arithmetic here is a novel cryptanalytic method. Its `APS_COMPACT_X_INDEX_RESULT_20261116.md` has complete 24-Q Rust APS 75.172131ms vs same-run full-cold BSGS 73.817385ms; **APS remains slower** in that paired test and its finite-list S3 search is generic MITM equivalent.
- **Other live algebraic lanes:** #4692 exact blinded selector/RR9 registered but checkpoint-blocked; #4812 signed Horner/F3 14-variable correctness with no blind roots; #4822 four-variable RR9/Krylov 2/2 focused tests but no completed blinded extractor; #4891 actual `f4_fp` S4 at factor-base sizes 6/8 takes 1.7–10.7 seconds, k>=10 timed out at 120s, not a Prime31 full solver. Run55–60 #4901 two-summand exact support/hybrid rank and target visibility are *conditional sampling models* beyond their certified algebraic equalities.
- 2018 Amadori–Pintore–Sala [DOI 10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009) is one-relation prior art. Pseudo-Mersenne field reductions are standard, e.g. [Solinas/generalized Mersenne arithmetic discussions](https://arxiv.org/abs/1108.3054). Repository KB MCP unavailable in this environment: this is a bounded repo/PR/paper prior-work audit.

## New completeness finding (exact algebra)

The run61 partial `Jac::add_fixed` implements only the generic mixed branch:
`U2=g_x Z^2; S2=g_y Z^3; H=U2-X; R=S2-Y`.
For a finite Jacobian point and rational G:
- `H != 0` is the regular mixed addition formula.
- `H == 0 && R == 0` is **Q = G** and requires doubling (run61 panics).
- `H == 0 && R != 0` is **Q = -G** and requires the point at infinity (run61 panics).
- An incoming infinity must map to **G**.

On the ordinary odd prime-order group these exceptions occur at scalars k ≡ 1, -1 and 0 mod n. Since any n consecutive fixed-generator increments visit every group point once, the exception branches are mathematically mandatory for **complete** repeated public shifting. A benchmark that excludes them is valid for its measured finite prefix but is not a drop-in complete solver component.

## Exact Mersenne reduction and independent tests

For 0 ≤ a,b < p and t=a*b < p² < 2^62, use `s=(t & p)+(t>>31)`; since `s < 2p`, subtract p at most once. Conditional one-subtraction addition and conditional one-addition subtraction are exact for canonical residues. Field reduction is standard arithmetic, not an ECDLP shortcut.

The independent `run62_independent.py` checks 120,025 canonical boundary/random operand pairs against unbounded Python modular operations, including near-(p-1)^2 products, and 96 different original-curve scalar-start chains × 128 consecutive additions = **12,288** projective additions against separately authored full affine EC arithmetic. Boundary chains include k=0,1,2,n-1,n-2, plus 88 seeded random k. **All local Python checks passed**. Known k is input solely to correctness fixtures, not a blind solving path.

The source-linked Rust `prime31_run62_complete_shift.rs` additionally checks 120,049 original `u128` modular boundary/random pairs and 96×128 group steps using the actual existing Rust `Arithmetic`; its intended same-process, five-repetition benchmark compares original complete affine, prior partial Jacobian, and new complete folded Jacobian on equal tested endpoints. Rust outcomes and exact medians remain **pending** until CI returns raw evidence.

## Quantified admission gate

Historical conditional 1,024-target model run59: 4,699,949 uniform shift probes. Historical BSGS reference: 170.113ms from a separate run. The per-shift all-in sensitivity is **36.195ns**, while run61 partial primitive was **26.943ns**, leaving **9.252ns/query** for *all other operations* in that hypothetical model. This is **not** a measured current same-host crossover and is not a lower bound for other algorithms.

Accept an exception-safe candidate only with all tests passing and median complete-shift below matched original affine. If candidate median >=36.195ns, shift-alone makes that specific historical comparison unfavorable *assuming the 4,699,949 expected shifts are actually required*; if faster, public-only algebraic extraction, free-row rank, setup, logarithm verification and generic controls are still unpaid. The model may have correlated or nonuniform samples, so do not promote extrapolations into complete ECDLP costs.

## Prioritized experiment queue / stop criteria

1. **P0, already assigned #4692/#4812/#4822:** unknown-index, public-Q-only RR9/F3 original-ideal-certified root extraction on held-out natural positives/negatives and exceptional points. No secret witnesses or pair/half tables in discovery. Capture correct roots, complete root-set qualification, timeouts and memory.
2. **P1, present PR #4901:** execute run62 source-bound CI and check the precise complete/partial/affine median plus correctness. If complete primitive is acceptable, integrate it behind an opt-in adapter within existing original Rust index-calculus code, not new EC solver. Charge failed queries and every exception.
3. **P2:** progress only after an actual blinded algebraic root: quotient rank with 88 free rows, independent recovered `[k]G=Q` and 1/32/128/1024 complete logs. Same-runner cold and amortized comparisons against strongest original Rust index calculus, optimized negation-BSGS and rho with setup/failed probes/linear algebra charged.
4. **P3:** direct F4 16-var W9 vs compact 14-var Horner vs 4-var RR9 ablation at matched caps, but treat #4891 multi-second small-base F4 and previous large-MQ matrix ceilings as admission warnings, not unsatisfiability evidence.

**Scientific disposition:** mathematical repair and a measurable engineering hypothesis, not an established non-generic index-calculus breakthrough. No new complete prime-field ECDLP speed advantage is supported at this stage.

## Source-specific Rust execution — PASS, measured component improvement

The earlier CI-pending status above is superseded for the initially introduced Rust source/workflow by **[Actions #45022038604](https://github.com/isogeny-labs/crypto/actions/runs/45022038604)**, job `137206872980`, **SUCCESS** on source/workflow commit `a0c63206805924035d35801deb9ca6fb645f23f7`, Rust 1.100.0 release (12.86 seconds Cargo build). It printed `correctness=PASS` for 120,049 exact canonical field pairs and 96 separate 128-step chains (12,288 real curve additions), including O, -G, G doubling and order wrap. All five per-arm endpoints were independently checked with the original `Arithmetic::scalar`. Full raw vectors and exact source binding are frozen in `RUN62_RESULTS.json`.

| Equal-runner Rust +G primitive, five 250,000-step repetitions | ns/step, median | All 5 ns/step |
|---|---:|---|
| Original complete affine `Arithmetic::add` | **289.9812** | 290.9099, 289.9687, 290.1010, 289.9325, 289.9812 |
| Run61-equivalent partial generic mixed Jacobian (`H=0` unsafe) | **23.6105** | 23.6105, 23.8345, 23.5962, 23.3134, 23.7230 |
| **Run62 complete exception-safe Mersenne-folded** | **21.5597** | 21.7635, 21.5597, 21.5963, 21.5482, 21.5085 |

Run62 is **13.4501× faster than original affine on the matched public +G primitive** and **1.0951× faster than the previous partial formula** despite adding completeness branches. The prior published 10.50× result used a different Actions run and should *not* be directly pooled into this new ratio.

The run59 **conditional** 4,699,949-probe projection now charges **101.330ms shift-only**, leaving only **68.783ms** (14.635ns/probe) for *all* other operations relative to the previous **cross-run** 170.113ms BSGS reference. This is a changed sensitivity calculation, not an executed ECDLP solve, not an independent root extraction, and not a same-host solver advantage.

The later workflow update also added a separate Python big-int verifier step. Its source-specific CI result is not included in the earlier #45022038604 receipt; the independent Python verifier had already passed locally. This is an explicit validation distinction, not an assumption of green CI.

## Second, fully independently replayed source-specific CI: PASS with important timing variance

The complete workflow, now including the **separate independent Python big-int verifier**, subsequently passed **[Actions #45022156215](https://github.com/isogeny-labs/crypto/actions/runs/45022156215)**, job `137207239654`, under Rust 1.100.0. Its job returned **SUCCESS** with Python 120,025 canonical field operand pairs and 96×128 full group additions, **plus** original-Rust 120,049 field pairs and 96×128 exact group steps; all accepted. All exception branches and five full benchmark endpoints match the original arithmetic. The result is independently implemented Python plus source-linked Rust checks, not independent signed PCC.

On this second GitHub runner repetition the complete median was **21.8733ns/shift**, the old partial median **21.9587ns/shift**, and original affine **384.8678ns/shift** (17.5954× complete vs affine). The complete-vs-partial ratio was only **1.0039×**, compared with **1.0951×** in the first job. This **fails any claim of a stable 9.5% arithmetic-reduction advantage over the prior partial implementation**. The original affine control also drifted substantially between jobs (289.98→384.87 ns), so cross-run speedups must not be pooled.

The reproducible conclusion is instead **mathematical completeness with virtually unchanged fast mixed-Jacobian throughput (~21.56–21.87ns/shift on these two jobs)**. The cross-run sensitivity for the run59 conditional 1,024-target model becomes approximately **101.33–102.80ms of shift-only operations**, leaving a hypothetical **67.31–68.78ms** for the rest versus the separately measured historical 170.113ms BSGS result. No complete log extraction was performed.

**Interpretation:** keep the complete exceptional-safe implementation as the correctness candidate, but further fold-only timing optimization is low-priority compared with genuinely blind non-generic root selection and actual complete-solver costs.
