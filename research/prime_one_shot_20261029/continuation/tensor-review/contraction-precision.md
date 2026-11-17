# Scoped primary-proof review: tensor contraction precision and factorization

2026-10-30 UTC. Reviewer: `/root/tensor_precision_review`. This is an independent
on-paper check of the algebraic and precision steps in Appendix D of the pinned
public manuscript. No ECDLP implementation, scientific computation, numerical
timing, formal proof certificate or claim of institutional endorsement is supplied.

## Source and scope

Source: _The Exponential Time Hypothesis is False_, October25,2026 manuscript,
revision `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`:

https://raw.githubusercontent.com/openai/math/7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf

PDF SHA256:
`5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123`.
Local files: `/workspace/audit/tensor-primary/openai-oct25.pdf` and `.txt`.
Read Theorem D.1 on printed p53 and all of D.4, printed pp57-64, with
particular focus on D.4.1, D.4.3-D.4.6. Visually checked original parameter
notation on p58 and the decisive precision/mode formulae on pp61-62; in
particular `D_arr=r^(2a)`, not `r*2^a`.

The public repository namespace and manuscript author label establish source
provenance only. They are not evidence that the theorem has received institutional,
external-peer or formal verification. The theorem is an explicit theorem with an
explicit proof, rather than an unproved hypothesis stated inside the manuscript.

## Verdict

I found no concrete algebraic or precision gap in the checked steps. I independently
re-derived the local valuation, interpolation-error, weighted-mode and packet
factorization arguments below. This is a favorable scoped proof review, not a
certificate of the whole manuscript, its SAT/ECDLP encodings, an implementation,
or an independently established new algorithm. A failure to locate a gap does not
itself certify the complete claimed theorem.

The entrywise creation-precision convention is essential. Reading canonical
lifting as a ring homomorphism, reevaluating old tensors at a new precision, or
reducing rational Hermite multipliers before accounting for their negative
valuations would invalidate the reasoning. The manuscript explicitly avoids
those shortcuts.

## 1. Coefficient ring, units and canonical lifts — D.4.1, pp57-58

The chosen monic f, irreducible modulo2, is irreducible over Q. For
O=Z_(2)[U]/(f), reducing a nonzero minimum-valuation coordinate vector modulo2
gives a nonzero residue-field element. Products of two such elements remain
nonzero, proving v2(xy)=v2(x)+v2(y) for K-elements. A scalar nonzero modulo2
has an odd multiplication-matrix determinant and therefore an inverse over
Z_(2). These justify the unit inversions used later.

For A=O[V]/(V^ell), divisibility is coordinatewise. The proof only needs
v2(product)>=sum of the lower bounds there; it does not incorrectly assert
multiplicativity for arbitrary elements of this ring with nilpotents.

Residue-distinct binary-coordinate lifts have unit pairwise differences, so
Lagrange interpolation works over every A_p. The fixed e=4ceil(log2 s) supplies
at least s^4 points, exceeding the required polynomial-degree interpolation
set for sufficiently large s under the displayed parameters.

At each call a tensor is replaced by fixed entrywise canonical representatives
at its creation precision pc. This need not preserve addition or multiplication.
That does not create an algebraic error: contraction is a polynomial in its
entries, so replacing entries by congruent lifts preserves its value modulo pc.
The call's deformation and later evaluations use those same representatives.
In deeper calls the old tensor entry is first computed modulo its old pc and
only its final coordinate residues are lifted. The proof does not require this
to agree with reevaluating the old algebraic formula directly at a higher
precision.

## 2. Cauchy polynomial and integral local charts — D.4.3, pp59-60

Every y-x belongs to S={-L,...,L-1}, so C(T)/(T-(y-x)) is a polynomial.
At T=0 the restriction to real ports is exactly the identity, because diagonal
entries equal1 and all off-diagonal entries retain the factor T. Therefore
F(0) is the desired lifted contraction.

For each t in the complete residue grid S, the valuation distribution of
nonzero differences t-q is the same. Hence v2(C'(t))=v2(C'(0)). With
alpha=2^(a+1), each factor t-q+alpha*z has coefficientwise content exactly
2^v2(t-q). In C(t+alpha*z)/C'(0), those contents cancel and the remaining
factor alpha*z contributes a+1 powers of2. Omitting the factor indexed by
y-x either removes alpha*z when y-x=t, leaving integral coefficients, or
removes at most a powers when y-x!=t, leaving at least one power. This proves
the asserted integral local polynomial, including the case z=0.

The prescribed evaluation cancels known powers of2 and then inverts only
odd scalar units. Direct evaluation of the uncancelled rational expression
at a root could instead produce an artificial0/0; that is an implementation
pitfall, not a defect in the polynomial construction.

## 3. Tapered Hermite precision — D.4.4 / Lemma D.5, pp60-61

The Hermite formula D.9 is the ordinary CRT interpolation formula applied
over K[V]/(V^ell). Each E_t(t+alpha*z) is an integral series with constant
term1. Its truncated reciprocal is integral, so forming the local quotient
does not consume extra two-adic precision. The truncation is in z, independent
of the coefficient-ring nilpotent variable V.

For i=v2(t), D=L/2^i and q(i)=h*max(0,u-i), the grid counts are exact:
for j=1,...,a-i there are D/2^j points at valuation i+j, and the same number
whose differences from t have valuation i+j. Points of valuation below i
cancel from the valuation sum. Consequently

    v2(E_t(0))=D sum_(j=1..a-i) j/2^j [q(i+j)-q(i)]
              >=-Dh sum_(j>=1) j^2/2^j
              =-6Dh.

The taper still has q(i+j)-q(i)>=-hj when it has reached zero; no boundary
level is omitted. The total jet multiplicity exceeds deg F, since the L odd
nodes alone contribute L*h*u>=3ML. Also q_t<=4M. Evaluation at -t/alpha
therefore loses at most4M(a+1) additional powers of2. The supplied precision

    p_t=p+8hD+8M(a+1)

exceeds the sum of these two losses by a nonnegative buffer, so each local
error is in2^p A and summation does not lose further powers. Rational common
denominators have polynomial bit length: there are polynomially many factors
and coefficients and each factor's numerator/denominator has O(a) bits before
multiplicity. Cancelling powers of2 after exact summation is valid because
the result differs from the integral F(0) by an element of2^p A.

The D.11 precision absorption also checks locally. Inherited p is at most
s*lambda^O(1)*2^(a_(d-1)), while an active node has
D_t>=2^(a_d-u)=2^(a_(d-1)+u). Because u>=lambda^(4/5) and M>H, that extra
2^u dominates the polynomial-lambda loss and the O(M(a+1)) term. Thus
p_t=O_C(MD_t/u) is compatible with the inherited precision, rather than
assuming it away.

## 4. Modes and weighted truncation — D.4.5, pp62-63

Packet orientations make W(t) local when i=v2(t): y-x=t flips bit i, and
the packet's opposite row/column orientation convention then gives equal
group symbols. A crossing row and tile instead have equal ith slot bits,
which implies v2(t-y0+x)<=i. Adding alpha*zeta does not change that valuation.

In D.12 the leading fraction has valuation at least a+1-i>=1, and its
geometric ratio has valuation at least1. The coefficient of the bth mode
is therefore divisible by2^(b+1), uniformly for every integral zeta. The
geometric remainder is zero modulo2^p_t. Tile mode L_Z is an integral linear
form in source-group bits, and each mode has exactly one target group.
The l_0=1-z signs and l_1=z factors do not change these divisibility bounds.

Substituting integral L_Z cannot lower valuations. Thus every omitted mode
monomial with sum w_Z*beta_Z>=p_t genuinely vanishes. There are at most
6M/D variables of each weight; the generating-function estimate gives
exp(O(sqrt(M*p_t/D)))=exp(O_C(M/sqrt(u))). No approximation by floating-point
weights or uncharged infinite series is required.

## 5. Packet factorization and immutable descriptions — D.4.6, p63

For a fixed beta, extracting target-mode coefficients factors over the
packet/group row products, while each source substitution depends only on
bits in its source group. Therefore the product factors by groups and the
Boolean summation is a product of child contractions, establishing D.16.
The manuscript explicitly handles empty source tiles, zero powers and absent
target rows, so these cases introduce no missing contributions.

The new packet's formula consists of local row factors, modes, beta and the
fixed creation-time kernel data. It does not call an earlier tensor-entry
routine. Old tensor vertices are retained with immutable descriptions instead
of expanded into new nested formulas. Thus the claimed absence of callback
composition at each recursive depth is justified at this algebraic level.
An implementation must preserve that representation to inherit the bound.

## Remaining review boundary

The separate cost reviewer covers total recurrence, graph shrink, descriptor
growth and summed time. My local checks are consistent with that accounting,
but do not substitute for a complete independent certification. I have not
audited the manuscript outside D.1/D.4, the separate Bochum ECDLP reduction,
the arithmetic/point encoder, or the native implementation against these
requirements. No numerical fallback or absence of a practical run is evidence
against the asymptotic theorem. Conversely, a source statement and these
scoped checks alone do not establish a new original ECDLP algorithm for the
parent task.
