# Status correction for workstreams.json (2026-10-24)

`workstreams.json` is not edited here. Each record below is a status field that reads `PR open` or `not started` while a research or benchmark directory for the same workstream exists on `origin/main`. The owner should reconcile each one. This note does not decide whether any PR has been merged.

The check was made against `origin/main` at `5cb553810e`. GitHub PR state was not queried, because outbound access to github.com is blocked in this environment, so every PR state below is unverified.

| Workstream | JSON status | PR in JSON | Directories on `origin/main` that show the work |
|---|---|---|---|
| W4 early abort at refutation degree | `PR open` | aburan28/crypto#1687 | `research/lit2026_w4_early_abort_20261011/`; `benchmarks/20261011-lit2026-w4-early-abort/` |
| W8 summation-polynomial models | `PR open` | aburan28/crypto#1712 | `research/lit2026_w8_summation_models_20261011/`; `benchmarks/20261011-lit2026-w8-summation-models/`; `benchmarks/20261011-lit2026-w8-quotient-s3/` |
| W10 prime-field S5 small-root oracle | `PR open` | aburan28/crypto#1691 | `research/lit2026_w10_s5_coppersmith_20261011/` |
| W11 generic-method engineering | `PR open` | aburan28/crypto#1692 | `research/lit2026_w11_generic_engineering_20261011/`; `benchmarks/20261011-lit2026-w11-generic-engineering/` |
| W12 flagged claims (zero-minor method) | `PR open` | aburan28/crypto#1713 | `research/lit2026_w12_flagged_claims_20261011/`; `benchmarks/20261012-lit2026-w12-zero-minor/` |

Not mismatched:

- W5 is `merged` in the JSON (PR #1714). Its directories `research/lit2026_w5_lane_batched_f4_20261011/`, `benchmarks/20261011-lit2026-w5-gate-first-lane-roots/` and `benchmarks/20261012-lit2026-w5-gate-first-rref/` exist, which is consistent with the status.
- W3 is `merged` and has `research/lit2026_w3_ggm_audit_20261011/`. Consistent.

Not checked:

- W1, W2, W6, W7 and W9 are `not started`. The only search made was for directories whose names contain `lit2026` (under `research/` and `benchmarks/`), and none was found for them. Their work may use other names, so this note does not establish that they have no directories.

Next action for the owner: confirm the state of PRs #1687, #1691, #1692, #1712 and #1713 on GitHub, then update `workstreams.json` in a separate change for any that are merged.
