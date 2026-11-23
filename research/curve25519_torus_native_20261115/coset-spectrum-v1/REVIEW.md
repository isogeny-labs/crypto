# Compact coset spectra: exact contract and the missing algorithm

Status: proposed representation and written equivalence argument, with a
separately checked finite algebra counterexample. No new coefficient solver,
native compiler, experiment, timing or exponent estimate is reported.
All producer assertions are unsigned. [pcc:unverified]

## Question and prior work

Can the membership product over k cosets be replaced by six UNKNOWN coset
indices while retaining the five unknown section coefficients? Does that
replacement supply a cheaper root-discovery algorithm?

Read before this choice: section-coeff-v1/DERIVATION.md,
section-compiler-v1/{NEXT,PROTOCOL,PROOF_MAP}.md, the frozen target-circuit-v2
contract, Cox's eigenvalue theorem, Sparse FGLM sections1.2--2.1 and the
authors' EUROCRYPT2014 paper section3.3. See SOURCES.md for retrieval gaps.
Symmetric variables, multiplication matrices, Chebyshev recurrences and
binary index encodings are classical. No novelty claim is made for them.

## Guarded spectral replacement

Retain the original fields, exact-order/positive-window hypotheses,
Fp-valued coefficients, target charts, monic sextic S(c), Q=[8]R and rational
torsion guard from the preceding work. Put

    A_S = Fp[X]/S,      lambda = A/(2X+A),
    g = T_q(lambda)^2,  omega = h^q,
    b_j = Re(omega^j)^2, H(T) = product_(j=1..k)(T-b_j).

The old map inverse guard makes lambda defined. The old positive-window
argument makes H split and squarefree; its pulled-back membership polynomial
E_B is split and squarefree too. Let M_g be multiplication by g on the SIX-
dimensional candidate algebra A_S and chi_g(T)=det(TI-M_g).

Proposed new constraints, in addition to the unchanged chart/map/torsion
conditions, are

    delta * Disc(S) = 1,
    1 <= j_i <= k (i=1..6),
    chi_g(T) = product_(i=1..6)(T-b_(j_i)).

The six indices are existential unknowns, not given labels or a candidate
list. Repeated j_i MUST be allowed: different raw t values may use the same
coset. Distinct original rows, signs and aliases still require the original
extractor/group replay. Optional sorting j_i<=j_(i+1) removes only a fixed
permutation multiplicity; it must permit equality.

### Written equivalence, precisely scoped

Assume all coefficient values lie in Fp and all stated original hypotheses.
From E_B mod S=0, squarefree E_B implies squarefree S. In the localized
quotient, this is H(g)=0. The classical eigenvalue theorem expresses chi_g
as a product over the six simple support roots, so each of its eigenvalues
is one b_j. This supplies a six-index multiset satisfying the new equations.

Conversely each linear factor of chi_g divides H, hence chi_g divides H^6
(repeated labels cause no problem). Cayley--Hamilton gives H(g)^6=0.
Disc(S)!=0 makes A_S reduced, so H(g)=0. Clearing the invertible map
denominator recovers E_B mod S=0. The old rational-norm and rational-torsion
arguments then apply, followed by the unchanged local sign/raw-index/group
replay. The spectral equations alone are not a substitute for that replay.
This is not an algebraic-closure equivalence without the Fp coefficient
restriction; a generic solver must price field extraction/filtering.
[pcc:unverified]

### Why the new discriminant guard cannot just be omitted

In F101[X]/X^2, multiplication by X has nonzero nilpotent matrix

    N = [[0,1],[0,0]],  chi_N(T)=T^2, N^2=0, N!=0.

For H(T)=T the repeated-index spectrum matches H^2, but H(N)!=0.
The same local-algebra construction exists in dimension6 with X^6.
This invalidates the general implication `matching spectrum => H(g)=0`
without reducedness; it does not by itself exhibit a malformed full native
section fixture. The finite Lean instance covers exactly the 2x2 identities.
Any attempt to remove Disc(S) needs a separate proof from the actual norm
constraints, or a sound treatment of the nilpotent parts. [pcc:unverified]

## Representation budget versus solving budget

At fixed sextic degree, binary Chebyshev computes g using O(log q)
operations in a constant-dimensional algebra. Precomputed public powers of
omega and unknown Boolean index bits compute each b_(j_i) in O(log k)
Fp2 operations. Six-by-six matrix/characteristic-polynomial work is constant
with respect to k and q. This gives a WRITTEN arithmetic-circuit construction
of size O(log q+log k); bit complexity also charges log p, constants, parsing,
guards and extraction. No native circuit of that size was built here.
[pcc:unverified]

Range encoding: write j_i=1+u_i, encode u_i and v_i with L=ceil(log2 k)
Boolean bits and impose u_i+v_i=k-1. Require 2*(2^L-1)<p to exclude modular
wrap. For the actual k20633, L15; twelve such words use180 Boolean unknowns
before optional sorting or arithmetic auxiliaries. This is search work
inside the solver, not free indexing. [pcc:unverified]

The required baseline is the already shipped original six-point target
circuit:1059variables/1089quadratic equations on its declared actual fixture,
with polylogarithmic index/power chains. Its count is inherited unsigned
evidence, not a new measurement. Both descriptions have five-dimensional
unconstrained target fibres generically before discrete membership. The new
proposal therefore must beat the point system on the SAME public inputs;
beating the4.6million-wire expanded section compiler alone would not show a
new scaling advantage. [pcc:unverified]

## The two quotient algebras must not be confused

A_S has basis 1,X,...,X^5 after choosing one coefficient tuple. It checks
that candidate. A root solver instead faces the coefficient/index/guard
ideal I in a polynomial ring in ALL unknown inputs (or an equivalent
eliminated ideal). If zero-dimensional, its quotient algebra has dimension
D=dim(Fp[c,index bits,guards]/I), including algebraic multiplicities.
Neither zero-dimensionality nor a bound on D follows from dim(A_S)=6.

Sparse FGLM takes the solution-ideal multiplication matrices as inputs or
accounts for constructing them. Its shape-position cost depends on D and
matrix sparsity, and a DRL basis is a separate prerequisite. Substituting
D=6 from A_S into that algorithm is an invalid application. A solver that
constructs dense D-by-D matrices spends at least D^2 writes; one that
explicitly visits every six-index tuple takes k^6 visits before other work.
These are necessary overheads of those specified implementations. A route
exclusion ALSO needs the relevant growing-family lower bound on D or k,
and the remaining phase/yield hypotheses. Failure of a sufficient upper
budget alone is not a disproof. No lower bound on all coefficient solvers
is derived. [pcc:unverified]

For example, on the conditional B=ell^(1/6), constant-yield model below,
a dense-matrix implementation with a proved D>=B incurs at least B^2
query writes. An all-six-index enumerator on a declared path k=B^theta
incurs B^(6theta) visits; theta>=1/3 reaches that same phase boundary.
Neither a bound D>=B nor a growing path theta is established here for the
new system. These are checkable conditions for rejecting a NAMED extension,
not observed disproofs of the guarded spectral proposal. [pcc:unverified]

No route is rejected because a cap expires, a finite sample has no root,
a basis is incomplete, or a circuit has many wires. A false algebraic
implication can instead be rejected by an exact counterexample, with the
statement and counterexample domain retained as above.
[pcc:unverified]

## Exponent gate and next discriminating work

Use subgroup order ell, effective base size B=ell^b, complete cold query
cost C=B^(gamma+o(1)), and inverse useful-relation yield ell^(y+o(1)).
If B useful rows are needed, relation work has exponent b*(1+gamma)+y.
At the conditional six-sum model point b=1/6,y=0, this phase has strict
exponent below1/2 only when gamma<2. This is a conditional model calculation;
neither yield, independence/rank nor any other attack phase is supplied here.
All original ROUTE_CRITERIA_V3 obligations remain. [pcc:unverified]

1. Make one versioned Rust semantic comparator for the guarded spectrum,
   original fast membership and original indexed extractor. Include repeated
   labels, distinct raw aliases, identity images, map poles and nilpotent
   counterexamples. Do not enumerate the huge original base.
2. Before a solver run, specify an actual coefficient-elimination operation,
   public-only query API and hard resource limits. Explain how it avoids
   all-index branching, all-triple construction and unpriced solution-matrix
   setup. A proposal that only says `apply FGLM to the 6x6 matrix` fails here.
3. Qualify unknown-input root discovery on a declared blinded finite domain,
   with complete original projection and matched point-circuit comparator.
   Distinguish cap, unsupported, incomplete observation and certified absence.
4. Establish a growing-family cold-work/yield/memory bound before extending
   an exponent claim. A representation-size bound is only the input to this
   test. A bounded failure does not disprove other solvers or the full route.

Decision: keep the exact spectral representation as a scoped candidate;
reject the inference of a constant-dimensional FGLM solve. Do not launch a
new generic cap-raising campaign from this assessment. The next missing
object is a costed elimination operation, not another large gate export.
[pcc:unverified]
