# 2026-11-16 — Prime31 signed S9 zero-fibre theorem [pcc:unverified]

This focused log supplements `docs/session-logs/2026-11-16.md` **without replacing it**. On the current main/ref that shared canonical log exceeded the GitHub contents API's inline-content limit (the connector returns content empty with a nonempty blob SHA), so a safe append was unavailable through this connector. The existing log must be preserved and reconciled before an archive merge.

## What was done (commands, files, features)

- Read repository AGENTS.md, previous main/feature session logs, merged crypto PR #4391 and latest prime31 B31 4+4 and S9 net multiplicity sources; compared #4228/#4349, crypto-autoresearcher Semaev-alternatives summary and errata, cryptanalysis auxiliary-IC Rust source and PKM/APS papers. Research-KB MCP and shell wrapper unavailable in this runtime; GitHub source fallback with no novelty-by-absence claims.
- Observed prior PR #4391 merged into main at 2026-11-16 21:31:48Z (merge 90ea6cb), its branch was deleted. Preserved its Rust S9 short-base indexing fix and separate orientation-fibre CI results on main rather than reopening an already merged PR.
- Established pure group-theoretic theorem: injective four-signed-leaf net map on D4 implies there is **no nonzero eight-signed-leaf net d with group sum O**. Proof splits d into two D4 vectors and invokes uniqueness for Phi(u)=Phi(-v). For B31, the only signed zero net d=0 corresponds to C(34,4)=46,376 distinct x-only cancellation polynomials. Conditional instance premise is previous successful original-curve source-bound [Actions 43952234526](https://github.com/isogeny-labs/crypto/actions/runs/43952234526) reporting 618,885 distinct group points for 618,885 D4 signatures on the actual Dickson-depth-six base; that report remains producer-only [pcc:unverified].
- Created fresh feature branch `research/prime31-s9-zero-fibre-theorem-20261116`; wrote [preregistered theorem and acceptance protocol](../../research/prime31_ic_20261115/native/S9_ZERO_FIBRE_THEOREM_AND_PROTOCOL_20261116.md) BEFORE code changes; extended the existing native Rust `prime31_s9_b31_signature_multiplicity.rs` to assert unique net zero conditionally on actual four-sum injectivity and emit 46,376 cancellation x-polynomial count. Strengthened B4 unit golden check. Extended pinned GitHub Actions workflow's new-branch trigger and original-output assertions. Producer source is checked in; CI [43961833386](https://github.com/isogeny-labs/crypto/actions/runs/43961833386) was queued at last check, not green.

## Decisions made (and why)

- On this specific fixed 31-point Dickson factor base, eight-leaf target-independent noncancelling relations to O are obstructed by four-sum injectivity; do not waste algebraic solver budget hunting such nontrivial zero relations while pretending cancellations increase factor-base rank. Target-conditioned **nonzero** labelled residual R=aG+bQ remains a legitimate algebraic relation problem.
- Distinguish a single net-zero vector from 46,376 different x-only cancellation polynomials, and from generic 4+4 ordered witness multiplicity. Continue to require explicit sign/ordinate/infinity recovery.
- Do not claim a faster non-generic ECDLP solver from these correctness/absence diagnostics. Previously frozen producer 48-target fully cold APS 109.734280ms versus unfolded-rho64 99.473873ms with overlapping uncertainty. Tuned WZ rho and mature external IC frontier remain unmeasured.

## Open threads (unfinished, pick up next time)

- Inspect actual latest-head [Actions 43961833386](https://github.com/isogeny-labs/crypto/actions/runs/43961833386) and new PR CI; fix source if it fails; examine retained natural/negative evidence, independent replay and signed PCC.
- Implement a real original-ideal-certified target-conditioned S9/K961-S5 `f` and signs extractor using existing #4228/#4349 Rust tools without generic half-sum table or 48,903,492 x-f enumeration; first blinded B4/B8 controls, then full rank32 prime31 unknown scalar recovery and cost.
- Compare complete cold prime31 against tuned native WZ rho and strongest external PKM/APS/F4/CAS. No paid infrastructure, merges or leaderboard claims follow solely from this scope.

## Errors (what went wrong, how it was handled)

- The former PR #4391 branch was automatically deleted after merge, causing the old-branch report update to fail with a GitHub 404; resumed on new main-derived branch.
- Canonical 2026-11-16 session-log content was returned empty by the GitHub inline content API although the SHA was nonempty; no overwrite attempted. This scoped supplementary log retains the handoff without corrupting another concurrent session's canonical log.
- New Rust source compiled/tested only prospectively by CI; local Cargo, Conductor and research KB unavailable. No unverified execution success or generalized speedup is claimed.
