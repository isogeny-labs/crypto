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
