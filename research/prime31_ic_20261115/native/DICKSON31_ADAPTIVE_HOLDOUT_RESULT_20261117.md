# Prime31 Dickson31 adaptive eight-sum holdout: 96 public targets

**November 17, 2026. Source-specific producer result [pcc:unverified].**

This completes the preregistered **generic 4A+4A membership control** on the real ordinary, nonsingular, cofactor-one curve over p=2^31−1, NOT a novel non-generic/index-calculus extractor, NOT an unknown-scalar ECDLP recovery, and NOT a full-cost speed win. The actual curve is
\`y^2=x^3+847514846*x+2087444724\` over F_2147483647,
prime order \`n=2147446831\`, \`G=(625037715,2100627251)\`.

## What was executed

- Frozen before measurement: [protocol](DICKSON31_ADAPTIVE_HOLDOUT_PROTOCOL_20261117.md), 96 fresh distinct caller-known k sampled from fixed SplitMix64 state \`0x533942484f4c4431\` via \`k=1+splitmix64(state) % (n-1)\`; sample is deterministic and slightly biased by modulo reduction. No k is an input to the 4A membership oracle.
- Actual depth-six Dickson base \`Base::build(PRIME31,6)\`: 31 canonical base points, with 62 signed tokens and **618,885 distinct four-leaf group points**. Reuses the original Rust code in [prime31_s9_b31_half_sum_oracle.rs](examples/prime31_s9_b31_half_sum_oracle.rs) with an isolated appended test module; preserves original 24-target and planted controls.
- For each public Q, try R=Q+[a]G in the fixed order \`a=0..8\`, stop at the first exact eight-leaf signed support, and record every miss/negative exhaustively. Two independent previously frozen miss controls use caller k=1610011808 and 1264623843.
- Exact source at **4dcbbce6e3480c55d97e115d802833062955c1d1**, immutable native arithmetic base \`1df714645aa683cf975bc76fcacb9842cb4780fd\`, pinned Rust 1.100.0, Ubuntu 24.04 GitHub-hosted runner.
- **PASS** [source-specific GitHub Actions #44169834979](https://github.com/isogeny-labs/crypto/actions/runs/44169834979): original native Rust library tests **40/40**, new isolated Rust holdout **1/1**; parsed JSON boundary assertions. [Raw full transcript artifact #14366792594](https://github.com/isogeny-labs/crypto/actions/runs/44169834979/artifacts/14366792594). GitHub metadata reports ZIP digest \`sha256:1d957459b4e1b55230b7fc8ad96ee9cf133a6564556305573140c40dd43027a5\`; archive **not independently rehashed**. Earlier source version before adding setup accounting also passed [#44169385150](https://github.com/isogeny-labs/crypto/actions/runs/44169385150). Its entire 98-case cohort, public point inputs, signed witnesses, probe counts and native operation counts are identical to the final run; only diagnostic time/setup fields differ.

## Exact observed outcomes

| Dickson31 public target panel | No shift a=0 | First retry a=1 | Second retry a=2 | Still unresolved by a=8 |
|---|---:|---:|---:|---:|
| 96 fresh unknown-to-worker Q | **88** | **7** | **1** | **0** |
| Two frozen a=0 nonmembers | **0** | **2** | **0** | **0** |

For the 96 fresh Q, there were **105 exact membership attempts**: **96 successful**, **9 exhaustive negative** trials. For all 98 cases together, **109 attempts**: **98 successful** and **11 exhaustive negatives**. Every successful signed eight-leaf witness is checked against original-curve point arithmetic inside the native Rust oracle; every failed membership query must exhaust 618,885 four-leaf probes.

A separately implemented affine arithmetic readback of the first successful artifact reconstructed the actual Dickson31 base with seed 3, rechecked **98/98 public Q=[k]G**, all 98 original-group eight-leaf signed sums, all 109 public residual shifts, and the 11 negative exhaustive-probe-count assertions; zero mismatches. The final run returned bit-identical target/witness/probe/op-count records. This is independent **arithmetic readback performed by the producer session**, not a signed PCC or an independent verification organization.

**The observed 96/96 bounded-transform support result is not a guarantee of full-curve coverage.** The nine public shifts are consecutive and correlated, not nine independent uniform random choices. In addition, the toy tester knows k only to construct and replay public synthetic targets; no unknown logarithm was recovered by the membership code.

## Charged diagnostic cost (same source-specific CI)

| Native Rust metric | Value |
|---|---:|
| 4A index construction wall | **296.826437 ms** |
| 4A index construction group additions | **678,993** |
| HashMap capacity, entries (not bytes or RSS) | **917,504** |
| Full 98-case study incl base construction | **5,182.860432 ms** |
| Sum of 109 charged residual/oracle attempt walls | **4,880.092516 ms** |
| Total four-sum lookups/probes | **18,049,427** |
| Total group additions inside the 109 attempt scopes | **18,050,343** |
| Total finite-field multiplications inside attempts | **1,155,200,856** |
| Total finite-field inversions inside attempts | **18,050,013** |

These are development-run wall diagnostics and source counters, not calibrated hardware-equivalent cost, independent repeated-host A/B, or process peak RSS. The earlier merged [#4532](https://github.com/isogeny-labs/crypto/pull/4532) producer fully charged **same 1,024-Q** comparison recorded mixed IC 2,221.474 ms versus cached BSGS 170.113 ms. The present **98-case generic oracle** cannot be directly divided by the 1,024-Q shared BSGS result to claim a matched speed ratio. It does nevertheless establish that this unoptimized affine group-sum membership control is not a plausible speed breakthrough.

## Why the 22/24 legacy coverage is unsurprising, and what not to assume

The naive four-sum random-subset heuristic treats \`|4A|^2 = 383,018,643,225\` as independent eight-sum samples and falsely predicts essentially universal coverage. But these ordered half-pairs collapse under cancellations/permutations to only **5,654,193,417 distinct integer eight-leaf net signatures** (the exact preexisting formal count). A **different, unproven** random-*net-signature* occupancy model would predict

\`Pr(8A hit) ≈ 1-exp(-5654193417/2147446831) ≈ 0.92814\`.

The fresh unshifted sample hit rate **88/96 = 91.67%** is consistent with that crude heuristic. This is **not a theorem, statistical sampling guarantee, or proof that the special base behaves randomly**; it is a diagnostic explanation of why 22/24 natural hits and exact nonmembers can coexist despite 4A having 618,885 points.

## Actual algebraic lead and prioritized next experiment

**P0: real, sign-free RR9/S9 extraction over this Dickson31 base, not the generic 4A oracle.** Given only public R and the 31 public base abscissae, use the existing Rust quotient/F4 and rootfinding from [#4228](https://github.com/isogeny-labs/crypto/pull/4228) / [#4349](https://github.com/isogeny-labs/crypto/pull/4349) to find the eight-leaf factor polynomial, signs and original-curve witness without materializing a 4A point-sum index. Consider the H31/K961/S5 circuit from [the rational quotient bridge](M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md), or the public RR9 condition \`H_B^8 mod M_(A,V,R) = 0\`; *a cheap verifier is not the missing coefficient extractor*. Require exact B8 positive/negative fibres and then all 96 Q plus two frozen negatives on actual Dickson31. Explicitly count repeated roots, cancellations, identity intermediates, and failed affine charts. Any solution that enumerates 618,885 4A group points or the \`F_p^128\` signed quotient remains generic-equivalent and is not the sought innovation.

**P1: actual complete unknown logarithms.** With non-generic relations, use the existing Rust modular LA and base-log recovery, then recover k from each verified \`R=Q+[a]G=∑ epsilon_i P_i\`, yielding \`k=∑ epsilon_i log_G(P_i)−a (mod n)\`. This identity is exact but only executable if the **base logs have been recovered** and the relation extractor is real. Charge setup, all failed transforms, full relation rank and modular LA, memory, reconstruction and final \`[k]G=Q\` against matched 1,024-Q shared BSGS, tuned rho and external mature prime-field index calculus. P1 is not performed in this test.

**P2: avoid concurrent work**. Batched 1024-Q comparison [#4596](https://github.com/isogeny-labs/crypto/pull/4596), sparse D9 black-box rank [#4604](https://github.com/isogeny-labs/crypto/pull/4604), low-memory 3-sum [#4614](https://github.com/isogeny-labs/crypto/pull/4614) are separately owned. Prior seven-quotient [#4581](https://github.com/isogeny-labs/crypto/pull/4581) and fixed B8 rarity [#4612](https://github.com/isogeny-labs/crypto/pull/4612) must not be repeated as non-generic extraction. The previous \`XY_COORDINATE_PROJECTION_PILOT_20261115.md\` records that naive y-only B31 membership adds 34 unregistered points and requires separate labels.

## Source review and evidence limits

Repo research, PRs #4513/#4532/#4581/#4604/#4612, autoresearcher #3301/#3305, cryptanalysis #1536; relevant prime-field prior art Petit–Kosters–Messeng PKC 2016 (DOI 10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala FFA 2018 (DOI 10.1016/j.ffa.2018.01.009). The research-KB MCP and authenticated EKS/Conductor CLI were unavailable in this connector-only run; GitHub source and externally verified publication metadata were substituted. No claim that all earlier work or literature was exhaustively searched. No paid infrastructure, PR merge or performance leaderboard mutation.

**Result boundary:** exact new generic membership baseline and falsifiable test data; **zero** public-only sign-free extracted relations and **zero** unknown-k ECDLP solutions in this new gate. All measurements \`[pcc:unverified]\`.
