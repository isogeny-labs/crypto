# Restricted pivot reuse through checked compression

2026-11-02. Derived construction and static integration only. Numerical,
corruption, heap and performance trials: zero. Original prime-field scope remains.

## Preserved pivot decision

The selector found first nonzero column j of A and cached one or two selected
columns. Proper deletion yields A=diag(0_j,T), q=n-j>0, with zero prefix RHS.
For each selected index i, T e_(i-j) is exactly the tail of A e_i. The first
column of T is the nonzero tail of original column j. Its diagonal entry and
the first nonzero offdiagonal entry are unchanged after subtracting j from
indices. Thus the deterministic selector on T chooses exactly the cached
scalar/pair indices shifted by j. This covers the secondary-diagonal branch:
its selected i need not equal j. Retain both its selected column and column j.

The prior driver recompressed T and then queried these columns again. The
candidate performs the same compression, checks its factorization certificate,
then rebinds the restricted selection to the compressed operator without a
source query. It continues into the unchanged paired transport and Schur path
in that iteration, with size, metric labels and view updated to q. Previous
ZeroFrame, early witness, rank-unknown semantics and final root checks remain.

## Compression equality and scope of the check

The preserved independent certificate checker verifies G=UR, H=KR^T,
H=VS, G'=US^T, K'=V, shapes/canonical entries and rank minors. Consequently
G'K'^T=GK^T. At fixed dimension q and distinct wraps a,b, the characteristic
polynomials X^q-a and X^q-b are coprime. The Sylvester displacement operator
is invertible, so equal displacement and wraps imply the same represented T.
Only after this verification does the helper construct the new operator and
its bound Selection. A zero reported displacement rank is rejected because
the retained first column is nonzero. No dense displacement is formed.

The checker is copied byte-for-byte as compression_certificate.py to avoid
ambiguous imports from other sibling certificate modules. This turn does not
execute it on numerical instances: the claim here is implementation of this
conditional runtime gate plus static preservation of its source.

Deletion and cached-column provenance remain internal assumptions. Guards
check consumed selection, successful deletion, dimension, prime/meter/wrap
identity, source signature, canonical column shapes, zero prefix, nonzero
first-column tail, distinct shifted pivot indices and matching metric view.
They do not independently prove the deletion identity or certify forged caches.
The deletion receipt reserves a one-use selection_rebound flag before arithmetic;
a second use of that same receipt is rejected. Deliberate object/dictionary
forgery or concurrent mutation remains outside this internal API contract.

## Derived schedules and costs

Let q be restricted dimension, s its input generator width (previous width+2),
g the first factorization rank and t the final compressed generator width.
The compression producer is unchanged. The additional checker performs four
skinny-product identity checks:
- G=UR: q*s*g products;
- H=KR^T: q*g*s products;
- H=VS: q*g*t products;
- G'=US^T: q*t*g products.
Total logical integer products: 2*q*g*(s+t). Modular reductions:
q*(s+2*g+t). These are plain Python checker operations, separate from Meter
field arithmetic and the producer's schedule. Derived expected counters and
receipt fields are explicitly labeled; actual checker runtime counts are not
verified. Shape/minor/canonical/rank tests and logical sums also remain work.
No inversions or source queries occur in this checker.

Removed reduced-selector source queries: 1 when the first diagonal is nonzero;
2 for secondary-diagonal or pair pivots. At compressed width t, the preserved
source kernel schedule assigns 4*t+8 instrumented native products per query.
Thus removed kernel products are v*(4*t+8), v in {1,2}, with q-dependent
operand bit lengths. The offdiagonal selector branches also avoid their two
reciprocal-consistency field products; the first-diagonal branch had none.
No net runtime improvement follows from comparing these heterogeneous units.

Native work includes full cached-column validation, zero-prefix scans, shifted
indices, view matching and allocation. The helper copies (k+1)*q cached field
entries for k selected columns plus the first column, and slices two q-entry
metric arrays. Compression certificates retain their existing skinny matrices.
No total-heap claim is made. Prior search queries remain in the deletion trace;
the rebound selection has source_queries=0, with avoided queries separately
recorded. No source call has been relabeled as free.

## Validation and next work

Two full declared-substitution AST comparisons cover byte-identical checker
copy and complete driver integration. A static helper audit excludes source
apply calls and checks verification precedes Operator/Selection construction.
All Python sources parse. These checks do not execute the runtime gate.
The existing cost admission still prevents numerical trials and new builds.

The funded panel must retain full prior cohorts and controls. Add exact
reference/candidate pivot sequences after deletion; first/secondary diagonal
and pair; j=1,n-1 and q=1; nonconstant metrics; alternating deletion/Schur
frames; both RHS outcomes; mutation of each certificate factor, output and
rank; malformed/consumed receipt and mismatched sources; exact root equations;
query widths/counters; aliases and lifetimes. A valid modified factorization
representing the same displacement must remain acceptable.

Load sibling module paths from companion-zero-delete/DERIVATION.md plus this
directory. Import driver_reuse.solve_or_witness. Original modules and manifests
remain unchanged. Next examine whether the independently checked compression
boundary can be shared with other consumers, avoiding duplicate certificates
while preserving the exact equality guarantee. Also assess zero-prefix
incidence on the actual public feature family when execution is funded.

## Sources reviewed

Local companion-generator-compression/{compress,certificate}.py and
DERIVATION.md; companion-zero-delete/{deletion,driver_delete,selection_delete}.py.
Standard displacement injectivity and cyclic-shift conventions:
Bostan et al. (2017), introduction/equation (2),
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf.
No new minimal-rank theorem or paper-wide fast complexity bound is claimed.
