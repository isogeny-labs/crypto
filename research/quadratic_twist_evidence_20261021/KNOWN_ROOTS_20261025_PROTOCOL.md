# Checked factor-base roots: fixed experiment protocol

Frozen before new implementation execution, 2026-10-25. Extends PR2381.

## Hypothesis, reference and exact population

Replace the phase-demand solver's repeated field-wide membership scan by an
explicit, setup-validated root list. Require identical mathematical behavior:
all constrained roots, ordered certificates, promotions, operational and expanded
witness degrees, useful rows/ranks, failed queries, base logs and recovered scalars.
The reference is the immutable phase-demand source and ten canonical records at
7fe697ae9b5d7f460e5d23ac160e22d1f94eba9c. This is an algorithm-request comparison,
not a timing, calibrated cost, intrinsic regularity or attack-speed claim.

Use every F43 size2 policy in native/reduced-s3-20261025/cases (ten files),
with frozen adaptive-s3 certificates and phase-demand records. Each arm executes
40 target-independent setups (four roles per policy) and400 separate nonidentity
source recoveries. Thus the paired experiment executes80 setups/800 recoveries.
No favorable policy/target selection. Main, twist, matched and pulled-back roles,
all ten nonidentity targets, all failures and sign lifts remain. Known-answer
labels construct/check public points outside recovery; recovery accepts Q only.
The previous F43 source map, curve identities and exceptional isomorphism scope
remain: main icv1-fp6-t0-74779950 P=(31,18), twist icv1-fp6-t0-c6a8a47c
U=(24,26), r11, phi(x,y)=(41x,11y). U is not EC1-registered.

## Exact candidate and accounting

Keep the prior incremental row algorithm, term order, insertion order, cap,
promotions and all29 counter definitions. Extend the vector with three counters:
root_entries_validated, root_order_checks, root_polynomial_bindings. A reference
has zero new entries. The candidate setup checks two canonical, finite base x
coordinates against its ordered declared list (two entry checks), strict ascending
order (one adjacent check), and both membership polynomials against the expanded
monic two-factor polynomial (two polynomial bindings). It evaluates each of the
two membership polynomials at both listed roots (four charged membership evals).
All validation occurs once per setup and is included in every hypothetical cold
record. Invalid roots/poly bindings reject, never silently repair.

At a query, iterate x then y in the checked ordered list, evaluating the reduced
S3 constraint on each of the four pairs. Leave all subsequent code unchanged.
The reference scans x in0..42 and y in0..42 for each accepted x, exactly129
membership evaluations per query. Constructive factorization and a Lean domain
root-set theorem justify completeness; having two zero evaluations alone is not
accepted as a proof of the absence of other roots. Setup/root validation is
separately emitted. Coordinate maps, signs, group requests, all matrix builds and
witness operations remain charged. Counters are differently weighted and may
be nested: never add them into a scalar total. Unpriced costs from phase demand
remain unpriced, including field tables, hashing, traversal, sorting and allocation.

The independent Rust path uses its own field/group/polynomial arithmetic and
counter implementation. It reconstructs both arms, compares the reference exactly
to the frozen phase records, and batch-audits every distinct solver certificate
with the unchanged independent semantic checker. Every candidate certificate must
match the reference bytes. Root declarations and membership coefficient arrays
are emitted per setup. Index hashes are checked, not treated as semantic evidence.

## Proof, controls and interpretation

Before native execution prove product-membership root equivalence over an
integral domain, two-root and Cartesian constrained-solution specializations,
and actual F43 main/twist first-policy instances, without assuming completeness.
Use the unmodified repository Lean gate. No trusted native evaluation.
A new source-bound IDC1 polynomial record checks the expanded monic quadratic
identity only, not the algorithm, speed, or completeness theorem.

Rehashed controls: omitted declared root, extra declared root, false membership
polynomial, false setup demand, false matrix demand, missing failed query,
false base log, false recovered scalar, false source binding. Every one must
reject with no receipt. Preserve exact corrupted bytes and failures. Certificate
false-plus-one and false source-hash controls must reject as well.

Acceptance: all80 setups/800 scalar checks pass; reference equals prior evidence;
all candidate certificates/roots/queries/recoveries match, with only declared
membership/term-visit/root-validation counter changes. A lower membership request
count is a verified query-interface change, not a complete cost improvement.
A mismatch refutes this implementation and is retained; repair then rerun the
same population without dropping a target. A null/loss selects another lead.
No external-fastest, rho, end-to-end S, timing or leaderboard claim is admitted.
No additional uploads, credential reuse, new resources or auto-merge.

## Reproducibility and budgets

Commit this protocol before executing new native work. Commit compiled producer,
auditor, source hashes and exact root checks before first execution. Use a fresh
output directory for every attempt. Native command caps600 seconds, full iteration
envelope3600 seconds, existing local host only, two build workers, no paid compute.
Keep raw sources/logs/control payloads, compact deterministic archive, report,
graph/diagram and inspected PDF. Fresh root cargo test --release --locked --lib,
22 auditor backend tests, workflow metadata, naming and artifact-policy checks.
The pre-edit actual Conductor call exited127; no reservation/replacement claimed.
Review the existing lead atlas before selection; update its handoff for next review.
Probe/profile the frozen baseline with current-main tools/profile.sh if available;
missing access/tools are preserved, not fabricated as cost evidence. A profile
never supplies a speed result. No Python research arithmetic/harness/verifier.
