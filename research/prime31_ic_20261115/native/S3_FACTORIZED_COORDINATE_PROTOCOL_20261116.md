# Pre-registered S3 x-fibre factorized-coordinate root experiment (prime31), 2026-11-16

## Objective and classification

Test a **fixed-pair-root arithmetic kernel improvement** in the existing complete Rust APS-style one-relation ECDLP solver on the ordinary cofactor-one curve E/Fp, p=2^31-1, n=2147446831, a=847514846, b=2087444724. The operation is classical elliptic-curve coordinate algebra, and explicit finite pair enumeration is **generic decomposition / MITM equivalent** under the repo's two-track classification; a faster clock is not a new non-generic index-calculus complexity result. A true algebraic-track result will require actual target-conditioned original-ideal extraction (B4/B8 then B31 S9).

Review before proposing: repository `AGENTS.md`, `docs/session-logs/2026-11-16.md`, `research/prime31_ic_20261115/native/APS_POST_JAC_PROFILE_REPORT_20261116.md`, prior PRs #4188, #4373, #4383, #4391, #4228/#4349; `src/cryptanalysis/prime_fast.rs` already uses signed point-sum roots (prior art in this repo), and `src/cryptanalysis/aps_single_relation.rs` uses S3. External primary context: Petit–Kosters–Messeng, PKC 2016 (DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI 10.1016/j.ffa.2018.01.009; prior symmetrized Semaev work (FPPR improvement, DOI 10.1186/s40736-015-0012-6). No priority claim. Shared research-KB is not mounted in the GitHub connector environment; repository cross-search + primary sources are the explicitly limited fallback. Do not infer no other prior work.

## Mathematical identity, requirements and exceptions

For **distinct** public abscissae x≠z, let u²=f(x), v²=f(z), f(t)=t³+at+b over Fp. The two finite roots of S3(x,z,T) are exactly

    t_plus  = (u+v)²/(x-z)² - x - z
    t_minus = (u-v)²/(x-z)² - x - z.

This follows from the elliptic chord x-coordinate law and S3's elimination identity, is independent of the signs selected by either x-fibre square root, and requires no group-law evaluation or public ordinate reads **during discovery**. The existing labelled source guarantees global x uniqueness, hence x≠z across partitions; enforce it as a checked condition, never quietly discard a zero denominator. If x=z becomes admitted later, implement the tangent/identity exceptions and assert exact root-set equality before use.

New kernel: compute x-fibre square roots only from f(x), batch-invert nonzero (x-z)² by row, compute both roots, sort/dedup, intersect with exactly the unchanged third partition, then use the existing signed point identity, nonzero beta, and [k]G==Q replay. The baseline `S3` polynomial-coefficient solver remains intact. Test all distinct-pair root sets against the original `s3_coeff_mode` polynomial with exact Fp evaluation, including positive/negative fibre signs, before the benchmark. No generic group sum/difference lookup during candidate discovery. Reuse source, field, labelled walks, batch inversions and final scalar checker verbatim.

## Frozen benchmark, before implementation and execution

- 24 **new** nontrivial public targets Q=[k]G, distinct from prior deterministic panel seeds, with caller-side scalar k never passed to either solver. Public deterministic seed `0x4633_4348_4f52_4430`; no filtering or replacement of hard targets. These are development known-answer holds, not unguessable scientific holdouts.
- Three complete independent cold paths on each Q, equal partition sizes `[384,384,16384]`, maximum 6 labelled rounds per solve: A existing S3-coefficient/x-fibre/Jacobian incumbent; B factorized x-fibre chord roots; B' exact candidate A/A repeat. Rotate order `[A,B,B']`, `[B,B',A]`, `[B',A,B]` by case index. Every arm independently creates its entire public labelled source and pays setup, rejected duplicates/identities, all attempted S3 pairs, failed rounds, root discovery, candidate verification, full scalar replay and return.
- Check all 72 outputs for exact independently reconstructed Q/k, and require **same** signed recovered relation indices/signs/alpha/beta, attempts/round statuses and actual pair counts across arms for each Q. Any difference must be explained as a correctness failure, not timed as a valid comparative win.
- Record cold in-process wall, source cost, S3 cost, pair counts, x-fibre field roots, batch inversions, individual successes/failures, order, caller verification and per-attempt raw outputs. Do not compare against an older host/run or timed F4 partial progress.
- Development acceptance for B: 24/24 all arms complete with identical relation transcripts, B/B' A/A aggregate drift ≤5%, B / arithmetic mean(A, same-host A/A if available) ≤0.90 (otherwise B/A ≤0.90), and at least 18/24 paired target wins. This screen requires a distinct fresh Q confirmation and independent PCC before any ratchet. Losing/neutral outcome preserves incumbent. Do not change thresholds or prune cases after observing data.
- Run locked Rust 1.100 release example tests and the new `--factorized-s3-confirm` through the existing Rust-only CI job; freeze the actual source SHA, job URL, full stdout JSON. No paid EKS job, no leaderboard change, no security-exponent or optimized-rho superiority assertion.

## Non-generic science gate following this engineering test

Assess whether the existing symmetric-B8 signed/infinity gate, selected-root certified quotient from #4228/#4349, and original ideal can construct **natural** target-conditioned root witnesses at B4/B8 without iterating the admitted factor-base multisets or pair-support roots. Exact output from the original signed S9 ideal, all exceptional denominators, useful independent relation rank, failed natural calls, and complete cold cost must survive comparison with mature PKM/APS/F4 controls; otherwise do not call it a non-generic solver.
