# Beyond the repeated-source kernel: exact Boolean-cell support certificates

UNEXECUTED new frontier hypothesis; no source or measured evidence is changed.
The coordinator reports the cached-cubic profile passed65 DISTINCT conditional
cells and1040 candidate k, with zero supported parameters. Reported full-batch
medians were6.9009s generic-per-k,2.76224s cached-generic,2.4367s explicit-cached:
2.832x against the old generic component. Small controls reportedly covered665
cubics and144 positives. These are coordinator-supplied component observations,
not author reruns, global all-roots closure or matched F4/F5/F6 gains.

Single-double source patterns have affine-centroid rank at most3; five distinct
sources can have rank4. Heuristically, an83-bit target with five18-bit source
choices gives2^90/2^83 raw expected mass, whereas one collision leaves at most
four free source choices,2^72/2^83 before combinatorial/lift constants. This is
NOT a distribution proof, exact count or measurement. It explains why repeated
strata may often be empty and why further empty-chart kernels cannot establish
the desired hard all-roots result. The next experiment targets the full quintic
support condition on the actual73-bit relaxed norm family, including rank4.

## Prior-work overlap examined before proposing the changed mechanism

Read research/actual_binary_jet_rank_20261015_v1/PROOF.md,
actual_binary_jet_information_bounds_20261015_v1/PROOF.md,
binary_newton_cell_cut_v1/PROOF.md, actual_norm_cell_20261028_v2/PROOF.md,
coupled_line_cells_20261029_v1/PROOF.md and odd_support_cell_20261028_v1/PROOF.md.
The old jet method already couples SAME73 bits and SAME three-bit lift quotient
across places; rank saturation alone is not exclusion. Its precision4 bounds
still leave at least2^44 states. Newton faces can reject whole bounded cells,
but unresolved jets do not establish support. Coupled field-line partitions
can retain exponentially many outer cosets. The odd one-parameter gcd route is
classical and does not prove a field-line cover of the actual F2 family.

The changed proposal operates in the exact Boolean FUNCTION ring of the73
shared parameter bits and the exact field-valued quintic quotient, rather than
substituting independent place/root/lift variables. It may fail by expression
size or lack of constant-span certificates. Sparse Boolean ANF and linear-span
refutations are classical; novelty priority is NOT established.

## Exact symbolic semantics and the limited whole-cell pruning theorem

Freeze the existing first h,k=0 cell and its fully verified relaxed affine map
H(t)=H0+sum(t_i Hi), t inF2^73, from166 raw r/s bits. Every prefix cell fixes
some of these SAME canonical kernel coordinates; remaining coordinates are
shared by all coefficients and all stages. Work over
B=K[t_i]/(t_i²-t_i). Unique multilinear ANF represents exact functions on the
Boolean cube; coefficient vectors are actualK elements. In this ring squaring
sum(c_m t^m) gives sum(c_m² t^m), retaining monomial supports. Multiplication
uses monomial UNION, with XOR cancellation of equal monomials. These facts
are exact; no sampled interpolation or formal-Jacobian approximation is allowed.

Let H be monic quintic. Compute remainders of X^(2i), i0..4, by monic division
overB; this is legal with no parameter-dependent inversions. Their coefficient
ANF degree is at most4 because the highest exponent8 needs at most4 reductions,
each multiplying by an affine coefficient ofH. If R has parameter degreee,
its square modH has degree at most e+4, not2e+4: Boolean Frobenius preserves
ANF supports. StartingR=X and iterating18 cached subspace steps therefore
has degree at most72. Three more quotient squares compute R8, with degree at
most min(73,84). This is only a degree bound: the number of possible monomials
can still be exponential and no tractable search follows.

For every specialization, all roots ofH lie inV including multiplicities iff
R8=0. If a WHOLE cell is independently certified squarefree, R=0 suffices.
A sufficient squarefree-cell certificate is an exact nonzero CONSTANT resultant
Res(H,H') inB; alternatively a verified Boolean-polynomial inverse proves the
resultant is a unit. If no such certificate is available, retain R8 and never
drop the repeated-root branches. Squarefree all-distinct H is the rank4 frontier,
but generalR8 rejection is even stronger because it rejects repeatedH too.

Expand the five field-coordinate residual polynomials into415 binary ANFs.
View these as binary vectors indexed by their shared monomials INCLUDING1.
If an exact XOR combination of residual coordinates equals CONSTANT1, then
the whole prefix cell has no supported norm parameter: every supported point
would make the left side0. Emit the selected coordinate indices and the full
coefficient cancellation certificate; a separate reviewer replays it exactly.
No constant1 in this linear span means UNKNOWN, not a supported point. Products
of residual equations are not silently added; any stronger XL/Macaulay closure
would be a separate preregistered variant and cannot masquerade as newGB-free
complexity. Actual source-rank4 witnesses require full distinct root recovery.

## Concrete bounded experiment and disjoint cover accounting

Create new source, independent derivation/code review, then freeze inputs/hashes.
First exhaust tiny parameter cubes (at most12 bits) independently using direct
specialization, literal polynomial norms and full source-product/root oracles.
Test repeated and squarefreeH, nonconstant resultants, nonlinear residuals,
corrupted constant1 certificates and explicit monomial-cap refusal.

Then symbolically process the ACTUAL73-bit cell using fixed breadth-first
prefix splitting on canonical kernel coordinates0..7. At most256 depth8 cells,
511 nodes,90s/4GiB/1core; global activeANF monomial cap65536 and percoefficient
cap4096, global serialized evidence32MiB. Monomial count/cost must be preflighted
before each multiplication materializes more terms. If exact expressions or
certificate construction exceed a cap, retain that node UNKNOWN or split only
within the declared remaining node/depth budget. Do NOT enumerate its2^65 or
more completions. Keep every rejected/UNKNOWN prefix as a prefix-free leaf
antichain; the exact sum of2^(73-depth) over leaves must equal2^73. Parent
rejection covers both descendants without materializing them. Global cell
UNSAT is permitted only if EVERY leaf has a verified exclusion certificate.
Any UNKNOWN leaf preserves an unresolved count bound. Report exact rejected
parameter mass and remaining prefix dimensions, not source root counts.

Record maximum symbolic monomial count, each stage's actual ANF degree,
constant-span rank, complete certificate bytes, total time and processRSS.
The falsifiable prediction is at least one positive-dimensional actual prefix
cell is excluded with exact support residuals under the stated caps, beyond
coefficient-span/old jet information. A fully capped/no-certificate result
refutes this particular compression mechanism on the fixed input and motivates
a different invariant or multi-bit partition; it never justifies an identical
rerun with a larger timeout. No lower bound on surviving dimension is guessed.

No independent q is introduced: field quotientR8 already has exact support
semantics. If later bounded polynomial lifts/jets are combined, the SAME shared
three quotient bits must be used at every place, and all eight lifts retained.
This experiment remains one fixed h,k norm chart; original polynomial ghost,
auxiliary and exceptional-chart coverage and all outer h/k search remain open.
The cached verifier is a building block, not itself an outer search algorithm.

## Counterexample to assuming an affine support residual

OverGF4 let V={0,1}, L=X²+X, H_u=X²+uX+1 with the TWO shared binary bits ofu.
Then R=LmodH=(u+1)X+1 and R²modH=(u³+u)X+u². Although H is F2-affine inu,
the first residual coefficient is not: for omega²=omega+1,
f(omega)=omega², f(omega²)=omega, but f(1)=f(0)=0. Thus ordinary Gaussian
rank on input coefficient directions cannot replace exact ANF support search.
The exact all-supported parameter is u=0, H0=(X+1)². A faster quotient kernel
only evaluates this nonlinear condition more cheaply; it supplies no global
search, affine-rank reduction or impossibility proof.

For odd fields the exact general criterion is H divides L_D^m, as the existing
odd_support_cell proof shows. The Boolean-Frobenius degree bound above is
characteristic-two specific and must NOT be transferred to NIST primes or odd
extensions. That existing separate odd-characteristic proof stream stays open;
this plan claims neither new odd coverage nor requested8x release completion.

The a=1 anchors are also special torsion points, not generic source-point tests.
For A0/B1 binary doubling, lambda=x+y/x, x(2P)=lambda²+lambda+A,
y(2P)=x²+(lambda+1)x(2P). At P=(1,0) or(1,1), lambda=1 or0,
so BOTH double to(0,1), a nonidentity two-torsion point, and have exact order4.
Thus their shifted rational relation targets T-2P agree. This explains a strong
source of cross-ordinate duplication, but exact polynomial-output duplication
still requires recorded coefficient comparison;65 distinct parameter charts
must not be reported as65 distinct root/support problems. The coordinator has
reported1040 candidate occurrences and the reviewer is checking distinct Q
counts. This reinforces the choice to leave these collision/torsion anchors
and test general five-distinct/rank4 quintic cells next.
