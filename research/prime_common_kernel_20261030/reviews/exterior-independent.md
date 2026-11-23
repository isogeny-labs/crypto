# Independent review of an exterior-kernel proposal

2026-10-30 UTC. Written mathematical review only. No solver, timing run,
performance experiment, or ECDLP instance was run. The statements below are
elementary algebraic consequences of the explicitly defined deformation;
they are not a formal verification of a contraction algorithm. No claim of
mathematical priority or an improved ECDLP algorithm is made.

## Sources read before the analysis

* The pinned Oct25 manuscript, `/workspace/audit/tensor-primary/openai-oct25.txt`,
  D.4.2--D.4.6 (packet packing, the actual Boolean extension in D.8, precision,
  crossing modes and the factorized child identity).
* `/workspace/crypto/problems/correlated-packet-common-kernel.md` and the
  matched-level and binomial-counterfamily reviews in
  `research/prime_tensor_variants_20261030/reviews/`.
* The lead atlas entry
  `crypto-autoresearcher/knowledge/leads/ecdlp/exotic/LEAD-EXO-ideas-corpus-algebraic-transforms.yaml`.
  Its spinor/matchgate lead has unsupplied constructors. This is prior
  coverage, not evidence that this particular deformation is impossible.
* Mortier et al., *Fermionic tensor network methods*,
  [arXiv:2404.14611v2](https://arxiv.org/html/2404.14611v2), especially
  Sections 2.1--2.4: a Fock coefficient depends on the order of the modes;
  changing that order supplies occupation-dependent signs. Graded tensor
  calculus is an established way to track them. Its applicability to a
  proposed ordinary Boolean recursive algorithm still requires an exact
  translation and a charged graph construction.
* Epple, Fries and Hinrichsen, *Implementing the sine transform of fermionic
  modes as a tensor network*,
  [arXiv:1705.10186v1](https://arxiv.org/html/1705.10186v1), Section V:
  lifting one-particle factorizations to fermionic diagrams is established
  prior work. Its swap bookkeeping must be retained.
* [Macaulay2's Cauchy-decomposition documentation](https://www.macaulay2.com/doc/Macaulay2/share/doc/Macaulay2/TensorComplexes/html/_cauchy__Map.html)
  for the exterior power of a tensor product. An exterior lift of a tensor
  product is not simply a tensor product of the two exterior lifts.

## 1. A sound common-origin deformation, with a different off-origin meaning

For a K-by-K matrix W over a commutative ring, define a 2^K-by-2^K matrix

    Gamma(W)[A,B] = det W[A,B] if |A|=|B|,
                   0 otherwise,

where A and B are subsets in their fixed ordinary order. Empty minors equal
one. Cauchy--Binet gives Gamma(UV)=Gamma(U)Gamma(V), with the intermediate
sum ranging over all subsets of the full intermediate one-particle space.
Also Gamma(I)=I. Consequently

    F_ext(T)=sum_(A,B) a(A)b(B) Gamma(W(T))[A,B]

has the correct original contraction at T=0 whenever W(0)=I. If every entry
of W has degree at most d_W, its degree is at most K*d_W. Integral local
charts for W give integral charts for every determinant without division.
These observations are positive. They do not supply a group-factorized
recursive evaluation of F_ext.

The source's D.8 is instead a product of affine Boolean interpolation
factors. It is not Gamma(W). For example, its row response can be nonzero
between assignments of different Hamming weight; Gamma(W) preserves weight.
The source's crossing-mode identity D.16 therefore cannot be reused for the
exterior deformation without proving a replacement identity.

One must also not write Gamma(U tensor V)=Gamma(U) tensor Gamma(V). The
one-particle spaces on the two sides differ: the first Fock space has
dimension 2^(dim U * dim V), while the second has dimension
2^(dim U + dim V). The Cauchy/Schur decomposition is a more elaborate
representation-theoretic statement and supplies no free small description
for arbitrary endpoint tensors.

## 2. A rank-one update becomes squarefree, not a new counting exponent

Let

    W(z)=A+sum_j z_j u_j v_j^T.

Every minor is of degree at most one in each z_j. To see this, expand the
determinant multilinearly in its columns. A term choosing the same update
j in two columns has two columns proportional to u_j, so its determinant
vanishes. This argument is an identity over the integers and remains valid
over the source's truncated 2-adic coefficient rings; no inverse of two is
used. Coefficients can thus be indexed by subsets of update indices rather
than arbitrary exponent tuples.

Suppose a *separately proved* factorization supplied at most A_0 such
variables of each positive valuation weight w, and coefficient divisibility
at least 2^(sum_j w_j). The standard upper envelope for the surviving
subset count is

    N(p) <= exp(theta*p) product_(w>=1)(1+exp(-theta*w))^A_0,

whose logarithm is at most theta*p+A_0/(exp(theta)-1). Optimizing gives

    N(p) <= exp(O(sqrt(A_0*p))).

This has the same order as the ordinary-partition envelope used in D.15.
It can change constants and remove repeated use of one update, but by
itself does not turn the square-root exponent into a two-thirds exponent.

There is also a scoped lower count for the *formal* dense variable family:
take A_0 independent indices at every weight 1,...,k, where
k=floor(c*sqrt(p/A_0)) and c is a small constant. All subsets fit the budget
once A_0*k*(k+1)/2<p. There are 2^(A_0*k) such subsets. In the regime
p/A_0 sufficiently large this is exp(Omega(sqrt(A_0*p))). It is not a
lower bound on the number of nonzero coefficients, a better representation,
or the runtime of a different contraction algorithm.

Low rank in the one-particle space must not be confused with low rank of
the full Fock operator. For example Gamma(I+z e_1 e_1^T)=I+z*n_1, and n_1
has ordinary matrix rank 2^(K-1) over a characteristic-zero field. A useful
group-separation property, if present, must be proved rather than inferred
from that one-particle rank.

## 3. Exterior lifting does not by itself remove passive-axis channels

The one-particle block of Gamma(W) is exactly W. Thus a nonzero crossing
entry of W is still a nonzero transition of Gamma(W), with the same
valuation. For example, if U=I+2z E_(1,0) and V=I, the transition from
the occupied mode (0,0) to (1,0) in Gamma(U tensor V) has coefficient 2z.
It did not gain a second factor of two by exterior lifting.

This refutes only the shortcut that exterior statistics alone suppresses
every one-axis transition. If a carefully constructed real-slot code
already excludes that pair of endpoints, this example does not apply to
the restricted support. The density and common-family proof for such a
code remain separate obligations.

The same caution is visible for a two-particle minor. For 2-by-2 matrices
A and B, the pointwise product satisfies the exact identity

    det(A .* B)=det(A)*B11*B22 + A12*A21*det(B).

Consequently a determinant of product entries is generally a sum containing
one-axis terms, not a product of both axis determinants. B with every entry
one leaves det(A) unchanged. This is an algebraic diagnostic, not a claim
that these particular matrices are forced by the proposed packet kernel.

## 4. Regrouping signs cannot generally be compressed to total parities

Here is a direct group-separation witness. Take the fixed mode order
`a1,b1,...,an,bn` and let P interchange the two modes of each adjacent pair.
On input occupation strings `(a,b)` its output is `(b,a)`. Its exterior
matrix element on this support is

    Gamma(P)[(b,a),(a,b)] = (-1)^(sum_j a_j*b_j).

Each disjoint two-mode permutation has empty- and one-particle
coefficients +1, and its full two-particle coefficient is its determinant,
which is -1. Adjacent disjoint pairs do not cross one another in this fixed
order, so their contributions multiply.

Allocate group A the input a-modes and their matched output b-positions;
allocate group B the input b-modes and their matched output a-positions.
Thus P is local as a *one-particle matching* for these row and column
groups. After the local matching deltas are imposed, its ordinary Boolean
separation matrix is nevertheless

    H_n[a,b]=(-1)^(a dot b).

For any a,a', summing the product of their rows over b is zero if a!=a'
and 2^n otherwise: at a coordinate where a and a' differ, pair every b with
its bit-flipped mate. Hence H_n H_n^T=2^n I, so H_n has rank 2^n over a
characteristic-zero field. An exact ordinary separable sum across these
two groups needs at least 2^n terms. A separator of c Boolean states
provides at most 2^c such terms, so c>=n for this exact convention. This
refutes *generic* constant-parity separation of the ordinary Boolean
exterior map, even when the one-particle matching is group-local.

This rank statement must not be transferred carelessly to a truncated
2-adic ring. H_n is the n-fold tensor power of the two-by-two Hadamard
matrix, which integer elementary row/column operations take to diag(1,2)
up to signs. Their tensor powers take H_n to a diagonal with entries
2^|S| indexed by subsets S. Some vanish modulo 2^p when |S|>=p; every
sign vanishes as a distinction modulo two. The characteristic-zero rank
proof alone is not a uniform modular rank proof. At p>=2, examples with
the same group parities and different phase already refute total-parity-only
descriptions.

P is not asserted to equal an actual source Cauchy node. Source W(t)
follows an order-preserving partial translation, and its center-node signs
may avoid this witness. Reordering all modes in advance can simplify
this one example; the question is whether one fixed ordering and charged
gauges work for *all* levels and recursive groupings. This is not an
impossibility proof for that question.

For completeness, a pure regrouping witness exposes the origin of signs:

Take 2n one-particle modes in fixed order

    a1,b1,a2,b2,...,an,bn,

and stably regroup them to all a's followed by all b's. On occupation bits
a,b the shuffle phase is

    chi(a,b)=(-1)^(sum_(i>j) a_i*b_j).

For n=2, a=(1,0) and a'=(0,1) have the same total occupation and parity;
with b=(1,0), the phases are respectively +1 and -1. Therefore a single
total parity or occupation count for each group does not determine this
phase. It remains distinguishable modulo four. An assertion that constant
group-parity states always suffice needs additional structure.

Over a characteristic-zero field, the matrix of chi over all a,b has rank
2^(n-1). Let E have E_ij=1[i>j] over F2; it has rank n-1. The rows of chi
are the Walsh characters of b indexed by E^T a. There are exactly 2^(n-1)
distinct characters and they are linearly independent. Hence an exact
ordinary separable sum over the two groups needs at least that many terms.
This is not a rank lower bound over Z/(2^p), where two is not a unit, and
is not a lower bound on a different 2-adic algorithm.

For a block-local one-particle kernel the actual Fock matrix involves a
row shuffle sign *and* a column shuffle sign. They sometimes cancel, for
example on the diagonal identity. An arbitrary-shuffle example alone
does not prove an obstruction for the source's Cauchy family. The stronger
block-local permutation witness derived independently by the exterior-kernel
reviewer is the relevant check if a generic no-signs factorization is
asserted. It is still not a realization by the actual source Cauchy kernel.

In particular, a specially chosen ordering by a constant number of endpoint
pair classes may simplify the sign. That possibility is open here. It
requires a *single* chosen exterior ordering, and proof that all level-wise
regroupings, previous tensor groupings, and recursive packet merges are
accounted for. Choosing a new order and resetting the exterior kernel at
each level is another common-family change unless its shuffle gauges are
carried through exactly.

## 5. Cauchy minor formulas give efficient entries, not free decoupling

Every minor of a one-dimensional Cauchy matrix has its usual Vandermonde
quotient expression. This can expose additional divisibility: if k row
positions and k column positions each lie in a tile of stride 2^(i+1),
while every crossing denominator has valuation at most i, the two
Vandermonde factors contribute at least (i+1)*k*(k-1) to the numerator
valuation. If the common scalar numerator has valuation at least a+1,
the determinant is divisible by at least

    2^(k*(a+1-i)+k*(k-1)).

This is a scoped same-tile estimate; it is not valid for arbitrary unions
of tiles without rechecking the differences. The quadratic term reflects
the distinct moment degrees already present in a squarefree expansion.
It does not alone prove a smaller global mode family.

For actual ring computation, the safe method remains evaluating each
integral W entry by its factored formula and then using a division-free
determinant algorithm. A Vandermonde quotient cannot be implemented by
inverting denominators of positive 2-adic valuation modulo 2^p. Their exact
cancellation or a charged extra precision must be proved. Determinant entry
evaluation is polynomial in the packet dimension and precision; summing
its coupling with arbitrary input endpoint tensors is a different task.

## 6. Intermediate holes cannot be silently removed

Let P select the real packet modes from a larger two-axis grid. Generally

    P U V P^T != (P U P^T)(P V P^T),

because the left side may propagate through an intermediate mode absent
from the selected set. For a two-mode example select mode 1 and use
U=E12, V=E21. The left side is [1] while both restricted factors vanish.
Applying Gamma does not repair this: its one-particle block contains that
same counterexample. Factorizations through the full ambient grid may be
valid, but their intermediate Boolean modes, packet degrees, and entry
description costs must all be retained and charged.

## Verdict and discriminating next obligation

The exterior lift is a valid alternative *common* deformation with an
unchanged origin, a reasonable degree bound, integral local charts and
efficient individual entries. These useful facts do not establish either
the desired two-axis mode density or the recursive group-factorization
identity. Multiaffine update variables alone retain the old square-root
counting envelope. Arbitrary regrouping signs and intermediate holes are
specific places where an invalid shortcut would enter.

The next check should be an exact identity for a *fixed* chosen packet
ordering and real-slot support, expressing Gamma(W(t+alpha*z)) as a sum of
products over the proposed child groups. It must display the actual sign
states, prove the claimed number and valuations of modes, retain ambient
intermediate modes if used, and show those choices persist recursively.
Only after that identity can one infer a new partition count or attempt
the full global runtime. This review does not prove that no such identity
exists.
