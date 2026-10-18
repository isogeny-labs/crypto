# Fresh end-to-end experiments for the three supplied ECDLP leads

Date: 2026-10-18. This is new execution, not reconstruction of unlocated
historical records. Preserve the earlier audit and every attempted run.
User requirement: implement and run all three leads in aburan28/crypto,
including an explicit genus-2 transfer, and resolve each quoted numerical
claim against fresh evidence. A plan, coverage-only census or map-only
demonstration does not satisfy that requirement.

## Common execution and accounting

Use native Rust. Algorithms receive the curve, subgroup, P, Q and seed;
they never receive the planted logarithm. The measurement runner independently
checks the returned k by [k]P=Q. Register new methods and extension-field
instances in ecbench; run fresh ecbench.spec/v1 sessions and replay all
deterministic records. Common fixture generation and order certification
are recorded separately from cold attack setup. Charge factor-base building,
root finding, projection, transfer, walk initialization, relation collection,
linear algebra, fruitless cycles, retries and verification.

Primary observations are exact field/group/native-operation counts, not wall
time. Use the same field implementation for source and destination. For odd
quadratic extensions, expand products, squares and inversions into F_p
operations. Binary arithmetic uses its explicitly identified ambient field
unit. Report both field cardinality bits and actual subgroup-order bits.
Retain native additions, coefficient arithmetic, RNG draws, graph operations,
polynomial operations and hash/table probes as explicit counters.

Price I/M in {1,10,100,1000} and S/M in {0,0.5,1}: twelve arithmetic cells.
Apply unpriced native-operation charges {0,0.1,1} source-addition equivalents
to every arm, including rho. A ranking that flips is unresolved. Record
cold and target-dependent costs separately; no reused setup is free.
No wall-time ranking without the repository's isolation and A/A gates.

## Lead 1: subfield summation-polynomial decomposition

Fresh binary coverage cohort: q=8,16,32,64,128,256,512; three deterministic
ordinary curves per q with b outside F_q, for exactly 21 curves over F_(q^2).
Enumerate actual rational points and complete unordered pair sumsets, retaining
repeats and the identity target. Check the trace homomorphism, doubled image,
full-group and prime-subgroup coverage separately. Retain descended-b controls;
the half-group theorem is not assumed to prove the 3/8 limit.

Odd coverage cohort: p=5,7 and n=2,3,4,5, three deterministic curves per cell,
with 32 same-size sign-orbit/torsion-matched random controls. Retain every
control count. These are exact finite censuses, not a universal equivalence
theorem or a test whose variance is a Poisson formula.

End-to-end DLP: run the n=2 binary cohort and odd quadratic fixtures with
eight independent targets per curve. Compare direct source rho (plain and
negation) with complete index calculus. Implement actual S3-root finding
and the binary coefficient-splitting quadratic solver, verify every lifted
decomposition by group addition, use unknown projected factor-base logarithms
and incremental elimination over the prime subgroup order. Report whole
decomposition calls and individual root calls separately, including setup.
The original 2.7–66 and 1,000M figures are hypotheses to check, not budgets
or facts to reproduce by changing a cost definition.

## Lead 2: prime-field PKM bases and solving

Eighteen cells: Kummer and generic split-Dickson fibers; depths 2,3,4;
prime intervals [4096,8192), [8192,16384), [16384,32768). In every cell,
use the first 23 eligible primes with 2^depth dividing p-1. Check fiber
degree and distinctness independently by enumeration and group generation.
Use actual rational base sizes and 32 matched random controls per curve.
Record rejected/degenerate fixtures; never silently replace a failed run.

Complete DLP comparisons use the same fixed fibers, random-base controls,
S3 root solver, unknown base logarithms and linear algebra on identical
targets and source rho references. Eight targets per eligible curve.
This measures a specific complete native solving route. It does not claim
that all possible Gröbner/tower solvers have been exhausted.

## Lead 3: explicit quadratic-extension/genus-2 transfer

Use the Bernstein–Lange Scholten family (paper sections 2–3):
r=33+56i, s=159+56i, i^2=-1, p=3 mod 4, and
`-384*y^2=r*x^3+s*x^2+conj(s)*x+conj(r)`.
Its sextic has rational branch point z=1. Substitute z=1+1/t and
Y=t^3*y_C to obtain an exact quintic with one rational infinity.
Implement Cantor arithmetic on that model and the pullback-plus-conjugate
map tau from E(F_(p^2)) into J(F_p). Implement the dual cover norm and
verify norm(tau(P))=[2]P, tau(P+Q)=tau(P)+tau(Q), identity and torsion cases,
on-curve/divisor validity, descent to F_p, and order-r preservation.
No large formulas with unhandled denominators may silently skip targets.

Field-size levels: approximately 16,20,24,28,32,36,40 bits for p^2.
Select three deterministic primes per level for which the certified group
order has a usable odd prime subgroup. This gives 21 curves. Sixty targets
per curve give 1,260 paired target workloads. Primary arms: direct plain
rho, direct negation rho, transferred generic Jacobian rho and the signed
double-large-prime index calculus. Each answer is independently verified.
The actual subgroup bits and cofactor are always reported; p^2 bits are
not relabelled as subgroup bits.

The graph collector must distinguish graph cycles, negative signed cycles,
zero-target-coefficient cycles and useful dependencies. Retain the first
graph-cycle position and actual solve position. Independent uniform known-label
trials are an additional control on one target per curve at every size;
their scalar-generation cost is charged in full. Do not equate one such
trial with one walk addition.

Certify larger source orders using a Hasse-interval BSGS witness and exact
prime-factor/order checks. Small instances additionally use exhaustive
point and Jacobian enumerations. Stop a fixture on an arithmetic/map/order
failure and preserve it. Correctness smoke runs are separate from the
frozen primary sessions. Deterministic trial caps and all finalized fixture
identities are written before primary execution. A cap or timeout remains
a failed/exhausted record, not a changed target or an omitted denominator.

## Deliverables and acceptance

Ship native source and tests, explicit model/map derivations and certificates,
frozen fixture manifests/specs, sealed sessions and replay receipts, complete
stage/native counters, twelve-cell and native-charge sensitivity tables,
source-linked Markdown/PDF/SVG/PNG reports, and a row for every original
claim: confirmed with stated scope, contradicted under fresh conditions,
replaced by measured values, or still unresolved with a precise reason.
Update the existing PR and evidence-gap record. Do not mark the task complete
until all three end-to-end paths have executed and their answers passed.
