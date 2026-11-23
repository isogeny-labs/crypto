# Compact diagonal of the actual character-feature Gram matrix

2026-11-02. Derived identities, implementation and static checks; numerical,
corruption, heap and performance trials: zero. No frozen weights are modified.

## Exact source family

Use the public compiler in reference_constructor.py, copied byte-for-byte from
direct-cyclic-companion/generators.py. Coefficient prime r>3, 1<=q<r,
gamma in F_r minus {0,1}, all feature weights nonzero. The patterns are (0),
(0,d) and (0,d,e), with 0<d<e<q. Write ell_P=q-max(P). The symmetric Gram is

    B = w_empty 11^T + sum_P w_P sum_(a=0)^(ell_P-1)
                                      gamma^a v_(P,a) v_(P,a)^T,
    v_(P,a)(k) = product_(d in P) chi(k+a+d).

The companion source is A=D^-1 J B J, with D_ii=gamma^i. Thus
A_ii=gamma^(-i) B_(r-1-i,r-1-i); diagonal zeros are unaffected by the scale.
This component compresses the diagonal, not the entire operator or extractor.

## Constant background and q corrections

For the quadratic character, chi(x)^2 is 1 at nonzero x and 0 at x=0.
For fixed k,a, the offsets d are distinct modulo r, so at most one factor is
zero. Consequently the squared feature is 1 minus the sum of its zero
indicators; no inclusion-exclusion cross terms occur. Put z=-k mod r and

    S_ell = (gamma^ell-1)/(gamma-1),
    H = sum_P w_P S_(ell_P), C = w_empty + H,
    Delta_z = sum_P w_P sum_(d in P) gamma^(z-d) 1_[d,d+ell_P)(z).

Every interval lies in [0,q), since d<=max(P). Therefore

    B_kk = C-Delta_z   if 0<=z<q,
           C           otherwise.

The diagonal has one constant background and at most q exceptional positions.
For A, exceptional coordinates are r-1 and 0,...,q-2; the bulk interval is
[q-1,r-1) in half-open indexing, of length r-q. This includes q=1 correctly.

## Geometric interval-event construction

Scale Delta_z by gamma^(-z). Each (P,d) contributes the constant
w_P gamma^(-d) throughout [d,d+ell_P). Add that value at d and subtract it
at d+ell_P in an event array of length q+1. Prefix-sum the array and multiply
coordinate z by gamma^z. This recovers every Delta_z without computing any
quadratic character or enumerating the r evaluation points.

Patterns are streamed in the exact original weight order. Powers and inverse
powers need only length O(q). With

    M = 1+q(q-1)/2 patterns,
    L = 1+2(q-1)+3(q-1)(q-2)/2 pattern-offset incidences,

the implemented build schedule is exactly:
- field products: 3q-1+2M+L;
- field additions/subtractions: 2+2M+2L+q;
- field inversions: 2;
- interval boundary updates: 2L.

These are derived schedules, not measured counters. Build time is O(q^2)
field operations, plus O(q^2) native input/pattern validation. Arithmetic
workspace arrays are O(q); caller-owned O(q^2) weights and Meter history are
separate. The returned record has q correction field entries plus gamma inverse,
H and C, with prime/window metadata. No r-entry array is allocated by build.
A B-diagonal lookup uses at most one subtraction; an A-diagonal value also
needs the metered binary power gamma^(-i) and a product. Zero-diagonal candidate
locations use q exceptional indices and at most one bulk interval, never an
r-entry list. They are necessary candidates for zero columns, not certificates.

## Exact implications for zero prefixes and weights

If C!=0, every bulk diagonal entry of A is nonzero. Thus every zero column
must lie among the q exceptional coordinates, and a root zero-column prefix
has length at most q-1: index q-1 already has nonzero diagonal. If C=0, bulk
diagonals vanish, but this alone proves neither zero columns nor singularity.
Even all nonzero diagonals do not prove nonsingularity (the all-ones matrix is
a familiar counterexample). These are root statements for this public family.

Hold gamma and all nonconstant feature weights fixed. The nonzero constant
weights that make any diagonal zero are exactly the nonzero members of

    F = {-H} union {Delta_z-H : 0<=z<q}.

There are at most q+1 distinct values. If q<=r-3, at least one allowed nonzero
constant weight avoids all diagonal zeros. The diagnostic returns F only;
it neither chooses a weight nor changes any frozen input. Avoiding -H alone
makes C nonzero and supplies the prefix bound. For the first companion diagonal,
there is at most one forbidden nonzero constant weight: Delta_1-H if q>1,
and -H if q=1. Under a hypothetical uniform nonzero constant weight, its
zero probability is either 0 or 1/(r-1); this is a conditional counting result,
not an empirical frequency or an assertion about the frozen protocol.

## Schur boundary and next use

For a principal block partition B=[E F; F^T H_tail] with invertible E,
S=H_tail-F^T E^-1 F. Its diagonal at coordinate i is
B_ii-B_iI E^-1 B_Ii. The second term need not share the interval-event form.
Do not pass this root diagonal record to a Schur child, or infer future pivots
from initial nonzero diagonals. Source identity, permutations and scaling must
be bound explicitly before a selector consumes such a record.

Next implement a root-only pivot proposal from this compact diagonal with
constructor provenance, preserving fixed weights and a full fallback when all
diagonals vanish. Separately investigate structured updates for Schur diagonal
corrections. No solver integration or weight substitution is included here.

## Checks and pending execution

Static audit verifies byte-identical reference constructor, exact original
pattern-expression AST, no field-size range loop or character/source call in
build, and Python syntax. These checks do not validate arithmetic correctness.
independent_check.py supplies a separate direct character-feature evaluator
for explicitly supplied finite points; it is implemented but unexecuted.
It checks diagonal values only, not source columns or Schur closure.

Funded validation must retain existing cohorts and add q=1,r-1; gamma of
small order; coincident/zero event sums; C=0/C!=0; z=0,q-1,q boundaries;
all r points on designated toy fields; complete direct-compiler diagonals;
weight-order corruption; claimed schedule counters; compressed zero-location
sets; forbidden-weight diagnostics; and an explicit Schur non-closure control.
No numerical job, sweep or new build was launched under the existing cost gate.

## Sources reviewed

NIST DLMF 27.9, definition of quadratic character values:
https://dlmf.nist.gov/27.9 .
Local direct-cyclic-companion/DERIVATION.md and generators.py specify the exact
weighted feature family and A=D^-1 J B J mapping. The interval-event formula
is derived here from those definitions; no claim of a new character theorem.
