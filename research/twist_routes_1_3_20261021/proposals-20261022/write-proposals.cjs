#!/usr/bin/env node
// Document/card metadata generation only. This file performs no experiment.
'use strict';
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const dir = __dirname;
const root = path.resolve(dir, '../../..');
const study = 'research/twist_routes_1_3_20261021';
const base = '9dff98b15fd91887d41a73419a03abe17f9e24ca';
const fixturePath = `${study}/execution-v1/fixtures.json`;
const evidence = `${study}/reports/EXECUTION_RESULTS.md`;
const parent = {
  1: 'XP-20261021-twist-trace-factor-base',
  2: 'XP-20261021-twist-cover-transfer',
  3: 'XP-20261021-twist-symmetry-solver'
};
const experiments = [
  {
    id: 'E18', slug: 'twist-actual-three-torsion', route: 2, priority: 'P1',
    title: 'Actual 3-torsion and pairing certificates', kind: 'construction bridge',
    question: 'Do the four companion-matrix candidates lift to explicit Galois-equivariant anti-isometries on actual 3-torsion, including for public P-521 parameters?',
    motivation: 'The public module panel selected P-521 and three toy curves, but constructed no torsion basis or Weil pairing. This is the first missing certificate, before any quotient or performance question.',
    hypothesis: 'At least one matrix candidate can be represented by actual torsion points with the claimed Frobenius action and inverse Weil pairing. A positive answer establishes only this bridge.',
    inputs: 'Use the three positive fixtures at p=211, 751 and 10111 from PUBLIC_3_MODULE_ANALYSIS.json, plus public P-521. Keep all seven negative ordinary toy rows and the four negative NIST rows as filter controls. Use the exact registered models, twist parameter and public orders; no NIST target logarithm is supplied.',
    method: 'Construct the 3-division polynomial and a declared extension field. Search extension degrees 1 through 8 in increasing order; certify irreducibility and all coordinate conversions. Give independent order-3 points P,Q, their primitive Weil pairing, and both Frobenius matrices in explicitly related symplectic bases. Replay every candidate map and check its graph is Galois stable and reverses the pairing. Native BigInt field/torsion support is a prerequisite for P-521; the toy u128 adapter is insufficient.',
    controls: 'A matrix with the wrong determinant, a deliberately conjugated wrong Frobenius matrix, dependent torsion points and each negative module row must fail the appropriate check. An empty matrix set is not passed to a point constructor as a success.',
    outputs: 'Exact field polynomials and bases, all torsion coordinates, basis-change matrices, pairing values, Frobenius images and independent verification transcripts. Record construction operations and peak memory, separately from the certificate conclusion.',
    decision: 'A verified anti-isometry enables E19. A mismatch is a specific module-to-geometry defect to resolve. Extension/resource exhaustion leaves that candidate unresolved; it does not refute existence.',
    budget: 'Proposed pilot: one CPU, 4 GiB per process, 1,800 seconds per positive curve, extension degree at most 8. Preserve each attempted field and each failure. This is a cap, not an estimated runtime.',
    dependencies: [], blockers: ['Actual torsion/pairing implementation, native BigInt adapter and an independent certificate checker are unimplemented.'],
    references: [`${study}/reports/execution-20261022T0032Z/PUBLIC_3_MODULE_ANALYSIS.json`, `${study}/PUBLIC_3_MODULE_PROTOCOL_V1.md`]
  },
  {
    id: 'E19', slug: 'twist-three-gluing-geometry', route: 2, priority: 'P1',
    title: 'Indecomposable gluing, a genus-2 curve and explicit maps', kind: 'geometric hypothesis',
    question: 'Does an E18 anti-isometry yield an indecomposably principally polarized quotient with an explicit same-field genus-2 model and subgroup transfer?',
    motivation: 'A compatible Frobenius polynomial is not a curve. A curve without maps is not an ECDLP transfer. The original positive three-gluing requirement remains unfulfilled.',
    hypothesis: 'One of the certified toy anti-isometries yields a usable genus-2 quotient. Public P-521 is a separate construction-only extension after the toy implementation is verified.',
    inputs: 'Process every E18-certified anti-isometry on the three positive toy models, up to the four candidates reported per model. Retain every rejected candidate. Attempt at most the four public P-521 candidates only after the same formulas and checker pass the toy stage.',
    method: 'Construct the graph quotient of E times its twist, certify the induced principal polarization and geometric indecomposability, descend a smooth genus-2 curve to the declared base field, and provide quotient/pullback/norm maps with their actual degrees. Prove each composition identity and the exact kernel intersection with the source prime subgroup; normalize recovery by the actual coprime degree. Do not infer degree or injectivity from the numeral 3 alone.',
    controls: 'Include a documented reducible/product-polarization example, a non-anti-isometric graph and identity/ramification cases. Compare independent point counts and divisor-map replay. A product principally polarized surface, matching j-invariant or characteristic polynomial cannot pass the smooth-curve certificate.',
    outputs: 'Polarization and kernel certificates, curve equation, smoothness/genus/infinity checks, explicit rational maps, exceptional charts, exact subgroup order, scalar recovery and all failures. Initially measure construction and map costs only; no Jacobian-IC improvement follows from existence.',
    decision: 'A fully checked toy model becomes an E24 arithmetic input and a future destination-solver input. A failed anti-isometry closes only that candidate. P-521 construction success remains a public-parameter result, with no NIST DLP execution.',
    budget: 'Proposed pilot: one CPU, 4 GiB and 3,600 seconds per anti-isometry. Stop that cell on a singular model, failed descent, failed map identity or resource cap; retain the failed certificate.',
    dependencies: ['E18'], blockers: ['Explicit gluing/polarization/model/map constructors and independent replay are missing.'],
    references: [`${study}/EXPERIMENTS.md`, 'research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md']
  },
  {
    id: 'E20', slug: 'twist-higher-gluing-filters', route: 2, priority: 'P2',
    title: 'Other small gluing primes and actual torsion modules', kind: 'applicability search',
    question: 'Do gluing primes 5, 7, 11, 13 or 17 expose additional public-parameter candidates that the 3-torsion filter misses?',
    motivation: 'The 3-torsion panel tests one gluing degree. A negative row there does not exclude another prime, while a positive matrix row still needs the geometric checks in E18/E19.',
    hypothesis: 'Some frozen curve/twist pairs have suitable small-prime torsion modules at another degree. This proposes an applicability census, not a new gluing construction.',
    inputs: 'All ten ordinary toy fixtures and all five public NIST P-curves, crossed with primes 5,7,11,13,17. Use the original public orders to reconstruct trace and characteristic. Reject a cell whose gluing prime is the characteristic or divides the selected source subgroup order.',
    method: 'Derive and independently check the actual necessary Frobenius conditions, then solve the anti-intertwiner linear system and determinant condition. Separate companion-matrix results from actual torsion action, especially scalar/repeated-eigenvalue cases. For each prime, select the first two positive toy fixture IDs in lexical order, if any, for explicit torsion/pairing replay; retain a no-positive status rather than generating a favorable replacement fixture.',
    controls: 'Wrong-trace and wrong-determinant matrices, the known 3-torsion panel as a regression control, characteristic-prime rejection and a repeated-eigenvalue synthetic module control. Matrix counts are not geometric certificates.',
    outputs: 'A complete 75-cell ordinary/public filter table, all exclusions, module types, candidate matrices and bounded actual-torsion replay. Actual quotients at the larger degrees remain a separately frozen extension of E19.',
    decision: 'A verified positive adds a specifically named construction candidate. An all-negative table excludes only this finite prime set. Rising map/construction costs are reported before expanding gluing degree.',
    budget: 'Proposed pilot: one CPU, 4 GiB, 15 seconds per matrix cell; at most two explicit toy torsion constructions per prime with a 1,800-second cap each. No public large-prime torsion construction is included in this pilot.',
    dependencies: [], blockers: ['A generalized native module enumerator and actual-torsion checker are not yet implemented.'],
    references: [`${study}/public-three-gluing.gp`, `${study}/reports/execution-20261022T0032Z/PUBLIC_3_MODULE_ANALYSIS.json`]
  },
  {
    id: 'E21', slug: 'twist-coupled-component-bases', route: 1, priority: 'P1',
    title: 'Restrictions coupling the two Frobenius components', kind: 'collection hypothesis',
    question: 'Can a predicate involving both the rational and anti-rational components find useful relations more cheaply than direct collection on exactly the same projected base?',
    motivation: 'Unrestricted extra twist fibers duplicate rational traces. A predicate coupling the components is a concrete way to test a different search representation while preserving the original separation argument.',
    hypothesis: 'A coupled predicate makes decomposition easier even when the final rational relation set is unchanged. Merely increasing witness multiplicity does not support this hypothesis.',
    inputs: 'Use the ten ordinary fixtures and the fixed training/holdout split above. For general extension points P define `T=P+Frob(P)` and `U=P-Frob(P)`. Test `x(T)+lambda*x(U) in W_h` and `x(T)*x(U) in W_h`, with lambda=1,2,3 and h=2,4,8; the product predicate is one family independent of lambda, giving 12 distinct configurations. Here x(U) uses the embedded twist coordinate and `W_h={0,...,h-1}` in Fp.',
    method: 'First enumerate all extension points at p=59,211,251. On larger fixtures use at most 2^16 deterministic public point proposals per predicate. Record every trace fiber and construct signed projected bases of sizes 8,12,16 in canonical encoding order. Compare the auxiliary system, direct rational collection on exactly that base, a rational elimination of the auxiliary predicate, and a matched random base. Handle T=O or U=O as separately enumerated branches. Freeze at most two configurations using training data before opening holdout outcomes.',
    controls: 'Pure-twist helpers, uncoupled predicates with matched acceptance rate, duplicated witnesses, shuffled auxiliary labels, exact same projected base and the strongest ordinary decomposition method. Unsupported or undersized bases remain failed cells.',
    outputs: 'Exact projected sets, fiber histograms, canonical signed rows, target-row span, verified scalar recovery and complete construction/collection/projection costs. Distinguish a new row from a cheaper way to find an existing row.',
    decision: 'Only a lower verified cost per independent projected row on heldout data justifies a solver/pipeline extension. Equal projected rows with no cost decrease close the tested predicate. No result bypasses Frobenius separation.',
    budget: 'Proposed pilot: one CPU, 4 GiB; at most 2^20 counted group operations per relation cell and 60 seconds as a separate fail-safe. All construction and failed cells count. Exhaustive and sampled populations are reported separately.',
    dependencies: [], blockers: ['Coupled-family constructors, complete counters and independent projected-row replay are required.'],
    references: [`${study}/execution-v1/fixtures.json`, `${study}/reports/EXECUTION_RESULTS.md`]
  },
  {
    id: 'E22', slug: 'twist-projected-energy', route: 1, priority: 'P2',
    title: 'Projected additive energy and relation-yield diagnostics', kind: 'structural diagnostic',
    question: 'Do coupled trace images have an additive structure that predicts more useful decompositions after cardinality and fiber multiplicity are controlled?',
    motivation: 'Counting more lifts is inconclusive. Projected additive energy and actual target decomposition histograms distinguish a structured rational image from witness duplication.',
    hypothesis: 'A frozen projected family has higher useful decomposition yield than matched random subsets of the same rational group. This is a diagnostic and supplies no unknown-log algorithm by itself.',
    inputs: 'The E21 exact projected sets, existing ordinary bases and sizes 8,12,16. Use all targets on the three exhaustive pilot curves and the fixed public target split on the other fixtures. Include 32 public matched-random-base seeds per size.',
    method: 'Compute pair-sum multiplicities, additive energy, three-term target decompositions and independent signed-row yield using actual group operations. Optional character/Fourier coordinates may use known toy logs only inside a separate diagnostic checker; those logs must never enter the collector, selector or reported algorithm cost. Preserve raw histograms and compare identities among configurations rather than treating repetitions as independent data.',
    controls: 'Matched base cardinality, sign quotient, support, acceptance rate and construction budget; fiber-resampled copies of one base; shuffled labels; interval-like and random rational bases. Test whether observed target yield follows the full histogram rather than its mean alone.',
    outputs: 'Exact toy distributions and heldout yield/rank tables, counted diagnostic work and uncertainty across independent random bases. Record any mismatch between energy and usable target-row span.',
    decision: 'A robust structural difference selects a specifically frozen family for further work. An energy increase without independent-row or cost improvement is not a collection win. A known-log diagnostic is never presented as an executable DLP shortcut.',
    budget: 'Proposed pilot: one CPU, 4 GiB, at most 2^20 pair/group operations per base and 60 seconds per histogram cell; an incomplete histogram stays incomplete.',
    dependencies: ['E21'], blockers: ['Projected sets must exist; diagnostic/collector separation and independent row checking are unimplemented.'],
    references: [`${study}/reports/EXECUTION_RESULTS.md`, 'research/notes/index-calculus/RESEARCH_INDEX_CALCULUS_FACTOR_BASE.md']
  },
  {
    id: 'E23', slug: 'twist-partial-relation-elimination', route: 1, priority: 'P1',
    title: 'Mixed extension helpers and partial-relation elimination', kind: 'collection hypothesis',
    question: 'Can general extension-point helpers reduce collection cost after every helper is eliminated, compared with the identical rational partial-relation graph?',
    motivation: 'Pure twist helpers cannot contribute a new odd-order source component. General extension points have a rational component and permit a partial-relation experiment, but their projected variables must be accounted for honestly.',
    hypothesis: 'An auxiliary representation collects partial relations cheaply enough to pay for graph storage, elimination and projection. The possible gain is computational; no new mixed-group identity is assumed.',
    inputs: 'The ten ordinary fixtures, E21 bases of sizes 8,12,16, and helper caps 16,32,64. Use relation arities 2 and 3. Admit an r-primary projector only after checking the exact extension-group order and required coprimality; otherwise emit a typed unsupported cell.',
    method: 'Verify each proposed relation in the actual extension group. Build unknown columns only for nonzero r-primary projections of general helpers; pure twist helpers are zero in that source component. Compare zero-helper, pure-twist, general-helper and direct-rational-helper arms. Apply the identical graph/Schur elimination policy, stopping on target-row span with recoverable scalar, and replay every final row in E. Neither known scalar witnesses nor hidden helper logs may guide collection.',
    controls: 'A deliberately fake twist-log column must be rejected. Compare the same projected helper identities, same graph, same target and same stopping rule; include graph cycles yielding only zero rows and a standard rational large-prime-style control.',
    outputs: 'Partial rows and their exact group witnesses, projector certificates, helper identities, graph components, elimination transforms, nonzero independent final rows, scalar recovery and all collection/storage/elimination costs.',
    decision: 'A heldout complete-cost improvement supports this collector. A larger partial graph with unchanged useful span does not. Pure-twist-only output must agree with the E29 separation certificate.',
    budget: 'Proposed pilot: one CPU, 4 GiB; cap each cell at 2^20 group operations, 2^16 stored partial rows and 120 seconds. Record whichever cap fires and the unresolved target span.',
    dependencies: ['E21', 'E29'], blockers: ['Certified projectors, exact helper identities, replayable graph elimination and full counters are missing.'],
    references: [`${study}/reports/EXECUTION_RESULTS.md`, 'research/notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md']
  },
  {
    id: 'E24', slug: 'twist-base-field-jacobian', route: 2, priority: 'P1',
    title: 'Base-field Jacobian arithmetic and independent arbitrary-class replay', kind: 'implementation and validation bridge',
    question: 'How much of the present cover cost comes from the Fp6 chart, and do arbitrary-class maps replay in an independent same-field representation?',
    motivation: 'The ten covers passed native class tests in an odd-degree Fp6 branch chart. PARI checked orders, not an independent arbitrary-class group law or norm. That distinction remains open.',
    hypothesis: 'A balanced two-infinity Fp implementation avoids avoidable representation cost while preserving the exact same classes and maps. Faster arithmetic alone would be an implementation-stage result.',
    inputs: 'All ten frozen selected cover models and their explicit chart conversions. Replay all 3,551 classes on the smallest Jacobian, the original 256 public sampled classes on each other model, and all original map/associativity controls. Add E19 models only after their curve/map certificates pass.',
    method: 'Implement the actual balanced divisor conventions over Fp, including both infinity points and exceptional branches. Supply a checked conversion to the existing Fp6 chart and certify base-field descent. Use an independently implemented divisor law or an applicable external CAS for arbitrary-class addition, pullback and norm. Compare native operation counts on identical classes, not point counts as a substitute for divisor replay. Include degree-3/4 irreducible support factorization, rather than only split support.',
    controls: 'Origin, ramification, both infinity points, non-split supports, inverses, repeated support and the entire smallest class population. A mismatched infinity convention, wrong field conversion or non-descended class must fail.',
    outputs: 'Explicit model and divisor conventions, chart/field conversion certificates, independent class-map replay, factorization coverage, stage counters, peak memory and unsupported branches. Keep order certificates separate from group-law certificates.',
    decision: 'A verified base-field stage improvement enables a correctly priced cover pipeline comparison. Missing independent arithmetic or exceptional charts leaves transfer validation partial. No destination decomposition solver is supplied merely by changing charts.',
    budget: 'Proposed pilot: one CPU, 4 GiB, 3,600 seconds per fixture with per-class checkpoints. Preserve partial exhaustive progress and failures; do not replace the required whole smallest population with samples.',
    dependencies: [], blockers: ['Balanced Fp arithmetic, independent arbitrary-class checker and complete non-split factorization support are missing.'],
    references: [`${study}/runner/src/jacobian.rs`, `${study}/reports/EXECUTION_RESULTS.md`]
  },
  {
    id: 'E25', slug: 'twist-homogeneous-solver-control', route: 3, priority: 'P0',
    title: 'Applicable homogeneous signature baseline with complete output certificates', kind: 'solver-control closure',
    question: 'Can a same-characteristic signature solver be compared fairly after homogenization, saturation and dehomogenization are fully verified and priced?',
    motivation: 'The installed signature-mode attempt rejected nonhomogeneous inputs and a diagnostic reported a different working characteristic. Its output is not an applicable same-field baseline.',
    hypothesis: 'A supported homogeneous path or a different applicable signature implementation can supply the missing baseline. This is a solver-coverage hypothesis, not a predicted native win.',
    inputs: 'Start with p=59,211,251, target seed 0 from the original solver fixtures, arities 2 and 3, all four original encodings and grevlex/lex: 48 input systems. After passing applicability/correctness, extend to the original 1,930 configured V3 cells with their original identities, not newly favorable replacements.',
    method: 'First verify the mode and executable really support the requested finite characteristic; if they do not, mark that backend inapplicable and select a supported implementation. Construct the homogenized ideal, saturate by the homogenizing variable where required, and separately record components at infinity. Check dehomogenized ideal membership both ways and replay all projected solutions. Certify a claimed complete basis by every required S-pair reduction and generator inclusion, or an independent complete-basis comparison. Ideal equality alone does not certify a Gröbner basis.',
    controls: 'Wrong-characteristic output, intentionally truncated bases, lost affine branches, spurious infinity components, same-field direct F4/q0 and a homogeneous positive control. Include all encoding/decoding, saturation and solution-extraction work.',
    outputs: 'Exact transformed inputs, actual field/mode/version, complete-basis or explicit incomplete status, projected solution sets, operations, memory and all failures. Compare with the fastest available applicable external implementation; name unavailable proprietary references.',
    decision: 'Only verified equal tasks admit a stage comparison. If no applicable signature implementation is available, that baseline remains missing rather than being replaced by a wrong-characteristic run.',
    budget: 'Proposed pilot: one CPU, 4 GiB, 30 seconds per engine/system with the same hard envelope for each compared engine. Freeze timeout semantics and record extraction/checking separately before extending the grid.',
    dependencies: [], blockers: ['Applicable signature backend, homogenization/saturation adapter and complete-basis checker are not yet pinned.'],
    references: [`${study}/SOLVER_PROTOCOL_V3.md`, `${study}/reports/EXECUTION_RESULTS.md`]
  },
  {
    id: 'E26', slug: 'twist-block-elimination', route: 3, priority: 'P2',
    title: 'Frobenius blocks, hybrid elimination and bounded invariant encodings', kind: 'solver hypothesis',
    question: 'Can splitting rational/anti-rational variable blocks or eliminating a small helper block reduce certified solver work on the same projected relation task?',
    motivation: 'Equivalent product/chain/Boolean/symmetric encodings were checked only on the first solver grid. Structure-aware elimination is a distinct algorithmic possibility, even when no new geometric relation exists.',
    hypothesis: 'A bounded block elimination or hybrid enumeration removes expensive intermediate terms more cheaply than direct F4 on an equivalent task.',
    inputs: 'The ten original ordinary fixtures, base sizes 8,12,16 and arities 2,3,4. Freeze four candidates: direct addition chain; Frobenius block order; enumeration of one at-most-eight-value helper followed by elimination; and one explicitly derived first-pair invariant encoding. Use training targets first and retain every failed encoding.',
    method: 'Prove and exhaustively test projection equivalence on the smallest fixtures, including signs, denominators and identity branches. If field equations are added, declare the finite-field task change and compare the same constrained task in every arm. Record Macaulay dimensions/nonzeros, reductions, complete-basis certificates and output lifting. Freeze variable orders and at most one selected candidate before heldout targets; use E25 for an applicable external reference.',
    controls: 'The best direct rational low-degree chain, all original encodings, identical finite bases and target rows, wrong-sign/denominator negative systems, enumeration overhead and an A/A run. Lower displayed degree or ideal equality without completeness cannot pass.',
    outputs: 'Native systems and exact projection certificates, complete/incomplete solver statuses, stage work, root extraction, memory and cost per independent verified projected row.',
    decision: 'A verified heldout stage gain can proceed to a complete pipeline comparison under E27. Otherwise record a loss, tie or incomplete cell for each frozen encoding; no coordinate reformulation is automatically called an advance.',
    budget: 'Proposed pilot: one CPU, 4 GiB, 30 seconds per solver cell; cap helper enumeration at eight values and at most four encodings. All branch solves are charged to the same candidate cell.',
    dependencies: ['E25'], blockers: ['Candidate derivations, native adapters, full completeness/lifting checks and counters are missing.'],
    references: [`${study}/runner/src/solver.rs`, `${study}/reports/EXECUTION_RESULTS.md`]
  },
  {
    id: 'E27', slug: 'twist-complete-cost-accounting', route: 3, priority: 'P0',
    title: 'Complete primitive accounting and a cold one-target replay', kind: 'measurement closure',
    question: 'Do any apparent IC stage or pipeline savings survive complete symmetric pricing of construction, solver, projection, verification and linear algebra?',
    motivation: 'The initial panel has unpriced high-level arithmetic, symbolic work and extraction. Its ratios are sensitivity calculations and cannot select a calibrated winning IC baseline.',
    hypothesis: 'No direction of improvement is assumed. The experiment closes an accounting gap so that a win, loss or unresolved comparison has a defensible unit.',
    inputs: 'First instrument the original 24 arms on p=59,211,251. Then replay the full original ten-curve/eight-target grid with all 1,920 attempts, including empty-base cells. Freeze the best verified ordinary-IC baseline for each workload before evaluating any new E21/E23/E26 pipeline.',
    method: 'Instrument primitive field operations through all reference and candidate code, including inversions with a frozen algorithm, extension conversion, rejected candidates, F4/symbolic work, roots, signs/deduplication, matrix work and target recovery. Derive any GAE conversion from the declared counted reference law and expose its assumptions. Keep raw counts, setup and online intervals. Apply the 0/0.1/1 GAE sensitivity band symmetrically to any remaining unpriced units; if material units remain, the complete-cost conclusion remains unresolved. Obtain admitted isolation and A/A stability before making wall-time claims.',
    controls: 'Identical-base ordinary IC, the strongest matched rho available for each curve, a same-binary A/A arm, cold construction per target and independently recomputed counters. An error is an attempted cost, not a zero-cost solved workload.',
    outputs: 'Every phase and failed attempt, target-span stopping and verified scalar, peak RSS, count definitions, complete or explicitly incomplete costs, independent replay/aggregation and matched rho reference. Preserve the original campaign as a separate frozen version.',
    decision: 'A lower verified cost than the frozen best IC baseline is an IC improvement within this workload. Beating rho is a separate test. An unexplained sensitivity reversal or missing material cost blocks a complete-cost claim.',
    budget: 'Proposed pilot: one CPU and 4 GiB per arm, 120 seconds per one-target cell, checkpoint after each arm/target. Full-grid runtime remains unestimated; pilot measurements are required before scheduling it.',
    dependencies: [], blockers: ['Complete counters, admitted calibration/isolation, independent aggregation and a runnable frozen protocol are missing.'],
    references: [`${study}/E15_SPEC_V1.json`, `${study}/PIPELINE_PROTOCOL_V1.md`, `${study}/reports/EXECUTION_RESULTS.md`]
  },
  {
    id: 'E28', slug: 'twist-heldout-selection', route: 3, priority: 'P2',
    title: 'Heldout family selection and an honest solver portfolio', kind: 'adaptive-search hypothesis',
    question: 'Can a selector using only public instance features improve cold one-target IC after all search and selection costs are included?',
    motivation: 'A large candidate grid can produce a favorable cell by chance. A useful adaptive method must choose without seeing the target logarithm or the heldout outcome.',
    hypothesis: 'A small frozen selector chooses a better family/solver than one fixed global configuration often enough to pay its own cost.',
    inputs: 'Only completed training outputs from E21/E22/E26 and the complete counters from E27. Freeze at most four candidate configurations and one selector. Final evaluation uses fresh target labels 32 through 63 on the four ladder-replicate-1 holdout curves; earlier labels 16 through 31 are validation, not fresh final evidence.',
    method: 'Use public coefficients, field size, base size, degree/sparsity and budgeted pilot statistics as features. Freeze a deterministic rule on the training curves and labels 0 through 15; hash it before final targets are generated. Charge feature extraction, pilot failures and any per-target tuning. Report cold one-target cost first; any amortized reuse is a separate workload with an explicit target count and setup interval.',
    controls: 'Best frozen fixed IC configuration, matched rho, a random selector with the same pilot budget, an A/A selector and a hindsight oracle clearly labeled non-executable. Neither verifier-only known logs nor final-target outcomes may enter the selector.',
    outputs: 'Selector source/hash and allowed features, every candidate/pilot attempt, choices, failures, verified final scalars and paired complete costs. Evaluate all final cells, including unsupported choices; do not replace them after inspection.',
    decision: 'A complete-cost reduction on the sealed final population supports an adaptive engineering improvement and justifies independent replication of this frozen selector. A training-only gain, oracle result or failed heldout transfer does not support that conclusion.',
    budget: 'Proposed pilot: one CPU, 4 GiB, at most four configurations and two budgeted pilot probes per target; 120 seconds total per selector cell, including probes. Final resource estimates await E27.',
    dependencies: ['E21', 'E22', 'E26', 'E27'], blockers: ['Verified candidates, complete counters, a frozen selector and unopened final target set are required.'],
    references: [`${study}/execution-v1/fixtures.json`, `${study}/reports/EXECUTION_RESULTS.md`]
  },
  {
    id: 'E29', slug: 'twist-projection-span-certificate', route: 1, priority: 'P1',
    title: 'Concrete separation and projected target-span certificates', kind: 'formal and structural bridge',
    question: 'Can the existing abstract Frobenius-separation argument be connected to the actual finite-field embedding and to the exact row transformations used by an auxiliary collector?',
    motivation: 'Nine Lean theorems cover abstract/conditional statements, but the concrete field/twist/Frobenius and row-span bridges remain open. These checks prevent a partial-relation matrix from inventing source information.',
    hypothesis: 'For the stated odd-order and projector hypotheses, pure-twist helper columns add no source target span; general helper projections must be represented explicitly. This is a proposed formalization of the existing obstruction, not a universal computational lower bound.',
    inputs: 'The existing Lean project and exact p=59 finite-field/twist embedding, then the three frozen even/composite controls. Consume E23 relation matrices only after a separate group replay; synthetic matrices with invalid twist-log columns are negative controls.',
    method: 'Prove the concrete twist embedding and Frobenius identities, instantiate the odd-subgroup/projector hypotheses, and connect each recorded matrix elimination to the projected rational row module. Ship the relevant identity certificates and Lean proofs without sorry or custom axioms. Search bounded even/torsion controls for counterexamples to each deliberately weakened hypothesis. State precisely which equalities are general proofs, exhaustive finite checks or samples.',
    controls: 'Remove the odd-order assumption, use a wrong Frobenius sign, omit the factor-two normalization, inject a fake twist-log unknown and supply a noninvertible projector denominator. Each must fail verification or trigger the correctly typed exception.',
    outputs: 'Lean source and axiom audit, concrete field/embedding certificates, exact matrix transforms, target-span replay and negative controls. The proof must not conclude that equivalent encodings have equal computational cost.',
    decision: 'A checked bridge constrains E23 and closes part of the original proof map. Failure identifies the exact missing hypothesis or implementation defect. It does not retire the coupled-representation or geometric routes.',
    budget: 'Proposed pilot: one CPU, 4 GiB, 600 seconds per Lean module/build shard and exhaustive finite checks only on the smallest frozen fixtures. Compilation failure or cap exhaustion leaves the bridge open.',
    dependencies: [], blockers: ['Concrete finite-field/Frobenius/twist and matrix-row-module formal bridges are unimplemented.'],
    references: [`${study}/lean/README.md`, `${study}/execution-v1/fixtures.json`, 'docs/identity-certificates/README.md']
  }
];

// Never demote a future runnable/funded card while refreshing proposal prose.
for (const e of experiments) {
  const p = path.join(root, 'backlog', `XP-20261022-${e.slug}`, 'card.json');
  if (!fs.existsSync(p)) continue;
  const c = JSON.parse(fs.readFileSync(p));
  if (c.status !== 'blocked' || c.protocol.path !== null || c.code.commit !== null ||
      c.run.argv.join() !== 'false' || c.cairn.funding.units !== 0) {
    throw new Error(`Refusing to overwrite a promoted, pinned, runnable or funded card: ${c.id}`);
  }
}

const payload = {
  schema: 'twist.follow-up-design/v1', created: '2026-10-22',
  status: 'proposed_not_started', evidence_base_commit: base,
  execution_started: false, scheduled: false, funding_units: 0,
  novelty_established: false, speedup_established: false,
  original_program: `${study}/experiments.json`, original_requirements_preserved: true,
  evidence: [evidence, fixturePath],
  fixtures_sha256: crypto.createHash('sha256').update(fs.readFileSync(path.join(root, fixturePath))).digest('hex'),
  priorities: {P0: 'Close a prerequisite for an admissible comparison.', P1: 'Test a concrete route or construction bridge.', P2: 'Broaden applicability, diagnose structure or test adaptation.'},
  training_fixture_roles: ['structural', 'ladder-8-0', 'ladder-10-0', 'ladder-12-0', 'ladder-14-0'],
  holdout_fixture_roles: ['ladder-8-1', 'ladder-10-1', 'ladder-12-1', 'ladder-14-1'],
  new_target_labels: {training: [0,15], validation: [16,31], final_E28: [32,63]},
  experiments: experiments.map(e => ({...e, status: 'proposed_not_started', backlog_id: `XP-20261022-${e.slug}`}))
};
fs.writeFileSync(path.join(dir, 'experiments.json'), JSON.stringify(payload, null, 2) + '\n');

const intro = `# Follow-up experiments for twist routes 1–3

Date: 2026-10-22. **Design only: E18–E29 are proposed and have not run.**
These twelve proposals extend [PR #2380](https://github.com/aburan28/crypto/pull/2380).
They preserve every open requirement in the [original E00–E17 program](../EXPERIMENTS.md)
and the [bounded execution report](../reports/EXECUTION_RESULTS.md).
No new computation, lease, funding transaction or scheduled task is created.

The first scientific questions are actual P-521/toy 3-gluing (E18/E19),
coupled Frobenius-component restrictions (E21), and mixed-helper elimination
(E23). E25 and E27 close solver-reference and accounting gaps needed to judge
those routes. E24 and E29 close independent arithmetic and concrete proof gaps.
E20/E22/E26/E28 widen the search without assuming a gain. All are retained;
priority orders work and does not cancel a proposal.

## What the current evidence leaves open

The [frozen source snapshot](https://github.com/aburan28/crypto/tree/${base}/${study})
is the evidence base. Its module filter names P-521 as a 3-torsion candidate;
the quotient, polarization and maps are unconstructed. Its cover arithmetic
has native class checks but no independent arbitrary-class replay. Its solver
panel lacks an applicable signature baseline and complete-basis certificates
for every reported partial output. Its IC comparisons leave material work
unpriced. None establishes a calibrated twist-assisted IC speedup or novelty.

The Frobenius separation argument continues to apply to adding pure twist
points to the original factor base. The collection hypotheses below test
the cost of finding verified projected rational relations; they do not assume
that an auxiliary representation creates a new group relation. Geometry,
construction, solver-stage improvement and complete IC improvement are
separate conclusions with separate certificates.

## Fixed design rules

- **Inputs:** reuse the exact [fixture manifest](../execution-v1/fixtures.json).
  Its full SHA-256 is pinned in the [machine matrix](experiments.json).
  The ten ordinary fixtures are the two structural models and the eight
  ladder models. The three even/composite controls keep their original roles.
  Train on both structural models plus ladder replicate 0 at each size;
  hold out ladder replicate 1 at each size. This is a fixed retrospective
  split of already public curves, not a claim that those curves were unseen.
- **New targets:** for experiments requiring new targets, labels 0–15 are
  training, 16–31 validation, and 32–63 fresh final E28 evaluation. A native
  generator must domain-separate with \`twist-followup-20261022-v1\`, the full
  curve ID, experiment ID and label, then rejection-sample a nonzero scalar
  using SHA-256. The exact serialization and generator revision must be frozen
  in the runnable protocol. Known scalars are verifier-only; collectors and
  selectors receive points. E25/E27 explicitly reuse their historical inputs.
- **Controls:** same projected base, targets, sign conventions, arity,
  subgroup and target-span stopping rule. Compare the best verified ordinary
  IC and strongest applicable external solver, with a matched rho reference.
  Setup, failed attempts, projection, deduplication, verification, linear
  algebra and target recovery all count. Cold one-target cost is primary.
- **Readiness:** native Rust implementations, exact inputs/argv/source pins,
  independent checkers, checkpointing and a committed hashed protocol are
  required before a card becomes ready. Every card is blocked and unfunded;
  its failing sentinel is not a launch command. Proposed caps below are
  limits, not runtime estimates or resource reservations.
- **Decisions:** identical verified output with a cost reduction against the
  frozen best IC baseline is the improvement test. A stage result stays a
  stage result. Toy correctness, necessary filters, more lifts, lower degree,
  an energy statistic or ideal equality alone do not pass it. Preserve all
  failures and inconclusive cells. A bounded failure does not prove
  nonexistence. Do not use heldout outcomes to redesign the same evaluation.
- **Formal claims:** this document proposes obligations; it asserts no new
  algebraic identity. Any executed round claiming one owes an identity
  certificate and the repository's Lean/source-bridge evidence.

## Priority and ownership

P0 closes a comparison prerequisite; P1 tests a route/bridge; P2 broadens or
diagnoses it. All statuses below are **proposed, blocked, not started**.

| ID | Proposal | Priority | Route | Depends on |
|:--|:--|:--|:--|:--|
${experiments.map(e => `| [${e.id}](#${e.id.toLowerCase()}) | ${e.title} | ${e.priority} | ${e.route} | ${e.dependencies.join(', ') || 'existing evidence'} |`).join('\n')}

The [machine matrix](experiments.json) holds the same designs and backlog IDs.
The [dependency diagram](PROGRAM.svg), editable [Graphviz source](PROGRAM.dot)
and [PDF report](REPORT.pdf) accompany this report. Every new proposal has
its own card; the three original route cards remain the parent trackers.
`;
const sections = experiments.map(e => `
## ${e.id}

**${e.title}.** ${e.priority}; route ${e.route}; ${e.kind}.
[Blocked backlog card](../../../backlog/XP-20261022-${e.slug}/README.md).

**Question.** ${e.question}

**Why now.** ${e.motivation}

**Hypothesis, untested.** ${e.hypothesis}

**Inputs and finite grid.** ${e.inputs}

**Procedure.** ${e.method}

**Controls.** ${e.controls}

**Required outputs.** ${e.outputs}

**Decision and falsifier.** ${e.decision}

**Bounded pilot.** ${e.budget}

**Dependencies and blockers.** ${e.dependencies.join(', ') || 'Existing frozen evidence only.'} ${e.blockers.join(' ')}

**Evidence pointers.** ${e.references.map(p => `[${path.basename(p).replaceAll('_', ' ')}](../../../${p})`).join('; ')}.
`).join('');
const closing = `
## Literature and novelty gate

Summation polynomials, symmetry reductions, gluing, large-prime partial
relations, homogenization and adaptive solver portfolios are established
techniques. The possible contribution here is a verified construction or
measured method for these exact twist-assisted tasks, if one is found.
**Novelty has not been established.** These are hypotheses and replication/
measurement gaps, not twelve new algorithms.

Before promoting a positive result, audit its exact construction, assumptions,
algorithm and complexity against at least these existing source leads and
the repository's [cover/decomposition ledger](../../notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md):

- [Semaev, summation polynomials, ePrint 2004/031](https://eprint.iacr.org/2004/031).
- [Faugere–Huot–Joux–Renault–Vitse, symmetrized summation polynomials, EUROCRYPT 2014](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf).
- [Joux–Vitse, cover and decomposition, ePrint 2011/020](https://eprint.iacr.org/2011/020), with its extension-field assumptions kept explicit.
- [Broker–Howe–Lauter–Stevenhagen, genus-2 constructions](https://arxiv.org/abs/1403.6911).
- [NIST SP 800-186, public parameters](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf).

These links are inherited source leads, not a fresh exhaustive literature
search. A future positive result must identify its closest prior method and
say whether it is a construction, implementation, encoding or accounting
improvement. Nothing here is a NIST DLP or security claim.

## Delivery and unchanged records

The request to keep proposing experiments is addressed by twelve concrete
designs, a machine matrix, twelve blocked cards, this report, its editable
dependency diagram and PDF. No scientific experiment ran in this follow-up.
Validation is document/link/hash checking, native backlog validation and the
repository's required checks; those checks are not experiment outcomes.

The prior protocols, raw sessions, receipts, result reports and PDFs stay
unchanged. The canonical catalog/browser and cover assessment receive no new
models. The checked canonical views—\`docs/index-calculus-scoreboard.html\`,
\`docs/ic-leaderboard.html\`, \`docs/ic/progress-timeline.json\`, the cover
assessment diagrams and the original study \`PROGRAM.svg\`—stay unchanged:
this round adds proposed dependencies, no verified curve, edge or measurement.
No new benchmark entry is warranted because no performance was measured.

## Proposed dependency diagram

The following final PDF page contains the vector diagram. Every dashed edge
is a proposed dependency, not a verified transfer or measured improvement.
Construction and collector branches may be developed while accounting and
solver prerequisites are closed. E19 models can enter E24 only after their
certificates pass; E20 positives require a new degree-specific protocol.
`;
fs.writeFileSync(path.join(dir, 'README.md'), intro + sections + closing);

for (const e of payload.experiments) {
  const cardDir = path.join(root, 'backlog', e.backlog_id);
  fs.mkdirSync(cardDir, {recursive: true});
  const card = JSON.parse(fs.readFileSync(path.join(root, 'backlog', parent[e.route], 'card.json')));
  Object.assign(card, {
    id: e.backlog_id, title: `Twist ${e.id}: ${e.title}`, created: '2026-10-22',
    authors: ['twist follow-up design lane'], status: 'blocked',
    status_note: `Proposed ${e.id}; not started. ${e.blockers.join(' ')} A separate runnable protocol and source/checker pins are required.`,
    blocked_on: ['problem:problems/twist-trace-constraint-and-solver-gap.md', ...e.dependencies.map(id => `xp:${payload.experiments.find(x => x.id === id).backlog_id}`), `note:${e.blockers.join(' ')}`],
    links: {lead: `${study}/proposals-20261022/README.md#${e.id.toLowerCase()}`, notes: [`${study}/proposals-20261022/experiments.json`, `${study}/proposals-20261022/README.md`, `backlog/${parent[e.route]}/README.md`, ...e.references], prs: [2380]},
    question: e.question, decision_value: e.decision,
    protocol: {path: null, blob: null},
    code: {repo: 'aburan28/crypto', commit: null, branch: 'codex/twist-ic-routes-1-3-20261021', build: []},
    resources: {cpus: 1, ram_gb: 4, disk_gb: 1, gpu: null, est_core_hours: null, est_wall_hours: null, notes: `${e.budget} Full resources remain unestimated. The nominal disk field is not an estimate; calibrate and freeze it before readiness.`},
    run: {argv: ['false'], env: {}, timeout_per_shard_s: 1},
    checkpoint: {file: null, resume_flag: null},
    outputs: {result_schema: 'Not frozen; required evidence fields are in the linked design. No result exists.', verify: [], land: `${study}/followup-runs/${e.id.toLowerCase()}`},
    owner_hint: 'unassigned'
  });
  card.cairn.tasks = [e.backlog_id];
  card.cairn.env.recipe = 'Design only; implement native driver, independent checker, checkpoints and source/protocol pins before readiness.';
  card.cairn.exec.publish = `runs/${e.backlog_id}`;
  card.cairn.funding = {units: 0, rationale: 'Blocked proposal with no run estimate, below the existing funded seed cards. Adding it changes no runnable-work priority or funding.'};
  card.cairn.notes = 'No lab publication, lease, run, resource allocation or funding transaction has been performed. The failing sentinel is intentional.';
  fs.writeFileSync(path.join(cardDir, 'card.json'), JSON.stringify(card, null, 2) + '\n');
  fs.writeFileSync(path.join(cardDir, 'README.md'), `# ${e.title}\n\n**${e.id}: proposed, blocked, not started.** Route ${e.route}; priority ${e.priority}.\n\n[Full finite design and controls](../../${study}/proposals-20261022/README.md#${e.id.toLowerCase()}).\n[Parent route card](../${parent[e.route]}/README.md).\n\n## Question and decision value\n\n${e.question}\n\n${e.decision}\n\n## Inputs and method\n\n${e.inputs}\n\n${e.method}\n\n## Controls and evidence owed\n\n${e.controls}\n\n${e.outputs}\n\n## Prerequisites\n\n${e.blockers.join(' ')}\n\n${e.dependencies.length ? 'Dependent designs: ' + e.dependencies.join(', ') + '.' : 'No new experiment dependency; the linked frozen evidence is the starting point.'}\n\n## Resources and cost derivation\n\n${e.budget}\n\nThese are proposed process caps, not calibrated runtime/disk estimates. Full\ncore-hour and wall-hour estimates remain null until a pilot and runnable\nprotocol exist. Funding is zero, below the existing funded seed cards; no\nallocation, lab publication or spending is performed. The card retains an\nintentional failing command and is not runnable.\n\n## Before ready and before done\n\n- [ ] Implement native Rust driver and an independent checker.\n- [ ] Freeze exact inputs, source, controls, seeds, counters, complete cost intervals and resource estimates.\n- [ ] Commit and hash PROTOCOL.md; pin merged code; fill checkpoints and real verifier/run argv.\n- [ ] Validate the ready card and preserve every failed or inconclusive cell.\n- [ ] Land the bounded result, independent replay, report, diagram and inspected PDF.\n- [ ] Meet the stated decision criterion; distinguish construction, stage and pipeline results.\n\nNovelty and any IC speedup remain unestablished. The original E00–E17\nrequirements are preserved. This card records a proposal, not a completed run.\n`);
}
console.log(`Wrote ${experiments.length} proposal designs and blocked cards; performed no experiments.`);
