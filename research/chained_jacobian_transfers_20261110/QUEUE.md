# Chained ECDLP transfers — persistent queue

Owner: autonomous research session, starting from PR #3877. Updated 2026-11-11.
This is the canonical queue; completed entries stay here. Results require their
own evidence and PCC standing; a checked box is not a verification verdict.

| ID | Priority / state | Question and acceptance criteria | Next action / evidence |
|---|---|---|---|
| CT01 | implemented; unsigned local tests | Native whole-fiber pullback and both source norms, with exceptional support. | `native/`, all 102 elliptic inputs and 10404 rational classes. Original receipt retained. |
| CT02 | eight replay PRs merged; unsigned | Compare arbitrary rational class arithmetic with pre-existing implementations. | PR1469/1471/1473/1474/1475/1476/1477/1478 merged; latestf387be120. Exact-head focused43014776243 succeeds on9cd20ac5f; broader C++ failures remain distinct. Shifted source767c945a0 has complete targeted12,317-class replay; current full46-test CI pending. Independent root flat-Fp6 replay128 cases each17/101 passes. |
| CT03 | 48 Lean lemmas compiled; geometry partial | Formalize multihop kernel criteria, model identities and prime-field descent boundaries. | Gates22+8+7+12+6+5+7+4 declarations; four new shifted-cover translation/recovery lemmas. Full geometric Picard/Richelot/finite-field bridge remains open. |
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
| CT12 | bounded prototype implemented and checked; unsigned | Supply balanced genus2 arithmetic without a rational Weierstrass point. | Cryptanalysis28ba66101: monic smooth split sextics, prime5..101. Both10404-class original/translated models match old engine; F37 primary example/1211 associativity triples pass. E109 cover has11663 classes, all109 source scalar/norm[2]/inverse55 controls, arbitrary-class norm image109/equal107 fibers. Production/catalog integration remains open. |
| CT13 | operational fixed map implemented; unsigned | Assess non-split Frobenius-stable Richelot kernels for full odd elliptic groups. | Cryptanalysis9cd20ac5f/PR1478 evaluates determinant11 map/dual; all285 rational classes, both norms[2], bijection/additivity and all19 source scalar/recovery checks pass. Source/target curve distinction remains separate from unpolarized-Jacobian isomorphism. |
| CT13a | complete bounded assessment; unsigned | Preserve full stable-pairing and codomain/equivalence controls. | NONSPLIT_KERNEL_CHECKPOINT.md links source/report/PDF and five Lean certificate lemmas. Raw targets[10,6,14,2,12,10,9]/[10,11,14,15,12,7,9],delta11/6; reflection identifies them. |
| CT13b | implemented and checked; unsigned | Evaluate actual non-split correspondence and dual with prime-field coefficient descent. | NONSPLIT_MAP_CHECKPOINT.md: first nondegenerate pairing,delta11,w=3Y, F17⁶ intermediates, whole quadratic fibers and balanced infinities. All285 source/target classes descend; both norms[2], rational image285, all19 relations and recovery[4]/inverse5 pass. No solver or cost advantage. |
| CT14 | open; bounded support only | Integrate balanced arithmetic with the production backend and exact catalog fields/models. | Prototype is not coverage of1940 maps or616 prime-field rows. Large primes, extension fields, nonmonic/inert models, normalized targets and catalog replay remain unsupported. Keep the catalog problem open. |
| CT15 | implemented and checked; unsigned | Test a shifted quadratic cover of the existing full-order109 source. | SHIFTED109_CHECKPOINT.md binds767c945a0: shift2, cubic[56,13,6,1], stable kernels1 degenerate/2 smooth reflection-equivalent; delta63/raw scale22; all12,317 classes each direction dual norms[2]/descent/bijection/additivity and all109 scalar/recovery[4]/inverse82 controls pass. Independent field replay128 and four Lean lemmas pass. Full current CI pending; unshifted negative retained, no cost result. |
| CT16 | next unblocked chain obligation | Determine accepted F17/F101 targets' next rational nonbacktracking kernels and Jacobian distinction. | Assess target bracket discriminants/Frobenius action, all stable pairings and previous-dual kernel. A curve PGL2 distinction alone is insufficient for unpolarized-Jacobian nonisomorphism; test Galois2-torsion over extensions and retain any forced backtrack. No outcome accepted yet. |

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
