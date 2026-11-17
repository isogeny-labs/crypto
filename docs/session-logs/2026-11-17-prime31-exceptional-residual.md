# Prime31 RR9 exceptional residual signed-jet branch
2026-11-17 UTC · draft PR #4546 · producer [pcc:unverified]

## What was done (commands, files, features)
- Read current AGENTS.md, docs/ACTIVE.md and the PR #4545/#4541 descriptions; created the branch from #4545 to reuse the actual Rust RR9 source and avoid unrelated main changes. Tracked the task in ACTIVE before the research change and opened draft PR #4546.
- Research KB MCP and its repository CLI are not accessible in this runtime; reviewed GitHub source and prior PRs #4466, #4539, #4541, #4545, #4373, #4228/#4349, #4513, #4532. Literature fallback: Amadori–Pintore–Sala, FFA 51 (2018), DOI 10.1016/j.ffa.2018.01.009; standard elliptic local-parameter/Riemann–Roch arguments.
- Frozen the proposed algebraic identity, positive and negative tests, and stop/go thresholds in `research/prime31_ic_20261115/rr9/RR9_EXCEPTIONAL_RESIDUAL_PROTOCOL_20261117.md` before Rust modification.
- Updated existing `research/prime31_ic_20261115/native/examples/prime31_rr9_evaluation.rs` to handle residual x equal to base x using signed R-point vanishing or confluent -R tangent jets. Added deterministic distinct-x group-sum fixtures for both branches, exhaustive 256-sign audit per fixture, absent-target negatives on the same x-fibre, and coefficient identity controls. Triggered pinned Rust GitHub Actions workflow on commit `5fdd8ad4d987b6b6d48be8b71df82468950d8d59` (run 44064526434; result pending at this log entry).
- Independent preflight used standard original-curve arithmetic in local scratch only, not a replacement solver: from real G=(625037715,2100627251), p=2147483647, a=847514846, b=2087444724, the R-leaf fixture yielded expected/recovered B8 sign masks [0,127] and -R-leaf fixture [0], with 14 same-x candidate negative residuals available in each. This is *not* Rust CI or PCC evidence.

## Decisions made (and why)
- This is a correctness completion of the algebraic chart, not a new relation-finding speedup. For x=r, the old identity -(x-r)M=L_+L_- degenerates to 0=0; direct evaluation instead yields M(r)=V(r)[(3r²+a)V(r)-2sA(r)]. The two root branches are F(R)=0 versus tangent F'(-R)=0. Using the selected leaf's sign avoids accepting tautological 0=0 as membership.
- Positive fixtures are built using existing original group arithmetic **only in the independent test oracle**. The signed coefficient solver takes only public residual and factor-base points. Signs are still enumerated: label generic/support-enumerative.
- No paid compute, merge, history rewrite, leaderboard change, independent PCC or comparative runtime claim. #4532's matched cached BSGS result is unchanged.

## Open threads (unfinished, pick up next time)
- Inspect the source-specific run 44064526434 after completion. Preserve and fix any Rust errors without weakening the frozen test. Publish raw CI evidence and complete result note, then mark PR ready for review if source-level verification passes; stacked PRs cannot auto-merge before parents.
- Route to actual public-only nonenumerative RR9 coefficient extraction with existing Rust F4/quotient (B8 and then B31 with repeated roots); only measure a complete ECDLP candidate after exact blinded fibres are validated.
- Obtain independent signed PCC standing before upgrading scientific claims.

## Errors (what went wrong, how it was handled)
- No source compile result at time of this entry; protocol predicts the x=r branches, but tests are not claimed to have passed yet.
