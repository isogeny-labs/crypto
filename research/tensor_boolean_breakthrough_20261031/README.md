# Tensor–Boolean breakthrough experiments

Date: 2026-10-31
Status: experiment design complete; E0 core implemented; no affected measurements taken
Repository baseline: `e3741fd5bbcc1f52a86be609587e7cf4ceeef566`

## Outcome

The next useful move is not an unrestricted SAT sweep. It is a staged attempt to find a **compressible invariant of the exact boundary tensors** produced by the hashed prime-field ECDLP encoding. The first decisive study is a boundary-tensor flattening-rank census against topology-preserving rewired nulls. It can reject the most direct version of the low-rank hypothesis cheaply, and a positive result tells us which contraction engine to build.

The portfolio below turns the proposed workflow into six gated experiments plus one SAT/BDD triangulation study. Experiments 0–1 are the immediate work. The others are not authorized to consume benchmark budget until their predecessor gate passes.

## Portfolio

| ID | Experiment | Question | Decisive observable | Advance gate |
|---|---|---|---|---|
| E0 | Canonical IR equivalence and mutation gate | Can every compared network be regenerated, hashed, and replayed without changing its semantics? | Exact count, every interface-prefix count, least witness, mutation rejection | 100% round-trip equality and all declared mutations detected |
| E1 | Boundary-tensor rank census | Do natural hashed networks have lower balanced flattening rank than graph- and predicate-preserving rewired nulls? | Exact integer rank on admitted cuts; normalized rank growth | Held-out target/null gap and slope gap pass the frozen thresholds |
| E1S | SAT/BDD mechanism triangulation | Do exact Boolean solvers exploit the E1 signature, rather than merely receive a differently named input? | Complete solver work, peak state, and rank/work correlation on the same pairs | Same-sign natural/null effect in two frozen solvers and held-out rank/work association |
| E2 | Local Fourier and partition-rank census | Is any E1 compression explained by a stable local basis or predicate family? | Certified local ranks and composed factor dimensions | A composition certificate predicts at least 20% lower held-out exact work |
| E3 | Certified rewrite/order search | Can semantics-preserving rewrites expose that invariant and lower exact contraction cost? | Exact operations, peak live states, equivalence certificates | At least 2x held-out improvement on the three largest sizes, plus a lower growth slope |
| E4 | Structure-specific exact contraction engine | Does a rank/Fourier-aware engine beat the strongest exact baseline on the same IR? | Exact semiring/bit operations and peak bytes; wall time secondary | At least 2x operations and memory on the two largest held-out sizes, all answers equal |
| E5 | End-to-end relation oracle | Does the mechanism survive relation extraction, failures, verification, and linear algebra? | Complete relation yield and fully priced work | Improvement on four increasing sizes and an unseen target; still no asymptotic claim |

The machine-readable cards are in [`experiment-matrix.json`](experiment-matrix.json). E1 has a narrative preregistration in [`PROTOCOL.md`](PROTOCOL.md) and a schema-valid draft record in [`protocol.draft.json`](protocol.draft.json).

## Why E1 comes before solver search

SAT, tensor contraction, BDDs, variable elimination, and algebraic elimination are different ways of exploiting repeated low-dimensional messages. A solver race can show that one implementation wins on a ladder; it usually does not identify the reusable mathematical reason. E1 asks directly whether the natural network's boundary tensors occupy a smaller space than a close null. E1S then gives the SAT hypothesis a fair test on exactly those paired instances: parsimonious CNF/CNF-XOR exports, two pinned solvers, full setup and verification work, and a preregistered rank/work association.

The null changes only the global wiring of interior ports while preserving the vertex predicates, vertex degrees, edge count, interface, local evaluation code, cut width, and each vertex's cut-degree profile. Conditioning on a preregistered solution-count stratum prevents a trivial “one family is mostly unsatisfiable” explanation. This does not make the null uniquely correct; the report must show both conditioned and unconditioned panels.

## Immediate implementation status

1. **Implemented:** a canonical, versioned network IR with stable edge/port ordering, interface order, pins, predicate semantics, and provenance.
2. **Implemented on tiny controls:** exact IR round trip and all five E0 mutations. See [`E0_IMPLEMENTATION.md`](E0_IMPLEMENTATION.md).
3. **Implemented 2026-11-01:** the exact `tensor-census` core materializes the boundary tensor for an explicit canonical vertex cut and computes fraction-free integer ranks. See [`E1_IMPLEMENTATION.md`](E1_IMPLEMENTATION.md).
4. **2026-11-01 follow-up:** production semantic cuts and an exact equality-
   signal quotient are now implemented. Four frozen natural cells and two null
   families are retained in
   [`../prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md`](../prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md).
   The legacy raw-edge contractor is infeasible even on F5; minimal typed
   rewiring leaves conditioned ranks unchanged, while arbitrary rewiring still
   lacks an exact projected model counter. The full E1 protocol remains draft.
5. **Kernel-checked 2026-11-01:** a nonzero-minor certificate gives a finite rank lower bound and forces the dimension of every exact linear separator factorization; a uniform exponential family lemma would force exponential work in that architecture. See [`RIGOROUS_BOUNDS.md`](RIGOROUS_BOUNDS.md).
6. **Derived 2026-11-18:** the encoding-independent slot relation cannot satisfy that exponential rank hypothesis uniformly over all seeds or with nonnegligible probability under the source threshold: its rank is at most its decomposition count and the expected count is below two. An exceptional selected seed sequence is not excluded. The low-level BTNIR edge-flattening family remains separate and open. Claim records have unverified PCC standing. [pcc:unverified] See [`SEMANTIC_RANK_BOUND.md`](SEMANTIC_RANK_BOUND.md).
7. **Kernel-checked 2026-11-18:** the semantic rank/count premises alone do not imply the no-subexponential-ECDLP goal. Lean now states the parameterized goal, checks the sufficient exponential-work bridge, and supplies a countermodel to the unsupported implication. This does not assert a real fast ECDLP solver. [pcc:unverified] See [`NO_SUBEXPONENTIAL_ECDLP_GOAL.md`](NO_SUBEXPONENTIAL_ECDLP_GOAL.md).
8. **Kernel-checked 2026-11-18/19:** polynomial universal simulation of a vanishing fixed-arity work family gives one subexponential work bound, a fixed numerator divided by arity remains vanishing after any fixed polynomial simulation degree, and the dense row bound gives the exact natural inequality behind numerator `3`. The capped-repetition tail has an exact geometric sum, the corrected first-moment plus factor-12 Markov composition gives an explicit `1/12` joint coverage event, a near-uniform sampler loses at most the squared distortion on any pair event, and characteristic-polynomial root counting plus bounded residue injection leaves at least `7/8` good planted shifts. The exact pinned upstream v2 modules additionally check the concrete row collector, adaptive cap bound, tag surjectivity, prime-power generation and the uniform-generator trial expectation, transcript independence, prime-power block invertibility, planted kernels, the finite round-failure budget and successful-round correctness. A second checked recovery route applies Mathlib's verified Bird determinant to every adjugate cofactor and proves that the resulting executable `ZMod` formula equals the nonsingular matrix inverse under a unit determinant. Every recurrence state is now materialized in an array-backed vector; an instrumented scalar model proves the exact per-entry count, a `4(n+1)^4` determinant budget, and a `4(n+1)^6` all-cofactor budget. Substituting that executable inverse into the upstream `(-R B⁻¹ | I)` construction now satisfies both planted-kernel laws; the exact `Ttop`, `Rint`, `nu`, `ZMod q` adapter is checked locally, and the numerator-6 natural-power bound is checked. Kaarelson's unrestricted dense Theorem 5 claims the required fixed-arity ECDLP family with exponent `3/m + o(1)`, so sparse recovery is not required for the qualitative diagonal; it is needed only for the sharper planted numerator `2`. Applying either dense route to prime-field ECDLP still depends on curve-specific character and energy sums, oracle/coverage binding, an identity-inclusive instantiation of the sampler transfer, operand bit costs and integration into the external pinned source tree, the remaining expected-time assembly, and universal-machine specialization. [pcc:unverified] See [`FIXED_ARITY_DIAGONAL.md`](FIXED_ARITY_DIAGONAL.md).
9. **Kernel-checked 2026-11-19:** the exact-pin standalone project
    [`lean-zsmul-torsion/`](lean-zsmul-torsion/) proves the parity-safe
    group-law bridge `n • P = 0 -> ΨSq n (x(P)) = 0`, including even indices.
    It pins the open Mathlib PR #41197 contributor fork exactly; upstream or
    main-project dependency integration was still open at this checkpoint and
    is closed by item 14 below. This removes the general
    torsion-to-division-polynomial obligation, but not Semaev construction,
    curve-specific estimates, collection/runtime assembly, or the final
    subexponential theorem. [pcc:unverified]
10. **Kernel-checked 2026-11-19:** the production canonical recursive Semaev
    family is now integrated into the exact upstream dilation interface.
    Lean proves coefficient base change, descent of zero-sum vanishing from
    algebraic closure to the base field, simultaneous-scale evaluation,
    tuple-independent structural degree control, the explicit closed bound
    `deg S_(2m)(s t) ≤ (2m) * 2^(2m-2)`, and that every exact upstream bad
    dilation is a root at `u⁻²`. Thus uniform nonzeroness on nontrivial tuples
    is the sole remaining family-specific premise in the Semaev
    `DilationNondeg` bridge. An exhaustive coefficient audit of `S4(st)`
    over every smooth short curve over F5, F7, and F11 found zero nontrivial
    identically-zero specializations; this is bounded evidence, not an
    all-field or all-arity proof. Full subexponential ECDLP is not established.
    [pcc:unverified]
11. **Kernel-checked 2026-11-19:** the first nonzeroness arithmetic layer is
    explicit. A nontrivial tuple has an odd value fibre; every assignment of
    point signs `±1` leaves a nonzero signed coefficient of absolute value at
    most `2m`. A direct endpoint now extracts a Galois-projected coefficient
    that stays nonzero modulo `p` when `|Gamma| * 2m < p`. In the cubic-CM
    branch a second direct endpoint proves the projected Eisenstein scalar
    nonzero under `3(|Gamma| * 2m)^2 < p`. The function-field relation and
    torsion-coordinate contradiction remain open.
    [pcc:unverified]
12. **Kernel-checked 2026-11-19:** canonical Semaev vanishing over an
    algebraically closed field now yields occurrence signs `±1` for any
    chosen lifts of the x-coordinates. The relation is regrouped over the
    tuple's finite image without assuming the ambient function field finite;
    for a nontrivial tuple, one grouped coefficient is nonzero and has
    absolute value at most the arity. Injectivity of elliptic-curve point base
    change now descends the signed relation from an algebraic closure to any
    field where the chosen lifts already exist. Canonical roots in the
    explicit splitting field and the ordinary Kummer projector are also
    composed: a nontrivial vanishing tuple yields a bounded nonzero
    coefficient whose Galois-cardinality multiple annihilates its canonical
    point. Under `|Gamma| * arity < p`, the same scalar is now proved nonzero
    in every characteristic-`p` coefficient field, giving the exact
    nonzero-index/torsion input pair for `ΨSq`. The production-pin
    torsion-to-`ΨSq` implication remains open. In the cubic-CM branch, one
    equivariant relation now generates all Galois twists; equality of the
    partner x-coordinate with the CM image constructs the `±` sign and the
    Eisenstein norm-torsion relation.
    [pcc:unverified]
13. **Kernel-checked 2026-11-20:** the concrete prime-field `A = 0` interval
    instantiation is composed. Canonical roots are proved nonsingular and
    Galois-equivariant, repeated quadratic factors are proved to have a
    Galois splitting field without a false injectivity premise, the interval
    collision constructs the transported cubic-CM endomorphism and its orbit
    relation, and canonical Semaev vanishing supplies the one ordinary signed
    point relation. Under
    `3 * (|Gal| * (n + 3))^2 < p`, the resulting Eisenstein index is proved
    nonzero in `ZMod p` while its integer action annihilates the selected
    canonical point. The automorphism group is injectively encoded by one
    fixed-root bit per selected quadratic root, proving
    `|Gal| <= 2^(n + 3)` even with repeated radicands; consequently the
    arity-only condition `3 * (2^(n + 3) * (n + 3))^2 < p` is sufficient.
    The final uniform endpoint now splits classically between pairwise
    nonsquare products (ordinary Kummer projection) and an existing square
    pair (cubic-CM projection), returning a canonical point with a nonzero
    integer torsion index modulo `p` in either case. This closes the `A = 0`
    Semaev-to-nonzero-torsion branch. At this checkpoint the production
    torsion-to-division-polynomial integration was still open; item 14 closes
    it. Relation yield, expected-runtime assembly, and a subexponential ECDLP
    theorem remain open.
    [pcc:unverified]
14. **Kernel-checked 2026-11-20:** the parity-complete scalar-multiplication
    formula from Mathlib PR #41197 is rebased onto the production Mathlib pin
    as an exact source overlay. Lean now proves both affine and Jacobian
    `n • P = 0 -> ΨSq n (x(P)) = 0`, transports the canonical short-cubic
    curve back to its constant prime-field model, and proves that the embedded
    function-field variable stays transcendental in the quadratic splitting
    field. Consequently every nonzero-scale canonical point has no torsion
    index that is nonzero modulo `p`, and
    `zmodCanonicalSemaevInterval_ne_zero` rules out canonical Semaev vanishing
    under the explicit bound
    `3 * (2^(n + 3) * (n + 3))^2 < p`. The final production build completes
    9,136 jobs and the repository proof gate reports
    `PASS 461 thm 0 trusted`. This closes the concrete `A = 0` canonical
    nonvanishing branch, not relation-yield/energy estimates,
    oracle-to-coverage binding, expected-runtime assembly, universal-machine
    bit-cost specialization, or a subexponential ECDLP algorithm.
    [pcc:unverified]
15. **Kernel-checked 2026-11-20:** the canonical nonvanishing argument now
    handles arbitrary repeated tuple values and reaches the exact production
    dilation interface. The tuple is regrouped over its finite image; a
    surviving bounded coefficient is handled either by a chosen-term Kummer
    projector, which permits collisions among all unchosen radicands, or by
    the cubic-CM pair projector. Lean proves the resulting function-field
    Semaev evaluation nonzero, transfers that fact to the actual
    simultaneous-scale polynomial, proves
    `canonicalDilationPolynomial 0 B m t != 0` for every upstream nontrivial
    interval tuple, and constructs `ECDLP.L49.DilationNondeg` with explicit
    degree `(2m) * 2^(2m-2)`. This certificate is composed with the exact
    upstream `avg_energy_dil` theorem. The concrete dilation fibre count is
    then proved equal to the upstream factor-base cardinality. Combining its
    exact first moment with Markov's inequality gives a set containing at
    least one twelfth of all nonzero dilations on which the factor base has
    size at least `|T|/2` and the energy is below `12 * meanBound`. For
    `12n < |T|`, the generic-coverage inequality and the checked dilation
    transport map give at least
    `(f^(n+1)/2)^2 / (12 * meanBound)` original-curve points with a nonempty
    generic decomposition. For the minimal-root factor base, Lean additionally
    proves `Mdil <= C_n * p` for the explicit fixed-arity constant
    `C_n = (2n+1)!! * 8^(n+1) + 4 * D_n * 16^(n+1)` and
    `D_n = 2(n+1) * 2^(2(n+1)-2)`. Each good dilation consequently covers at
    least the fixed fraction `1 / (48 * 4^(n+1) * C_n)` of the field-sized
    scale. Thus canonical nonvanishing, average energy, and an explicit
    constant-density coverage fraction are closed for the bounded `A = 0`
    family. Binding the decomposition machine's labels to the row collector,
    independent-relation collection and restart probabilities,
    expected-runtime assembly, and bit-complexity specialization remain open;
    no subexponential ECDLP algorithm is yet proved. [pcc:unverified]
16. **Next:** add the production semantic cut generator, matched rewired-null constructor and fresh-process A/A runner; then run only scratch preflight fixtures disjoint from the declared ladder and fill binary, analysis, runtime, memory, and power fields.
17. **Only after that:** freeze and commit `protocol.json`, derive seeds from its hash, and generate E1 data.

The current `Network` stores predicates behind `Arc<dyn Predicate>`, so closure identity cannot be serialized faithfully. E0 therefore uses a hybrid semantic representation: predicates at or below 16 ports carry their complete truth table; larger production predicates carry a strict typed operation that is also their live evaluator. Tag, parameters, truth table or typed program, port order, interface and pins are bound into one SHA-256 identity. Untagged predicates are admitted only through an explicit bounded-truth-table policy for tiny fixtures and controls. Canonicalizing graph shape alone would be insufficient.

This hybrid requirement is a pre-freeze clarification of the original tag-only draft. Tags make the IR interpretable but do not by themselves establish that an imported closure has the same semantics. An initial universal-table implementation was rejected by its own representative control: the tiny hash network contains a 132-port Fourier relation, so a 16-port cap cannot cover production. The typed program supplies exact, replayable semantics for those vertices without materializing a `2^132` table. No affected data preceded either correction.

## Interpretation discipline

- A low finite-size rank is a mechanism lead, not a subexponential ECDLP result.
- A full-rank E1 result rejects this registered boundary-compression route on these cuts and sizes; it does not rule out every Boolean or algebraic representation.
- An E1S win nominates a specific solver/encoding bundle. It does not establish that “SAT” in general is the mechanism, and an E1S loss does not negate a passing E1 rank result.
- Local low rank in E2 does not advance unless a checked composition rule lowers a global message bound.
- E3 search uses a frozen training panel and an untouched holdout panel. Search-time wins do not count.
- E4 compares complete target-dependent work on identical semantic instances. Setup, precision growth, failed candidates, extraction, and verification remain charged.
- E5 is the first stage allowed to use the phrase “relation-oracle gain.” None of E0–E4 establishes an ECDLP pipeline gain.

## Requirement-to-evidence map

| Requirement | Artifact | Status |
|---|---|---|
| Devise multiple experiments | This portfolio and `experiment-matrix.json` | Complete at design scope |
| Make the next action concrete | E0 implementation slice and E1 protocol | Complete at design scope |
| Include controls and falsifiers | `PROTOCOL.md` §§ Controls, decision rule, invalidation | Complete at design scope |
| Prevent post-hoc search claims | Frozen train/holdout policy in E3 and protocol freeze gate | Complete at design scope |
| Implement semantic comparison gate | `canonical.rs`, `census.rs` and `E0_IMPLEMENTATION.md` | Semantic cuts, equality quotient and held-out structural controls implemented; full arbitrary-null/final ladder open |
| Produce measured evidence | No rows or exports exist | Not started; intentionally blocked by draft status |
| Establish a tensor/Boolean breakthrough | Requires E1–E5 evidence and, for asymptotics, a theorem | Open |

## Files

- [`PROTOCOL.md`](PROTOCOL.md) — E1 estimand, construction, controls, analysis, and gates.
- [`protocol.draft.json`](protocol.draft.json) — draft `experiment.protocol/v1` record.
- [`experiment-matrix.json`](experiment-matrix.json) — machine-readable experiment cards and dependencies.
- [`E0_IMPLEMENTATION.md`](E0_IMPLEMENTATION.md) — canonical-IR format, controls, evidence boundary, and open obligations.
- [`E1_IMPLEMENTATION.md`](E1_IMPLEMENTATION.md) — exact boundary contraction/rank core, local controls and remaining generator gates.
- [`RIGOROUS_BOUNDS.md`](RIGOROUS_BOUNDS.md) — finite separator bounds, conditional family/work theorem, certificate contract and proof map.
- [`SEMANTIC_RANK_BOUND.md`](SEMANTIC_RANK_BOUND.md) and `semantic-rank-20261118.pdf` — source-threshold semantic upper bound, matrix-scope split, Lean map and visual.
- [`FIXED_ARITY_DIAGONAL.md`](FIXED_ARITY_DIAGONAL.md) — universal capped-repetition scheduler, fixed-arity specialization, primary-source pins and remaining ECDLP obligations.
- [`lean-zsmul-torsion/`](lean-zsmul-torsion/) — exact-pin Lean proof that scalar torsion forces the full squared division polynomial `ΨSq` to vanish.
- [`lean/DilationSemaevBridge.lean`](lean/DilationSemaevBridge.lean) — exact
  canonical-family bridge to upstream `DilationNondeg`; vanishing and the
  explicit degree `(2m) * 2^(2m-2)` are checked, while uniform nonzeroness
  remains open.
- [`lean/DilationScaleExplicitDegree.lean`](lean/DilationScaleExplicitDegree.lean)
  — multivariate/tower evaluation equivalence and the checked total-degree
  derivation of the closed arity bound.
- [`lean/DilationNonzeroBounds.lean`](lean/DilationNonzeroBounds.lean) —
  odd-fibre signed-coefficient survival and explicit prime-characteristic
  bounds for the ordinary and cubic-CM Galois projector branches.
- [`lean/SemaevRelationSigns.lean`](lean/SemaevRelationSigns.lean) —
  normalization of the algebraic-closure Semaev witness to arbitrary chosen
  x-fibre lifts, injective descent to their field of definition, and
  finite-image regrouping with a surviving coefficient, canonical
  splitting-field lifts, and ordinary Kummer projected torsion.
- [`audit_s4_dilation.py`](audit_s4_dilation.py) and
  [`s4-dilation-audit-20261119.json`](s4-dilation-audit-20261119.json) —
  exhaustive bounded prime-field `S4(st)` coefficient audit.
- [`BOUNDS_REPORT.md`](BOUNDS_REPORT.md) and `bounds-20261101.pdf` — one-page visual bound/scope brief.
- [`SEARCH_20261101.md`](SEARCH_20261101.md) — reconciliation of the requested separator, small-root and special-prime leads.
- [`REPORT.md`](REPORT.md) — visual decision brief.
- `report.pdf` — rendered report after validation.
- [`VALIDATION.md`](VALIDATION.md) — generated-artifact and protocol checks.
