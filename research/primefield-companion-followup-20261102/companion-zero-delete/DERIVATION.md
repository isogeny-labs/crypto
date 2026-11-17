# Zero-prefix coordinate deletion

2026-11-02. Derived identities and static integration checks. Numerical,
corruption, heap and performance trials: zero. Prime-field source scope unchanged.

## Assumptions and block identity

Let the current square source A have dimension n over the caller-certified
prime coefficient field, with distinct canonical wraps a,b and displacement
Z_a A - A Z_b = G K^T. The diagonal metric D has nonzero entries and
D A = A^T D. The internal selector has checked columns 0,...,j-1 zero and
column j nonzero, where 0<j<n. For t<j, A_it=0 implies
D_tt A_ti = D_ii A_it = 0. Hence those rows are also zero and
A = diag(0_j,T), q=n-j>0. Inherited trailing metric still symmetrizes T.

Let S_q be the nilpotent down-shift (zero wrap), e its first unit vector,
and f its last unit vector. The trailing blocks of both original shifts
are S_q, since j>0. Taking the trailing block of the displacement equation gives

    S_q T - T S_q = G_tail K_tail^T.

Restoring distinct companion wraps at the smaller dimension gives

    Z_a(q) T - T Z_b(q)
      = G_tail K_tail^T + a e (T^T f)^T - b (T e) f^T.

Thus G'=[G_tail, a e, -b T e] and K'=[K_tail, T^T f, f]. Width grows
from w to w+2 before subsequent exact compression. The formula includes q=1:
S_1=0, so the correction becomes (a-b)T. Distinct a,b give unique recovery
under the same companion operator model; deleting coordinates does not require
switching characteristic or elliptic source field.

The selector now retains its first nonzero column even when it chooses a
secondary diagonal pivot elsewhere. Its tail gives T e without a new query.
One transpose query A^T e_(n-1) gives the last row, whose tail is T^T f.
The implementation checks that both retained first column and returned last row
have zero prefix. These are local guards, not an independent symmetry proof.

## RHS and exact lifts

If some b_t!=0 for t<j, y=e_t/b_t satisfies A^T y=0 and y^T b=1.
The driver returns this local witness through all previously accumulated
primal/dual frames and still performs the original root witness check.
No last-row query or reduced operator is built in this branch. The trailing
rank has not been computed: derived_rank is null, rank_lower_bound is the
number of already eliminated pivot coordinates. No full-rank claim follows.

Otherwise b=(0,b_tail). A solution of T x=b_tail lifts to (0,x), choosing
zero for the deleted free coordinates. A normalized trailing witness lifts
by the same zero padding, preserving annihilation and pairing. rank(A)=rank(T),
so deletion never increments pivot count. ZeroFrame retains size, cut, meter
and consumed metadata only; no source, metric, RHS or field array. Both lift
modes use the same exact padding, and reject consumed or malformed input.

## Integration and charged costs

The new driver retains the old zero-source terminal, pivot/Schur path,
metric views, compact block frames and final root equations. After a nonzero
selection with j>0 it checks the prefix RHS. The zero-RHS branch appends a
ZeroFrame, slices RHS/labels/view, installs the q-dimensional operator, then
continues the loop. Consequently it runs exact compression and pivot selection
again on the smaller operator; their costs are not removed or hidden.
The j=0 path uses the same paired transport arithmetic with the extended
selection record. All-zero selection remains handled by the existing zero
source logic. Repeated proper deletions strictly reduce dimension.

Derived local schedule, excluding the already-completed selection:
- Nonzero prefix RHS: at most j RHS tests, one field inversion, an n-entry
  witness allocation, no source query. Existing outer frame/root checks remain.
- Zero prefix RHS: j tests, one transpose source query at original (n,w),
  q field products and one subtraction outside that query; slice/copy 2qw
  generator entries and append 4q entries. Operator construction copies these
  generators again. Source query follows the preserved kernel schedule of
  4w+8 instrumented native products with original dimension-dependent bit sizes.
- Target dimension q=n-j; target width w+2 before recompression. No clock or
  net cost comparison is inferred from the dimension change.
- Retaining first column adds at most n field entries to the selection in the
  secondary-diagonal branch; other branches alias the already-retained tuple.
- ZeroFrame stores two size/cut integers, meter reference and consumed flag;
  lifts allocate j zeros and copy q tail entries. No source or field arithmetic.

The zero-prefix frequency for the public feature family is unmeasured. The
feature compiler still has its existing dimension/setup costs. This is an
integrated conditional reduction, not evidence that every instance shrinks.

## Static checks and required dynamic panel

Three complete AST comparisons against declared substitutions cover selector,
paired transport and driver integration. All Python files parse; a frame AST
audit excludes source and field-arithmetic calls. These checks do not establish
dynamic numerical correctness. Preserve the entire funded upstream cohort.
Add j=0,1,n-1; q=1; first/secondary diagonal and pair search; repeated deletions;
nonconstant inherited metrics; nonzero prefix RHS at every location; zero RHS;
singular and nonsingular tails; alternating deletion/Schur/permutation frames;
exact dense independent displacement and lift checks; final root equations;
rank unknown at early witness; counts, lifetime, malformed/consumed records.

Internal mutable records and sources must not be forged or changed between
selection and use. The new prefix/column checks do not turn caches into public
certificates. If an external cache boundary is introduced, independent
source-column verification is a separate requirement.

Load the sibling paths described in companion-metric-solver/DERIVATION.md,
plus companion-metric-views and this directory. Import driver_delete.solve_or_witness.
No frozen reference module is replaced. The existing cost gate still prevents
numerical trials and new builds. Next remove repeated selection by transporting
the existing selected columns into the deleted coordinate system, while
binding them to the new operator and preserving compression provenance.

## Prior work consulted

Bostan, Jeannerod, Mouilleron and Schost (2017), introduction and equation (2):
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf.
Uses the standard Sylvester displacement and cyclic down-shift conventions;
no randomized regularization or paper-wide complexity bound is inherited.
Local sources: companion-zero-prefix/{selection_zero,paired_zero,driver_zero}.py,
companion-recursive-inverse/recursive.py and companion-metric-views/metric_views.py.
