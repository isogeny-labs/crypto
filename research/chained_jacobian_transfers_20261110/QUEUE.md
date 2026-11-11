# Chained ECDLP transfers — persistent queue

Owner: autonomous research session, starting from PR #3877. Updated 2026-11-11 after the execution workspace became unavailable around14:29UTC.
This is the canonical queue; completed entries stay here. Results require their
own evidence and PCC standing; a checked box is not a verification verdict.

| ID | Priority / state | Question and acceptance criteria | Next action / evidence |
|---|---|---|---|
| CT01 | implemented; unsigned local tests | Native whole-fiber pullback and both source norms, with exceptional support. | `native/`, all 102 elliptic inputs and 10404 rational classes. Original receipt retained. |
| CT02 | twelve replay PRs merged; latest elliptic7 replay CI passes; unsigned | Compare arbitrary rational class arithmetic with pre-existing implementations. | PR1469/1471/1473/1474/1475/1476/1477/1478/1479/1480/1481/1482 merged; latest29c4d7206. Consumer ef9a54259/PR1483 exact-head43038507853 passes native25/reference51 unit+two integrations,one old benchmark ignored. PR1483 merge pending applicable analysis/CI review. Existing independent flat-Fp6 and separate branch/Montgomery oracles retained. |
| CT03 | 68 durable helper receipts; four recovered source helpers; geometry partial | Formalize multihop kernel criteria, model identities and prime-field descent boundaries. | Prior gates22+8+7+12+6+5+7+4+7+7+6 remain durable. EllipticSeven.lean adds four coordinate/scalar helpers transcribed after workspace loss; pre-loss local gate4 passed but original receipt/PCC58dbb8 uncommitted. Rerun only these if originals unavailable. Full geometric Picard/Richelot/finite-field bridge remains open. |
| CT04 | complete fixed-fixture map; target equivalence found; unsigned | Explicit genuine Jacobian-to-Jacobian hop, kernel, dual and nonzero subgroup transport. | All10404 classes, kernel4/image2601, both dual norms[2]. C2 is isomorphic to C: this is an endomorphism after model identification (PCC-a282a9), not a new destination. |
| CT05 | blocked: checkpoint + budget visibility | Freeze controlled native source/destination/chain DLP comparisons including construction, transport, solver, recovery and verification. | Restore S3 checkpoint access and establish live IsoLab budget/capacity; use EKS skill before launch. Local unit tests are development work, not benchmarks. |
| CT06 | partially blocked | Reconcile cross-repository prior art and update the lead atlas without duplicating closed leads. | `crypto-autoresearcher` at `8edec422a60d87b2cbc656fb02f99f42eba959c7` is vault locked; no KEY binding. KB MCP/CLI unavailable. Preserve an integration handoff here; never store plaintext there. |
| CT07 | active maintenance; crypto merge blocked; workspace unavailable | Keep #3877 reviewable; preserve failures and follow actual branch protections and CI. | Frozen PCC ledger fork unresolved. Root library6184pass/13fail/133ignored; tracked restoration fixes6,5 absent archives/2 clock causes remain. No full-suite green claim. EMERGENCY_RESUME_20261111.md names exact lost local receipts and GitHub recovery; execution stopped by unavailable workspace. |
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
| CT15 | implemented and checked; unsigned | Test a shifted quadratic cover of the existing full-order109 source. | SHIFTED109_CHECKPOINT.md binds767c945a0: shift2, cubic[56,13,6,1], stable kernels1 degenerate/2 smooth reflection-equivalent; delta63/raw scale22; all12,317 classes each direction dual norms[2]/descent/bijection/additivity and all109 scalar/recovery[4]/inverse82 controls pass. Independent field replay128 and four Lean lemmas pass. Full exact-head43022364261 succeeds native25/reference46;1479 merged3e76a274f. Unshifted negative retained,no cost result. |
| CT16 | complete; forced-backtrack negative preserved; unsigned | Determine accepted F17/F101 targets' next rational nonbacktracking kernels and Jacobian distinction. | TARGET_FROBENIUS_CHECKPOINT.md binds63cf80560/PR1480: both targets6-cycle,only previous-dual stable kernel; source/targetFp3-two-torsion16/4 obstructs unpolarizedFp-Jacobian isomorphism. All16 geometric classes each way,kernel4/image4,all256 torsion additions/Frobenius checks pass. Seven Lean finite/conditional lemmas compile. Exact-head43024621372 passes native25/reference47;1480 merged57ab3ad05. Other routes remain open. |
| CT17 | implemented and checked; unsigned | Decompose accepted odd rational Jacobian groups through both elliptic quotients. | SPLIT_QUOTIENT_CHECKPOINT.md bindsda66ab9d9/PR1481: all285/12317 reciprocal inverses/bijection/paired additions,both norms,cross0,sum[2],odd reconstruction pass; every19/109 transferred scalar has coordinates([4]P,0). Seven Lean cleared/conditional lemmas pass. Exact-head43026964101 passes native25/reference49;1481 mergedee3f177b4. No solver/cost gain. |
| CT18 | complete module assessment; four candidates now have elliptic maps; unsigned | Assess rational odd-prime maximal-isotropic kernels beyond closed Richelot neighbors. | ODD_KERNEL_CHECKPOINT.md binds75e5064d1/PR1482,merged29c4d7206 after43030710539 success:25/50-unit+one integration. Coprime/irreducible-block argument excludes3/5 bothp and7 p17;101ell7 has4 abstract planes. All130/806/2850 RREF planes,40/156/400 isotropic counts/pairing ratios and6 Lean helpers retained. Actual Jacobian odd map remains CT20. |
| CT16b | separate oracle compiled and replayed; unsigned | Independently validate branch/Frobenius data with another arithmetic implementation. | ClosedPR3918/source3a972adb7 original std-only Rust oracle compiled unchanged;formatting-only CI integration also passes Cargo/strict Clippy. Source/hash/custody/rawreceipts bind both source cubic factorizations,both target sextic Rabin irreducibility,all pair/matching fixed counts at degrees1/2/3/6. No producer field/Richelot/Cantor import,no whole-map/signed verdict. PCCed9aff. |
| CT17a | optional additional direct control; not executed | Compare elliptic projectors with supplied CRT scalar maps. | PR3877 comment supplies17 scalars210/76,101 scalars9266/3052,halves10/8 and55/57. Preserve as prospective all-class operator equalities; existing full decomposition already passes. No new solver/cost claim. |
| CT19 | implemented elliptic kernels/maps/duals and separate replay; CI passes; unsigned | Construct fixed101 degree7 elliptic maps and independent algebraic replay. | ef9a54259/PR1483,ELLIPTIC_SEVEN.md:four actual maps,exact curve/both global x[7],all109/113 group/scalar/recovery/j/coordinate controls; unchanged Kohel construction and independent Montgomery verifier. Frozen TSV/manifests/report/diagram/PDF durable.43038507853 passes25/51-unit+two integrations. Root helpers/harness recovered; original root PCC7e9547/58dbb8 unavailable,never manufacture. No actual Jacobian7-hop/advantage. |
| CT20 | four explicit F101 genus2 target models inventoried; independent local oracle passes; signed/native map outstanding | Glue four compatible degree7 elliptic image2-torsion pairs and establish genuine (7,7) Jacobian continuation. | New CT20_F101_GLUING_INVENTORY.md and ct20_gluing_oracle.py: all 4 smooth F101 target sextics and #J12317; 2 nonsquare-leading charts unsupported by simple monic normalization; 2 supported monic sextics with quotient swap and scales7/21. Local development oracle passed; GitHub rewritten exact-head CI pending, PCC unsigned. Next actual balanced divisor map/global kernel[2], both12317-class dual[7], 64 product4-torsion lifts, original109 recovery and full cost controls. |

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

Read EMERGENCY_RESUME_20261111.md first. CT19 is published and focused CI passes;
CT20_PLAN.md is prospective. Root pending PCC IDs were not recovered: preserve
original worktree records if restored, otherwise retain fresh tests/gates and
register fresh records. No local execution can continue while the environment
is unavailable. GitHub recovery is not an S3 admission bypass. [pcc:unverified]

Read today's session log, this table and CHECKPOINT.md. Inspect actual heads
and uncommitted diffs first. Reuse immutable passing receipts when inputs have
not changed; do not repeat the completed two-hour root library run. Rerun
affected checks after new arithmetic changes. Never rerun historical Python
research for extension work. Recheck access before any new experiment.


## CT20b (2026-11-11) — conditional global degree-seven factor lemma

**State: written geometric argument plus 15-class symbolic negative-control test, unsigned; actual Jacobian map unimplemented.** See `CT20B_GLOBAL_FACTOR_CERTIFICATE.md`, `ct20_two_torsion_graph.py`, and `ct20b_torsion_graph.svg` in stacked CT20b review PR (on CT20 #3920). Assuming the *actual* source and target HLP product pull/norm maps have the matching kernel graphs, branch-pair norms yield N0(J0[2]) subset G0; A=P1 alpha N0 factors through geometric [2] to degree49 Phi, with dual[7] identities. Producer script enumerates all15 nonzero branch pairs (3 zero,4 each graph element); incorrect rotated matching fails12/15. This is **not** an independently certified correspondence or cost result. Next independently establish the target quotient correspondences and run full12,317-class balanced divisor maps/dual checks and64 elliptic4-torsion lifts. Preserve root PCC ledger/archive gates. [pcc:unverified]


## CT21 — rational Richelot frontier after degree-seven gluing (2026-11-11)
**[pcc:unverified]** New supported CT20 B+A/B+B source charts each have three Frobenius-stable quadratic pairings, one degenerate (det=0) and two nondegenerate. Determinants B+A `(0,28,73)` and B+B `(0,33,68)`; the two nondegenerate codomains within each chart differ by `t -> -t`. Frozen representative Richelot output sextics: B+A `[7,15,20,20,95,24,82]`; B+B `[1,63,81,46,18,69,84]`. Local stdlib polynomial/Rabin/Fp2 census finds irreducible degree-six outputs and common #J=12317; each has exactly one rational Richelot kernel (the dual), a forced-backtrack boundary for a *further single rational degree-two hop*. Source CT16 control checked too. See `CT21_RICHELOT_FRONTIER.md`, `ct21_richelot_neighbors.py`, and bounded regression workflow in stacked CT21 PR. Producer check only, independent CI/PCC and original geometric 7-hop still outstanding. Continue exact balanced-divisor P1/N1, both 12,317-class dual[7] validations and 64-lift torsion action to select *which* kernel is transported. No paid compute/benchmark.


## CT22 — mod-4 HLP quotient / invariant split Richelot kernel (2026-11-11)

**[pcc:unverified]** Isolated Rust test ct22_mod4_gluing.rs and
CT22_FOUR_TORSION_INVARIANT.md enumerate an *abstract*, 256-element
E[4] symplectic module with a 4-element HLP gluing graph, 64 lifts,
16 quotient J[2] elements, 15 Lagrangian subgroups, and exactly three
Frobenius-stable Lagrangians for both p=101 elliptic traces
t=-7,-11 (each 1 mod 4). The opposite/split Richelot norm kernel
has order four and intersection sizes [4,1,1] with those stable planes.
Under the **conditional actual HLP diagrams** with odd-degree
alpha, N1 Phi=alpha N0 shows that Phi maps the split kernel to
the split kernel and only permutes the two nonsplit kernels. The
illustrative mod4 B=I+F (det=3 mod4) is not any actual Kohel map;
a rotated wrong gluing is rejected. Producing Python cross-check
passed; new Rust workflow/exact-head execution and independent PCC
are pending. This narrows CT21's *unlabeled* successor
curve-isomorphism class, but does **not** replace actual E[4]
maps/kernel labels, geometric divisor correspondences, the 12,317-class
dual[7] tests, 109 scalar recovery, or full-cost benchmarks. No
cloud spending. Keep root #3877 ledger/archive gates untouched.


### CT22b concrete extension-field result (2026-11-11)

**[pcc:unverified]** CT22 now includes ct22_actual_e4.py:
constructs *actual* F101^6 elliptic E[4] classes via cubic-field
half-point x=r +/- sqrt(3r^2+a) and y=zeta*u (zeta^2=2).
Evaluates all four degree-seven Kohel maps and normalized duals
at all 16 source/target E[4] elements, 256 group-addition pairs
per map, plus Frobenius and curve identities. Then forms all
64 actual product E[4] lifts of the HLP graph for CT20 B+A/B+B,
replays two product maps and complete 16-class quotient-addition
tables, and checks the norm-kernel image and wrong-graph negative.
Local Python producer-run self-check passed; exact-head CI
and independent signed PCC are pending. This is a genuine
finite-field elliptic-torsion map, **not** a genus-two balanced
divisor isogeny. Next: match six target branch points to actual
product four-torsion cosets for both nonsplit Richelot labels,
then execute full 12,317-class Phi and dual[7] tests.
