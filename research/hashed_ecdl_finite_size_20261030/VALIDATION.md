# Planning-artifact checks

2026-10-30. Documentation-only change; no algorithm or scientific execution.

- `bash research/hashed_ecdl_finite_size_20261030/build-report.sh` exited zero
  using Graphviz, pdfLaTeX and Ghostscript. The current report has two pages.
- Rendered both pages with `pdftoppm -scale-to 1200 -png` and visually
  inspected each. The revised diagram's labels are legible; no overflow or
  undefined-reference warning appears in the build output.
- The card parses with `jq`; its status is draft, funding is zero, and both
  frozen protocol and code commit are null. Its TODO commands name no
  existing executable. It is not runnable or submitted.
- The native `backlog` validator is unavailable on this workspace's PATH;
  canonical cross-field validation remains for CI. Schema/required fields
  were reviewed against `backlog/schema/card.v1.json` at the base commit.
- No library code changed and no new code tests were added. No library or
  benchmark suite was run for this source review. The older 44/55-test
  receipts are separate implementation evidence, not checks of this plan.
- Conductor is not installed. `docs/CONDUCTOR.md` documents its warn/allow
  fallback. No reservation is claimed. A separate worktree/branch based on
  `d405c591e9bb8afc58c16f81b677e73cbb0f7951` keeps the conflicted #2924
  checkout untouched. Scope is this study, its draft backlog card and one
  append-only session-log entry.
- Run `sha256sum -c artifacts.sha256` from this study directory to check
  source/render consistency. Git whitespace checks are required before
  publication. PR checks/review are independent gates and remain pending.

No runtime, relation-yield, solving-degree, sub-rho, challenge-scalar or
novelty claim was validated. No frozen evidence or existing panel changed.
