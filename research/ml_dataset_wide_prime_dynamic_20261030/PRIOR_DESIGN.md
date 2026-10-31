# Wide-prime m4..15: proposed dynamic ordinary-prime F4 increment

Design only, 2026-10-30. Inspected local source at e3fbd87b36f and its m3
corrections84dd1d3549a/281b4ba11c3, native prerequisite b2867d1, in
/home/ec2-user/crypto-ml-wide-prime-m3-adapter-20261030. No source changes,
builds, tests, fixtures, draws or cloud work performed for this design. The m3
worktree is frozen for parent validation. Original m2..15, families, solvers,
policies, intermediate-corpus and S3 objectives remain in scope; this proposes the
next F4 implementation increment, not a claim that the remaining program is done.

## Recommendation

Implement a separately bounded ordinary-polynomial F4 engine with active variable
count1..28, checked multi-coordinate monomials, and the existing PrimeField/Fp64
arithmetic. Reuse the canonical m3..15 chain builder and explicit original worker
context. Port the dimension-independent scalar row-reduction kernel and ordinary
S-pair/symbolic-preprocessing logic; do not change the established packed tower
representation globally. Use a small product-criterion-only F4 initially, rather
than also porting intricate Gebauer–Möller or signature criteria. Retaining more
pairs changes efficiency, not the polynomial ideal or algorithm identity.

This is the smallest faithful route found in the inspected source: it combines
an existing correct wide coefficient backend with ordinary dynamic monomials and
an actual matrix-F4 loop. Group-domain enumeration remains an independently
labelled witness/domain checker. It cannot stand in for native algebra, and a
verified group witness alone cannot satisfy the new native-execution gate.

## What exists, and why it is not already the required solver

| Source | Reusable part | Limitation / disposition |
|---|---|---|
| src/ml_dataset/solvers.rs prime_chain_polynomials, prime_chain_input, verify_prime_f4_chain_input | Complete original m3..15 affine chain, canonical coefficient/exponent encoding and leaf/auxiliary domain map | Already constructs2m-2 coordinates; existing narrow visitor is not a wide solver. Preserve the encoding and old metadata, add distinct execution profile. |
| src/cryptanalysis/f4_fp.rs | Real degree-selected F4, Vec<u32> monomials, ordinary pair selection/symbolic preprocessing, explicit-domain solve_visit_traced recursion | f4_with_arithmetic constructs Barrett at about2800; Barrett asserts p<2^32; dense rows contain u32 coefficients. solve_visit_impl explicitly rejects p>=2^32 near4912. Poly's u64 coefficient type does not make the engine wide-safe. Packed path is <=15 variables near2807; vector fallback already exists. A wholesale genericization touches a large established optimization/checkpoint surface. |
| src/cryptanalysis/f4_tower_generic.rs | PrimeField row arithmetic, sparse Row/Acc, echelon near570, eliminate near710, bounded events/admission/report semantics | GPoly/GTowerRing/State use imported packed Mono and MAX_FREE4. Current safe profiles admit2 or4 free coordinates. Numeric kernel is dimension-independent, but symbolic state and old memory constants are not. |
| src/cryptanalysis/fp_backend.rs | Fp64 Montgomery arithmetic and checked canonical conversions; generic row and lazy-accumulator APIs | Fp64 supports a larger field range, but dataset profile remains<=2^61-1. Expanding field sizes is a separate scope decision. Raw Montgomery words must never become serialized coefficients. |
| src/cryptanalysis/groebner_f4.rs and symmetrized_semaev.rs | Dynamic BigUint MPoly, textbook reductions/S-pairs/Buchberger for small independent references | Header discusses matrix-F4, but inspected public functions provide Buchberger/reduce_basis, not a ready bounded wide matrix-F4 API. Do not infer implementation from the header. No original worker admission envelope. |
| src/cryptanalysis/sig_fp_tower.rs | Signature-order and regular-reduction design, prior quotient-ring correctness controls | Uses TowerRing/Fp and MAX_FREE packed signatures. Not a ready dynamic wide-prime F5. Porting it now adds another representation plus syzygy/rewrite correctness obligation. Keep as a later named solver track. |
| src/cryptanalysis/matrix_f5_f2.rs | Boolean row-criterion history and comparison controls | F2BoolPoly/mask arithmetic, not general-prime F5. Its own documentation distinguishes its criterion from full incremental signature-compatible F5. |
| src/cryptanalysis/f6_wide_geometry.rs | Existing exact binary four-summand geometric closure and group checker | Explicitly meet-in-the-middle, not a Gröbner algorithm. Cannot be relabelled as arbitrary-m prime F4/F5/F6. |
| src/cryptanalysis/wide_groebner.rs, inherited_f4.rs, sparse_macaulay.rs | Boolean sparse structure, bounded-row and inherited-state engineering patterns | Binary square-free monomials/XOR rows. “Wide” refers to binary variables/coordinates, not odd-prime coefficient arithmetic. |
| src/cryptanalysis/hky_fp_tower.rs | Independent closure/fall-degree definitions and queue accounting lineage | Explicit p<2^32, MAX_FREE8 and packed u128; computes HKY closure rather than F4. Neither a28-variable engine nor an interchangeable solver label. |
| src/cryptanalysis/prime_root_trace.rs | Bounded distinct-root extraction over p<=2^61-1 with independent arithmetic verifier | Current MAX_DEGREE8, MAX_WORK1,000,000. Can be reused for admitted univariates; higher degree must produce explicit root-degree censorship or a separately reviewed extension. |

## Canonical system and representation

For m in4..15: V=2m-2 in6..28; original variables are x0..x(m-1), followed by
u0..u(m-3). There are m-1 S3 equations plus m membership polynomials. Every leaf
has the same distinct x-domain (1..32 values); auxiliaries remain all_Fp. At m15,
there are29 polynomials and at most13*(m-1)+33*m=677 input terms. Input degree is
at most32. These are checked input bounds, not expectations that elimination will
finish within any particular memory/time budget.

Proposed private Mono28 stores [u16;28], with active length on the ring. Unused
coordinates MUST be zero. Construction, multiplication, quotient, lcm, degree,
and comparison use all active coordinates; multiplication uses checked_add and
returns explicit exponent admission failure. No bit packing, tower masks, or
truncation of coordinates4..27. Compute degree from coordinates rather than store
an independently mutable cached degree. Grevlex is total degree followed by the
reverse-coordinate comparison with the existing convention. Test against the
independent Vec<u32> comparator for every active dimension and highest coordinate.
Do not accidentally use derived array lexicographic Ord as polynomial order.

The fixed maximum28 represents the user's full arity interval; it is not a lower
arity limit. General module entry accepts each V1..28 for synthetic controls,
while dataset admission checks V==2m-2. Use GPoly-like separate monomial/coefficient
vectors, canonical merge/zero removal/monic normalization, all slots preadmitted.
Serialize exponent arrays of exactly V coordinates, never silently truncate a
longer array or pad a malformed short one. Keep original public equations unchanged
and store normalized native input separately or reconstruct it deterministically.

## Minimal actual F4 loop

1. Validate original dimensions, coefficient range/primality, input counts, degrees
   and canonical terms before normalization/allocation. Preserve the full original
   input even when budget prevents entry.
2. Build a deterministic monic basis with ordinary row reduction, charging its
   initial matrix and copies. Keep stable basis IDs; no basis-removal optimization
   in the first version. Detect a genuine constant unit distinctly.
3. Store ordinary critical pairs with full lcm and degree. Skip only the ordinary
   coprime/product criterion initially. Track pairs above the chosen degree limit;
   a nonempty above-limit set prevents unrestricted completion claims.
4. Select the lowest nominal-degree batch with deterministic pair-ID tie order.
   Form actual S-polynomials or both cancelling multiples (not an unexplained
   one-sided row); record source IDs/multipliers in bounded diagnostics.
5. Symbolic preprocessing closes the selected support under chosen basis reducers.
   Search stored leading monomials by divisibility, rather than enumerate all
   possible exponent divisors. Choose a fixed deterministic reducer tie rule.
   Every unseen support monomial, reducer multiple and map insertion consumes
   admitted rows/columns/terms/work/storage before allocation or arithmetic.
6. Sort columns in explicit grevlex order and perform generic prime-field sparse
   Gaussian elimination using the existing scalar kernel design. Promote rows with
   new leading terms; zero rows are counted, not dropped from cost accounting.
7. Continue until pairs exhausted, unit derived, degree/work/storage/deadline cap,
   or observer refusal. Preserve selected-but-unfinished pairs on interruption.
   Native complete must mean all admitted ordinary pairs processed; distinguish
   degree-truncated completion of a selected slice from a complete Gröbner basis.

Do not enable full-rank/staircase/Hilbert early exits without their original
hypotheses and separate certification. Initial generators can have degree32 even
when the requested pair-degree bound is12; preserve and explicitly report that
existing distinction. Do not claim every processed row had degree<=12 merely from
pair selection. If a new strict global row-degree profile is chosen instead, that
is an explicit profile change with an input-degree refusal, never dropped generators.

A dedicated module avoids destabilizing both f4_fp's narrow optimized visitors and
f4_tower_generic's quotient-ring semantics. Initially keep row code private to the
new module with clear source provenance; extract a shared numeric kernel only under
separate ownership and differential tests. Do not expose a fake generic API whose
implementation still calls four-coordinate helpers.

## Budget, storage and observable boundaries

Keep original process deadline, memory and trace binding unchanged. Initial hard
ceilings should remain bounded by the current defaults: native work<=16,777,216,
conservative storage<=min(original memory/4,64MiB), basis<=256, rows/columns/pairs/
steps/events no greater than their current admitted defaults. Adapter bytes remain
min(original trace_bytes/32,4MiB). Admission refusal at high arity must name the
actual exhausted quantity; native_backend_invoked and useful reduction are separate.
Increasing a ceiling requires a frozen amendment and resource/budget accounting.

Four-coordinate tariffs are NOT portable. Freeze an audited cost table before
implementation based on V, H basis entries, P pending pairs, T source terms,
R rows, C columns and Z actual nonzeros. Proposed charge units:

* Monomial comparison/divides/lcm/product: reserve a constant times(V+1), with
  constants covering checked coordinate operations and degree/order comparison.
* Reducer lookup: reserve at least H*(monomial predicate + tie comparison) per
  frontier monomial. Scan is O(H*V), not an exponential divisor odometer.
* Pair insertion: reserve comparisons/allocation for H new candidates and each
  queued pair visit. Omitting GM avoids the old interval-search proof obligation.
* Polynomial products: reserve every T coordinate shift, field multiplication,
  term insertion and canonical sort comparison before forming the temporary.
* Elimination: reserve each initial row/accumulator and each pivot update before
  it occurs, including normalization/inversion, sparse merge and fill-in. A
  separately admitted batch upper bound may replace per-pivot charges only if
  source review proves it covers all loops and concurrent copies.

These are audit formulas, not measured instruction counts. Do not simply copy the
old all-at-once kernel overestimate and then declare every large-m case implemented
because it immediately refuses; useful execution on fixed original-chain controls
is a release gate. A staged scalar kernel with checked permits per pivot can keep
conservative accounting without precharging an impossible dense worst case for
an otherwise sparse step. No parallel kernel in the first increment removes
concurrent-allocation ambiguity; later parallelism needs an independent reservation.

Use actual size_of(Mono28), size_of(Fp64::Elem), size_of(Fp64::Wide), Vec headers,
capacity growth, map buckets, index arrays, pending pairs and all simultaneously
live buffers in storage reservations. Prefer explicit try_reserve_exact under
permits and bounded vector lookup structures to undocumented allocator/map-growth
assumptions. Each allocation/copy is conservatively charged and nonrefunded, as in
the existing native profile; report this distinctly from RSS/live memory. Returned
basis and diagnostics are charged too. Observer errors remain sticky and bounded.

## Algebraic extraction and independent group validation

The first increment must actually run dynamic F4 on the original V-variable
system. A bounded independent group checker may still certify positives, but its
provenance cannot imply those positives were recovered from the algebra. Add
certificate_origin = native_root_candidate or independent_group_prefix in restricted
records, and keep native invocation/reduction/extraction completion separate.

For algebra-derived solutions, reuse the explicit-domain visitor design rather
than invoke its narrow engine unchanged. Maintain original-to-active variable map,
substitute assignments exactly, rerun bounded dynamic F4 on residual systems, and
verify every candidate against ALL ORIGINAL equations with BigUint arithmetic.
Prefer a basis univariate; invoke prime_root_trace only within its degree/work
limits and bind its independently checked root report. Otherwise branch on an
unassigned leaf's finite domain. Never enumerate an unrestricted auxiliary over
all p values merely because the small-prime visitor has that fallback. If only
unrestricted auxiliaries remain with no admissible univariate, emit an explicit
algebraic extraction stop. Higher-degree root extraction is a separate extension,
not a reason to silently cap m. Preserve all visited residual systems and accepted
prefixes within bounded event/payload limits.

For group checking, extend the m3 policy to ordered signed m-tuples with stable
mixed-radix cursor. D^m fits u128 for D<=64,m<=15 (maximum2^90), but not usize;
compute checked u128 totals and generate indices incrementally. A tuple needs m-1
prefix additions, and first certificate replay needs m additional additions from
identity, all within the same original group-add ceiling. Check deadline before
each call, retain partial stage and reserve the worst-case m-dependent record
before work. Continue after first certificate under the bounded policy; publish
that dedicated certificate immediately with idempotent bytes, as corrected in m3.

Auxiliary ui corresponds to prefix sum of leaves0..i+1. If ANY such prefix is
infinity, the full affine-chain tuple is not applicable; record each prefix and
exceptional position, not a fabricated auxiliary coordinate. Later finite prefixes
do not repair an earlier missing affine coordinate. Group matches remain valid
independently, but cannot refute an affine native unit. Only an all-affine tuple
with all original residues zero is an affine root.

A full group negative requires all D^m signed tuples, including all exceptional
prefixes, completed with no matching candidate, required replays and persistence.
At the existing1024 tuple ceiling most higher-m domains cannot finish; preserve
that censorship explicitly. All m4..15 cases remain eligible for native work and
bounded labelled prefixes. Do not narrow the requested corpus to arities/domain
sizes where negative completeness is convenient. Algebraic affine-root exhaustion
also does not prove group nonexistence because the affine encoding omits infinity.

## Replay schemas and publication boundaries

Use a new dynamic-wide profile/version with V and m explicit. Retain the old
canonical chain input byte-equivalent, including its old encoding metadata; do
not repurpose old m3 four-coordinate typed records. New result includes original
launch/limits, actual native state/stop, bounded basis/diagnostics, extraction
origins and partial statuses. Monomial arrays have exact V length and zero-free
canonical coefficient strings. Domain/witness/prefix/solver outcome dependencies
are uniquely manifest-bound, with bounded per-file and aggregate reads BEFORE any
legacy/shared-input parse. Bound every emitted summary/counter, including retained
producer outcome under outer censorship; latest m3 A–F corrections are mandatory
lineage, not optional refinements.

The typed raw-censored-prefix disposition remains distinct from full mathematical
replay. Nested live payloads are opaque hash-checked evidence unless independently
promoted by a specific verifier. Do not erase a persisted first positive on later
caps, or permit an outer timeout to acquire native/group completion labels.
Retain exact reproducible normalization/variable maps and partial stages. Full
native basis derivation remains false until derivation certificates and independent
checks are implemented; native producer statistics are not fall-degree proofs.

No exporter edits without its owner. New restricted labels must not enter public
features. Existing feature consumers must be checked for exact V arrays, old chain
input compatibility and the new native/extraction profile. S3 publication still
requires scoped budgeted PUT/HEAD/GET/hash and restore/replay through the parent
workflow. This design launches nothing and changes neither original total budget
nor generator ownership.

## Fixed release controls and implementation order

A. Freeze representation, algorithm, cost table, stop semantics and dependency
schemas; reserve new native/adapter paths and independent numeric-kernel ownership.
B. Implement ordinary native dynamic F4 and independent small reference checks.
C. Admit every V=2m-2,m4..15; test coordinate4,15,27 and no trailing truncation,
canonical order, multiplication overflow, exact/one-less work/storage and sticky
observer failure. Cross-check low-dimensional prime17 examples with f4_fp and
BigUint Buchberger; verify S-pair/normal-form properties on completed small cases.
D. At every requested arity run fixed synthetic sparse systems using the highest
coordinate and require useful matrix reduction, not an admission/begin callback.
Then fixed original-chain seed42/curve7/generator1/key1 at n47/53/61,both twists,
baseP,-P, with actual point order precondition appropriate to m. Positive targetmP
is fixed, with known construction scalar kept outside the solver. No resampling
when a native/resource/control gate fails. Native useful-execution and independent
positive-certificate gates must both pass; they are not interchangeable.
E. Test higher-m exceptional prefixes explicitly (begin P,-P then fixed remaining
summands), stable first certificate and later censoring. Complete negative controls
at tractable small arities plus high-arity censored-negative controls; do not claim
all-negative completeness at m15 under1024 tuples. Use deliberately bounded toy
reference cases for all-variable semantic checks without pretending production
budgets completed the full signed domain.
F. Implement algebra-derived residual/univariate extraction and independent root
certificates. Track unsupported auxiliary/higher-univariate states explicitly.
G. Central rehashed tests: last-coordinate coefficient/exponent, variable map,
auxiliary-domain narrowing, omitted/relocated descriptors, signed-domain order,
infinity stage, replay reservation, exact terminal and outcome counters, byte
padding/shortage, censored retained proof flags and early-payload promotion.
H. Parent-owned compilation, focused/full applicable checks, independent source
review and fixed CLI restore/replay gates before corpus/cloud amendments.

Local engineering lineage: research/ml_dataset_wide_prime_f4_20261029/DESIGN.md,
IMPLEMENTATION.md, M3-NATIVE-DESIGN.md, M3-NATIVE-IMPLEMENTATION.md,
M3_ADAPTER_CONTEXT.md, M3_ADAPTER_IMPLEMENTATION.md and
M3_FIRST_CERTIFICATE_PERSISTENCE.md; f4_fp.rs references
research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md; sig_fp_tower.rs
references RESEARCH_PKM_TOWER_ORACLE.md; inherited_f4.rs references
RESEARCH_INHERITED_F4.md; fp_backend.rs references research/fp_backend_20261015.
Those references identify prior implementation lineage; no historical benchmark
or theorem claim was imported into this design.

Still outstanding beyond this increment: actual m4..15 implementation and useful
execution, scalable algebraic root extraction, broader field sizes if requested,
faithful prime F5 and separately defined F6 capabilities, other named family/solver
comparisons, complete instruction/derivation/fall-degree certificates, consumer
support and verified durable scientific corpus production. No item is silently
relabelled as complete because bounded group enumeration can find a witness.
