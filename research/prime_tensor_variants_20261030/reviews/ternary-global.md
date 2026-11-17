# Global review of the ternary contraction adaptation

2026-10-30 UTC. Independent mathematical review by `/root/ternary_global_review`.
No implementation, cryptographic attack, benchmark, timing or novelty certificate
is supplied here. The result is a written proof audit, not a Lean certificate.

## Verdict and exact scope

After the target-specific mode correction, the construction supports a complete
**three-adic analogue of Theorem D.1** of the pinned October 25 manuscript. I find
no remaining global proof gap in that analogue. In particular, the precision,
interpolation, recursive-description and space arguments do not require a new
hypothesis beyond the analogue's explicit input model. This statement concerns
modulus powers of three. It does not silently convert arbitrary mod-two tensor
entries, improve the source exponent, establish novelty, certify a general
implementation, or supply an essentially new ECDLP reduction.

For Boolean 0/1 input tensors, it computes the exact nonnegative integer
contraction by taking root precision P0=M+1. Thus this application does not need
any conversion from mod-two inputs. Formalization of the full theorem remains
open; the packet and one-level finite units cannot replace that formalization.

## Sources actually reviewed

- `/workspace/audit/tensor-primary/openai-oct25.txt`, Theorem D.1 and all of
  D.4.1--D.4.7 (text lines3012--3040 and3256--3648). This is the pinned
  October25,2026 manuscript, *The Exponential Time Hypothesis is False*.
- `/workspace/audit/next_ternary_packet_review.md`, for the order-three orbit
  packing, the locality statement and the indispensable target split.
- `/workspace/audit/next_ternary_precision.md`, for the changed derivative,
  Hermite-valuation and three-adic error bounds.

The source's method is inherited: edge collision packing, Cauchy deformation,
tapered Hermite recovery, valuation-weighted modes, recursive groups, immutable
creation-precision entry lifts and depth-first evaluation. This is an adaptation
of that method, not an independent method-discovery claim.

## A concrete theorem

Fix positive constants C,A,b and a fixed polynomial vertex-count bound. Let s>=2
and let Dlen be the total graph and tensor-description length. Suppose a
bipartite Boolean-edge multigraph has at most Cs edges, at most polynomially many
vertices and maximum degree at most ceil(s^(9/10)). Its entries lie in

    B_(P0,ell)=(Z/3^P0 Z)[V]/(V^ell),  1<=P0,ell<=Cs.

For every vertex and assignment of its Boolean ports, assume its specified
entry is uniformly computable from its description in time

    (s+Dlen)^b * 2^(A*s^(9/10))

and space (s+Dlen)^b. Then the described ternary adaptation computes the
contraction deterministically in

    (s+Dlen)^O(1) * 2^O(s/sqrt(log s))

bit operations and (s+Dlen)^O(1) bits of space. Constants depend only on the
fixed bounds above. Parallel edges have separate Boolean ports. Isolated
vertices contribute their scalar entries and may be processed first.

For 0/1 input entries and M edges, the integer contraction is between0 and2^M.
Taking P0=M+1 and ell=1 recovers it exactly because 2^M<3^(M+1). Increasing the
fixed constant C to C+1 accommodates this precision also at small s. This
corollary does not assert that a scalar-verification encoding has sufficiently
few edges; an encoding-size proof is still required before applying it to a
cryptographic problem.

## 1. Parameters and coefficient arithmetic

Use lambda=ceil(log2 s), epsilon=2^-20, r=67, Delta=ceil(s^(9/10)), and
H=floor(s/floor(sqrt(lambda))). Calls with M<=H are direct-summation leaves.
At an active depth-d call put

    u_d=floor(epsilon*lambda/2^d),
    a_d=2*sum_(j=0..d) u_j,   a_(-1)=0,
    Lslot=3^a_d,             Darr=67^(4a_d).

Lslot is a packet's slot count, not the input-description length Dlen.

Take e=4lambda and choose a monic f of degree e with coefficients0,1,2 below
its leading coefficient and irreducible mod3. Enumeration of3^e candidates,
with deterministic divisibility testing by all monic polynomials through
degree e/2, takes polynomial time in s. A finite field has an irreducible
polynomial in every positive degree. Put

    O=Z_(3)[U]/(f), K=Q[U]/(f), A=O[V]/(V^ell), A_p=A/(3^p).

The minimum three-adic coordinate valuation on K is multiplicative because
O/3O is a field. Elements nonzero modulo3 are units, using their multiplication
matrix over Z_(3). Arithmetic modulo3^p, including these inversions, is
polynomial in s,p,ell. The3^e canonical coordinate lifts supply pairwise
unit-difference interpolation points. Since3^e>=s^4, they suffice below.

Each tensor stores an immutable creation precision pc. To use an old tensor at
higher precision, compute its complete entry modulo3^pc and lift those final
coordinates canonically. Do not recompute the entry's defining formula at a
new modulus, and do not lift its intermediate factors. Canonical lifting is
not a ring homomorphism. This is exactly the creation-precision discipline
needed by the recursive correctness argument.

## 2. Ternary collision packing

Each vertex label has2a coordinates in F67. A pair class therefore has4a
coordinates and there are Darr classes. Independent uniform vertex labels give

    E sum_gamma (X_gamma-M/Darr)^2 <= 2M*Delta.

Expand the expectation over ordered edge pairs. Disjoint endpoint sets make
zero contribution; there are at most2M*Delta ordered pairs sharing an endpoint,
including parallel/equal edges, and each contribution is at most1. Conditional
expectation fixes labels deterministically in polynomial time: there are
67^(2a) label candidates per vertex, and each relevant conditional probability
concerns at most four vertices.

Assuming

    M>=2*Darr^2*Delta, M>=Lslot*Darr, 2*Lslot<=Delta,             (P)

every class load is at most M/Darr+sqrt(2MDelta). Pad classes to a common
Hpad, rounded up to a multiple of Lslot. Then Hpad<=3M/Darr.

Write S(A,B)=(-A-B,A). It has order3. On a pair class coordinate, use S on
component1 and S^-1 on component2. Their product action U_i is fixed only if
all four entries are zero (char67 is different from3). Different i commute.
An orbit therefore has size q=3^a' dividing Lslot. Ternary slot digits apply
the corresponding U_i powers; every class in the orbit has Lslot/q slot
preimages. Exactly Hpad*q/Lslot packets cover that padded orbit. The total
packet count is Hpad*Darr/Lslot<=3M/Lslot.

Every selected coordinate/component/symbol has exactly1/r of all padded
pair classes on each side. Thus its real-port mass is at most3M/r per side.
Nothing requires the label action to preserve an individual row label within
a packet; the pair class at its assigned slot supplies the correct labels.

## 3. Deformation and local charts

Use grid G={-Lslot,...,2Lslot-1}, alpha=3^(a+1),
C(T)=product_(t in G)(T-t), and

    W_xy(T)=C(T)/(C'(0)*(T-(y-x))).

Every slot difference is in G, so W is polynomial of degree3Lslot-1 and
W(0)=I on real ports. Deform the wire identifications by the source formula
D.8 using l0(z)=1-z and l1(z)=z and the chosen complete entry lifts. The
resulting F has degree at most M(3Lslot-1), and F(0) is the lifted contraction.

G is a complete residue system mod3^(a+1). At every t its nonzero differences
have valuation k on exactly2*3^(a-k) positions, k=0,...,a. Hence all C'(t)
have the same valuation. Factoring the known three-parts before inversion
shows W(t+alpha*z) belongs to O[z], and hence F(t+alpha*z) belongs to A[z].
This is a coefficientwise cancellation argument, not division by C'(0)
modulo3^p. It also evaluates correctly at roots of C.

At nonzero t let i=v3(t)<u and c=(t/3^i)mod3 in{1,2}. Group vertices by
component c of their i-th label coordinate. If y-x=t, the i-th ternary digits
satisfy y_i=x_i+c modulo3. Component1 uses S, whereas component2 uses S^-1;
for c=1 or2 respectively the column label at y equals the row label at x.
Thus W(t) is local. Lower digits of x and y agree, so no hidden carry spoils
this implication.

## 4. Recovery and polynomial interpolation

Set h=ceil(2M/u), q_i=h*max(0,u-i), D_t=Lslot/3^i, and

    p_t=p+4hD_t+8M(a+1).

There are2Lslot unit nodes, so the total positive multiplicity is at least
2Lslot*h*u>=4MLslot>M(3Lslot-1). Also h<=3M/u and q_i<=3M.
For the Hermite factor E_t of source D.9, direct valuation counting gives

    v3 E_t(0)
      =2D_t*sum_(j=1..a-i) (j/3^j)*(q_(i+j)-q_i)
      >=-3D_t*h,

using q_(i+j)-q_i>=-hj and sum_(j>=1)j^2/3^j=3/2. Nodes whose two relevant
valuations both equal i contribute zero; they must not be dropped by falsely
using the binary claim that they do not occur. The boundary nodes of valuation
a have multiplicity0 because u<=a and require no nonexistent coordinate a.

E_t(t+alpha*z) has integral coefficients and constant term1; its truncated
reciprocal is integral. Evaluation of degree<q_i at -t/alpha loses at most
3M(a+1) digits. Input jet errors at precision p_t therefore contribute errors
of valuation at least

    p_t-3D_t*h-3M(a+1)>=p.

The exact coordinatewise rational Hermite reconstruction remains integral
and correct modulo3^p. Its rational multipliers are products of polynomially
many O(a)-bit factors with polynomial multiplicity. Even multiplying all
rational denominators gives polynomial bit length. Thus exact recovery has
polynomial bit cost and space, rather than a unit-cost rational oracle.

Obtain each local chart by evaluating at
Q=M(3Lslot-1)+1 residue-distinct extension points and Lagrange-interpolating
the full polynomial. Unit differences justify interpolation modulo3^p_t,
including the truncated V coefficient ring. For sufficiently large s,
Q<s^4<=3^e. The number of active t and Q values is polynomial in s.

## 5. Corrected modes, children and entry descriptions

Partition column slots into tiles y=y0+3^(i+1)j. A tile has one source
group. If a row crosses to this tile, locality implies
v3(t-y0+x)<=i, so adding alpha*zeta preserves this denominator valuation.
The geometric expansion of W has b-th coefficient divisible by3^(b+1),
with source moment sum_(y in tile)j^b X_y. The omitted tail vanishes modulo
3^p_t. These coefficients are computed by explicit three-part cancellation
and unit inversion, including at zeta=0.

There can be TWO crossing target groups, drawn from A,B,-A-B. Make a
separate formal mode for each target even when their numerical source moments
agree. This restores one source and one target per formal variable. A mode
has weight w=b+1; there are at most

    2*(3M/Lslot)*3^(i+1)=18M/D_t

modes of each weight. Keeping only tuples sum_Z w_Z beta_Z<p_t is exact
modulo3^p_t. The source's generating-product argument, with6 replaced by18,
bounds their number by

    exp(O(sqrt(M*p_t/D_t))).                                  (B)

Tuples have polynomial descriptions and admit polynomial-delay, polynomial-
space enumeration, using all-zero continuation for every feasible prefix.

For each tuple and group g, put its old vertices on one child side and one
new tensor per packet on the other. Its ports are precisely that packet's
row and column bits in group g. Extract all targeted mode exponents in the
row-factor product and multiply all source moments to their prescribed
exponents. Each formal mode has exactly one source and target; therefore
coefficient extraction and substitution factor by group. Summing independent
group bits proves the exact analogue of source D.16. Repeated numerical
source forms in two different formal modes cause no defect in this identity.

A child has at most6M/67<M/8 edges. Old degrees are unchanged; a new packet
vertex has at most2Lslot<=Delta ports. Its entry can be evaluated by enumerating
choices in at most Lslot affine row factors, each with polynomially many
terms. This costs exp(O(Lslot*log s)) arithmetic operations and polynomial
space. Source moments use repeated squaring; no explicit exponential table
is stored. The description includes packet data, tuple, interpolation value
and its creation precision. Earlier descriptions are immutable references;
new packet formulas do not recursively invoke old tensor entry formulas.
Only polynomial description data and vertices accumulate along a path.

## 6. Global admission and inherited precision

With the above edge shrinkage, an active depth satisfies

    H<M<=Cs*8^-d, d<=log2(lambda)/6+O_C(1).

Consequently u=Theta_C(lambda/2^d)>=lambda^(4/5) eventually,
a<=4epsilon*lambda, and M>=u. The exponents of Lslot and Darr as powers of s
are respectively at most

    4epsilon*log2(3), 16epsilon*log2(67),

both strictly smaller than1/1000. Thus (P) holds at every active call for all
sufficiently large s: 2Darr^2Delta=O(s^(.902))=o(H), with the other admission
inequalities easier. These bounds use r=67, not the obsolete r=64 constants.

Summing the precision increments along a path gives

    p_d<=s*lambda^O(1)*3^(a_(d-1)).

For an active node i<u, D_t>=3^(a_d-u)=3^(a_(d-1)+u). Since M>H and
u>=lambda^(4/5), the extra3^u absorbs the inherited polynomial-logarithmic
factor and the M(a+1) evaluation term. Hence

    p_t=O_C(MD_t/u),

which turns (B) into exp(O_C(M/sqrt(u))). All precisions remain polynomial
in s, since a_d=O(epsilon*lambda). This proves the inductive invariants and
precision bounds used above rather than assuming them at each recursive call.

An effective algorithm tests the displayed finite parameter inequalities at
each call. If they fail, it directly sums that call. The estimates show that
failure can occur only for finitely many root sizes s, with a threshold
depending on fixed constants. Their extra cost is absorbed by a fixed
constant in the claimed asymptotic bound. No noncomputable threshold advice
or heuristic admission test is needed.

## 7. Time and space

The number of child calls per active depth-d call is at most

    s^O(1)*exp(O_C((s/sqrt(lambda))*(sqrt(2)/8)^d)).

There are O_C(log lambda) depths. Multiplying these bounds, or summing their
logarithms along each level, gives exp(O_C(s/sqrt(lambda))) total calls;
the polynomial-per-depth factors contribute only O(lambda*log lambda) to
the exponent. At leaves M<=H, direct summation costs2^H assignments and
polynomially many tensor entry evaluations each. Both the original
2^(A*s^.9) evaluator bound and exp(O(Lslot log s)) new-tensor bound are
absorbed by exp(O(s/sqrt(lambda))). All arithmetic and data management
have fixed polynomial factors in s+Dlen.

Traverse depth first, keeping one tuple sum and one group product at a time.
The graph, immutable referenced descriptions, current indices, Taylor data,
precisions and rational-recovery workspace are polynomially bounded. There
are only O(log lambda) stack levels. Tensor entry evaluators have polynomial
space by hypothesis or by their explicit enumerations. This proves the stated
polynomial-space bound. Final projection gives the base-ring coordinates.

## What is and is not established

The written three-adic theorem and its Boolean-count corollary close under
their stated computational model. The formal proof gate remains open for
this global result; a faithful structural locality theorem is useful partial
formalization, not certification of the whole proof. A general recursive
ternary implementation is also absent, even if all finite units pass.

No better asymptotic exponent follows. Every critical mechanism is adapted
from D.4, and the ECDLP encoding/reduction would still be inherited from the
separate recent ECDLP paper. Calling this an essentially novel prime-field
ECDLP breakthrough would therefore be unsupported.
