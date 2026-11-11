# Chained ECDLP transfers — persistent queue

Owner: autonomous research session, starting from PR #3877. Updated 2026-11-11.
This is the canonical queue; completed entries stay here. Results require their
own evidence and PCC standing; a checked box is not a verification verdict.

| ID | Priority / state | Question and acceptance criteria | Next action / evidence |
|---|---|---|---|
| CT01 | active | Replace the legacy Python path with native arithmetic; cover all rational elliptic inputs, nonsplit fibers, repeated support, ramification, both infinities and the chosen Weierstrass base point. | Dependency-free Rust crate under `native/`; tests of both quotient norms and the 2-isogeny. |
| CT02 | ready after CT01 | Compare canonical Mumford addition with a pre-existing implementation; test arbitrary rational divisor classes, not only pullback image. | Use `src/prime_hyperelliptic` as differential oracle in a root example; do not call a producer's own tests independent verification. |
| CT03 | ready | Derive subgroup kernel criteria for multihop chains and prime-field descent boundaries; formalize feasible structural lemmas without assuming the conclusion. | Primary-source review in `SOURCES_20261111.md`; retain exact geometric/formal bridge gaps. |
| CT04 | ready after CT02 | Construct a genuine Jacobian-to-Jacobian hop (e.g. Richelot), with explicit correspondence, kernel and nonzero subgroup image. | Reject singular targets and vanishing composites; deduplicate equivalent subgroup actions. |
| CT05 | blocked: checkpoint + budget visibility | Freeze controlled native source/destination/chain DLP comparisons including construction, transport, solver, recovery and verification. | Restore S3 checkpoint access and establish live IsoLab budget/capacity; use EKS skill before launch. Local unit tests are development work, not benchmarks. |
| CT06 | partially blocked | Reconcile cross-repository prior art and update the lead atlas without duplicating closed leads. | `crypto-autoresearcher` at `8edec422a60d87b2cbc656fb02f99f42eba959c7` is vault locked; no KEY binding. KB MCP/CLI unavailable. Preserve an integration handoff here; never store plaintext there. |
| CT07 | active maintenance | Keep #3877 reviewable; preserve failures and follow actual branch protections and CI. | Historical ecbench audit failure references unrelated missing archived files; merge status to be rechecked on final head. |

## Session constraints

- Public synthetic inputs only. Preserve the 2026-11-10 Python snapshots and
  receipt as history; new research execution is Rust per AGENTS.md.
- No remote job launched. No cloud spend authorized beyond repository ceilings;
  current remaining daily/monthly allowance is unknown.
- No checkpoint bypass: an S3 gate failure blocks experiments, while local
  builds, unit tests, static checks and proof development can continue.
- GitHub feature commits and this queue are the durable recovery path available
  in this environment. Record exact commands, remaining work and PR URLs.

## Immediate resume

Read today's session log and this table. Inspect the actual branch head and
uncommitted diff before running anything. Build/test `native/` first, then the
root differential example. Never rerun historical Python research for extension
work. Recheck access blockers before any new experiment.
