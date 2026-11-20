# Prime31 W8 weight-alignment AB/BA gate (preregistered, 2026-11-19)

## Why this comparison is necessary

The original 31-rank/1,024-log Rust control in PR #4728 uses slot weights **[1,3,5,7,11,13,17,19]**. The proposed transformed-slot F3 / 14-variable algebraic extractor in PR #4812 uses **[1,2,4,8,16,32,64,128]**. They define different relation languages. Hence the demonstrated four-window full-rank and whole-solve performance of the first cannot be used as measured downstream performance for the second. Conversely, the planted F3 correctness certificates are not complete-solver measurements for the odd-weight system. We must bridge this before asserting ready-to-integrate.

## Frozen paired study (before implementation)

- Reuse, without copying, the existing original Rust 4+4 grader implementation in `research/prime31_ic_20261115/native/examples/prime31_w8_windowed_rank.rs`, the pinned native `Arithmetic`, `PRIME31`, `check_curve`, the original factor base and four windows, seeds and independent verifier.
- Two independently executable arms on **the same GitHub Actions runner** with identical frozen streams: `odd` = [1,3,5,7,11,13,17,19], `pow2` = [1,2,4,8,16,32,64,128]. Each builds its own transformed signed slot supports and 4+4 generic grading tables, then collects natural public residuals to rank 31 and verifies 1,024 previously unknown public discrete logarithms.
- Keep all online failed shifted residuals, full setup, rank, online timings and field-operation counters in separate source-specific JSON records. No hidden scalar or planted witness may be passed to the grading oracle.
- The original 40 native library regression tests and focused tests of both arms must pass. Independently replay **every accepted training relation and all 1,024 target logarithms** via different Python BigInt elliptic arithmetic. Publish explicit source SHA, CI link and raw artifacts.
- All claims are restricted to correctness and **generic-equivalent 4+4 oracle** speed; the test deliberately does not replace 4+4 grading with a non-generic algebraic extractor. [pcc:unverified]

## Registered outcomes / fail criteria

- **Pass bridge correctness** only if both arms attain rank 31, every `[log]G=Q`, 1,024/1,024 verified logs, all 64 slot abscissas are distinct per window and independent verification succeeds.
- **Bridge failure** if pow2 has rank deficiency, nonordinary supports, >32-offset cap failures, or any verifier disagreement. Record failures rather than silently changing windows, weights or seeds.
- **Cost is descriptive**, including all misses and setup; ratios must come from the matched same runner. This does **not** establish a nongeneric index calculus advantage.
- If pow2 passes, attach its *actual* online residual count, natural hit rate, factor-base logs and per-phase cost to the public-only F3/F4 owner PR #4692 and variant PR #4812. The F4 extractor must be public-only and eliminate generic 4+4 calls before a whole-solver speed claim.
- Independently benchmark public-target Q sampled uniformly from the group, not just planted positive relations: planted sampling is witness-size-biased, `P_planted(Q=q)=N_q/16^8`, whereas real residuals are uniform. The paired full-solver experiment already uses uniform scalar targets, and any F4 positive-only suite must be supplemented by a blinded uniform panel.

## Source audit

- PR #4728 native `prime31_w8_windowed_rank.rs` W = odd weights, 1,024 generic-oracle logs, [2026-11-18 source-bound report](W8_WINDOWED_B8_RANK_RESULT_20261118.md).
- PR #4812 [transformed-slot F3 protocol](TRANSFORMED_SLOT_F3_PROTOCOL_20261119.md) and verified original Rust polynomial checks: T_i = x([2^i]P_j).
- PR #4692 public-only nonenumerative extraction remains open; PR #4816 negative unweighted RR9 quotient results are not the same relation family; PR #4822 sparse-Wiedemann D9 is a separate extraction route; PR #4793 contains the matched-budget methodology.
- Literature scope: Yokoyama et al., *Complexity bounds on Semaev's naive index calculus method for ECDLP*, JMC 2020, DOI 10.1515/jmc-2019-0029. This experiment is a finite-size bridge and claims no algorithmic novelty. Shared research-KB is unavailable in this runtime.

## Explicit status

Preregistered protocol only. No new runtime results exist at the moment this protocol is committed; a later addendum must cite the successful exact-source run and independently verified outputs before claiming a pass.
