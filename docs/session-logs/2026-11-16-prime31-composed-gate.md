# 2026-11-16 — prime31 composed APS full-cold gate (scoped handoff)

The canonical `docs/session-logs/2026-11-16.md` was checked via GitHub but its content was empty in the GitHub connector response while the blob SHA was nonempty; therefore no unsafe replacement of that shared file was attempted. This scoped, same-day handoff does not replace the canonical log.

## 22:43–22:48 UTC — baseline and source review

### What was done (commands, files, features)
- Read `AGENTS.md`, merged #4391, open #4451 and #4454, source Rust `examples/prime31_aps_one_relation.rs`, existing source CI, and primary APS 2018 and Kudo/Yokota/Takahashi/Yasuda literature.
- Inspected successful source-pinned #4454 Actions [43972364656](https://github.com/isogeny-labs/crypto/actions/runs/43972364656), job 133667525871. WIN4: 24/24 each arm, old 72.911111 ms, new 70.035338 ms, rho32 74.631259 ms, rho64 72.290834 ms. Public-y separately: window4 sqrt 66.855314 ms, window4 public-y 64.404234 ms, rho32 66.371901 ms, rho64 73.884110 ms. Both new engineering gates reported **false** because reductions (3.94% and 3.67%, respectively) were below their preregistered 5% thresholds. These are different target sets; neither provides a matched joint binary→composed comparison.
- Discovered successful #4451 exact-source runs [43961833386](https://github.com/isogeny-labs/crypto/actions/runs/43961833386) and [43962417379](https://github.com/isogeny-labs/crypto/actions/runs/43962417379), neither independently PCC-signed.

### Decisions made (and why)
- Extend the existing open #4454 instead of opening a duplicative prime31 PR. Test the *composition* on a fresh 24-Q panel with binary / window4 / window4-publicY / repeat / rho32 / rho64 arms, all fully cold and same public Q.
- Keep unchanged #4451 zero-fibre theorem, prior F4/quotient negative leads and #4391 baseline. The new change is APS arithmetic engineering, not a new non-generic solver or a claim of new asymptotic ECDLP complexity.
- Research KB MCP / local wrapper, EKS and local Cargo unavailable; primary publisher and GitHub source/Actions are the fallback.

### Open threads (unfinished, pick up next time)
- Read latest exact-source composed CI verdict and artifact before promoting any timing claim; audit 24 target records and first/last reported scalars. Preserve failed arms.
- Compare winning candidate against separately tuned WZ rho and mature external APS/PKM; obtain independently signed PCC if proposing a leaderboard row.
- Pursue original-ideal-certified target-conditioned S9/K961-S5 f-from-R extraction, not generic B31 four-sum scanning.

### Errors (what went wrong, how it was handled)
- No local Rust/Cargo or EKS runner available in this environment; emitted source and pinned CI, did not simulate a measured speedup.
- GitHub connector canonical log content too large/empty; preserved existing content and used scoped log rather than overwriting others.

## 22:49 UTC — implementation and CI check-in

### What was done (commands, files, features)
- Preregistered `APS_COMPOSED_WIN4_PUBLICY_PROTOCOL_20261116.md` at commit `d6df1ec9f52e1c4021dfe9c7c12d833c8f88244e` **before implementation**.
- Implemented a new opt-in `--aps-composed-win4-publicy-24q` Rust mode in the existing native example at `2eedb58a2e46604aae1dcef528da77d9ff6eb427`, preserving all source controls. The six-arm result JSON includes full cold ns, source/summation ns, all solutions, signed S3 transcript comparisons, sqrt counts, chrono order, A/A repeat, and rho status.
- Added pinned Rust 1.100 Ubuntu 24.04 workflow `.github/workflows/prime31-aps-composed-24q.yml` at `d09e7174072a65a9d4340c31ac74e20250f65167`; output is retained even if a performance gate fails.

### Decisions made (and why)
- Source-preserving equality is a hard failure; sub-5% improvements are retained as negative rather than manufacturing a win by retuning thresholds. The complete cold ratio includes source setup, oracle, unknown scalar recovery, and verification.
- Source-bound CI is producer evidence [pcc:unverified]. Do not assert success before checking exact workflow.

### Open threads (unfinished, pick up next time)
- Get CI logs and report actual pass/fail. If source compile breaks, repair rather than discarding experiment.

### Errors (what went wrong, how it was handled)
- No source build was available locally; compilation must be established by GitHub Actions exact source execution.
