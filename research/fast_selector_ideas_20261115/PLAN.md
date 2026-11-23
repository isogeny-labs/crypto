# Three hypotheses for a prime-field fast selector

15 November 2026. Status: research proposals and hand reformulations, all
unverified. No selector implementation, numerical experiment, timing or new
Lean build is reported. The requested deliverable is new ideas; the retained
long-term acceptance criterion is a complete classical subexponential ECDLP
algorithm. Completing this plan does not discharge that criterion.

The first candidate to investigate is colored rational interpolation. It has
an explicit witness, a small coefficient description and direct final checks.
The second asks how to batch its two-point completion step. The third is a
different search architecture based on redundant, correlated representations.
These are new directions for this thread, with no literature novelty claim.

## Input, accounting and desired contract

Use a nonsingular E: y^2 = g(x) = x^3 + Ax + B over F_p, p > 3, and a
known odd prime-order subgroup G of order r. Do not infer curve/subgroup
identity from bit length. The m colored lists each contain b independently
sampled mixed-label records R_ij = [a_ij]P + [b_ij]Q. Labels remain hidden
from selection. Reject infinity and repeated abscissas by the existing
point-only cleanliness rule. Return one actual record per color, with signs
if required, check its sum on E, recover the scalar and verify it on E.

Write n = ceil(log2 r), N for full input length, and B0 = mb. The existing
balanced family has m = max(3,ceil(sqrt(n))), b = ceil(r^(1/m)). A concrete
research target is a known per-attempt cap and useful-answer floor of the
form

    T <= 2^(C*m*log2(m+1)) * B0^c * N^c,
    theta >= 2^(-C*m*log2(m+1)),

for absolute effective constants. These are proposed targets, not bounds
already obtained. Charge preprocessing, branch states, failed attempts,
field bit operations, auxiliary records, output expansion and recovery.
Neither a small witness nor a compact circuit establishes this contract.

The previous source-pinned assessments are REPORT.md, SELECTOR-JOINS.md and
INTERVAL-JOINS.md at [204e6f7602](https://github.com/isogeny-labs/crypto/tree/204e6f7602f7ca5ad5ecade0fa54d78a29c7e5f4/research/single_relation_reduction_20261115).
Their small fixed-slope and independent-uniform interval models do not analyze
the three algorithms proposed here. Their exact assumptions remain intact.

## A. Colored rational interpolation through a norm polynomial

For each color, preprocess the known coordinates into

    F_i(X) = product_j (X - x_ij),
    Y_i(x_ij) = y_ij,  deg(Y_i) < b.

Seek a nonzero function f = u(x) + y*v(x) in L(mO), with

    deg(u) <= floor(m/2),
    deg(v) <= floor((m-3)/2),
    deg gcd(F_i, u + Y_i*v) >= 1  for every i.

The candidate norm is H = u^2 - g*v^2. Its degree is at most m. With clean
abscissas and the ordinate conditions above, m selected zeros would account
for the full zero divisor of f. The standard genus-one divisor criterion
then identifies their group sum with O. Conversely, a positive zero-sum
tuple has a principal divisor with pole mO. This is a hand reformulation
using classical divisor theory, not an independently verified new theorem.
The norm equation alone loses the ordinate/sign choice; retain the gcd
conditions or explicitly recover and check the signs. [pcc:unverified]

Proposed search: keep the color constraints factorized and merge constraints
using shifted approximation modules / rational interpolation. Branch on
coefficient or pivot conditions only when required, memoizing equivalent
states. Compare this with directly searching for a colored singular minor
of the m-by-B0 evaluation matrix of L(mO). No claimed state compression
algorithm currently accompanies this proposal.

The missing lemma is a bound on the number and degree of retained states
under the actual random colored input law. A method needing B0^m states,
enumerating field coefficient vectors, or constructing the full quotient
algebra fails the desired contract. Worst-case elliptic-code decoding
hardness gives no license to assume a general efficient decoder; equally,
it does not settle this particular random-instance target.

First discriminating work: write the exact module state and update rule;
derive a worst-case cap and an actual-law useful-yield statement before any
performance run. A finite implementation could later check the witness
equivalence against exhaustive colored tuples, including wrong-ordinate and
nonzero-function controls. Such a check is pending, not performed here.

## B. Batch the two-point completions of many partial divisors

For an anchor divisor D consisting of m-2 selected records, consider the
two-dimensional space L(mO-D). A basis f0,f1 gives a projective key

    key_D(R) = [f0(R) : f1(R)].

On the remaining two colors, matching keys is the proposed completion
interface. Work projectively and handle zeros of f1. The colors are disjoint
from the anchors; fixed zeros are removed in interpreting the degree-two
linear system. Checking D+R+S ~ mO gives the final exact relation. This
standard pencil interpretation is a hand reformulation. [pcc:unverified]

An explicit single-anchor hash join is a baseline, not the proposed advance.
The unconditioned expected number of completions for an independent anchor
is b^2/r. Enumerating anchors until a collision therefore leaves the
principal search burden. In particular, an efficient basis update does not
make the number of anchors free. [pcc:unverified]

Proposed algorithmic question: can a factorized family of anchor divisors
share basis updates and collision tests through modular composition or a
structured Schur-complement representation? The batched object must report
one witness without expanding all its anchor sums or all its projective
keys. Simply renaming the partial-sum set as a polynomial is insufficient.

First discriminating work: exhibit a batch containing exponentially many
implicit anchors, its exact representation/update/query operations, and a
bound on every expansion. Compare against the explicitly enumerated
single-anchor baseline. If the batch description grows with its expanded
anchor count, this version supplies no selector cap. This branch is closely
related to A, but attacks the amortization step rather than the decoder.

## C. Redundant representations with canceling auxiliary shifts

Keep the original independent input law. Add a charged pool of independent
mixed-label auxiliary points. For a partial combination X+Y, represent it
as (X+T)+(Y-T), with T selected from that pool. Place such canceling choices
inside the join tree and allow multiple representations of the same original
relation. Apply coordinate filters to the adjusted children, rather than
assuming independent interval samples retain old filters automatically.

Every state retains its full sparse coefficient vector on the primitive
input/auxiliary records. After cancellation, a valid answer must meet the
original colored-record contract and have a nonzero vector modulo r.
Derived local identities alone are not useful answers. Preserve the
point-only selection rule throughout; do not plant a closing point with
inherited labels and call it an independent input.

The proposed mechanism is representation multiplicity combined with
curve-specific algebraic batching. Generic group additions and more random
representations alone provide no cost argument. Rebracketing alone also
does not provide an arbitrary supply of distinct useful representations.

For Y equal to the number of surviving representations of a fixed useful
relation, target an effective second-moment estimate

    E[Y]^2 / E[Y^2] >= 2^(-o(n)).

This is a required lemma, not a demonstrated estimate. Charge the storage
and construction needed to obtain those representations. Common auxiliary
choices can create strong correlations: never replace this obligation by
an independent-trials formula from the representation count alone.
Specify conditioning on a clean input and on selector coins before taking
these moments.

First discriminating work: specify a four-leaf tree completely, expand its
formal record vectors, and derive its exact survival count and second
moment. State which operation exploits the field-coordinate equations.
The generic-only control and all-formally-zero outputs must remain visible.
Only after this survives should a deeper tree or implementation be planned.

## Prior art and access receipt

KB unavailable: no loaded research-kb connector; scripts/research-kb status
reports no executable. Fallback: the three exact repository assessments
above, targeted primary-source web queries and bounded primary reading.
Queries covered elliptic-code minimum distance, ECDLP/list decoding,
representation-based birthday searches, and rational-interpolation recovery.

- [Cheng, Hard Problems of Algebraic Geometry Codes](https://arxiv.org/html/cs/0507026),
  sections 2-5, especially Theorem 1 and its divisor argument, and the
  ECDLP reduction in section 4, were read. They identify the coding connection
  and its worst-case difficulty. The old preprint has historical statements;
  do not promote them to a current unconditional time lower bound.
- [Zhang--Liu, Solving ECDLP via List Decoding](https://doi.org/10.1007/978-3-030-31919-9_13):
  publisher metadata/abstract were read. The ePrint PDF at
  https://eprint.iacr.org/2018/795.pdf returned 403 through both web and curl.
  The full algorithm/cost analysis was not read; the coding connection is
  therefore explicitly prior work, with comparison still pending.
- [Delaplace--May, representations over F_p^2](https://eprint.iacr.org/2019/800):
  indexed primary abstract/excerpt identifies a quadratic-field scope.
  Full ePrint access returned 403; the author-hosted PDF returned 404 and
  CIRM sources failed. No complexity claim is borrowed or transferred to
  prime fields, and no detailed novelty audit is asserted.
- The previous Wagner, fixed-slope and interval receipts remain at the
  source-pinned study. No general exclusion of adaptive/correlated lists
  is inferred from the earlier uniform-input calculation.

## Deliverables and next action

| Requirement | Status/evidence |
|---|---|
| Develop non-tensor selector ideas | Three proposed interfaces above |
| Keep prime-field, hidden-label contract | Common input and provenance rules |
| Identify the absent complexity step | A: state count; B: batch expansion; C: moments/cost |
| Literature and uncertainty | Primary reading/access receipt above |
| Make it reviewable | This plan, editable diagram, report source and PDF |
| Implement or establish a fast selector | Pending; no such result claimed |
| Complete end-to-end algorithm | Original acceptance criterion remains open |

Start with A's exact approximation-module state and its merge rule. B is the
first structured subproblem to try inside that state. C is an independent
alternative if the polynomial/module representations grow too fast. No
numeric, native, paid or qualification run is authorized or launched by
this ideation pass. Existing frozen source, controls and raw evidence stay
untouched. Canonical measured charts are unchanged because this round adds
only proposals. Formalization of a new central
lemma belongs after an exact candidate statement is written, and is pending.

Publication is separate from PR4311 because its later automatic merge
changed the frozen PCC prefix. See publication-observation.json for source
pins and hashes; do not rechain that ledger or replace frozen receipts.
