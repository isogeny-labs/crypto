# Preregistered Prime31 1,024-Q weighted-M6 versus shared-BSGS cost frontier

Date: 2026-11-18 UTC. Lane: PR #4714. **This is a prospective execution protocol, not evidence of a speedup.** All eventual producer-side results are [pcc:unverified] pending independent signed PCC.

## Scientific question

Does the previously measured complete six-weight B31 Rust 3+3 parity index-calculus-style solver (PR #4714) beat the strongest *shared-precomputation BSGS* on the **same 1,024 public points and machine**, when setup, relation collection, missed shifts, elimination and verification are paid? If no, record the loss and constrain the next nonenumerative algebraic extractor's budget.

Use unchanged original ordinary prime-order cofactor-one curve:
`p=2147483647; E:y^2=x^3+847514846x+2087444724; n=2147446831; G=(625037715,2100627251)`. No weak or specially constructed curve substitution. Existing Rust arithmetic and exact source remain the only production solvers.

## Locked inputs and algorithms

- Q corpus: **source-identical** `research/prime31_ic_20261115/native/examples/prime31_mixed_base_amortized_ic.rs` 1,024-target deterministic output. Require schema `prime31.mixed_4p4_amortized_complete_ic.v1`, 1,024 distinct points, 1,024 caller-only scalar replays on E and source setup rank 61. No known scalar may be passed into any solver.
- Candidate: `research/prime31_ic_20261115/native/examples/prime31_weighted_m6_batch.rs`; six public weights 1,3,5,7,11,13, B=31 first-canonical-x points, unchanged training seed `0x4d36545241494e31`, B31 rank goal 31/31, train cap 512, per-Q shift cap 64; only *new* behavior is to read the existing public Q corpus, passing Q alone to the existing oracle. Retain stand-alone old-target behavior without external input. Classification: generic-equivalent **3+3 group sum enumeration**, not genuine nonenumerative IC.
- Reference: append **unchanged** merged `research/prime31_ic_20261115/mixed_base/prime31_shared_bsgs_1024q_suffix.rsfrag` to its original pinned Jacobian Rust source via the established #4596 workflow. Its locked baby-table sizes are m=46,341;262,144;1,048,576. Compare against the cheapest fully charged complete arm, not only m=sqrt(n). Reference source must not be edited.
- Pin source-only runner checkout of original native Rust at `1df714645aa683cf975bc76fcacb9842cb4780fd`, and Rust 1.100.0; overlay only candidate and existing corpus generator and BSGS suffix from PR branch.
- Execution: GitHub-hosted ubuntu-24.04 x86, `CARGO_BUILD_JOBS=2`, `RAYON_NUM_THREADS=1`. Compile binaries *before* timed executable calls, run A/B and B/A pair (A=weighted M6, B=shared BSGS), preserving four complete raw JSON transcripts, target corpus and summary. The BSGS binary's three reference arms are fully paid separately. No cherry-picking and no post-hoc retuning.
- Time: candidate `total_ns_diagnostic` from start of program, including curve checks, base generation, B2 self-test, table setup, relation training, all failed target queries, rank solve, scalar replays. Reference: each `fully_charged_complete_batch_ns` = BSGS table setup plus 1,024 solves + scalar replay. The reference program's initial corpus read and input validation is *not* in its per-arm timer; record this known accounting asymmetry, separately price it if it threatens crossover. Source corpus preparation is a one-time **shared test fixture**, not solver setup. Record wall clock and field/group counts, RSS where available.
- Acceptance: candidate rank 31/31, all 1,024 unknown logs complete and equal to independent caller-only k; exact public Q lists identical per run and to reference; BSGS 1,024/1,024 in each arm. Report every failed/censored run, refusal, and mismatched target rather than a win. Explicit BSGS-to-M6 ratio with orientation stated, both A/B and B/A.
- Asymptotic verdict: even a M6 win would **not** demonstrate non-generic ECDLP complexity because the oracle explicitly forms generic 3+3 sum tables. A true non-generic claim still requires public-residual-only F4/RR9 roots *without* such a table.

## Previous leads and why this is distinct

- Original full 1024-Q mixed IC and shared BSGS #4532; prior #4596 same-corps shared BSGS/prime_fast comparison. A different M6 candidate was benchmarked only against unbatched M6 in #4714.
- B31 coordinate-rank null #4652, exact quotient+batch B31 #4647, mixed four-plus-four #4623, independent-mask negative theorem #4716, isogeny transport #4671 and cubic inverse #4701; none establishes an M6-versus-strongest-generic win.
- Nonenumerative RR9 algebraic extractor remains *separately owned* by #4692; this experiment does not implement or preempt it. Earlier F4/S4-S3 chain #4188/#4228 and `S4_S3_CHAIN_TARGET_DEPENDENT_GATE_20261116.md` failed their registered caps.
- Prime-field literature already includes Petit–Kosters–Messeng, PKC 2016 (DOI 10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala, FFA 2018 (DOI 10.1016/j.ffa.2018.01.009). No novelty from parity counting, batch inversion or Semaev polynomials is claimed.
- Research-KB MCP/CLI and EKS are inaccessible in this connector runtime; bounded direct repo/primary literature audit, no exhaustive novelty clearance.

## Prespecified decisions

If weighted M6 loses the best BSGS: retain M6 as algebraic-control dataset only, prioritize public-residual RR9/F4/quotient, and calculate the per-call nonenumerative extractor budget compatible with full target recovery on 1,024 Q. If M6 wins only at the stage level, report that scope and require all-target replays. Signed independent PCC gate remains mandatory before leaderboard promotion.

This protocol is committed *before* the new cohort benchmark; do not retroactively modify its assumptions after seeing results.
