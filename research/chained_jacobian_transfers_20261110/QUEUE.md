# Chained ECDLP transfers — persistent queue

Owner: autonomous research session, starting from PR #3877. Updated 2026-11-11.
This is the canonical queue; completed entries stay here. Results require their
own evidence and PCC standing; a checked box is not a verification verdict.

| ID | Priority / state | Question and acceptance criteria | Next action / evidence |
|---|---|---|---|
| CT01 | implemented; unsigned local tests | Native whole-fiber pullback and both source norms, with exceptional support. | `native/`, all 102 elliptic inputs and 10404 rational classes. Original receipt retained. |
| CT02 | baseline published; successor local checks pass; unsigned | Compare arbitrary rational class arithmetic with pre-existing implementations. | Cryptanalysis PR #1469 at aff82387 pins crypto18db046; 26 tests/1 existing benchmark ignored. Successor development checks cover both 9216-class sets; immutable pin replay next. |
| CT03 | 17 Lean lemmas compiled; geometry partial | Formalize multihop kernel criteria, model identities and prime-field descent boundaries. | 9 structural + 8 polynomial lemmas, gates21+8 declarations; torsion invariance and model identities added. Full geometric Picard/Richelot bridge remains open. |
| CT04 | complete fixed-fixture map; target equivalence found; unsigned | Explicit genuine Jacobian-to-Jacobian hop, kernel, dual and nonzero subgroup transport. | All10404 classes, kernel4/image2601, both dual norms[2]. C2 is isomorphic to C: this is an endomorphism after model identification (PCC-a282a9), not a new destination. |
| CT05 | blocked: checkpoint + budget visibility | Freeze controlled native source/destination/chain DLP comparisons including construction, transport, solver, recovery and verification. | Restore S3 checkpoint access and establish live IsoLab budget/capacity; use EKS skill before launch. Local unit tests are development work, not benchmarks. |
| CT06 | partially blocked | Reconcile cross-repository prior art and update the lead atlas without duplicating closed leads. | `crypto-autoresearcher` at `8edec422a60d87b2cbc656fb02f99f42eba959c7` is vault locked; no KEY binding. KB MCP/CLI unavailable. Preserve an integration handoff here; never store plaintext there. |
| CT07 | active maintenance | Keep #3877 reviewable; preserve failures and follow actual branch protections and CI. | Historical ecbench audit failure references unrelated missing archived files; merge status to be rechecked on final head. |
| CT08 | both finite endpoint shortcuts found; unsigned | Check for a cheaper direct expression for the elliptic endpoint function. | Initial102 points: τ φ [2], τ=(82U+60,98V). Successor96 points: standard2-isogeny after shift u-1 and[2], τ=(100U+80,91V). PCC-f23cee/PCC-6e9e6b; include both direct controls. |
| CT09a | model parameterization implemented; fixed F101 | Generalize quadratic splittings and chart bases without mixing class domains. | New model-tagged backend; successor C_s coefficients[85,0,9,0,6,0,1], C_t[21,0,83,0,81,0,20], roots1/2, delta91. #J=9216 each, rational2-torsion4/16 proves base-field nonisomorphism conditional on counts, r=3 retained. PCC-ae88cc; no field-family claim. |
| CT09b | open construction task | Add a second nonbacktracking Jacobian hop, preserving an odd subgroup and tracking equivalences. | Use successor's rational splitting, predeclare the next ordered pairing, retain any degenerate/product/backtracking outcome and independently validate the chain. |
| CT09c | open implementation task | Parameterize the odd-prime native field backend before growing-size measurements. | Validate prime/nonresidue inputs and overflow bounds; independent arithmetic on multiple predetermined fields. No extrapolation from p=101. |

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
