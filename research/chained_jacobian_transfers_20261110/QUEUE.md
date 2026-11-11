# Chained ECDLP transfers — persistent queue

Owner: autonomous research session, starting from PR #3877. Updated 2026-11-11.
This is the canonical queue; completed entries stay here. Results require their
own evidence and PCC standing; a checked box is not a verification verdict.

| ID | Priority / state | Question and acceptance criteria | Next action / evidence |
|---|---|---|---|
| CT01 | implemented; unsigned local tests | Native whole-fiber pullback and both source norms, with exceptional support. | `native/`, all 102 elliptic inputs and 10404 rational classes. Original receipt retained. |
| CT02 | four replay PRs merged; unsigned | Compare arbitrary rational class arithmetic with pre-existing implementations. | PR1469/1471/1473/1474 merged; latest852a464a7 pins crypto75a737b8. Focused run42999610365 succeeds on exactdf2b78a2; broader C++ analysis failures remain distinct. Latest local extension: native25/reference33 pass,1 old benchmark ignored. |
| CT03 | 29 Lean lemmas compiled; geometry partial | Formalize multihop kernel criteria, model identities and prime-field descent boundaries. | Gates22+8+7+12 declarations; four new coefficient/closed-orbit lemmas. Full geometric Picard/Richelot/finite-field bridge remains open. |
| CT04 | complete fixed-fixture map; target equivalence found; unsigned | Explicit genuine Jacobian-to-Jacobian hop, kernel, dual and nonzero subgroup transport. | All10404 classes, kernel4/image2601, both dual norms[2]. C2 is isomorphic to C: this is an endomorphism after model identification (PCC-a282a9), not a new destination. |
| CT05 | blocked: checkpoint + budget visibility | Freeze controlled native source/destination/chain DLP comparisons including construction, transport, solver, recovery and verification. | Restore S3 checkpoint access and establish live IsoLab budget/capacity; use EKS skill before launch. Local unit tests are development work, not benchmarks. |
| CT06 | partially blocked | Reconcile cross-repository prior art and update the lead atlas without duplicating closed leads. | `crypto-autoresearcher` at `8edec422a60d87b2cbc656fb02f99f42eba959c7` is vault locked; no KEY binding. KB MCP/CLI unavailable. Preserve an integration handoff here; never store plaintext there. |
| CT07 | active maintenance; crypto merge blocked | Keep #3877 reviewable; preserve failures and follow actual branch protections and CI. | Frozen PCC ledger fork unresolved. Full root library6184 pass/13 fail/133 ignored; restored tracked files fix6 failing tests,5 need absent archive and2 unavailable kernel clock metadata. No full-suite green claim. |
| CT08 | both finite endpoint shortcuts found; unsigned | Check for a cheaper direct expression for the elliptic endpoint function. | Initial102 points: τ φ [2], τ=(82U+60,98V). Successor96 points: standard2-isogeny after shift u-1 and[2], τ=(100U+80,91V). PCC-f23cee/PCC-6e9e6b; include both direct controls. |
| CT09a | model parameterization implemented; fixed F101 | Generalize quadratic splittings and chart bases without mixing class domains. | New model-tagged backend; successor C_s coefficients[85,0,9,0,6,0,1], C_t[21,0,83,0,81,0,20], roots1/2, delta91. #J=9216 each, rational2-torsion4/16 proves base-field nonisomorphism conditional on counts, r=3 retained. PCC-ae88cc; no field-family claim. |
| CT09b | implemented; immutable replay merged; unsigned | Add a second nonbacktracking Jacobian hop, preserving an odd subgroup and tracking equivalences. | Predeclared pairing(2,18),(-2,28),(-18,-28), scale20,delta67,target[38,9,29,62,86,17,8],base11. #J9216,target2-torsion8,reverse-kernel intersection1,rational composite kernel8 with4 self-inverse,r3 preserved. PR1473 merged2262c1d11. |
| CT09c | implemented; immutable replay merged; unsigned | Parameterize the odd-prime native field backend before growing-size measurements. | Feature small-prime, checked odd p≤4096, enumeration≤101; fixed13/29/101 plus separately declared31. Orders180/960/9216/1008; torsion4/4,4/16,4/16,16/4; preserved r5/5/3/7. PR1474 merged852a464a7, focused exact-head replay success. |
| CT10 | written global argument + compiled algebra; unsigned | Replace finite endpoint equality by an algebraic shortcut derivation. | GLOBAL_SUCCESSOR_COLLAPSE.md: n R f*=tau phi[2] for char≠2,3,5; actual base translation essential. Seven cleared ring lemmas and all-curve/whole-fiber four-field controls. No full Lean Picard construction or claim about the final Jacobian-valued map. |
| CT11 | bounded norm and chained pullbacks implemented; unsigned | Preserve a full prime-order source and implement arbitrary closed-point norm, without duplicating the existing Enge collector. | CUBIC_NORM_20261111.md: E109/F101 to genera4/13; arbitrary-class genus4 norm, closed degrees1..4;130 witnesses independently replayed. Secondary E19/F17 all361 pairs, ramification and degree12. Genus13 inverse/norm restricted to exact image. No cost result. |
| CT12 | next unblocked arithmetic task | Supply balanced genus2 arithmetic without a rational Weierstrass point. | Derive degree2 effective representatives relative to K=∞++∞−; implement monic even sextics; compare all10404 old-fixture classes with the existing odd-chart engine before applying to E109. Catalog-wide support remains open. |
| CT13 | prospective; depends on CT12 | Assess non-split Frobenius-stable Richelot kernels for full odd elliptic groups. | Lack of individually Fp-rational quadratics is not a universal kernel obstruction. Predeclared E19/F17 has potential Frobenius(3,3); prove factor/orbit/descent claims and implement the actual map before reporting a transfer. |

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

Read today's session log, this table and CHECKPOINT.md. Inspect actual heads
and uncommitted diffs first. Reuse immutable passing receipts when inputs have
not changed; do not repeat the completed two-hour root library run. Rerun
affected checks after new arithmetic changes. Never rerun historical Python
research for extension work. Recheck access before any new experiment.
