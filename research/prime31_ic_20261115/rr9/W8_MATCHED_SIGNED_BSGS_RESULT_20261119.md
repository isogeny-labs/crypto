# Prime31 same-runner signed shared BSGS control — source-specific evidence tracker

**2026-11-19 UTC.** Stacked scientific control on [#4770](https://github.com/isogeny-labs/crypto/pull/4770) via [#4774](https://github.com/isogeny-labs/crypto/pull/4774). The candidate and every correctness check reuse the pinned original Rust prime-field curve arithmetic, not a replacement ECC implementation. The independent BigInt verifier uses a separate Python curve implementation.

## Registered original-curve benchmark
- Prime field p=2147483647; E: y^2 = x^3 + 847514846*x + 2087444724; prime group order n=2147446831; cofactor 1; G=(625037715,2100627251).
- Full 1024-public-target original-W8 and superincreasing-W8 index-calculus generic-table arms against a *cached signed-BSGS* control, on the same pinned Rust 1.100.0 GitHub Actions runner. Exact same 1024 public Q and reconstruction checks across all six arms.
- Sources: [registered protocol](W8_MATCHED_SIGNED_BSGS_PROTOCOL_20261119.md), [native signed BSGS example](../native/examples/prime31_shared_signed_bsgs.rs), [independent group verifier](../native/verify_prime31_shared_signed_bsgs.py), [source-specific workflow](../../../.github/workflows/prime31-shared-signed-bsgs.yml). Original EC arithmetic pinned at commit 1df714645aa683cf975bc76fcacb9842cb4780fd. Reuses #4770's original/ternary W8 example without modifying its algorithm or seed.
- **Status at report registration: source/CI launched; no new timing claim before exact-source Rust and independent Python replay pass.**

## Exact signed-BSGS invariant (independent of empirical timing)
Set m=32768, h=2m+1=65537. Store jG with x-coordinate canonical up to sign for 1<=j<=m. For every d in {0,...,n-1} take
\`i=floor((d+m)/h)\`, \`r=d-i*h\`. Then \`-m <= r <= m\`, and \`0 <= i <= floor((n-1+m)/h)=32767\`.
A search of public \`Q-i*hG\` against the x-keyed baby table yields +/- jG (or identity), hence candidate \`d=i*h+/-j (mod n)\`. Confirm every recovered d via independent original Rust \`[d]G==Q\`. Table size is exactly 32768, giant probes per target at most 32768, and the expected number is on the order of half that on uniform Q. This is a **generic complete** baseline, not index calculus, and is not independently certified as the fastest optimized implementation.

## Previous leads / areas already searched
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) public-only RR9/F4 unknown signed index extractor — the actual non-generic bottleneck; no tested complete extractor.
- [#4728](https://github.com/isogeny-labs/crypto/pull/4728) four B8 windows with complete rank31 and 1024 unknown public logs, *but generic 4+4 relation tables*; [#4770](https://github.com/isogeny-labs/crypto/pull/4770) paired ternary support improves this generic control by 1.262x.
- [#4772](https://github.com/isogeny-labs/crypto/pull/4772) public H/Jacobian with exact fixed-arity support restriction; [#4773](https://github.com/isogeny-labs/crypto/pull/4773) conditional two-prefix 7-to-5 RR9 reduction with 256 branch cost; [#4548](https://github.com/isogeny-labs/crypto/pull/4548) degree-5 public Macaulay misses all outside-support negative rejections and all positive coefficient recoveries.
- Historical [#4532](https://github.com/isogeny-labs/crypto/pull/4532) cached BSGS ~170ms/1024 on a **different** host, thus not a valid paired benchmark against the W8 control; use for context only.
- Cross-repo autoresearcher norm/determinant negatives SA-04/SA-18, coordinate SA-19, cryptanalysis prime-field/extension-field routes checked via recent repo notes; shared KB MCP unavailable.
- Published prior art Petit–Kosters–Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1 and Amadori–Pintore–Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009. This experiment is not a novelty claim.

## Next non-generic tests (unchanged)
1. Freeze public-only unknown-R coefficient extraction on the B8 46-positive/18-negative target corpus, no group tables, measured algebraic solving degree, rows, memory, time, complete negative/censored accounting.
2. Replace only relation extraction in verified rank31/four-B8-window native batch. Test 1,64,1024 public targets with independent signed witness replay, full setup, all failed shifts, matrix and online work.
3. Compare against *optimized* mixed index calculus and independently optimized BSGS/rho on identical runner/curve/targets (not just this native affine BSGS reference); measure on growing prime-field ladder with scaling m,B, rather than extrapolate from a 31-bit demonstration.
4. Retain source-specific independent replay separate from a signed PCC validation receipt.

No demonstrated cheaper non-generic complete solver is asserted by this control alone.


## Exact-source six-arm result — verified 2026-11-19 01:26 UTC

**GitHub Actions [#44436754627](https://github.com/isogeny-labs/crypto/actions/runs/44436754627) PASSED on candidate source commit \`12f7906e9c0703dd838a1b1b614b31d4b99e46fc\`.** Original EC library at \`1df714645aa683cf975bc76fcacb9842cb4780fd\`. Rust 1.100.0 original library regression 40/40, new signed-BSGS 2/2, original W8-control tests 3/3 passed. Both BSGS 1024-target runs and all four W8 complete runs independently passed Python BigInt group arithmetic and full target-stream identity checks. The repeated BSGS arms had identical non-timing records (tables, all 1024 outputs, counted operations and exactly 16,768,762 giant probes). Signed independent PCC validation remains outstanding.

| Same GitHub runner, 1024 public Q each, all setup included | OLD W8 generic-table IC | TERNARY W8 generic-table IC | Signed shared BSGS (generic reference) |
|---|---:|---:|---:|
| Mean fully charged wall time | 9,122.589 ms | 7,834.933 ms | **3,342.793 ms** |
| Exact group additions | 44,344,469 | 35,150,980 | **16,940,250** |
| Exact finite-field multiplications | 2,837,165,077 | 2,249,470,574 | **1,084,071,298** |
| Exact inversions | 44,329,042 | 35,146,369 | **16,937,174** |
| Public Q logs verified / run | 1,024/1,024 | 1,024/1,024 | 1,024/1,024 |
| Mean setup only | 110.979 ms | 117.998 ms | **7.307 ms** |
| Natural training and linear algebra | included | included | not applicable |
| BSGS table memory | not applicable | not applicable | 32,768 signed x-key entries |

The signed shared-BSGS reference is **2.729x faster** than the original W8 generic-table full batch and **2.344x faster** than ternary W8, across paired two-run means. BSGS A/B took 3,379.561 / 3,306.026 ms; ternary 7,750.837 / 7,919.030 ms; original 9,404.034 / 8,841.143 ms. The W8 ternary advantage on this runner is 9,122.589 / 7,834.933 = **1.164x**, distinct from the **1.262x** in prior #4770's separate same-runner execution. These are real run-to-run differences; never transfer timing ratios between hosts or run series.

**Raw reproducibility:** [GitHub Actions seven-file artifact #14488208293](https://github.com/isogeny-labs/crypto/actions/runs/44436754627/artifacts/14488208293), SHA256 \`360a761d92aa41b18ca3f99e1709ac280c69d867e1c1284281158b9a06065753\`, 30-day retention until **2026-12-19 01:26 UTC**. [Durable exact producer summary](W8_MATCHED_SIGNED_BSGS_SUMMARY_20261119.json) stores six-arm times, group/field operations and all comparison ratios. Source file SHA256 signed-BSGS example \`353b82daaee18e1a0058e88b956e8e3d5312fd2fc5a3e1ada16cc2b1620515b2\`; old/ternary control example \`9e7c67c15dfe29e7cb17809139f85ba1979f9c6827c108c38b0c49eb01854f45\`.

### Interpretation and new experimental priority

This result supplies the **first same-runner, same-affine-arithmetic 1024-Q control** missing from #4770: even an unoptimized (affine EC arithmetic) signed shared-BSGS implementation decisively beats both complete W8 generic-table controls at p=2^31−1. It is **not** an optimized BSGS frontier record or a proof that a genuinely non-generic approach could not beat BSGS. This experiment introduces no new non-generic extractor. Generic table/weight refinements that only yield another small constant-factor win should be deprioritized.

Priority non-generic falsifier: implement a public-residual-only signed B8 RR9/F4 coefficient and unknown-index extractor with no 4+4 group sum table, no planted charts, and zero private scalar access; test the frozen **46 positive / 18 negative** natural-target panel, fully accounting for censored timeouts, rejected charts, F4 linear algebra and signed-point replay. Only if that passes integrate the real extractor with four B8 windows for 31/31 factor-log rank and all 1024 public Q; compare *same runner* to strongest existing mixed index calculus, optimized cached negation-BSGS and rho with full costs. Register a growing ordinary cofactor-one prime-field ladder to guard against fixed-arity false scaling conclusions.

**Scientific disposition:** qualified same-runner native baseline **complete**, source-specific arithmetic and Python replay **passed**, producer-only unsigned [pcc:unverified]. Cheaper complete **non-generic** prime-field ECDLP is **not yet established**.
