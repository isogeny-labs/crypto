# Direct solver admission — active preparation, not a completed run

Historical pre-code checkpoint below. The completed native qualification is
now recorded in QUALIFICATION_SCOPE.md and PCC-20261116-9dda07; the report is
PCC-20261116-44b39a. Full accounting and the broader scientific objective
remain open. Do not edit the now-frozen source/qualifier or original notes
to agree with a later implementation.

The guarded trace adapter is published under4397 at824bf03a. Native35f196,
existing-law Lean03a7ff and report5451de remain frozen. The broad scaling
objective is open. This note is working preparation for the actual solver;
no experiment, solver campaign, source test or paid job has been launched
in this preparation step. Do not present this note as a completed result.

## Sources actually read after trace publication

- Current repository f4_prime_dynamic.rs: MAX_VARIABLES28, borrowed
  Fp64Arithmetic context and u64 coefficients; modeled work/byte limits.
  Its API cannot directly take the existing1059-variable255-bit system.
  fp_backend.rs defines Fp64Arithmetic as a concrete struct, not the generic
  PrimeField trait. FpWide exists elsewhere, but this F4 interface is not
  automatically generic over it. No code is changed or engine run here.
- groebner_f4.rs and symmetrized_semaev.rs: generic-looking MPoly/FieldElement
  path, ordinary Buchberger rather than a matrix-F4 implementation despite
  the filename. try_buchberger has5000 pair steps and returns an error rather
  than a partial basis. A pair cap alone does not establish an allocation or
  complete field-work bound. The sparse checkout lacks some transitive
  source files; do not assume the whole root crate is needed or build it.
- Petit--Kosters--Messeng, PKC2016, primary text sections4/5 and general
  construction: https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf
  Low-degree membership composition and generalized root finding are already
  their framework; no new complexity bound is inherited from the analogy.
  Their comparison of fewer-variable expanded systems matters for the
  original/projected solver comparison. No novelty claim for that idea.
- Kosters--Yeo, Notes on summation polynomials, primary text at
  https://arxiv.org/pdf/1503.08001, especially Proposition2.1 and the warning
  preceding the complexity discussion. General/variable-arity hardness does
  not exclude the fixed six-sum distribution here. Characteristic-two
  first-fall phenomena cannot simply be transferred to this prime field.
- Official msolve landing page: https://msolve.lip6.fr/ identifies release
  v0.11.0 and F4/order-change features. Its exact supported coefficient range
  is NOT established by that page; do not assume255-bit support. Neither
  Singular nor msolve is on PATH in this shell; that is not proof that no
  installation exists elsewhere or that the isolated runner cannot supply it.
- Pinned Mathlib Algebra/Polynomial/Degree/Lemmas.lean, natDegree_comp.
  A scalar composition and a two-register arithmetic DAG are different.
  Working analytic lead: since the actual trace degree has prime factor
  q213156431, any decomposition of that polynomial into nonconstant scalar
  polynomial compositions must contain a stage of degree at leastq, by
  degree multiplicativity. This elementary conditional observation is NOT
  yet registered as a checked theorem and gives no solver lower bound.
  Verify the exact scope/primality/source bridge before citing it as a result.

KB unavailable in this runtime as already recorded: no loaded MCP/backend;
current-main wrapper read. No broad tree scan, solver run or benchmark.

## Concrete next implementation decision

Build a NEW isolated bounded sparse elimination adapter over the existing
BigUint prime-field arithmetic and dynamic sparse monomials, leaving old
engines/source qualifiers unchanged. Candidate owner path:
tools/torus-sparse-elimination. Preserve coefficient modulus exactly; never
truncate to64 bits, reinterpret a small-prime solve as an actual-field solve,
or assume CRT transfers solution sets. Research arithmetic stays Rust.

Before writing code, complete the protocol with exact method, field-work and
allocation accounting, limits, input cases and output classes. Candidate
baseline is ordinary sparse Buchberger with standard pair criteria; a
matrix-F4 optimization is a separate version, not a filename claim.
Read the relevant coefficient/reduction API and primary polynomial-root
extraction method before selecting it. Avoid root-library reruns.

Interface: public original system (or a declared trace projection) and public
target only. Root witnesses must replay through the unchanged original
system and native signed-index/point adapter. Parameter/target constructors
must not pass planted labels/signs/scalars. A nonlinear residual, cap hit,
timeout/OOM or incomplete basis must have a distinct censored/unsupported
status, never an absence result. A unit-ideal absence needs an independently
checked ideal-membership certificate and its declared rational-field scope.

Qualification should use fixed unit systems with known original roots,
contradictions, nonlinear/rational-root controls, malformed inputs and forced
cap failures, followed by the SAME public relation fixtures in both
representations. It must not silently replace the full actual target by a
smaller curve. Small fixed controls are development evidence; the full
actual discovery/scaling objective remains open until its own admitted run.

For any later growing-family comparison: freeze targets, degree/order,
controls, limits and all outcomes; measure maximum degree reached, actual
matrix/pair/reduction work and memory, construction, root lifting/indexing,
useful yield and cold phases. Fewer terms, variables or an early degree fall
is not a successful cost criterion. Apply unchanged ROUTE_CRITERIA_V3 and
SUCCESS_CRITERIA, including the fixed-six-sum gamma<2 necessary condition
and the remaining family/yield/selection/kernel/recovery obligations.

## Publication and continuation

This is active task preparation, not a finished planning deliverable. Finish
the selected protocol/implementation, preserve failed attempts and prepare
its source-linked report/diagram/inspected PDF, native/Lean claims at their
actual scopes, then commit/push the SAME4397 lane PR through the maintained
helper. Preserve all1129 published ledger events and journal blocks. Do not
discard this unpublished work if the separate CI checker wakes; it is not a
CI repair. No new paid execution or solver campaign is scheduled here.
