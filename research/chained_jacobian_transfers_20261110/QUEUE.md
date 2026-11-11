# Chained ECDLP transfers — persistent queue

Owner: autonomous research session, starting from PR #3877. Updated 2026-11-11.
This is the canonical queue; completed entries stay here. Results require their
own evidence and PCC standing; a checked box is not a verification verdict.

| ID | Priority / state | Question and acceptance criteria | Next action / evidence |
|---|---|---|---|
| CT01 | implemented; unsigned local tests | Native whole-fiber pullback and both source norms, with exceptional support. | `native/`, all 102 elliptic inputs and 10404 rational classes. Original receipt retained. |
| CT02 | differential checks pass locally; unsigned | Compare arbitrary rational class arithmetic with pre-existing implementations. | Cryptanalysis oracle covers 10404 source pairs and the full Richelot image, 289 target subgroup pairs, and point-count orders of both models. Publish source pin and CI. |
| CT03 | 14 Lean lemmas compiled; geometry partial | Formalize multihop kernel criteria, model identities and prime-field descent boundaries. | 8 structural + 6 polynomial lemmas, standard axiom gates; full geometric Picard/Richelot bridge remains open. |
| CT04 | implemented for full rational fixture; unsigned | Explicit genuine Jacobian-to-Jacobian hop, kernel, dual and nonzero subgroup transport. | Degree-four Richelot map, all 10404 classes, dual norm [2], kernel 4, image 2601; PCC-854c56 and PCC-041999. |
| CT05 | blocked: checkpoint + budget visibility | Freeze controlled native source/destination/chain DLP comparisons including construction, transport, solver, recovery and verification. | Restore S3 checkpoint access and establish live IsoLab budget/capacity; use EKS skill before launch. Local unit tests are development work, not benchmarks. |
| CT06 | partially blocked | Reconcile cross-repository prior art and update the lead atlas without duplicating closed leads. | `crypto-autoresearcher` at `8edec422a60d87b2cbc656fb02f99f42eba959c7` is vault locked; no KEY binding. KB MCP/CLI unavailable. Preserve an integration handoff here; never store plaintext there. |
| CT07 | active maintenance | Keep #3877 reviewable; preserve failures and follow actual branch protections and CI. | Historical ecbench audit failure references unrelated missing archived files; merge status to be rechecked on final head. |
| CT08 | finite-fixture deduplication complete; unsigned | Check for a cheaper direct expression for the elliptic endpoint function. | All 102 source points: chain equals τ φ [2], τ(U,V)=(82U+60,98V). PCC-f23cee. Keep this shortcut in future comparisons. |
| CT09 | next unblocked construction task | Generalize the certified chart/correspondence to a parameterized odd-prime native backend before growing-size measurements. | Preserve this fixture; check prior art/KB and exact backend support before choosing fields. No extrapolation from p=101. |

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
root differential example and cross-repository oracle. Never rerun historical Python research for extension
work. Recheck access blockers before any new experiment.
