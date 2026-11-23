# Registered prime31 y-ranked factor-base comparison (2026-11-18)

## Claim under test
The ordered first-31 rational-x factor base may not be optimal for the original complete Rust eight-summand IC-style control. Compare it on the **same** ordinary non-singular cofactor-one E/F_p, p=2147483647, n=2147446831, G=(625037715,2100627251), a=847514846,b=2087444724, with a deterministic y-ranked base:
1. Scan x in increasing order and retain the first 256 x for which y²=x³+ax+b has a finite Fp root. Canonicalize y to min(y,p-y).
2. Sort those 256 points by (canonical y, x) and take the first 31; do not select based on target outcomes.
3. Compare against first-31 rational-x points. Both retain B=31, exact signed 8-summand support, **existing generic-equivalent** 4A group-sum enumeration and ±-folded Montgomery-batched complements from PR #4647, rank-31 modular recovery, public shifts a=0..8, and caller-blinded 256-Q corpus with seed 0x4d49583450333131.

## Registered acceptance / rejection
- Both bases contain 31 distinct affine rational points; no identity/order-2 point, no duplicate x; source/harness source prime field and group unchanged.
- Existing source's 256 targets and known-answer scalars are identical in both arms, no target secret or decomposition witness goes into candidate lookup.
- Both arms recover 31/31 independent factor-base logs, 16 separate public-source controls (unresolved controls accounted separately), and 256/256 actual unknown logarithms verified by [k]G=Q. Any BSGS fallback must be explicitly charged and reported, not hidden.
- Capture the **full** setup, source relations, negative queries, rank solving, controls, online logarithm recovery, final verification, operation counts and peak memory where available. No unmeasured reuse of the first-x half-sum table for the y arm.
- Run both orders AB (x→y) and BA (y→x) in pinned Rust 1.100 CI, same GitHub-hosted CPU class. Compare paired total cost and constituent failures/hit counts. Do not assert a speedup if AB/BA disagree or variation is host noise.
- Recheck first-x 244/256 first-shift hits on the same corpus; y hits are **not preregistered**.
- Repository output is source-specific Rust test logs and JSON transcripts. Report any CI malfunction distinctly from mathematical or performance failure.

## Classification / stopping
This is an **index-calculus engineering control**, NOT novel non-generic algebraic relation extraction or a claimed faster-than-rho ECDLP solver. The precise critical-path next gate remains **sign-independent target-conditioned RR9/S9 or balanced S4/S3 with Rust F4/rootfinder and no 4A table**. A y-ranked win alone is not a cryptanalytic break. Stop pursuing this base after two reproducible paired full-cost runs; do not re-optimize generic tables indefinitely.

## Prior work and duplicate avoidance
- Current stacked original Rust complete solver #4623; B31 vs B27/B29 #4629, Dickson/random/first-x null #4626, first-x complete #4635, paired ± quotient #4647. Same-curve matched shared BSGS owner #4596, D9 RR9 algebraic owner #4604; avoid taking their code or runs.
- Balanced S4/S3 bridge exists in `research/prime31_ic_20261115/native/examples/prime31_m8_balanced_f4_input.rs` and has several bounded F4 modes; this run does not duplicate it.
- Prior art: Petit–Kosters–Messeng, *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields*, PKC 2016, DOI:10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, *On the Discrete Logarithm Problem for Prime-Field Elliptic Curves*, FFA 2018 DOI:10.1016/j.ffa.2018.01.009; Yokoyama et al., *Complexity bounds on Semaev's naive index calculus method for ECDLP*, J. Math. Cryptol. 2020 DOI:10.1515/jmc-2019-0029.
- Source audit is GitHub+papers only, research-KB MCP and local CLI are not loaded in this execution environment. No novelty clearance claimed.

**Frozen before candidate edits.** [pcc:unverified]
