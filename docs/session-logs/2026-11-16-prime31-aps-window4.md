# 2026-11-16 — Prime31 APS four-bit public-jump research

Supplementary scoped log. The canonical `docs/session-logs/2026-11-16.md` is large; GitHub file fetch returned empty content with a nonempty blob SHA, so it was **not overwritten**. Keep this file with the related PR and reconcile the log later when a full checkout is available. [pcc:unverified]

## What was done (commands, files, features)

- Read current `AGENTS.md`, the latest #4451 prime31 S9 zero-fibre handoff, merged #4391, #4349, `APS_JOINT_SHAMIR_BATCH_RESULT_20261116.md`, `APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md`, native APS 3,874-line Rust implementation, `M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`, `research/prime_one_shot_20261029/README.md` and `research/norm_branch_rank_pilot_20261115/README.md`. Research-KB MCP/wrapper and Conductor/EKS were not attached in this GitHub-connector environment; fallback actual GitHub paths and primary PKM 2016 DOI 10.1007/978-3-662-49387-8_1, APS 2018 DOI 10.1016/j.ffa.2018.01.009, and Kudo et al. hybrid/symmetry followup were inspected. No novelty-by-absence claim.
- Inspected actual source-specific GitHub Actions runs [43961833386](https://github.com/isogeny-labs/crypto/actions/runs/43961833386) and [43962417379](https://github.com/isogeny-labs/crypto/actions/runs/43962417379) from separate active #4451. Both are **completed SUCCESS**: each passed 40 native library tests and two focused Rust example tests; net-signatures job uploaded artifact 14275589646, orientation-fibres job uploaded artifact 14275762608. This is producer CI, not an independent PCC-signed theorem.
- Created branch `research/prime31-aps-window4-jumps-20261116` from `main`; committed frozen experiment protocol `APS_WINDOW4_PUBLIC_JUMPS_PROTOCOL_20261116.md` **before** the source change.
- Modified the **existing** native Rust APS example (not a replacement solver): new 4-bit joint-window `aG+bQ` public-jump method, 30 affine multiples of G/Q computed by Jacobian repeated-add and one Montgomery batch inversion, exact 64-jump public PRNG and walk transcript preservation, complete full-cold 24-Q panel including incumbent APS, candidate, candidate A/A, unfolded rho32 and rho64, Rust exact-ladder and partition-transcript tests.
- Added source-pinned Rust 1.100 GitHub Actions workflow `prime31-aps-window4-jumps.yml` with expected full measurement artifact. Code locally uncompiled (no Cargo in connector runtime); CI must be read before any result is reported.

## Decisions made (and why)

- Optimization is engineering inside the existing published APS index-calculus implementation; the current S3 pair root discovery is generic MITM-equivalent. Do not claim a new non-generic attack or a fastest ECDLP solver from a narrower comparison.
- Prioritize a genuinely measured **cold whole-solve**, not jump microseconds alone. New panel compares actual original curve and identical unknown public Qs, paid table generation, attempts and scalar replay. Rho arms use previously implemented unfolded 32-/64-jump variants, not yet the tuned independent WZ implementation or external frontier.
- The separate exact S9 zero-fibre and squarefree identity falsifiers in PR #4451 are now source-specific CI-successful, but do not recover target-dependent nonzero factor polynomials; keep nonenumerative S9 as distinct fundamental research.

## Open threads (unfinished, pick up next time)

- Check exact-source workflow against new commit. If compilation or 24Q execution fails, fix and retest; if correct, inspect raw JSON and independently recompute totals, paired wins, success/failure, A/A drift. Commit measurement report with failures retained and set status [pcc:unverified] absent signed proof.
- Compare genuine strongest tuned WZ negation rho and fastest external mature IC. Do not leaderboard-promote without signed proof/validation gate.
- For a non-generic new mechanism, implement blinded target-conditioned S9/K961-S5 f-from-R and sign extraction using existing native certified quotient/rootfinding; B4/B8 exact positives/negative controls first, then B31 rank-32 complete unknown scalars with full cold costs.

## Errors (what went wrong, how it was handled)

- KB MCP and Conductor/EKS unavailable. Used source-specific GitHub and publication records; no EKS Spot workload launched.
- The canonical daily shared session-log GitHub content API returned empty text for its nonempty SHA. Did not risk overwriting concurrent work; recorded this scoped handoff.
- This session cannot claim local Rust compilation, independent execution certification, or a benchmark speedup before CI.
