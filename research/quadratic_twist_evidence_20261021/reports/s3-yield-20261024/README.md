# Actual twist useful yield, S3 degrees and source-DLP transport

2026-10-24 · `s3-yield-20261024` · PR #2381.

[PDF/large-asset upload pending](../../validation/s3-yield-20261024/ARTIFACTS_UPLOAD_PENDING.json) · [editable report](report.tex) ·
[degree/rank graph](profiles.svg) · [transfer/control diagram](bridge.svg) ·
[protocol](../../S3_YIELD_20261024_PROTOCOL.md) ·
[native records](../../native/s3-yield-20261024/) ·
[validation, failures and exact commands](../../validation/s3-yield-20261024/README.md).

Publication status: the four-page PDF was rendered and inspected locally.
Its upload, the PNG copies and raw run archive are held: automatic approval
review rejected sending this specific payload to the repository's S3 store
without explicit user authorization, citing possible non-public research data.
No upload was performed or retried. Exact paths/hashes/bytes and approval state
are in the linked pending manifest. Source, SVGs, small figure PDFs, compact
canonical cases/certificates and proof receipts are committed for review.

The requested next step was an actual twist system with better useful yield
and a lower relevant degree, plus a verified reduction for the original DLP.
There is a **finite, target-dependent positive example**, and a precise limit:
the gain is reproduced by a different factor base on the original main curve.
It is not a lower homogeneous regularity caused by twisting or a measured speedup.

## Actual example and useful rows

Work over F43 with main E:y^2=x^3+x and its d=2 monic twist
T:Y^2=X^3+4X. The already registered models are
`icv1-fp6-t0-74779950` and `icv1-fp6-t0-c6a8a47c`.
Both have order44; use r=11, P=(31,18) and U=(24,26).
The explicit **base-field** map

`phi(O)=O; phi(x,y)=(41*x,11*y)`

is additive and injective on actual Mathlib nonsingular points. It sends P to U.
`S3System.F43.scalar_reduction` proves, for arbitrary actual points A,B and
integer n, `phi(B)=n*phi(A) iff B=n*A`. It is not defined by a log lookup.
This j=1728, trace-zero example is a quadratic twist that is also base-field
isomorphic; this map is not asserted for arbitrary twists or ordinary curves.

The frozen policy selects the first two subgroup x coordinates in each arm,
then the least y point above each selected x:

| Quantity at the first public precomputation target | Main | Twist |
| --- | ---: | ---: |
| Factor-base points | (4,5),(13,19) | (15,9),(17,6) |
| Target | P=(31,18) | U=(24,26) |
| Ordered S3 root pairs out of four x pairs | 0 | 3 |
| Distinct accepted coefficient rows modulo11 | 0 | 2 |
| Rank increase for the two unknown base logarithms | 0 | 2 |
| Finite vanishing-space closure degree | 6 | 4 |
| Degree first deriving 1 | 5 | absent: satisfiable |
| Homogeneous leading-ideal first-zero degree | 3 | 3 |

The twist rows are `l0+l1=1` and `-2*l0=1 (mod11)`. Their determinant is
-2, which is nonzero modulo11; they recover `(l0,l1)=(5,7)`. Lean checks both
actual point relations, their determinant and the uniqueness of these logs.
It also checks that none of the main base's sixteen signed ordered words
decomposes P. The three ordered polynomial roots contain two independent rows,
not three independent relations.

The explicit native precomputation enumerates all sixteen signed words per
target and tests rank after each complete target. The main base reaches rank2
after three public targets (48 signed candidates); the twist base after one
(16 candidates). Thus both batches obtain two independent rows, from48 and16
signed candidates respectively. These are exact finite counts, not timings or
probabilities inferred from a population model. A PDP call can return multiple
rows: “two rows in one call” must not be called a probability of two.
The original conditional cost theorem is not instantiated with an invented
positive main first-target yield; that first-target main yield is zero.

## Actual systems and what the degrees mean

For each affine R=nP, n=1,...,r-1, the system is precisely

`S3_a,b(x,y,x(R))=0, g_B(x)=0, g_B(y)=0`,

where `g_B(t)=product_{u in selected x-base}(t-u)`.
All roots are lifted to actual signed point decompositions; repeated x's are
included. These are prime-field systems with explicit factor-base constraints,
not unperformed Weil-descent systems.

The native audit forms **every monomial multiple of each input of total degree
at most D**, D=max(4,|B|),...,12. Let M_D be their coefficient row space, V_D
the space of all degree<=D polynomials, and ev_D evaluation on every actual
root pair. Each row vanishes on those roots. The recorded closure criterion

`rank(M_D)+rank(ev_D)=dim(V_D)`

therefore certifies `M_D=ker(ev_D)`. The first such D in the frozen scan is
the reported **finite vanishing-space closure degree**. It is an exact property
of these input polynomials and this degree filtration. It is not identified
with a particular F4/F5 solver's degree, Castelnuovo–Mumford regularity, last
fall degree, or runtime. All1,904 target systems, including controls, close
within the cap; no cap value is substituted for a miss.

For the first main target, the degree4 matrix has rank12 in15 columns, while
there are zero roots. A source-bound explicit dual functional annihilates all
thirteen degree4 input multiples but maps1 to1. Lean proves
`S3DegreeFixture.no_degree4_refutation`. At degree5, an explicit polynomial
combination derives1, checked by `degree5_unit_certificate`; its universal
cleared integer identity is checked separately. Thus the first refutation
degree in this filtration is5. The main closure criterion is stronger here
and first holds at6. The twist's degree4 matrix also has rank12 in15 columns,
but has three independent root evaluations, so closure already holds at4.
Do not replace the main's refutation degree5 with its closure degree6 when
describing the work needed merely to reject that target.

In contrast, the homogeneous leading-form ideal is **identical** at fixed
factor-base size b=2 or3:

`I_top = < x^2*y^2, x^b, y^b >`.

Lean proves the actual S3 leading component, the leading components of the
two- and three-root base polynomials, and equality with this actual monomial
ideal. Its standard monomials satisfy i<b, j<b and (i<2 or j<2). There are
surviving degree-b monomials and none in degree b+1. The native Hilbert vectors
are `[1,2,1,0,0]` for b2 and `[1,2,3,2,0,0]` for b3. The first-empty degrees
3 and4 are sometimes called the input system's homogeneous degree of
regularity; the exact definition is stated here to avoid confusing them with
solver degree. **No strictly lower value occurs on a twist in this encoding.**
This is a proved obstruction for this encoding, not for all IC encodings.

## Pullback control and complete target sequence

Applying phi inverse to the twist base gives main points `(14,36),(13,24)`.
They produce the same two independent rows at P and the same degree4 closure.
`S3DegreeFixture.pullback_images`, `pulled_back_relation_sets` and both explicit
pulled-back relations are kernel checked, including the target as a third column.
For every finite integer coefficient candidate set, these transported accepted
coefficient sets are equal. Native replay checks exact coefficient rows, rank
progression and every matrix-degree profile in **both** transport directions.

Across all ten nonzero targets in the primary F43 comparison, both arms have
total closure degree52, hence mean52/10=5.2, and final relation rank2. The twist
has worse closure degree at targets4,5,6,7. The graph includes every target.
All twenty size2/size3 subset policies are retained, including ties and losses;
there are six first-target comparisons with both greater rank and lower closure
degree, all in this additional fixture. Only one comparison improves rank after
the preregistered first three targets. These are finite rank states, not a
stationary independent-row probability or a uniform advantage.

The independent Rust toy IC solver does not read the recorded scalar labels.
It constructs signed point relations, solves for base logs, then uses lookup
or signed-pair descent with charged target shifts. It recovers and checks all
eleven main targets through phi and independently with the main base. This is
eleven separate synthetic correctness cases; it supplies no one-target timing,
amortized cost, rho comparison or external-fastest result. All setup, failed
target attempts and transfer costs would have to be measured before a speed claim.

## Population, proofs and reproduction

The full942-model population over p=5,7,11,13,17,19 has274 shared odd-prime
subgroup cases. Of these,186 have r3 and cannot hold two distinct nonzero
subgroup x coordinates. The other88 yield104 size2/size3 prefix comparisons.
The additional F43 fixture supplies twenty subset comparisons. The total is
**124 comparisons /1,904 target systems**, including272 transported and272
inverse-transported controls. Eighteen population cases and the additional
fixture have the tested base-field scaling. No such reduction is inferred for
the other cases. A shared prime alone does not construct an efficient DLP map.

The C++ producer reuses unchanged full.cpp curve arithmetic. Rust independently
enumerates prime-field points with coefficient arithmetic and a linear addition
chain, builds S3 via a separate coefficient table and symmetric evaluation, and
uses batch RREF in a different monomial order. C++ uses incremental echelon
insertion. Both reconstruct every row/rank/degree; false rank, false degree,
omitted case and false scaling controls are rehashed and reject exit101 with
no audit receipt. This tests semantic checks beyond payload hashes.

| Verified claim | Lean declaration(s) |
| --- | --- |
| Actual S3 twist scaling | `S3System.twist_scaling_identity` |
| S3 and base leading forms | `s3_leading_component`, `quadratic_base_leading`, `cubic_base_leading` |
| Actual leading ideals | `headIdeal_generators`, `size2_leading_ideal`, `size3_leading_ideal` |
| Exact monomial-ideal cutoff | `monomial_member_iff`, `actual_head_ideal_cutoff` |
| Concrete scalar reduction | `S3System.F43.toTwist_injective`, `source_image`, `scalar_reduction` |
| Actual useful/nonexistent rows | `main_first_target_has_no_decomposition`, `twist_first_relation`, `twist_second_relation` |
| Independent rows/logs | `independent_row_determinant`, `recovered_base_logs` |
| Lower/upper refutation certificates | `S3DegreeFixture.dual_rows`, `no_degree4_refutation`, `degree5_unit_certificate` |
| Exact pullback relation sets | `pullback_images`, `pulled_back_relation_sets` |

There are38 new named statements, bringing the study to **144 named research
theorems /185 audited declarations** in twelve modules, zero failing/extra
trusted axioms. The41 other audited declarations include generated helpers.
Only propext, Classical.choice and Quot.sound occur. Lean4.34.1 and Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612` remain pinned. The unchanged gate passes.
The native source-bound polynomial records are **IDC1hf15c17737ed713d5** (S3
scaling) and **IDC1hcee4710ff9dcf7e7** (cleared degree5 unit identity), each32+16
points with false-plus-one and false-source controls. They are polynomial checks;
Lean separately checks characteristic43, actual points, the dual and reduction.
The previous ten study certificates are unchanged.

```sh
tools/lean/proof-gate.sh --out /tmp/twist-s3-proof research/quadratic_twist_evidence_20261021/lean
bash research/quadratic_twist_evidence_20261021/run-s3-yield.sh /tmp/twist-s3-fresh
cargo test --release --locked --lib --quiet
python3 -m unittest scripts.test_workflows_parse
```

The exact new CI command passes locally, including22 backend tests and both
certificates. The mandatory full library suite passes4,479 with119 existing
ignores; the initial loopback-denied run is retained. Four existing workflow
metadata tests pass. Python only queries existing atlas/YAML metadata and lays
out already recorded values; no Python arithmetic, relation harness or verifier
is introduced. The native commands respect600-second caps; there is no timing result.

## Remaining goal and preservation

The finite useful-row/closure example and actual F43 source-DLP reduction are
verified. A **strict homogeneous regularity advantage is refuted for this
S3/base-polynomial encoding**. A uniform useful-yield gain, a genuinely improved
regularity/solving-degree bound for a different system, and compatible complete
work bounds remain unresolved. To progress, specify a different actual
descent/rationality encoding whose leading ideal can change, retain a verified
subgroup reduction, and distinguish the improvement from ordinary factor-base
choice by a pullback control. That is a construction/proof obligation, not a
currently running job or a universal impossibility claim.

The new U=(24,26) is recorded explicitly with the existing model, subgroup and
map. It has no registered EC1 representation in the current registry; the old
twist EC1 record has generator(35,12) and is never reused as this generator's
identity. No cross-repository performance comparison or new identity registration
is claimed. Registry and cover inputs remain byte-identical. Both catalog
provenance dependencies at a11984a573441d5bf0d60ab1d26cfc34e71badf5 and
228cdc0d50f2e4a2058f6fb93d6cc7f535e2692d remain intact.

Earlier sources, certificates, historical Python evidence, failed attempts,
frozen reports and the positive pairing route are preserved. The lead-atlas
match was run before this experiment; its handoff retains existing dispositions.
Canonical cost/leaderboard/progress graphs, benchmark rows, cover diagrams,
Jacobian inventory and conflicted generated/claim files are unchanged because
this round adds no admitted performance point or new transfer-catalog entry.
The new local profiles and source-linked report include the negative controls.
Large-asset S3 publication remains an explicit additional hold.
General geometry, practical cost, exact-head CI/review, merge/integration and
actual shared Conductor gates remain open. Conductor still exits127; no shared
reservation or local replacement coordinator is claimed.
