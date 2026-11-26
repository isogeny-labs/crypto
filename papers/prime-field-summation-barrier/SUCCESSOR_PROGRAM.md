# Research program: the non-materializing escape hatch

## 1. Objective

For a general ordinary, non-anomalous elliptic curve \(E/\mathbb F_p\), settle
the following canonical question.

> Can compact Boolean or small-height encodings of a factor base be combined
> with a target-adaptive, non-materializing solver to obtain a fully audited
> \(p^{1/2-\varepsilon}\) ECDLP algorithm for some fixed \(\varepsilon>0\)?

A negative result must name a computational class.  Exact separated-linear
sketches, exact tensor-network cuts, random-sum-oracle algorithms, and
expanded recursive-resultant solvers admit the lower bounds below.  Compact
rational circuits show that bounded monomial support or naive partition width
is not the right general measure.  The central open step is an
algebraic preprocessing/query direct-sum theorem for \(m\ge3\).

Success means either:

1. a classical algorithm with total cost \(p^{1/2-\varepsilon}\), counting
   curve-specific preprocessing, relation extraction, rationality filtering,
   rank loss, and final linear algebra; or
2. a theorem giving \(p^{1/2-o(1)}\) time or time--memory cost for the stated
   non-materializing classes.

Small-field timings and a first-fall-degree conjecture do not meet either
criterion.

### Current theorem ledger and explicit nonclaim

The status of the program is:

| Result | Status | Scope |
|---|---|---|
| Theorems 1--2 | unconditional | exact separated sketches, tensor cuts, and one-sided rectangle filters |
| Theorem 3 | unconditional in its oracle model | target-adaptive evaluation of a random tuple-sum map |
| Theorems 5--10 | unconditional algebraic/counting statements | expanded Boolean inverse certificates, including curve-dependent rational-lift filtering below \(p^{1/4}\) |
| Propositions 11--12 | exact counterexamples to an overbroad inference | dense normal forms may have small rational circuits; two-leaf elimination is soft-linear |
| Theorem 13 | unconditional under uniform-target relation collection | every two-summand solver, regardless of its internal representation |
| Theorem 14 | unconditional for the stated representation class | multiplicity-preserving expanded resultants and explicitly supplied multipoint queries |
| Propositions 15--16 | exact reductions and scope boundaries | succinct eliminant trees and group-algebra convolution |
| Theorem 17 | unconditional density-one statement for fixed \(m\) | sparse-regime \(m\)-sum expansion and the resulting exact separated-sketch barrier |
| Theorem 18 | unconditional in the stated static-provenance model | product-tree preprocessing plus fully adaptive query branching |
| Proposition 19 | exact reduction plus literature audit | split-tree recovery is group \(k\)SUM-Indexing; current indexing upper bounds remain above one-half after relation accounting |
| Theorem 20 | exact exponent criterion | piecewise sub-rho conditions for preprocessing exponent \(s\) and amortized batch-query exponent \(t\) |
| Theorem 21 | unconditional bucket/quotient obstruction | arbitrary exact separate-label predicates, exact compositional buckets, affine algebraic-group linearizations, and low-degree isogeny quotients cannot compress the prime-order subgroup |
| Theorem 22 | unconditional one-sided label trade-off | compressing either partial-sum label set or the attainable targets forces a quantified false-verification blowup |
| Proposition 23 | exact target-dependent construction and audit | the map \(P\mapsto x(T-P)\) leaves only a sign ambiguity, but its scan/preprocessing balance remains strictly above one-half |
| Theorem 24 | unconditional in the degree-explicit selector model | target-adaptive algebraic leaf correspondences satisfy a preprocessing/query direct sum and hence a square-root relation barrier |
| Proposition 25 | unconditional compact-degree counterexample | multiplication-by-\(k\) has degree \(k^2\) but an \(O(\log k)\) group-law DAG, so expanded degree cannot be charged as compact-circuit work |
| Theorem 26 | unconditional in the explicit global-map catalogue model | compact single-valued rational branches satisfy a degree-free square-root direct sum |
| Proposition 27 | exact finite-domain boundary | arbitrary subgroup-point selectors exist in the reduced evaluation algebra, so global-map rigidity cannot extend without a compression/query hypothesis |
| Theorem 28 | unconditional in the symbolic straight-line recovery model | rational recovery from a succinct correspondence produces a global selector section and inherits Theorem 26's square-root direct sum |
| Proposition 29 | exact finite-field root-recovery counterexample | compact exponentiation recovers roots on many finite-field targets even when the generic correspondence has no rational section |
| Proposition 30 | exact finite-field hit-bound counterexample | a logarithmic-size power map sends every target into a fixed small set, so individual-leaf factor-base hits cannot support the final barrier |
| ARQP direct-sum conjecture | **open for \(m\ge3\)** | compact shared circuits, implicit evaluation points, batching, and witness recovery |

In particular, this manuscript does **not** claim an unconditional
\(p^{1/2-o(1)}\) lower bound for unrestricted arithmetic or rational circuits.
Such a claim is not obtained from degree, Bézout numbers, dense normal forms,
or the experiments.  The proved \(m\ge3\) barrier is the expanded-resultant [pcc:unverified]
bound of Theorem 14; the broader ARQP statement remains the successor target.

## 2. Canonical instance family

Let \(G=\langle P\rangle\subseteq E(\mathbb F_p)\) have prime order
\(n=\Theta(p)\). Fix \(N=2^k=p^{\alpha+o(1)}<p\) and encode

\[
F_k=\left\{\sum_{j=0}^{k-1}2^j b_j:b_j\in\{0,1\}\right\}
=\{0,\ldots,N-1\}.
\]

Only values lifting to points of \(G\) are retained, with a public
deterministic sign for each lift. Each factor-base coordinate has the faithful
quadratic encoding

\[
b_j^2-b_j=0,\qquad x=\sum_j2^j b_j.
\]

For \(m\) leaves, use a balanced tree of \(S_3\) equations and saturate all
exceptional addition denominators. The target \(R=aP+bQ\) is uniform in \(G\),
and its known label is \((a,b)\), not its discrete logarithm.

The relation-finding circuit receives:

- fixed public inputs \((p,E,F_k,m)\);
- target-dependent input \(x(R)\), or the full affine coordinates of \(R\);
- \(mk\) Boolean leaf variables and \(O(m)\) internal addition variables.

It must return a verified tuple of factor-base points, not merely decide that [pcc:unverified]
an algebraic-closure solution exists.

## 3. Exact incidence object

Let \(\Omega\) be the set of admissible unordered \(m\)-tuples of factor-base
points. Repetitions may be allowed. Define

\[
\sigma:\Omega\to G,\qquad \sigma(\omega)=\sum_{B\in\omega}B,
\]

and define the leaf-to-target incidence matrix over an arbitrary field \(K\):

\[
M_\sigma[\omega,R]=
\begin{cases}
1,&\sigma(\omega)=R,\\
0,&\text{otherwise.}
\end{cases}
\]

The corresponding order-\((m+1)\) tensor is

\[
\mathcal T(u_1,\ldots,u_m,R)
=\mathbf 1[\phi(u_1)+\cdots+\phi(u_m)=R].
\]

The matrix is the flattening that groups every leaf leg against the target
leg.

## 4. First exact extension beyond quotient materialization

### Theorem 1 (incidence-rank theorem)

For every field \(K\),

\[
\operatorname{rank}_K(M_\sigma)=|\operatorname{im}\sigma|.
\]

#### Proof

Every nonzero column is the indicator vector of one fibre
\(\sigma^{-1}(R)\). Distinct columns have disjoint, nonempty supports, so they
are linearly independent over every field. Columns outside the image are zero.
\(\square\)

### Corollary 1 (separated-sketch barrier)

Suppose an exact preprocessing/query method represents the incidence predicate
as

\[
M_\sigma[\omega,R]=\sum_{i=1}^{r}A_i(\omega)B_i(R).
\]

Then \(r\ge|\operatorname{im}\sigma|\). Thus, when the \(m\)-sum map covers
\(p^{1-o(1)}\) targets, a global exact separated sketch has dimension
\(p^{1-o(1)}\).

### Corollary 2 (tensor-network cut barrier)

In any exact tensor-network representation of \(\mathcal T\), the product of
bond dimensions across a cut separating all leaf legs from the target leg is
at least \(|\operatorname{im}\sigma|\).

#### Proof

Contracting either side of the cut gives a matrix factorization of the
leaf--target flattening through a vector space whose dimension is the product
of the cut bond dimensions. Apply Theorem 1. \(\square\)

### Theorem 2 (one-sided rectangle/filter trade-off)

Let \(s=|\operatorname{im}\sigma|\). Suppose a monotone filter is a union of
\(r\) combinatorial rectangles \(A_j\times B_j\) in
\(\Omega\times G\), and suppose it accepts every genuine incidence pair. It may
also accept false pairs, which are subsequently verified. Let \(F\) be the [pcc:unverified]
number of false pairs accepted after restricting to one representative leaf
tuple for each attainable target. Then

\[
F\ge \frac{s^2}{r}-s,\qquad\text{equivalently}\qquad
r(F+s)\ge s^2.
\]

In particular, charging one unit per stored rectangle and one unit per false
verification gives

\[
r+F\ge s.
\]

The same conclusion is distributional. If a randomized monotone filter covers
each genuine representative pair with probability at least \(1-\delta\), and
\(\mathbf r,\mathbf F\) are its random rectangle count and false-pair count,
then

\[
\mathbb E[\mathbf r+\mathbf F]\ge(1-\delta)s.
\]

#### Proof

Choose one representative \(\omega_T\in\sigma^{-1}(T)\) for every attainable
target. The induced \(s\times s\) incidence submatrix is the identity matrix.
Assign every diagonal entry to one rectangle covering it, and let \(d_j\) be
the number assigned to rectangle \(j\). If two distinct diagonal entries
\((\omega_T,T)\) and \((\omega_U,U)\) are assigned to the same rectangle, that
rectangle also accepts the two false cross-pairs
\((\omega_T,U)\) and \((\omega_U,T)\). Because assigned groups are disjoint,
these give at least

\[
\sum_j d_j(d_j-1)\ge \frac{(\sum_jd_j)^2}{r}-\sum_jd_j
=\frac{s^2}{r}-s
\]

distinct false pairs. The last inequality follows by minimizing
\(r+F\) subject to the displayed trade-off, or directly from
\(r+s^2/r-s\ge s\).

For a randomized realization that covers only \(c\) of the chosen diagonal
pairs, the identical argument on those \(c\) pairs gives \(r+F\ge c\).
Taking expectations and summing the per-pair coverage probabilities gives the
distributional statement. \(\square\)

This theorem covers exact monotone DNF/rectangle filters and one-sided filters
whose false positives are explicitly verified, including randomized filters [pcc:unverified]
with bounded false-negative probability. It does not cover cancellation
between signed or field-valued terms or adaptive rectangles selected after the
target is known. Those are the next boundaries.

### Theorem 3 (adaptive random-sum oracle barrier)

Let \(\Omega\) be any candidate domain, let \(|G|=n\), and choose
\(f:\Omega\to G\) uniformly at random. An algorithm may:

1. evaluate \(f\) on candidates before targets are known;
2. receive \(B\) distinct uniform targets in \(G\);
3. choose every subsequent candidate adaptively from the targets and its full
   previous transcript; and
4. reuse every evaluated value against every target.

Let \(D\) be the total number of distinct candidate evaluations, possibly a
stopping-time random variable. If \(Z\) is the number of reported
candidate--target pairs \(f(\omega)=T_j\), then

\[
\mathbb E Z\le \frac{B\,\mathbb E D}{n}.
\]

Consequently, if expected materialized work is
\(C=B+\mathbb E D\), producing \(N\) expected relations requires

\[
C\ge 2\sqrt{Nn}.
\]

If \(D\le|\Omega|\le p^{m\alpha+o(1)}\) and
\(N=p^{\alpha+o(1)}\), optimizing \(\alpha\) again gives the fixed-\(m\)
exponent \(m/(2m-1)>1/2\).

#### Proof

Conditioned on the entire preprocessing transcript, each precomputed candidate
value belongs to the subsequently sampled target set with probability \(B/n\);
linearity of expectation applies even when several candidates have the same
output.
Condition on the targets and the transcript immediately before a new,
previously unevaluated candidate is queried. Because \(f\) is a random
function, its value at that candidate is still uniform in \(G\), regardless of
how the candidate was selected. Since the targets are distinct, the new value
matches one of them with probability \(B/n\). Summing these conditional
expectations up to any bounded stopping time, then passing to an integrable
limit, gives

\[
\mathbb E Z=\frac{B}{n}\mathbb E D.
\]

If the algorithm suppresses some matches, this becomes an inequality.
Finally,

\[
B\mathbb E D\le\frac{(B+\mathbb E D)^2}{4}=\frac{C^2}{4}.
\]

Requiring \(\mathbb E Z\ge N\) proves the square-root bound. The domain cap [pcc:unverified]
gives the same two-branch optimization as the batched candidate theorem:
balance occurs at
\(\alpha=1/(2m-1)\). \(\square\)

The theorem allows full target adaptivity, arbitrary control flow, and shared
preprocessing, but only black-box evaluation of a random sum map. Therefore a
sub-square-root coordinate attack must exploit a demonstrable algebraic
property distinguishing the elliptic restricted-sum map from this oracle. A
claim that merely chooses candidates adaptively, without identifying such a
property, cannot change the exponent.

### Theorem 4 (exact limit of target-specific rank)

Let \(G\) be any finite abelian group of order \(n\), and let
\(A,B\subseteq G\) be distinct partial-sum supports. For \(T\in G\), define the
\(|A|\times|B|\) matrix

\[
H_T[a,b]=\mathbf 1[a+b=T].
\]

Over every coefficient field,

\[
\operatorname{rank}(H_T)
=r_T:=|\{(a,b)\in A\times B:a+b=T\}|.
\]

Moreover,

\[
\sum_{T\in G}\operatorname{rank}(H_T)=|A||B|,
\qquad
\mathbb E_{T\leftarrow G}\operatorname{rank}(H_T)=\frac{|A||B|}{n}.
\]

For any target set \(U\subseteq G\), the block-diagonal direct sum satisfies

\[
\operatorname{rank}\left(\bigoplus_{T\in U}H_T\right)
=\sum_{T\in U}r_T.
\]

#### Proof

For fixed \(T\), each row of \(H_T\) contains at most one nonzero entry, at
\(b=T-a\), and each column contains at most one, at \(a=T-b\). After deleting
zero rows and columns, \(H_T\) is a permutation matrix of size \(r_T\), proving
the first formula. Every pair \((a,b)\in A\times B\) contributes to exactly one
target, namely \(T=a+b\), which proves the sum and expectation identities. [pcc:unverified]
Ranks add across a block diagonal matrix. \(\square\)

### Consequence: ordinary rank cannot close the sparse regime

In the balanced constant-success regime, the distinct partial supports
typically satisfy \(|A||B|=\Theta(n)\). Theorem 4 then gives only

\[
\mathbb E_T\operatorname{rank}(H_T)=\Theta(1).
\]

Thus an ordinary matrix-rank lower bound on one specialized target cannot
prove \(p^{1/2-o(1)}\) query cost: the exact rank is often constant precisely [pcc:unverified]
where relation probability becomes constant. Aggregating targets makes the
rank grow only with the expected number of partial-sum matches, not with the
cost of finding them.

This is an exact limitation of proof Route A, not a heuristic about elliptic
sums. Any successful lower bound must measure search, preprocessing/query
trade-offs, cancellation complexity, proof complexity, or dynamic
repartitioning---something strictly stronger than the rank of \(H_T\).

### Theorem 5 (random Boolean inverse-certificate density)

Let

\[
\mathcal B_r=
\mathbb F_p[b_1,\ldots,b_r]/(b_1^2-b_1,\ldots,b_r^2-b_r).
\]

Evaluation identifies \(\mathcal B_r\) with all functions
\(\{0,1\}^r\to\mathbb F_p\). Choose \(f\) uniformly from the unit group
\(\mathcal B_r^\times\); equivalently, choose every value \(f(x)\)
independently and uniformly from \(\mathbb F_p^\times\). Write the unique
multilinear inverse as

\[
f^{-1}=\sum_{S\subseteq[r]}c_S\prod_{i\in S}b_i.
\]

Then:

1. with probability at least \(1-2/(p-1)\), at least \(2^{r-1}\) of all
   coefficients \(c_S\) are nonzero;
2. with probability at least \(1-2/(p-1)\), at least \(2^{r-2}\) nonzero
   coefficients have degree at least \(\lceil r/2\rceil\).

Consequently, both statements hold simultaneously with probability at least
\(1-4/(p-1)\).

#### Proof

Put \(g=f^{-1}\). Its values \(g(x)\) are again independent and uniform in
\(\mathbb F_p^\times\). Möbius inversion on the Boolean cube gives

\[
c_S=\sum_{T\subseteq S}(-1)^{|S|-|T|}g(1_T),
\]

where \(1_T\) has support \(T\). For nonempty \(S\), condition on all summands
except one. At most one value of the remaining uniform
\(\mathbb F_p^\times\) variable can make \(c_S=0\), so

\[
\Pr[c_S=0]\le\frac1{p-1}.
\]

The constant coefficient is \(g(0)\ne0\). Hence the expected number of zero
coefficients among all \(2^r\) monomials is at most \(2^r/(p-1)\).
Markov's inequality shows that the probability of at least \(2^{r-1}\) zeros
is at most \(2/(p-1)\), proving the first claim.

Let \(\mathcal H=\{S:|S|\ge\lceil r/2\rceil\}\). Its size is at least
\(2^{r-1}\). The same calculation and Markov's inequality show that, except
with probability \(2/(p-1)\), fewer than half its coefficients vanish. Thus at
least \(2^{r-2}\) high-degree coefficients are nonzero. A union bound proves [pcc:unverified]
the simultaneous statement. \(\square\)

### Corollary 3 (monomial-explicit refutation barrier)

Suppose an insoluble Boolean equation \(f=0\) is refuted by

\[
a f+\sum_{i=1}^{r}h_i(b_i^2-b_i)=1.
\]

After reduction modulo the Boolean equations, \(\bar a=f^{-1}\) is unique.
For random \(f\), any certificate that explicitly stores \(\bar a\) in the
multilinear monomial basis therefore contains
\(\Omega(2^r)\) monomials with overwhelming probability.

For \(m\) factor-base leaves with \(N=2^k=p^{\alpha+o(1)}\), one has
\(r=mk\) and

\[
2^r=N^m=p^{m\alpha+o(1)}.
\]

At the optimized exponent \(\alpha=1/(2m-1)\), this is
\(p^{m/(2m-1)+o(1)}\), exactly the earlier fixed-\(m\) frontier; in the
constant-success regime \(\alpha=1/m\), it is \(p^{1+o(1)}\).

This corollary covers certificate-producing, monomial-explicit polynomial
calculus, XL, and Macaulay-style elimination. It does not cover a compact
arithmetic circuit for a dense multilinear polynomial. Applying it to the
actual Semaev restriction requires proving that its reciprocal Möbius
coefficients have comparable density; random-function behaviour is a benchmark,
not an assumption about the elliptic system.

### First restricted-\(S_3\) sanity check

The accompanying Semaev inverse profiler was run on

\[
E/\mathbb F_{101}:y^2=x^3+7x+4,
\qquad |E(\mathbb F_{101})|=97,
\]

using the eight rational factor-base coordinates

\[
[0,3,4,10,13,16,17,20].
\]

The two leaf indices use six Boolean variables in total. Among all 48 rational
target \(x\)-coordinates, six made the restricted
\(S_3(x_1,x_2,x_T)\) a unit on the 64-point Boolean cube. Their unique inverse
certificates had:

- total support between 62 and 64 of the 64 multilinear monomials;
- full degree six;
- between 41 and 42 nonzero monomials of degree at least three.

All certificates passed pointwise round-trip verification. This is evidence
that the smallest tested restricted Semaev instance resembles the dense random
benchmark, but it is not asymptotic evidence and is not used in any theorem.
The next experiments must vary \(p\), \(k\), the curve, the factor-base map,
and the number of leaves.

### Multi-curve adversarial scan

The scan was extended to nine primes

\[
101,103,107,109,127,131,137,139,149,
\]

with two non-anomalous prime-order curves per prime and Boolean-indexed factor
bases of size four and eight. This gave 36 curve/base-size cases and 1,200
unit target polynomials.

For base size four (four Boolean variables), 870 unit targets were found:

- the weakest support was \(13/16=81.25\%\);
- the mean, first averaged within a case and then across 18 cases, was
  \(99.23\%\);
- every case had at least nine high-degree terms.

For base size eight (six Boolean variables), 330 unit targets were found:

- the weakest support was \(60/64=93.75\%\);
- the corresponding mean support fraction was \(99.20\%\);
- every case had at least 38 high-degree terms.

No compact certificate was found. The weakest case was
\(E/\mathbb F_{137}:y^2=x^3+3x+9\), whose group has order 127, with base size
four and target \(x=67\); its certificate used 13 of 16 monomials. These data
support further investigation of reciprocal density, but they still cover
only \(S_3\), two leaves, very small fields, and one deterministic choice of
factor-base coordinates. They cannot justify an asymptotic randomness
assumption.

The base-size-16 scan used 43 primes from 251 through 503, one prime-order
curve per prime. It found 1,969 unit targets. The weakest certificate used
\(248/256=96.875\%\) of the monomials, while the mean of the 43 case means was
\(99.73\%\). Again, this is diagnostic rather than asymptotic evidence.

### Theorem 6 (fixed-size generic full support)

Fix \(q=2^k\), formal factor-base coordinates
\(z_0,\ldots,z_{q-1}\), curve coefficients \(A,B\), and target \(t\). Put

\[
F_{ij}=S_3(z_i,z_j,t)
\]

and work on the parameter open set where the curve discriminant and every
\(F_{ij}\) are nonzero. Index \(i,j\) by \(2k\) Boolean bits. The pointwise
inverse table \(F_{ij}^{-1}\) has a unique multilinear expansion

\[
\sum_{S\subseteq[2k]}C_S(A,B,t,z_0,\ldots,z_{q-1})\,b_S,
\]

whose coefficients are rational functions of the parameters.

For \(q\in\{4,8,16\}\), the set on which every \(C_S\ne0\) is a nonempty
Zariski-open subset over \(\mathbb Q\), and after reduction over
\(\mathbb F_\ell\) for all but finitely many primes \(\ell\). Thus the
restricted-\(S_3\) inverse has full multilinear support generically at these
three factor-base sizes.

#### Proof

Each \(C_S\) is the Boolean Möbius sum of the \(1/F_{ij}\), hence a rational
function with integer coefficients. It suffices to exhibit one specialization
at which every denominator and every \(C_S\) is nonzero; then the product of
their numerators is a nonzero integer polynomial. Its nonvanishing locus,
intersected with the denominator and discriminant open sets, is nonempty and
Zariski open. A polynomial nonzero modulo one prime is nonzero over
\(\mathbb Q\), and its reduction remains nonzero outside the finite set of
primes dividing all relevant coefficients.

Exact witnesses, verified by the accompanying Möbius-transform code, are: [pcc:unverified]

1. \(q=4\): \(p=101\), \(A=7\), \(B=4\),
   \((z_i)=[0,3,4,10]\), \(t=0\), support \(16/16\);
2. \(q=8\): the same curve,
   \((z_i)=[0,3,4,10,13,16,17,20]\), \(t=21\), support \(64/64\);
3. \(q=16\): \(p=251\), \(A=1\), \(B=4\),
   \[
   (z_i)=[0,7,8,9,11,14,15,16,18,19,20,25,26,28,29,31],
   \]
   \(t=85\), support \(256/256\).

Every listed coordinate is the \(x\)-coordinate of a rational point on the
listed curve, and the restricted polynomial is a unit on its Boolean cube.
\(\square\)

This theorem proves that no coefficient identity forces sparsity at these [pcc:unverified]
sizes. It does not prove uniform support bounds for structured intervals as [pcc:unverified]
\(k\to\infty\): the formal coordinates vary independently in the generic
parameter space, whereas an interval or small-height factor base imposes
strong arithmetic specialization. The asymptotic task is to make the
exceptional hypersurfaces explicit enough to show that the structured
specializations avoid them, or to exploit a specialization that does not.

### Theorem 7 (generic full support for every Boolean lookup size)

Let \(p>3\), \(q=2^k\), and let \(z_0,\ldots,z_{q-1}\) be algebraically
independent factor-base coordinate parameters. In the universal nonsingular
short-Weierstrass family, put

\[
F_{ij}=S_3(z_i,z_j,t).
\]

On the open set where all \(F_{ij}\ne0\), index the table
\((F_{ij}^{-1})_{0\le i,j<q}\) by two \(k\)-bit blocks and take its Boolean
Möbius transform. Every one of the \(2^{2k}=q^2\) multilinear coefficients is
a nonzero rational function. Consequently the inverse has full multilinear
support on a nonempty Zariski-open dense subset for every \(k\).

#### Proof

For a coefficient indexed by a bit set \(S\), Möbius inversion gives

\[
C_S=\sum_{T\subseteq S}
(-1)^{|S|-|T|}\frac1{F_{i(T),j(T)}},
\]

where \(i(T)\) and \(j(T)\) are the integers encoded by the selected bits in
the two blocks.

Group equal denominators using the symmetry \(F_{ij}=F_{ji}\). An unordered
off-diagonal pair \(\{i,j\}\) occurs at most twice. When both orientations
occur, their signs are equal because swapping the two bit blocks preserves
\(\operatorname{wt}(i)+\operatorname{wt}(j)\). Its grouped coefficient is
therefore \(\pm2\), not zero because \(p>3\). A diagonal denominator has
coefficient \(\pm1\).

For \(S=\varnothing\), \(C_S=1/F_{00}\ne0\). For every nonempty \(S\), the
subcube contains an off-diagonal pair: activate one selected bit in its block
and leave the other block zero.

The standard absolute irreducibility of \(S_3(X,Y,t)\) on the universal
nonsingular curve implies that the polynomials \(F_{ij}\), for \(i\ne j\),
are irreducible. Distinct unordered pairs give nonassociate polynomials,
because their sets of independent \(z\)-variables differ. No such
off-diagonal polynomial divides a diagonal \(F_{\ell\ell}\), which omits at
least one of its two independent variables.

Choose an off-diagonal denominator \(F_{ij}\) occurring in \(C_S\), multiply
the alleged identity \(C_S=0\) by the product of all distinct denominators,
and reduce modulo \(F_{ij}\). Every term vanishes except

\[
\pm1\cdot\prod_{e\ne\{i,j\}}F_e
\quad\text{or}\quad
\pm2\cdot\prod_{e\ne\{i,j\}}F_e.
\]

This is nonzero modulo \(F_{ij}\), since \(F_{ij}\) divides none of the other
factors and the scalar is nonzero. Hence \(C_S\ne0\). The simultaneous
nonvanishing locus of the finitely many coefficient numerators and
denominators is therefore nonempty and Zariski open. \(\square\)

### Theorem 8 (generic full support for every fixed residue factor base)

Let \(p>3\), \(q=2^k<p\), and let
\(c_0,\ldots,c_{q-1}\in\mathbb F_p\) be distinct fixed residues.  This includes
the interval specialization \(c_i=i\), any fixed small-height list, and any
fixed subset selected in advance.  In the universal short-Weierstrass family,
put

\[
F_{ij}(A,B,t)=S_3(c_i,c_j,t).
\]

On the open set where the curve is nonsingular and all \(F_{ij}\ne0\), index
\((F_{ij}^{-1})_{i,j}\) by two \(k\)-bit blocks.  Every coefficient in its
Boolean Möbius transform is a nonzero rational function in \(A,B,t\).
Consequently, for this fixed residue list the inverse table has full
multilinear support on a nonempty Zariski-open subset of the three-parameter
curve--target space.

#### Proof

For a fixed unordered pair write

\[
s=c_i+c_j,\qquad u=c_ic_j,\qquad d^2=(c_i-c_j)^2=s^2-4u.
\]

The explicit third summation polynomial gives

\[
\begin{aligned}
F_{ij}
={}&A^2+A(-2st-2u)\\
 &+d^2t^2-2sut+u^2-4B(t+s).
\end{aligned}
\]

Regard this as a polynomial in \(B\) over the UFD
\(\mathbb F_p[A,t]\).  It has \(B\)-coefficient \(-4(t+s)\).  At the only root
\(t=-s\) of this coefficient, its \(B\)-free part is

\[
(A+s^2-u)^2,
\]

a nonzero polynomial.  Thus its two coefficients are coprime; \(F_{ij}\) is
primitive and linear in \(B\), hence irreducible in
\(\mathbb F_p[A,B,t]\).

The polynomials belonging to distinct unordered pairs are nonassociate.
Indeed, the coefficient of \(A^2\) forces the associate scalar to be \(1\);
the coefficient of \(B\) then forces equality of the sums \(s\), and the
coefficient of \(A\) forces equality of the products \(u\).  The two residues
are the roots of \(X^2-sX+u\), so the unordered pairs are equal.  Distinctness
of the \(c_i\) makes the index pairs equal up to orientation.

Now take any Boolean Möbius coefficient

\[
C_S=\sum_{T\subseteq S}
(-1)^{|S|-|T|}\frac1{F_{i(T),j(T)}}.
\]

After grouping the two orientations, every diagonal denominator has scalar
\(\pm1\), while every off-diagonal denominator has scalar \(\pm1\) or
\(\pm2\).  If both orientations occur, their signs agree because exchanging
the bit blocks preserves the total Hamming weight.  These scalars are nonzero
as \(p>3\).

Choose any grouped denominator occurring in \(C_S\), clear all distinct
denominators, and reduce modulo that irreducible factor.  Pairwise
nonassociation kills every other summand and leaves its nonzero scalar times
the product of the other factors.  Hence \(C_S\ne0\).  This also covers
\(S=\varnothing\), where \(C_S=1/F_{00}\).  Simultaneous nonvanishing of the
finitely many coefficient numerators, denominators, and the discriminant
defines the asserted nonempty Zariski-open set. \(\square\)

### Theorem 9 (quantitative finite-field full support below \(p^{1/4}\))

Under the hypotheses of Theorem 8, the proportion of triples
\((A,B,t)\in\mathbb F_p^3\) for which the curve is singular, some \(F_{ij}\)
vanishes, or some Boolean Möbius coefficient vanishes is at most

\[
\frac{2q^4}{p}.
\]

In particular, if \(q=o(p^{1/4})\), full support holds for a
\(1-o(1)\) proportion of curve--target triples.

#### Proof

There are \(D=q(q+1)/2\) distinct unordered denominators.  Each \(F_{ij}\)
has total degree at most \(2\) in \(A,B,t\).  Clear the distinct denominators
occurring in a Möbius coefficient \(C_S\).  Its resulting numerator has total
degree at most \(2(D-1)\), and it is a nonzero polynomial by Theorem 8.

Let \(H\) be the product of the discriminant, the \(D\) distinct
denominators, and one such nonzero common numerator for each of the \(q^2\)
Möbius coefficients.  Since \(\mathbb F_p[A,B,t]\) is a domain, \(H\ne0\), and

\[
\begin{aligned}
\deg H
&\le 3+2D+2q^2(D-1)\\
&=q^4+q^3-q^2+q+3
<2q^4
\end{aligned}
\]

for \(q\ge2\).  The Schwartz--Zippel bound gives fewer than
\(2q^4p^2\) zeros in \(\mathbb F_p^3\), proving the first two statements.

Write \(\varepsilon=2q^4/p\).  Averaging the exceptional-target proportion
over \((A,B)\), then applying Markov's inequality at threshold
\(\sqrt\varepsilon\), shows that all but at most \(\sqrt\varepsilon\) of the
curve parameters have exceptional-target proportion at most
\(\sqrt\varepsilon\).  This is \(O(q^2p^{-1/2})\); the discriminant factor has
already charged singular curves as entirely exceptional. \(\square\)

At the materializing balance \(\alpha=1/(2m-1)\), the exceptional proportion
in Theorem 9 is

\[
O\!\left(p^{\,4/(2m-1)-1+o(1)}\right).
\]

For \(m=3,4,5\) these exponents are respectively
\(-1/5\), \(-3/7\), and \(-5/9\).  Thus the finite-field genericity statement
is already asymptotically effective at all three sanity-check scales.

### Theorem 10 (uniformity under curve-dependent rational-lift filtering)

Let \(p>3\), and fix a master list
\(C=(c_0,\ldots,c_{L-1})\) of distinct residues in \(\mathbb F_p\).  For all
but a proportion at most

\[
\frac{4L^4+3}{p}
\]

of coefficient pairs \((A,B)\in\mathbb F_p^2\), the curve is nonsingular and
the following statement holds simultaneously for **every** subset of \(C\).

Choose any \(q=2^k\) distinct elements \(d_0,\ldots,d_{q-1}\) of that subset
and put

\[
G_{ij}(t)=S_3(d_i,d_j,t).
\]

Then all distinct unordered-pair polynomials \(G_{ij}\) are pairwise coprime
in \(\mathbb F_p[t]\).  Consequently, outside a set of fewer than \(2q^4\)
target values \(t\in\mathbb F_p\), the table \((G_{ij}(t)^{-1})\) is defined
and its Boolean Möbius transform has all \(q^2\) coefficients nonzero.

In particular, the statement applies to the curve-dependent subset

\[
C_E=\{c\in C:c^3+Ac+B\text{ is a square in }\mathbb F_p\},
\]

and hence to any power-of-two sublist of the rationally liftable interval
coordinates.

#### Proof

For every unordered master pair \(e=\{i,j\}\), write

\[
F_e(A,B,t)=S_3(c_i,c_j,t).
\]

Theorem 8 proves that the \(F_e\) are irreducible and pairwise nonassociate in [pcc:unverified]
\(\mathbb F_p[A,B,t]\).  They are primitive as polynomials in \(t\) over
\(\mathbb F_p[A,B]\): for \(i\ne j\), the coefficient of \(t^2\) is the
nonzero constant \((c_i-c_j)^2\); for \(i=j\), primitivity also follows from
irreducibility.  Gauss's lemma therefore shows that two distinct \(F_e,F_f\)
are coprime over \(\mathbb F_p(A,B)[t]\).  Their resultant

\[
R_{ef}(A,B)=\operatorname{Res}_t(F_e,F_f)
\]

is consequently a nonzero polynomial.

Each \(F_e\) has \(t\)-degree at most \(2\), and every coefficient has total
\((A,B)\)-degree at most \(2\).  The Sylvester determinant gives
\(\deg R_{ef}\le8\).  There are
\(D=L(L+1)/2\le L^2\) unordered master pairs.  Schwartz--Zippel and a union
bound show that the number of coefficient pairs on which some distinct
denominators acquire a common factor is at most

\[
8\binom D2p<4L^4p.
\]

The discriminant contributes at most another \(3p\) coefficient pairs.
Dividing by \(p^2\) proves the asserted curve-exception bound.  Pairwise [pcc:unverified]
coprimality passes to every subset, so no conditioning on the rational-lift
pattern is needed.

Fix a nonexceptional curve and one enumerated \(q\)-element subset.  In any
Boolean Möbius coefficient, group the two orientations of each unordered
pair.  As in Theorem 8, the resulting scalar is \(\pm1\) or \(\pm2\), hence
nonzero.  Clear the distinct denominators and reduce modulo any irreducible
factor of one chosen denominator.  Pairwise coprimality makes every other
term vanish and leaves the chosen nonzero scalar times a product not divisible
by that factor.  Thus every coefficient is a nonzero rational function of
\(t\).

With \(D_q=q(q+1)/2\), each cleared coefficient numerator has \(t\)-degree at
most \(2(D_q-1)\).  The union of the roots of all \(q^2\) numerators and all
denominators therefore has size at most

\[
2q^2(D_q-1)+2D_q
=q^4+q^3-q^2+q
<2q^4.
\]

Every other target has a defined inverse table with full Boolean support.
\(\square\)

If \(L=p^{\alpha+o(1)}\) with \(\alpha<1/4\), Theorem 10 gives a
density-\(1-o(1)\) family of curves and, on each such curve, a
density-\(1-o(1)\) family of targets.  Since a nonsingular elliptic curve has
\(\Theta(p)\) rational target \(x\)-coordinates, intersecting with rational
targets changes only the implied constant in the exceptional proportion.

The required number of rational lifts also follows without a distributional
heuristic.  For \(c\in C\), let

\[
I_c(A,B)=\mathbf 1[c^3+Ac+B\text{ is a square, including zero}].
\]

For distinct \(c,d\), the affine map

\[
(A,B)\longmapsto(c^3+Ac+B,\ d^3+Ad+B)
\]

is a bijection of \(\mathbb F_p^2\), since its linear part has determinant
\(c-d\ne0\).  Hence the \(I_c\) are pairwise independent under uniform
\((A,B)\), with mean \((p+1)/(2p)\) and variance at most \(1/4\).  Therefore

\[
\Pr\!\left[
\left|\sum_{c\in C}I_c-\frac{p+1}{2p}L\right|\ge\delta L
\right]
\le\frac{1}{4\delta^2L}.
\]

For every fixed \(\eta>0\), taking \(L=(2+\eta)q+O(1)\) and a sufficiently
small constant \(\delta=\delta(\eta)>0\) shows that at least \(q\) master
coordinates lift for all but \(O_\eta(1/q)\) of coefficient pairs.  Combining
this with Theorem 10 proves that, for \(q=p^{\alpha+o(1)}\) and [pcc:unverified]
\(0<\alpha<1/4\), a density-\(1-o(1)\) family of curves has a rationally
lifted \(q\)-element Boolean factor base whose restricted \(S_3\) inverse has
full support for all but \(O(q^4)\) targets.

### Proposition 11 (dense normal form does not imply a large rational circuit)

For the interval encoding

\[
x(b_0,\ldots,b_{k-1})=\sum_{r=0}^{k-1}2^rb_r,\qquad q=2^k,
\]

put

\[
f_t(b,c)=S_3(x(b),x(c),t).
\]

On any target for which \(f_t\) is nonzero on the Boolean cube, the pointwise
inverse \(f_t^{-1}\) is computed by an arithmetic circuit of size \(O(k)\)
with one inversion gate.  Nevertheless, on the curves and targets covered by
Theorems 9 and 10, its unique multilinear normal form has all \(q^2\)
monomials.

#### Proof

Two linear circuits of size \(O(k)\) compute \(x(b)\) and \(x(c)\).  The fixed
formula for \(S_3\) then uses \(O(1)\) additions and multiplications, and one
inversion computes its reciprocal.  On the stated unit locus the inversion is
defined at every Boolean assignment.  The normal-form support assertion is
Theorem 9 or 10. \(\square\)

Thus normal-form density is a valid lower bound only for representations that
materialize that normal form, or for proof systems in which inversion must
itself be expanded.  It gives no lower bound for general rational circuits.

### Proposition 12 (near-linear two-leaf elimination)

Let \(F=\{c_0,\ldots,c_{q-1}\}\subset\mathbb F_p\) be explicit and distinct,
and precompute

\[
P_F(Y)=\prod_{c\in F}(Y-c)
\]

together with its subproduct tree.  For a target \(t\), define

\[
R_t(X)=\operatorname{Res}_Y(P_F(Y),S_3(X,Y,t))
      =\prod_{c\in F}S_3(X,c,t).
\]

Then \(\deg R_t\le2q\), and

\[
\gcd(P_F(X),R_t(X))\ne1
\]

if and only if there exist \(x,y\in F\) with \(S_3(x,y,t)=0\).  Using fast
polynomial multiplication, the product, gcd, and recovery of one such pair
cost \(\widetilde O(q)\) field operations per target after
\(\widetilde O(q)\) preprocessing.

#### Proof

The resultant identity is the root-product formula for the monic polynomial
\(P_F\).  Each factor is quadratic in \(X\), so a balanced product tree
constructs \(R_t\) in \(\widetilde O(q)\) operations.  Its gcd with \(P_F\)
has precisely the first coordinates participating in a zero pair.  The
stored subproduct tree identifies one root \(x\in F\); then
\(\gcd(P_F(Y),S_3(x,Y,t))\) identifies a corresponding \(y\).  Fast gcd and
remainder algorithms preserve the stated soft-linear bound. \(\square\)

This is not a sub-square-root ECDLP algorithm.  Under the uniform-sum model,
a two-leaf target succeeds with probability \(\Theta(q^2/p)\).  Collecting
\(\Theta(q)\) relations therefore takes \(\Theta(p/q)\) target trials, and
soft-linear work per trial gives \(\widetilde\Theta(p)\) relation-finding
work before linear algebra.  The proposition instead demonstrates that
dense quotient normal forms can conceal compact, exact, non-materializing
elimination.

### Theorem 13 (solver-independent two-summand square-root barrier)

Let \(G\) be a finite abelian group of order \(n\), let \(F\subseteq G\) have
size \(N\), and let \(T_1,\ldots,T_B\) be independent uniform targets.  An
algorithm may use arbitrary preprocessing and, after seeing the targets,
return any verified ordered pairs \((x,y)\in F^2\) with \(x+y=T_j\).  If with [pcc:unverified]
constant probability it returns \(r=\Theta(N)\) relations sufficient to have
rank \(\Theta(N)\) on the factor-base columns, then its charged target-input
and relation-output work is

\[
\Omega\!\left(N+\frac{n}{N}\right)=\Omega(\sqrt n).
\]

This lower bound is independent of the internal solving representation and
therefore includes product trees, resultants, arbitrary target batching, and
unbounded curve-specific preprocessing.

#### Proof

Let

\[
Z=\sum_{j=1}^B
|\{(x,y)\in F^2:x+y=T_j\}|
\]

be the total number of available ordered decompositions, whether or not the
algorithm outputs them.  Every ordered pair contributes to a uniform target
with probability \(1/n\), so exactly

\[
\mathbb E Z=\frac{BN^2}{n}.
\]

On an event of constant probability the algorithm outputs \(r=\Theta(N)\)
distinct verified relations, hence \(Z\ge r\) there and [pcc:unverified]
\(\mathbb E Z=\Omega(N)\).  Therefore \(B=\Omega(n/N)\).  Reading the \(B\)
target inputs costs \(\Omega(B)\), while writing \(r=\Theta(N)\) sparse
relations costs \(\Omega(N)\).  Their sum is
\(\Omega(N+n/N)\), whose minimum is \(2\sqrt n\).  Ignoring preprocessing and
all internal computation only weakens the bound. \(\square\)

For \(m\) summands, the identical entropy calculation gives

\[
\mathbb E Z_m=\frac{BN^m}{n},\qquad
B=\Omega\!\left(\frac{n}{N^{m-1}}\right),
\]

and hence only the input/output floor

\[
\Omega\!\left(N+\frac{n}{N^{m-1}}\right)
=\Omega(n^{1/m}).
\]

Thus \(m=2\) is closed even against unrestricted solvers, whereas for every
\(m\ge3\) the missing exponent lies entirely in the cost of finding or
certifying decompositions inside a target fibre.  This is the exact point at
which an ARQP direct-sum theorem must add information beyond relation
probability and output entropy.

### Theorem 14 (expanded recursive-resultant barrier)

Fix \(m\ge2\) and a factor base of size \(N\).  An
**\(r\)-leaf expanded-resultant solver**, \(1\le r\le m\), has the following
properties:

1. its target-independent phase materializes a multiplicity-preserving
   univariate norm, resultant, coefficient array, or subproduct tree with one
   leaf for every tuple in \(F^r\), at cost at least \(N^r\);
2. for each target it explicitly supplies, probes, or materializes every tuple
   of the remaining \(m-r\) leaves against that object, at cost at least
   \(N^{m-r}\) (batched multipoint evaluation is allowed but is charged at
   least once per explicitly supplied evaluation point); and
3. it returns verified relations and needs \(\Theta(N)\) independent rows. [pcc:unverified]

For uniform targets in a group of order \(n=\Theta(p)\), its total charged
work satisfies

\[
\begin{aligned}
T_r
&=\Omega\!\left(
N^r+\frac{p}{N^{m-1}}N^{m-r}
\right)\\
&=\Omega\!\left(N^r+\frac{p}{N^{r-1}}\right).
\end{aligned}
\]

After optimizing \(N=p^{\alpha+o(1)}\), this is
\(p^{r/(2r-1)-o(1)}\).  Optimizing also over \(r\le m\) gives

\[
T_{\rm expanded}
\ge p^{\,m/(2m-1)-o(1)}
>p^{1/2}.
\]

#### Proof

The first two costs are part of the representation definition.  The exact
decomposition expectation from Theorem 13 with \(m\) leaves shows that
\(\Omega(p/N^{m-1})\) uniform targets are required to obtain
\(\Theta(N)\) rows, even if every decomposition of a target is returned.
Multiplying by the per-target remaining-leaf cost gives

\[
N^r+\frac{p}{N^{m-1}}N^{m-r}
=N^r+\frac{p}{N^{r-1}}.
\]

Writing \(N=p^\alpha\), the two exponents are \(r\alpha\) and
\(1-(r-1)\alpha\).  Their maximum is minimized at
\(\alpha=1/(2r-1)\), with value \(r/(2r-1)\).  This value decreases with
\(r\), so \(r=m\) is optimal among the allowed cut depths. \(\square\)

For \(m=3\), the nontrivial cut depths \(r=2,3\) give respectively
\(p^{2/3}\) and \(p^{3/5}\).  For \(m=4,5\), the best expanded depths give
\(p^{4/7}\) and \(p^{5/9}\).  These are unconditional for the stated
representation class: repeated resultant roots do not help because the
class stores the multiplicity-preserving norm or its tuple-leaf subproduct
tree.

The theorem also names the remaining escape hatch exactly.  A better solver
must keep an \(r\)-leaf eliminant in a representation with \(o(N^r)\) charged
size **and** combine it with the remaining leaves without performing
\(N^{m-r-o(1)}\) probes per target.  A nested unevaluated resultant gate is
not by itself such an algorithm; its construction, evaluation, gcd, root
recovery, and reuse costs must all be specified.

### Proposition 15 (the succinct-eliminant circuit boundary)

Let \(\Omega=F^m\).  For tuples whose sum is not the point at infinity, define
the multiplicity-preserving target eliminant

\[
H_\Omega(Z)=\prod_{\omega\in\Omega}
\bigl(Z-x(\sigma(\omega))\bigr).
\]

Partition \(\Omega\) by a binary tree, and for every node \(v\) put

\[
H_v(Z)=\prod_{\omega\in\Omega_v}
\bigl(Z-x(\sigma(\omega))\bigr),
\qquad H_v=H_{v_0}H_{v_1}.
\]

Suppose preprocessing stores a shared arithmetic-circuit DAG for this family,
with enough node access to evaluate the two children of a visited node.  A
target \(t\) satisfying \(H_\Omega(t)=0\) can then be resolved to a leaf tuple
by descending through a zero child.  The query uses only the circuits on one
root-to-leaf path, of depth \(m\log_2N\), plus final sign and rationality
verification.

Consequently, a lower bound for all implicit eliminant solvers must lower-bound
the size or path-evaluation cost of this shared circuit family.  The expanded
degree \(N^m\), the number of roots counted with multiplicity, and dense
Boolean or coefficient support do not provide such a lower bound.

#### Justification

The product identity follows from the partition.  At an internal node,
\(H_v(t)=0\) implies \(H_{v_0}(t)=0\) or \(H_{v_1}(t)=0\); evaluating both
children identifies a branch, and iteration reaches a tuple whose sum has
target \(x\)-coordinate.  Verification resolves the finite sign choices.
Tuples summing to the point at infinity can be stored in a separate flag or
excluded, since the queried target is affine.

For the limitation, the degree-\(D\) polynomial \(Z^D-1\) has \(D\) roots
over a suitable field extension but is computed by \(O(\log D)\)
multiplications using repeated squaring.  Likewise, a dense multilinear
normal form may have a logarithmic rational circuit by Proposition 11.
Therefore none of degree, root count, or expanded support is monotone under
straight-line circuit compression.  Over \(\mathbb F_p\), if an attainable
target set is all of \(\mathbb F_p\), its squarefree membership polynomial is
just \(Z^p-Z\), again computable in \(O(\log p)\).  Membership can therefore
be trivial even when recovering a witnessing tuple remains difficult; this is
why the proposition uses the whole factor tree rather than only its root.
\(\square\)

Writing the security parameter as \(\lambda=\log_2p\), the desired
\(p^{1/2-o(1)}\) preprocessing lower bound is
\(2^{(1/2-o(1))\lambda}\).  Proving this directly for unrestricted
straight-line circuits computing the explicit \(H_v\) family would constitute
an exponential arithmetic-circuit lower bound, not a routine Bézout or
Macaulay estimate.  This does not show that the bound is false; it shows why
the solver class must either retain a root-provenance/materialization
restriction, use a black-box or data-structure query model, or be attacked
algorithmically by searching for a succinct eliminant tree.

### Proposition 16 (group-algebra and Fourier reformulation)

Let \(K[G]\) be the group algebra of the finite abelian group \(G\), with basis
elements \([Q]\) for \(Q\in G\), and put

\[
a_F=\sum_{Q\in F}[Q].
\]

Then

\[
a_F^m=\sum_{T\in G}c_m(T)[T],
\qquad
c_m(T)=|\{(Q_1,\ldots,Q_m)\in F^m:
Q_1+\cdots+Q_m=T\}|.
\]

Thus batched relation finding is exactly the problem of recovering selected
coefficients, together with witnesses, of a sparse \(m\)-fold convolution in
\(K[G]\).

If \(G=\langle P\rangle\) is cyclic of order \(n\), write
\(\ell(Q)\in\mathbb Z/n\mathbb Z\) for the discrete-log coordinate of \(Q\).
Over a coefficient field containing a primitive \(n\)-th root \(\zeta\), the
Fourier formulas are

\[
\widehat a_F(k)=\sum_{Q\in F}\zeta^{k\ell(Q)},
\qquad
c_m(T)=\frac1n\sum_{k=0}^{n-1}
\widehat a_F(k)^m\zeta^{-k\ell(T)}.
\]

Over \(\mathbb C\), Parseval also gives

\[
\sum_{T\in G}c_m(T)^2
=\frac1n\sum_{k=0}^{n-1}|\widehat a_F(k)|^{2m}.
\]

#### Proof

Multiplication in \(K[G]\) is defined by
\([Q][R]=[Q+R]\); expanding \(a_F^m\) proves the coefficient identity. [pcc:unverified]
The remaining formulas are the ordinary discrete Fourier transform on the
cyclic group. \(\square\)

This identifies a genuine compression route, but not a prime-field shortcut.
Given all discrete-log labels, a full FFT computes every coefficient in
\(\widetilde O(n)\) operations, already larger than the desired square-root
bound.  Without those labels, an interval, multiplicative subgroup, or
small-height description in the ambient \(x\)-field does not supply the
Fourier phases \(\zeta^{\ell(Q)}\).  A useful sublinear method would therefore
have to be a sparse/partial convolution algorithm directly in the black-box
elliptic group, or exploit a new algebraic relation between \(x(Q)\) and
\(\ell(Q)\).

There is an exact black-box boundary.  If preprocessing and queries learn the
tuple-sum map only by evaluating it on \(D\) distinct tuples, then for \(B\)
uniform targets Theorem 3 gives

\[
\mathbb E[\text{reported matches}]\le\frac{BD}{n}.
\]

Producing \(\Theta(N)\) relations consequently forces

\[
B+D=\Omega(\sqrt{Nn}).
\]

Hence a succinct eliminant tree can beat the black-box bound only by computing [pcc:unverified]
node summaries from the coordinate algebra without evaluating the
corresponding leaf tuples.  Propositions 15 and 16 isolate the two sides of
the open problem: arithmetic-circuit compression of the eliminants versus
sparse convolution/inversion in the elliptic group.

### Theorem 17 (density-one sparse-regime sumset expansion)

Fix \(m\ge2\), let \(p>4m\), and let
\(C=(c_0,\ldots,c_{L-1})\) be distinct residues, where

\[
L\longrightarrow\infty,\qquad L^m=o(p).
\]

For \(E_{A,B}:y^2=x^3+Ax+B\), retain the \(c_i\) that lift to rational points
and choose either lift by any deterministic rule, producing a point set
\(F_{A,B}\).  Then for a density-\(1-o(1)\) set of nonsingular coefficient
pairs \((A,B)\),

\[
|F_{A,B}|=(1/2+o(1))L
\]

and the ordered \(m\)-sum map has support

\[
|\underbrace{F_{A,B}+\cdots+F_{A,B}}_{m\ {\rm times}}|
=\Theta_m(|F_{A,B}|^m).
\]

The statement is uniform over intervals, multiplicative subgroups, and other
fixed distinct-residue master lists; no randomness of the list itself is
assumed.

#### Collision-variety lemma

Fix two ordered coordinate \(m\)-tuples from \(C\) whose coordinate multisets
are different.  Across all \((A,B)\in\mathbb F_p^2\) and all choices of square
roots above the coordinates involved, the number of rational instances in
which the two point sums are equal is \(O_m(p)\).

#### Proof of the lemma

Work first over \(K=\overline{\mathbb F}_p(A,B)\).  For every distinct
coordinate \(c\), the polynomial

\[
f_c=c^3+Ac+B
\]

is an irreducible linear polynomial in \(A,B\), and the \(f_c\) are pairwise
nonassociate.  Their square classes are independent: a nonempty product has
odd valuation at one of its distinct prime factors.  Consequently adjoining
the square roots \(y_c^2=f_c\) gives a multiquadratic extension whose Galois
group independently changes \(P_c=(c,y_c)\) to \(-P_c\).

Equality of the two tuple sums would be an identity

\[
\sum_c a_cP_c=O,
\]

with some nonzero integer coefficient \(a_c\), \(|a_c|\le m\).  Applying the
Galois automorphism that changes only \(P_c\) to \(-P_c\), then subtracting
the two identities, would give \(2a_cP_c=O\).  This is impossible on the
universal nonsingular curve when \(p>4m\): the relevant bounded-order division
polynomial at \(x=c\) is a nonzero polynomial in \(A,B\).  Equivalently, the
fixed-\(x\) generic point is not torsion of order at most \(2m\).

Thus the collision locus is a proper subvariety of the finite
multiquadratic cover of the \((A,B)\)-plane.  With \(m\) fixed, projective
addition laws, the cover equations, and the relevant division polynomials
have degree \(O_m(1)\).  The collision locus has dimension at most one and
degree \(O_m(1)\), so it has \(O_m(p)\) rational points.  Addition-chart
exceptional loci and singular curves have the same bound. \(\square\)

#### Proof of the theorem

The pairwise-independence calculation following Theorem 10 gives
\(|F_{A,B}|=(1/2+o(1))L\) outside a density-\(o(1)\) set of coefficient pairs.

For a fixed curve let \(r(T)\) count ordered \(m\)-tuples from \(F_{A,B}\)
summing to \(T\), and write

\[
\mathcal E_m=\sum_T r(T)^2.
\]

This energy counts pairs of tuples with equal sums.  Pairs whose coordinate
multisets agree contribute at most \(m!|F_{A,B}|^m\).  There are
\(O(L^{2m})\) remaining ordered tuple pairs.  Summing the collision-variety
lemma over them and averaging over the \(p^2\) coefficient pairs gives
expected nontrivial energy

\[
O_m\!\left(\frac{L^{2m}}p\right)=o(L^m).
\]

Markov's inequality makes this \(o(L^m)\) for a density-\(1-o(1)\) family of
curves.  Hence \(\mathcal E_m=O_m(|F_{A,B}|^m)\).  Cauchy--Schwarz now gives

\[
|\operatorname{supp}r|
\ge\frac{(\sum_T r(T))^2}{\sum_T r(T)^2}
=\frac{|F_{A,B}|^{2m}}{\mathcal E_m}
=\Omega_m(|F_{A,B}|^m),
\]

and the reverse inequality is immediate. \(\square\)

### Corollary 17.1 (end-to-end exact separated-sketch barrier)

For the density-one curve family of Theorem 17, let
\(N=|F_{A,B}|=p^{\alpha+o(1)}\) with \(N^m=o(p)\).  Any exact global
separated representation

\[
\mathbf1[\sigma(\omega)=T]
=\sum_{i=1}^r U_i(\omega)V_i(T)
\]

has \(r=\Omega_m(N^m)\) by Theorems 1 and 17.  If its \(r\) terms are
materially stored or evaluated and uniform targets are used to collect
\(\Theta(N)\) relations, then

\[
T_{\rm total}
=\Omega_m\!\left(N^m+\frac{p}{N^{m-1}}\right)
\ge p^{\,m/(2m-1)-o(1)}
>p^{1/2}.
\]

Indeed, the exact expected number of ordered decompositions across \(B\)
targets is \(BN^m/\Theta(p)\), so \(B=\Omega(p/N^{m-1})\), even if every
available decomposition is returned.  Balancing this with the rank/storage
cost \(N^m\) gives \(\alpha=1/(2m-1)\).  This is a genuine
non-materializing-class barrier: it applies to exact separated sketches after
proving, rather than assuming, the required sparse-regime sumset expansion.
It still does not cover shared nonlinear arithmetic circuits or succinct
eliminant trees.  For \(m=3,4,5\), the resulting exponents are respectively
\(3/5\), \(4/7\), and \(5/9\).

### Theorem 18 (static-provenance eliminant direct sum)

Let \(G\) be any finite group of order \(n\), let
\(\sigma:\Omega\to G\) be any fixed candidate-sum map, and consider the
following static-provenance product-tree model.

1. Before targets are known, preprocessing chooses a set
   \(W\subseteq\Omega\) of \(D\) distinct candidate leaves and materializes
   enough provenance to recover each \(\omega\in W\).  It may build arbitrary
   exact product trees, factored polynomials, rational circuits, or shared DAG
   summaries over those leaves.  Creating the explicit provenance records is
   charged at least \(D\) operations or words, so \(P\ge D\).
2. The query algorithm receives \(B\) uniform targets.  It may evaluate any
   stored node, take target-dependent branches, share every evaluation across
   all targets, and return any number of verified leaf--target pairs. `[pcc:unverified]`
3. Every returned witness must be one of the preprocessed leaves.  Constructing
   a new target-dependent candidate or a new implicit subtree is outside this
   static-provenance model.

If \(Z\) is the number of distinct reported pairs
\((\omega,T_j)\) with \(\sigma(\omega)=T_j\), then

\[
\mathbb E Z\le \frac{BD}{n}.
\]

Consequently, producing \(R\) expected verified relations has charged work `[pcc:unverified]`

\[
C\;\ge\;P+B+T_{\rm verify}\;\ge\;D+B
\;\ge\;2\sqrt{Rn}.
\]

For a factor base of size \(N=p^{\alpha+o(1)}\), arity \(m\), and
\(R=\Theta(N)\) required rows, the domain cap \(D\le N^m\) yields

\[
C\ge p^{\,m/(2m-1)-o(1)}.
\]

In particular the exponents are \(3/5,4/7,5/9\) for \(m=3,4,5\), and are
strictly above \(1/2\) for every fixed \(m\).

#### Proof

For every stored leaf \(\omega\in W\) and target index \(j\), let

\[
I_{\omega,j}=\mathbf 1[T_j=\sigma(\omega)].
\]

Each target is uniform, so \(\mathbb E I_{\omega,j}=1/n\), independently of
collisions among leaf values and without any randomness assumption on
\(\sigma\).  Every reported pair is one of the pairs counted by these
indicators.  Therefore adaptive evaluation and branching can only suppress
matches, and linearity of expectation gives

\[
\mathbb E Z
\le\sum_{\omega\in W}\sum_{j=1}^B
\mathbb E I_{\omega,j}
=\frac{BD}{n}.
\]

Thus \(\mathbb E Z\ge R\) implies \(BD\ge Rn\).  Since \(P\ge D\), the
arithmetic--geometric mean inequality gives
\(C\ge D+B\ge2\sqrt{Rn}\).

For completeness, retain the domain cap rather than assuming that the square
root balance is feasible.  The least possible value of

\[
D+\frac{Rn}{D},\qquad 1\le D\le N^m,
\]

is attained at \(D=\min\{\sqrt{Rn},N^m\}\).  With
\(R=N=p^{\alpha+o(1)}\), the transition occurs at
\(m\alpha=(1+\alpha)/2\), namely \(\alpha=1/(2m-1)\).  Below the transition,
the target term is \(p^{1-(m-1)\alpha+o(1)}\); above it, the square-root term
is \(p^{(1+\alpha)/2+o(1)}\).  Both branches are minimized at the transition,
where their exponent is \(m/(2m-1)\). \(\square\)

The theorem is insensitive to the internal polynomial representation and
allows fully target-adaptive descent through a stored eliminant tree.  It
therefore closes the product-tree-plus-adaptive-branching subclass promised by
Gate 2.  Its exact boundary is provenance: it does not charge a compact
coordinate circuit merely because the circuit semantically represents
\(D\) leaves.  Extending the result requires a simulation or information
theorem showing that an implicitly generated split-\(S_3\) subtree carries
comparable charged information, or an explicit compact generator that
refutes that extension. `[pcc:unverified]`

### Proposition 19 (the group-\(k\)SUM-indexing boundary)

Let \(m\ge3\), and split the factor-base leaves into blocks of sizes \(a\) and
\(b=m-a\).
Materialize the ordered partial-sum arrays, retaining one tuple witness per
entry,

\[
A=(A_u)_{u\in F^a},\qquad B=(B_v)_{v\in F^b}.
\]

For a target \(T\), split-tree relation recovery is exactly the asymmetric
group-3SUM-indexing query

\[
\text{find }(u,v)\text{ such that }A_u+B_v=T.
\]

Equivalently, without first materializing the two partial arrays, the original
query is group-\((m+1)\)SUM-Indexing on the factor-base array: recover \(m\)
input entries whose sum is the challenge.  Tuple recovery, rather than mere
membership in the sumset, is part of the indexing problem.

This reduction is exact for every finite abelian group and preserves
preprocessing, online queries, and witness recovery up to the work required to
form the partial sums.  Consequently the full ARQP target contains a
structured group-indexing problem for which strong adaptive data-structure
lower bounds are already a major open problem in the standard integer model.

As an upper-bound stress test, take the 2026 Dinur--Golovnev
\(k\)SUM-Indexing trade-off and optimistically assume that it transports to
elliptic-group inputs with no extra cost.  To recover \(m\) leaves, set their
indexing parameter \(k=m+1\).  For \(0\le\delta\le1\), their reported space
and query exponents become

\[
S=\widetilde O(N^{m+1/2-\delta}),\qquad
Q=\widetilde O(N^\delta).
\]

Even ignoring any preprocessing time beyond writing \(S\), this family does
not cross the ECDLP square-root frontier.  In the sparse regime, collecting
\(\Theta(N)\) rows needs

\[
B=\Omega\!\left(\frac{p}{N^{m-1}}\right)
\]

uniform targets, so with \(N=p^{\alpha+o(1)}\),

\[
T_{\rm total}\ge
\max\left\{
p^{(m+1/2-\delta)\alpha},
p^{1-(m-1-\delta)\alpha}
\right\}.
\]

The two exponents balance at

\[
\alpha=\frac1{2m-1/2-2\delta},
\]

where the common exponent is

\[
E_{m,\delta}
=\frac{m+1/2-\delta}{2m-1/2-2\delta}
=\frac12+\frac{3/4}{2m-1/2-2\delta}
>\frac12.
\]

Charging any preprocessing work beyond writing \(S\)---including the paper's
stated \(\widetilde O(N^2)\) runtime in its symmetric 3SUM-Indexing case---plus
elliptic-group adaptation, verification, or factor-base linear algebra can
only increase this exponent.
Thus current \(k\)SUM-indexing upper bounds do not constructively refute the
ARQP barrier.  Conversely, the known unconditional adaptive lower bounds do
not establish the desired trade-off once the query makes more than a tiny
number of probes.  The next theorem would therefore require either new static
data-structure lower-bound machinery specialized to elliptic partial sums, or
an algebraic property absent from generic group-indexing instances.

#### Proof

The reduction follows by associativity:

\[
\sum_{i=1}^m P_i=T
\quad\Longleftrightarrow\quad
\left(\sum_{i=1}^a P_i\right)
+\left(\sum_{i=a+1}^m P_i\right)=T.
\]

The retained provenance converts a returned partial-sum pair back into the
\(m\) leaves.  The exponent audit substitutes the cited space and query
bounds into the exact uniform-target relation count and minimizes the maximum
of one increasing and one decreasing affine function of \(\alpha\).  The
displayed identity for \(E_{m,\delta}\) is direct algebra. \(\square\)

The literature statements and the distinction between cell-probe space,
preprocessing time, and algebraic-RAM work are recorded in
[`LITERATURE.md`](LITERATURE.md).  No integer-indexing algorithm is claimed to
apply to elliptic groups without that missing adaptation. `[pcc:unverified]`

### Theorem 20 (exact exponent criterion for an implicit data structure)

Fix \(m\ge3\).  Suppose a split-\(S_3\) relation data structure on a factor
base of size \(N\) has charged preprocessing

\[
P=N^{s+o(1)}
\]

and, on a batch of \(B\) uniform targets, has total target-dependent work

\[
T_{\rm query}=B\,N^{t+o(1)}.
\]

The batch may share arbitrary target-dependent computation and return every
verified decomposition it finds; output and verification are charged. Assume
\(0\le t<m-1\), constant rank gain, and \(N=p^{\alpha+o(1)}\) with
\(0<\alpha\le1/m\). The exact tuple-count upper bound gives the following
optimized relation-stage exponent: `[pcc:unverified]`

\[
E_{\rm rel}(s,t;m)=
\begin{cases}
\displaystyle\frac{s}{s+m-1-t},&s-t\ge1,\\[6pt]
\displaystyle\frac{1+t}{m},&s-t<1.
\end{cases}
\]

The two branches have the respective sub-rho criteria

\[
\begin{array}{rcl}
s-t\ge1&:&s+t<m-1,\\
s-t<1&:&t<m/2-1.
\end{array}
\]

If the conventional sparse factor-base linear-algebra charge
\(T_{\rm LA}=N^{2+o(1)}\) is included, put \(s_0=\max\{s,2\}\) and substitute
\(s_0\) for \(s\) in the same piecewise formula and criteria.

The statements involving \(N^2\) are conditional on that conventional linear
algebra cost model; they are not a lower bound for every possible structured
matrix algorithm.

#### Proof

Across a uniform target, the expected number of ordered decompositions is at
most \(N^m/p\). Linearity of expectation applies to the whole batch even if
all decompositions are returned and target-dependent work is shared.
Therefore collecting \(\Theta(N)\) rows requires at least

\[
B=p^{1+o(1)}N^{-(m-1)}
\]

uniform target queries.  With \(N=p^{\alpha+o(1)}\), preprocessing and all
queries therefore contribute exponents

\[
s\alpha,
\qquad
1-(m-1-t)\alpha.
\]

The first is increasing and the second decreasing because \(t<m-1\).  They
are equal at

\[
\alpha_*=\frac1{s+m-1-t}.
\]

This balance is feasible under \(\alpha\le1/m\) exactly when \(s-t\ge1\).
In that case substitution gives the first branch, and the inequality

\[
\frac{s}{s+m-1-t}<\frac12
\]

is equivalent to \(s+t<m-1\).

When \(s-t<1\), the balance lies beyond the permitted interval.  The maximum
is still decreasing at \(\alpha=1/m\), so the infimum is attained by taking
\(\alpha=1/m-o(1)\).  The query exponent tends to \((1+t)/m\), while
\(s/m<(1+t)/m\); this gives the second branch and its criterion
\(t<m/2-1\).  Adding an \(N^{2+o(1)}\) term only replaces the increasing
coefficient by \(s_0=\max\{s,2\}\), so the identical case split applies.
\(\square\)

This criterion turns the broad ARQP question into a falsifiable target. The
exponent \(t\) is amortized over the complete collection batch, so ordinary
multipoint evaluation and shared target processing do not escape the balance.
In the balanced sparse branch \(s-t\ge1\), a candidate must cross the line
\(s+t=m-1\), not merely compress the \(N^m\) expanded sum table.  In the
boundary branch \(s-t<1\), it must instead achieve \(t<m/2-1\).  The trivial
scan, full sum table, balanced meet-in-the-middle table, and the
Dinur--Golovnev family all remain on the non-sub-rho side; the last lies in the
first branch and has \(s+t=m+1/2\).  A barrier theorem may therefore prove the
corresponding piecewise inequalities for an explicitly closed elliptic
group-indexing model. [pcc:unverified]

### Theorem 21 (no compressed exact bucket filter or compositional quotient)

Let \(G\subseteq E(\mathbb F_p)\) be cyclic of prime order \(n\).  First,
suppose three label maps

\[
\lambda_A:G\to L_A,\qquad
\lambda_B:G\to L_B,\qquad
\lambda_T:G\to L_T
\]

and a predicate \(R\subseteq L_A\times L_B\times L_T\) decide addition
exactly:

\[
R(\lambda_A(P),\lambda_B(Q),\lambda_T(T))
\quad\Longleftrightarrow\quad P+Q=T.
\]

Then all three label maps are injective.  In particular,
\(|L_A|,|L_B|,|L_T|\ge n\); an exact filter based only on separately
compressed bucket labels is impossible even when its predicate is arbitrary.

Second, let \(L\) be any label set equipped with an arbitrary binary operation
\(\star:L\times L\to L\), and suppose a bucket label \(\psi:G\to L\) is
compositional:

\[
\psi(P+Q)=\psi(P)\star\psi(Q)
\qquad(P,Q\in G).
\]

No group, associativity, cancellation, or inverse axioms are assumed for
\((L,\star)\).  Nevertheless, exactly one of the following holds:

1. \(\psi\) is trivial, so the compatibility test
   \(\psi(A)\star\psi(B)=\psi(T)\) rejects no candidate pair; or
2. \(\psi\) is injective, so \(|L|\ge n\) and the label distinguishes every
   element of \(G\).

In particular there is no nontrivial compositional bucket map from
\(G\) to a label set of size \(o(n)\).  Group homomorphisms are only a special
case.

Two algebraic consequences hold.

- Every algebraic-group morphism from the elliptic curve \(E\) to
  \(\mathbb G_a\) or \(\mathbb G_m\) is trivial.  Thus no global rational
  parameter that is regular everywhere can transport elliptic addition to
  field addition or multiplication and simultaneously supply a smaller exact
  convolution domain.
- If \(\phi:E\to E'\) is an isogeny of degree \(d<n\), then
  \(\phi|_G\) is injective.  Hence a bounded- or sub-\(n\)-degree isogeny does
  not merge prime-order factor-base sums into smaller exact buckets.  Any
  isogeny whose kernel contains \(G\) has degree at least \(n\).

#### Proof

If \(\lambda_A(P)=\lambda_A(P')\), take \(Q=O\) and \(T=P\).  The predicate
accepts the label triple for \((P,O,P)\), hence accepts the identical label
triple for \((P',O,P)\).  Exactness gives \(P'=P\), so \(\lambda_A\) is
injective.  The same argument with the two summand positions exchanged proves `[pcc:unverified]`
injectivity of \(\lambda_B\).  If
\(\lambda_T(T)=\lambda_T(T')\), use the accepted triple \((T,O,T)\);
exactness of the identical label triple \((T,O,T')\) gives \(T=T'\).

Put \(K=\{P\in G:\psi(P)=\psi(O)\}\).  If \(P\in K\), compositionality gives

\[
\psi(O)=\psi(P-P)
=\psi(P)\star\psi(-P)
=\psi(O)\star\psi(-P)
=\psi(-P),
\]

so \(-P\in K\).  If \(P,Q\in K\), then

\[
\psi(P-Q)=\psi(P)\star\psi(-Q)
=\psi(O)\star\psi(-Q)
=\psi(-Q)=\psi(O),
\]

so \(P-Q\in K\).  Thus \(K\) is a subgroup.

Moreover,

\[
\psi(P)=\psi(Q)\quad\Longleftrightarrow\quad P-Q\in K.
\]

The forward implication follows by composing both equal labels with
\(\psi(-Q)\); the reverse implication follows from
\(\psi(P)=\psi((P-Q)+Q)=\psi(O)\star\psi(Q)=\psi(Q)\).
Hence the fibres of \(\psi\) are exactly the cosets of \(K\).  Prime order
forces \(K=\{O\}\) or \(K=G\), giving respectively the injective and constant
cases.  An injective label map forces \(|L|\ge n\).

A morphism \(E\to\mathbb G_a\) is a global regular function on the proper
geometrically integral curve \(E\), hence is constant.  The same is true for
a morphism \(E\to\mathbb G_m\), since composing with
\(\mathbb G_m\hookrightarrow\mathbb A^1\) is a global regular function.
A constant group morphism is the identity element.

Finally, \(\ker(\phi)\cap G\) is again either \(\{O\}\) or \(G\).  In the
second case the kernel scheme has at least \(n\) distinct geometric points,
so \(\deg\phi\ge n\).  Therefore \(d<n\) forces trivial intersection and
injectivity on \(G\). \(\square\)

This theorem rules out every exact separately labelled bucket predicate and
every smaller globally compositional label, including
additive/multiplicative coset labels,
globally compatible rational linearizations, and low-degree isogeny fibres as
the missing compact generator.  It does not cover one-sided filters with false
positives, target-dependent rational functions, joint encodings that do not
factor through separate labels, or data structures that exploit coordinates
without preserving addition.
Those mechanisms remain inside the indexing boundary of Proposition 19.
[pcc:unverified]

### Theorem 22 (one-sided separate labels force false verification)

Let \(G\) be a finite group, written additively, and let
\(A,B\subseteq G\) be finite nonempty sets.  Suppose

\[
\lambda_A:A\to L_A,\qquad
\lambda_B:B\to L_B,\qquad
\lambda_T:G\to L_T
\]

are label maps and \(R\subseteq L_A\times L_B\times L_T\) is a one-sided
addition predicate: for every \(a\in A\) and \(b\in B\),

\[
R\bigl(\lambda_A(a),\lambda_B(b),\lambda_T(a+b)\bigr)
\]

holds.  Let \(F\) be the number of false accepted triples
\((a,b,t)\in A\times B\times G\) with \(t\ne a+b\).  Put
\(k_A=|\operatorname{im}\lambda_A|\) and
\(k_B=|\operatorname{im}\lambda_B|\).  Also put
\(S=A+B\), \(s=|S|\), and
\(k_T=|\lambda_T(S)|\).  Then

\[
\begin{split}
F&\ge |B|\left(\sum_{C\in A/\lambda_A}|C|^2-|A|\right)
 \ge |B|\left(\frac{|A|^2}{k_A}-|A|\right),\\
F&\ge |A|\left(\sum_{D\in B/\lambda_B}|D|^2-|B|\right)
 \ge |A|\left(\frac{|B|^2}{k_B}-|B|\right),\\
F&\ge \sum_{H\in S/\lambda_T}|H|^2-s
 \ge \frac{s^2}{k_T}-s.
\end{split}
\]

Consequently \(F\) is at least the maximum of the three right-hand sides.  If
\(B_q\) independent uniform targets in \(G\) are queried and the filter
enumerates and explicitly verifies every accepted candidate pair, then its [pcc:unverified]
expected number of false verifications is at least

\[
\frac{B_q}{|G|}F.
\]

#### Proof

Fix \(b\in B\), a fibre \(C\) of \(\lambda_A\), and distinct ordered
elements \(a,a'\in C\).  The true triple \((a,b,a+b)\) is accepted.  The
triple \((a',b,a+b)\) has exactly the same three labels, so it is also
accepted, but it is false by cancellation.  The map

\[
(b,a,a')\longmapsto(a',b,a+b)
\]

is injective: the image supplies \(b\) and \(a+b\), hence determines \(a\),
and also supplies \(a'\).  These collisions therefore contribute

\[
|B|\sum_C |C|(|C|-1)
=|B|\left(\sum_C|C|^2-|A|\right)
\]

distinct false triples.  Cauchy--Schwarz gives
\(\sum_C|C|^2\ge |A|^2/k_A\).  The analogous argument, fixing
\(a\in A\) and colliding two elements of a \(\lambda_B\)-fibre, proves the [pcc:unverified]
second bound.

For every \(t\in S\), choose one pair \((a_t,b_t)\) with \(a_t+b_t=t\).
For distinct \(t,t'\in S\) in the same \(\lambda_T\)-fibre, the true triple
\((a_t,b_t,t)\) and the false triple \((a_t,b_t,t')\) have the same three
labels.  Distinct ordered pairs \((t,t')\) give distinct false triples because
the chosen pair determines its sum \(t\).  Thus a target fibre \(H\)
contributes \(|H|(|H|-1)\) false triples.  Summing and applying
Cauchy--Schwarz proves the third bound. [pcc:unverified]

Finally, each false triple has one target coordinate, so a
uniform query selects it with probability \(1/|G|\); linearity of expectation
over the batch gives \(B_qF/|G|\). \(\square\)

The theorem is unconditional for one-sided predicates that factor through
separate, target-independent labels and enumerate all accepted pairs.  It
quantifies the false-positive escape hatch left by Theorem 21: compressing a
partial-sum side or the attainable target set below its cardinality
necessarily creates a second-moment verification cost.  It does not cover
joint encodings, target-specific label maps, signed algebraic summaries whose
intermediate terms cancel, or algorithms that recover witnesses without
enumerating accepted pairs.  Those remain within the general
preprocessing/query problem. [pcc:unverified]

### Proposition 23 (target-shifted coordinate filtering does not beat rho)

Let

\[
A=(P_u)_{u\in F^a},\qquad B=(Q_v)_{v\in F^b},\qquad a+b=m,
\]

be ordered partial-sum arrays whose records retain their tuple witnesses.
Give the point at infinity a distinguished tag and define the
target-dependent labels on record values by

\[
\lambda_{A,T}(P)=x(T-P),\qquad \lambda_B(Q)=x(Q),
\]

using that tag when the argument is the point at infinity.  Hash the records
of \(B\) by \(\lambda_B\).  On target \(T\), scan the records \(P\in A\),
look up \(\lambda_{A,T}(P)\), and verify every returned record pair.

Every true record pair \(P+Q=T\) is returned.  For a fixed \(P\), the lookup
contains records with at most the two curve-point values \(T-P\) and
\(-(T-P)\); hence the only false group value is the opposite sign.  Duplicate
tuple witnesses are retained and charged as output.  With expected-constant
hash lookup, this explicit implementation gives

\[
P=\widetilde\Theta(N^b),\qquad Q=\widetilde\Theta(N^a).
\]

The most favourable separate-query split stores \(b=m-1\) leaves and scans
one leaf.  Collecting \(\Theta(N)\) relation rows still requires at least
\(pN^{-(m-1)}\) uniform targets by the exact tuple-count bound.  Thus this
implementation has relation-stage balance

\[
\widetilde\Omega\!\left(
N^{m-1}+\frac{p}{N^{m-2}}
\right).
\]

Writing \(N=p^{\alpha+o(1)}\), the terms balance at
\(\alpha=1/(2m-3)\le1/m\), with exponent

\[
E_m^{\rm shift}=\frac{m-1}{2m-3}
=\frac12+\frac1{4m-6}>\frac12.
\]

For \(m=3,4,5\), the exponents are respectively
\(2/3,3/5,4/7\).  They are also slightly worse than the explicit batched
candidate exponents \(3/5,4/7,5/9\).

#### Proof

If \(P+Q=T\), then \(Q=T-P\), so the two labels agree.  Conversely, equal
non-infinity \(x\)-coordinates on a short Weierstrass curve imply
\(Q=T-P\) or \(Q=-(T-P)\); direct group-law verification removes the latter.
The infinity tag handles the exceptional affine case.  Forming the hash table
and performing one lookup per scanned partial tuple give the stated costs.

For a general split, the query total over the necessary target batch is
\(pN^{a-(m-1)}=p/N^{b-1}\).  Whenever the internal balance is feasible, its
exponent is \(b/(2b-1)\), which decreases with \(b\); otherwise the
factor-base boundary is no better.  Hence \(b=m-1\) is optimal among these
separate-query scans.  Balancing \(N^{m-1}\) with \(p/N^{m-2}\) gives the
displayed value of \(\alpha\) and \(E_m^{\rm shift}\).  The conventional
\(N^2\) linear-algebra term does not increase that exponent for \(m\ge3\).
\(\square\)

This construction shows exactly why target-independent label bounds cannot be
extended by merely allowing a low-degree target shift: such a shift removes
the false-positive bottleneck.  The remaining cost is search over one partial
side.  Proposition 23 does not lower-bound more elaborate target-dependent
maps or shared multi-target algorithms; it identifies the scan that a genuine
non-materializing improvement must avoid. [pcc:unverified]

### Theorem 24 (degree-explicit algebraic-selector direct sum)

Let \(G\subseteq E(\mathbb F_p)\) have order \(n\), and let
\(\mathcal F\subseteq G\) be a factor base of size \(N\).  A
**degree-explicit algebraic selector catalogue** consists of finitely many
branches \(j\).  Branch \(j\) is a smooth projective curve \(X_j\), a
nonconstant target morphism, and \(m\) leaf morphisms,

\[
\pi_j:X_j\to E,\qquad
f_{j,i}:X_j\to E\qquad(1\le i\le m).
\]

This includes the normalization of a finite algebraic correspondence; an
ordinary rational selector is the special case \(X_j=E\) and
\(\pi_j=\operatorname{id}\).  Let \(W_j\subseteq X_j(\overline{\mathbb F}_p)\)
be the verified witness points for which \(\pi_j(x)\in G\),
\(f_{j,i}(x)\in\mathcal F\), and [pcc:unverified]

\[
\sum_{i=1}^m f_{j,i}(x)=\pi_j(x).
\]

On target \(T\), the query algorithm may choose branches adaptively, recover
any \(x\in W_j\) with \(\pi_j(x)=T\), verify its leaf tuple, and suppress any
successful output.  Repeated copies of the same witness point are counted
once.  Put [pcc:unverified]

\[
\delta_j
=\deg(\pi_j)
+\sum_{\substack{1\le i\le m\\ f_{j,i}\ {\rm nonconstant}}}
\deg(f_{j,i}),
\qquad
\Delta=\sum_j\delta_j.
\]

The degree-explicit model charges preprocessing \(P\ge\Delta\); in
particular, a branch catalogue, correspondence, or path expansion cannot
store projection degree \(d\) for less than \(d\) charged units.  Supplying
and reading a target costs at least one query unit, even when batch work is
shared.  For \(B\) independent uniform targets, if \(Z\) is the number of
distinct verified witness points returned, then [pcc:unverified]

\[
\mathbb E Z\le \frac{BN\Delta}{n}.
\]

Consequently, producing \(R=cN\) expected relation rows, for any fixed
\(c>0\), requires

\[
B\Delta\ge cn,\qquad
P+B+T_{\rm verify}\ge 2\sqrt{cn}.
\]

For \(n=\Theta(p)\), this is an unconditional
\(p^{1/2-o(1)}\) relation-stage barrier in the stated model, independent of
\(m\) and of the factor-base exponent. [pcc:unverified]

#### Proof

If a nonconstant morphism from a smooth projective curve to \(E\) has degree
\(d\), every geometric point fibre has at most \(d\) distinct points.
Therefore, for any nonconstant \(f_{j,i}\),

\[
\left|\{x\in W_j:f_{j,i}(x)\in\mathcal F\}\right|
\le dN.
\]

If branch \(j\) has at least one nonconstant leaf map, choosing any such map
gives \(|W_j|\le N\delta_j\).  If all leaf maps are constant, their sum is
one fixed point \(T_j\).  The relation equation then puts \(W_j\) inside the
fibre \(\pi_j^{-1}(T_j)\), so
\(|W_j|\le\deg(\pi_j)\le N\delta_j\).  Hence

\[
\sum_j|W_j|\le N\Delta.
\]

For a uniform target, the expected number of branch-\(j\) witness points above
it is \(|W_j|/n\).  Linearity of expectation over all targets and branches
applies even when branch selection and root recovery are adaptive, because
they can only suppress members of the fixed witness sets.  Thus

\[
\mathbb E Z
\le \frac{B}{n}\sum_j|W_j|
\le \frac{BN\Delta}{n}.
\]

The requirement \(\mathbb E Z\ge cN\) gives \(B\Delta\ge cn\).
Since \(P\ge\Delta\), the arithmetic--geometric mean inequality gives

\[
P+B+T_{\rm verify}\ge\Delta+B\ge2\sqrt{B\Delta}
\ge2\sqrt{cn}.
\]

\(\square\)

The theorem permits target-adaptive branch choice, multivalued algebraic root
recovery, non-materializing direct leaf output, false-positive verification,
and reuse of the catalogue across the complete target batch.  It rules out
bounded-degree rational reconstruction, explicit collections of
target-dependent algebraic parameterizations, and degree-expanded split-tree
correspondences as sub-rho mechanisms.

Its boundary is exact and important: a compact shared DAG may encode
exponentially many algebraic execution paths or a high-degree correspondence
without paying their expanded degree sum.  The theorem does not assert
\(P\ge\Delta\) for such representations.  Extending the direct sum to that
compact case requires a simulation theorem or a different search measure;
the unrestricted ARQP conjecture therefore remains open. [pcc:unverified]

### Proposition 25 (expanded degree is not compact-DAG work)

For every integer \(k\ge1\), the multiplication morphism

\[
[k]:E\to E
\]

has algebraic degree \(k^2\).  Nevertheless, binary double-and-add evaluates
\([k]T\) with \(O(\log k)\) elliptic group operations and a shared rational
DAG of size \(O(\log k)\).  If \(G\) has prime order \(n\) and
\(k\not\equiv0\pmod n\), then \([k]\) permutes \(G\), so for every
\(\mathcal F\subseteq G\),

\[
\left|\{T\in G:[k]T\in\mathcal F\}\right|=|\mathcal F|.
\]

Thus neither expanded map degree nor the degree sum in Theorem 24 is a valid
lower bound for unrestricted compact-DAG size. [pcc:unverified]

#### Proof

The standard degree formula for multiplication on an elliptic curve is
\(\deg[k]=k^2\), including inseparable degree in positive characteristic. [pcc:unverified]
The binary expansion of \(k\) uses at most
\(\lfloor\log_2 k\rfloor\) doublings and that many conditional additions.
On the cyclic group \(G\), multiplication by a nonzero residue modulo \(n\)
is a permutation, which proves the displayed identity. \(\square\)

### Theorem 26 (degree-free compact global-map direct sum)

Let \(G\subseteq E(\mathbb F_p)\) have prime order \(n\), and let
\(\mathcal F\subseteq G\) have size \(N\).  A catalogue contains \(r\)
branches.  Branch \(j\) consists of \(m\) fixed global rational maps

\[
f_{j,i}:E\dashrightarrow E\qquad(1\le i\le m),
\]

represented by arbitrary compact shared DAGs.  On its success set
\(U_j\subseteq G\), all outputs lie in \(\mathcal F\) and

\[
\sum_{i=1}^m f_{j,i}(T)=T.
\]

The query algorithm may choose branches adaptively after seeing the target,
and DAG computations may be shared within and across branches.  The explicit
catalogue model charges only \(P\ge r\), with no charge proportional to map
degree.  For \(B\) independent uniform targets and \(Z\) returned
branch--target relations,

\[
\mathbb E Z\le\frac{BNr}{n}.
\]

Hence producing \(R=cN\) expected rows requires

\[
Br\ge cn,\qquad
P+B+T_{\rm verify}\ge2\sqrt{cn}.
\]

This gives an unconditional \(p^{1/2-o(1)}\) barrier for compact,
single-valued global rational selector branches when \(n=\Theta(p)\).
[pcc:unverified]

#### Proof

Because \(E\) is a smooth projective curve and the target \(E\) is proper,
each rational map \(f_{j,i}:E\dashrightarrow E\) extends to a morphism.
For any morphism \(f:E\to E\), put \(C=f(O)\).  The rigidity lemma for
abelian varieties shows that

\[
\phi(T)=f(T)-C
\]

is a group endomorphism of \(E\).  The subgroup
\(\ker(\phi)\cap G\) is either \(G\) or \(\{O\}\), since \(G\) has prime
order.  In the first case \(f|_G\) is constant; in the second it is injective.
This conclusion is independent of the degree and representation of \(f\).

Fix a branch \(j\).  If some \(f_{j,i}|_G\) is injective, then

\[
|U_j|
\le\left|\{T\in G:f_{j,i}(T)\in\mathcal F\}\right|
\le N.
\]

If every leaf map is constant on \(G\), their sum is constant, so the relation
equation holds for at most one target and again \(|U_j|\le N\).  Therefore
\(\sum_j|U_j|\le Nr\).

For a uniform target, branch \(j\) succeeds with probability \(|U_j|/n\).
Adaptive selection can only suppress successful branch--target pairs, so
linearity of expectation gives

\[
\mathbb E Z\le\frac{B}{n}\sum_j|U_j|\le\frac{BNr}{n}.
\]

The relation requirement gives \(Br\ge cn\); using \(P\ge r\) and AM--GM
proves the cost bound. [pcc:unverified] \(\square\)

Theorem 26 includes arbitrarily high-degree multiplication maps, isogeny
chains, compact straight-line coordinate formulas that globally map
\(E\) to itself, and shared group-law DAGs.  Unlike Theorem 24, it does not
charge expanded degree.

It does not cover multivalued correspondences represented succinctly without
degree expansion, coordinate circuits that land on \(E\) only on a finite
branch domain rather than defining a global rational map, or branching
programs that implicitly encode exponentially many distinct rational output
tuples while paying for only a compact control DAG.  Those are now the exact
remaining compact-circuit cases. [pcc:unverified]

### Proposition 27 (finite-domain selectors defeat global rigidity)

Let \(G\subseteq E(\mathbb F_p)\) be a finite subgroup.  Its reduced
evaluation algebra is

\[
\operatorname{Fun}(G,\mathbb F_p)
\cong\prod_{T\in G}\mathbb F_p.
\]

Consequently every set-theoretic function on \(G\), including every choice
function that assigns one verified factor-base decomposition to each target
in an attainable set \(U\subseteq G\), is represented by elements of this [pcc:unverified]
finite coordinate algebra.  Its leaf functions need not be restrictions of
global rational maps \(E\dashrightarrow E\), and they may be neither constant
nor injective.

An explicit dense representation stores \(O(m|G|)\) field elements for an
\(m\)-leaf selector.  A perfect hash on affine point coordinates, with a
separate infinity record, gives \(O(1)\) expected lookup after
\(O(|G|)\) preprocessing.  This is not a sub-rho algorithm: its preprocessing
is already \(\Theta(n)\).  It is an exact counterexample to any attempt to
extend Theorem 26 to finite-domain formulas using only the rigidity lemma.
[pcc:unverified]

#### Proof

The reduced zero-dimensional scheme consisting of the rational points in
\(G\) is a disjoint union of \(|G|\) copies of
\(\operatorname{Spec}\mathbb F_p\).  Its coordinate algebra is therefore the
displayed direct product, whose elements are precisely all scalar-valued
functions on \(G\).  Encode each selected leaf by its affine coordinates and
an infinity flag.  Coordinatewise evaluation represents an arbitrary tuple
selector; the selected values satisfy the curve and relation equations on
\(U\) by construction, even though the representing coordinate functions
need not satisfy those identities on the ambient curve.  Storing and hashing
the table gives the stated upper bounds. \(\square\)

Proposition 27 isolates the remaining compact problem.  A useful attack must
compress these finite-domain selector tables or recover their entries from a
succinct multivalued correspondence while charging less than square-root
total work.  Proving that impossible is a static data-structure or
finite-function circuit lower bound; morphism degree, properness, and the
elliptic rigidity lemma alone cannot supply it. [pcc:unverified]

### Theorem 28 (straight-line correspondence recovery becomes a section)

Let \(K=\mathbb F_p(E)\) be the target function field.  For each branch \(j\),
let \(X_j\) be an irreducible algebraic correspondence with a dominant target
map

\[
\pi_j:X_j\dashrightarrow E
\]

and rational leaf maps \(g_{j,i}:X_j\dashrightarrow E\).
Suppose a symbolic query program, after curve-specific preprocessing, uses
only additions, subtractions, multiplications, and verified inversions in
\(K\), with no algebraic root oracle or target-dependent zero-test branch. [pcc:unverified]
If it generically recovers a point of \(X_j\) above the target, then it defines
a rational section

\[
s_j:E\dashrightarrow X_j,\qquad \pi_j\circ s_j=\operatorname{id}_E.
\]

The recovered leaf maps

\[
f_{j,i}=g_{j,i}\circ s_j:E\dashrightarrow E
\]

are therefore global rational maps.  An explicit catalogue of \(r\) such
recovery branches satisfies Theorem 26:

\[
\mathbb E Z\le\frac{BNr}{n},\qquad
P+B+T_{\rm verify}\ge2\sqrt{cn}
\]

when \(cN\) expected relation rows are required.  This conclusion is
independent of the degrees and compact representations of \(X_j\), \(s_j\),
and the leaf maps. [pcc:unverified]

#### Proof

Every register of the straight-line program is an element of \(K\), because
the four permitted operations do not leave the target function field.
The output coordinates satisfy the defining equations of \(X_j\) over the
generic point of \(E\) and project to that generic point.  Equivalently, they
give a \(K\)-rational point of the generic fibre of \(\pi_j\), which is the
same as a rational section \(s_j\).

Composing with each rational leaf projection gives
\(f_{j,i}:E\dashrightarrow E\).  Properness extends these maps across their
indeterminacies.  They are exactly the compact global selector maps covered
by Theorem 26, whose constant-or-injective subgroup argument and
preprocessing/query calculation give the displayed bounds. \(\square\)

Thus a succinct multivalued correspondence is not by itself an escape.  A
query algorithm outside Theorem 28 must perform a genuine algebraic extension
or root extraction, branch using target-dependent finite-field tests, or be
correct only on a finite target set without defining a generic section.
The first case requires an explicit root-recovery cost model; the latter two
are the finite-domain compression problem isolated by Proposition 27.
[pcc:unverified]

### Proposition 29 (finite-field root recovery need not be a section)

Let \(p\equiv3\pmod4\), and consider the correspondence over the affine target
line

\[
X:\quad y^2=t.
\]

There is no rational section over the generic target:
\(t\) is not a square in \(\mathbb F_p(t)\).  Nevertheless, on every
quadratic-residue target \(t\in\mathbb F_p\), including \(t=0\),

\[
y(t)=t^{(p+1)/4}
\]

satisfies \(y(t)^2=t\).  Binary exponentiation evaluates this finite-domain
root selector using \(O(\log p)\) multiplications, although its expanded
polynomial degree is \((p+1)/4\).  It succeeds on \((p+1)/2\) of the \(p\)
targets. [pcc:unverified]

#### Proof

The \(t\)-adic valuation of a square in \(\mathbb F_p(t)\) is even, whereas
\(v_t(t)=1\), so \(t\) has no square root in the target function field.  For
a nonzero quadratic residue, Euler's criterion gives
\(t^{(p-1)/2}=1\).  Hence

\[
\left(t^{(p+1)/4}\right)^2
=t^{(p+1)/2}
=t\,t^{(p-1)/2}
=t.
\]

The identity also holds at zero.  There are \((p-1)/2\) nonzero quadratic
residues, and binary powering has logarithmic straight-line complexity.
\(\square\)

Proposition 29 is not an elliptic relation algorithm: it neither enforces a
factor base nor recovers an \(m\)-leaf decomposition.  It is a sharp
counterexample to extending Theorem 28 from generic symbolic recovery to
finite-field root algorithms by circuit size or expanded degree alone.
Frobenius identities and exponentiation can select roots on a large finite
target set without producing a rational section.  A complete compact-circuit
barrier must therefore charge finite-field root extraction or prove that its
outputs still cannot hit the structured elliptic factor base often enough.
[pcc:unverified]

### Proposition 30 (compact finite-field maps can have tiny image)

Let \(d\mid p-1\), and put

\[
h_d(t)=t^{(p-1)/d},\qquad
A_d=\{0\}\cup\mu_d(\mathbb F_p).
\]

Then

\[
h_d(\mathbb F_p)=A_d,\qquad |A_d|=d+1.
\]

Binary powering computes \(h_d\) with \(O(\log p)\) multiplications.  Thus,
even for fixed \(d\), a logarithmic-size finite-field circuit sends all \(p\)
targets into a constant-size structured set.  With \(d+1\) fixed equality
branches, the values in \(A_d\) may be relabelled by any chosen \(d+1\)
elliptic factor-base points, producing a compact finite-domain leaf circuit
whose output lies in that factor base for every target. [pcc:unverified]

#### Proof

The group \(\mathbb F_p^\times\) is cyclic of order \(p-1\).  The power
homomorphism with exponent \((p-1)/d\) has image size

\[
\frac{p-1}{\gcd(p-1,(p-1)/d)}=d,
\]

and every image element is a \(d\)-th root of unity.  Hence its nonzero image
is exactly \(\mu_d(\mathbb F_p)\); zero maps to zero.  Binary exponentiation
has logarithmic circuit size.  The final relabelling uses a fixed branch for
each of the \(d+1\) possible values. \(\square\)

Proposition 30 does not produce an elliptic relation: after selecting one
factor-base point, the remaining leaves must still lie in the factor base and
sum with it to the target.  It does show that no bound of the form

\[
\bigl|\{T:f(T)\in\mathcal F\}\bigr|
\le |\mathcal F|\,\operatorname{poly}(\operatorname{size}f)
\]

can hold for arbitrary finite-domain arithmetic circuits.  The final compact
barrier must exploit the **joint** \(m\)-leaf addition constraint, witness
recovery, or preprocessing/query trade-off; a per-leaf hit estimate is
insufficient. [pcc:unverified]

### Exact remaining computational gap

Even full \(q^2\)-monomial support rules out only monomial-explicit
two-leaf inversion.  It is not, by itself, an arithmetic-circuit lower bound:
a dense multilinear polynomial can have a small factored circuit, and
Propositions 11 and 12 give explicit examples in the present problem.

The decisive target is now:

> Define a preprocessing/query model that contains fast product trees,
> resultants, gcds, branching, and split-tree elimination, and prove that [pcc:unverified]
> producing enough verified relations costs \(p^{1/2-o(1)}\); or give a [pcc:unverified]
> concrete data structure whose fully audited total exponent is below
> \(1/2\).

The appropriate measure must charge *search* or a preprocessing/query
trade-off, not merely the size of a rational representation.  A direct-sum
or cell-probe-style theorem for algebraic set-intersection queries is a more
credible target than a \(q^2\) inverse-circuit lower bound, which Proposition
11 disproves.  Proposition 15 also prevents replacing that conjecture by an
unrestricted straight-line-circuit claim justified only from eliminant degree.
Theorems 8--10 still delimit materializing proof systems and show that
lookup-coordinate independence, finite-field genericity, and rational-lift
filtering do not create sparse expanded certificates.

### Scope

These results rule out a globally low-rank representation that amortizes all
targets at once. They do **not** rule out:

- a target-adaptive coordinate circuit whose output formulas land on \(E\)
  only over a finite branch domain rather than defining global maps;
- a succinct multivalued correspondence whose expanded projection degree is
  not materialized;
- randomized algorithms that tolerate false positives and verify candidates;
- implicit meet-in-the-middle or collision walks;
- an algebraic solver whose internal representation is not a separated exact
  factorization.

The next theorem must address one of those cases without quietly reverting to
candidate enumeration.

## 5. Target theorem: an algebraic preprocessing/query direct sum

Define an **algebraic relation-query program** (ARQP) for fixed \(m\) as
follows.

1. A curve-specific preprocessing phase receives \((p,E,F,m)\) and constructs
   a data structure of size \(S\) using \(P\) charged field or word
   operations.
2. A query phase receives one target or a batch of targets.  It may use field
   arithmetic, verified inversions, comparisons and branching, fast [pcc:unverified]
   polynomial products, remainders, gcds, resultants, multipoint evaluation,
   sparse or dense linear algebra, and split-tree substitutions.  A
   degree-\(d\) polynomial operation is charged its actual algebraic-RAM cost,
   at least the input/output representation length and at most the chosen fast
   arithmetic bound \(M(d)\operatorname{polylog}d\).
3. Intermediate objects may be factored polynomials, rational circuits,
   straight-line programs, or shared DAGs.  The model does not charge their
   expanded Boolean normal forms unless they are actually materialized.
4. The program must return verified tuples of rational factor-base points. [pcc:unverified]
   False positives, root recovery, memory probes, and extension-to-base-field
   filtering are charged.  Curve-specific preprocessing may be reused, but
   target-dependent work may not be relabelled as preprocessing.

The ambitious target is:

> **ARQP direct-sum conjecture.** For a density-\(1-o(1)\) family of general
> ordinary curves, every ARQP that produces enough independent relations for
> the factor-base linear system satisfies
>
> \[
> P+\sum_{j=1}^{B}Q_j+T_{\rm verify}+T_{\rm LA}
> \ge p^{1/2-o(1)}.
> \]
>
> Equivalently, a violation must expose a concrete reusable data structure
> and query algorithm, which can then be audited as a
> \(p^{1/2-\varepsilon}\) ECDLP candidate.

This is deliberately a search/direct-sum statement, not a representation-size
claim.  Proposition 12 is contained in the model and shows that a single
two-leaf query can cost only \(\widetilde O(q)\).  The theorem must instead
show that preprocessing plus the number of target queries and recoveries
cannot all be amortized below the collision frontier.  To make the conjecture
provable, the next formalization should first restrict polynomial and rational
objects to explicit coefficient arrays, subproduct trees, and bounded-fan-in
shared circuits, with a simulation lemma for the listed primitives.

## 6. Three proof routes

### Route A: target-specialized flattening rank (resolved as insufficient)

Find a leaf partition \(L\sqcup R\) such that, for a positive fraction of
targets \(T\), the matrix

\[
H_T[u,v]=\mathbf 1[\sigma_L(u)+\sigma_R(v)=T]
\]

has large rank after quotienting duplicate partial sums. This matrix is a
partial permutation matrix, hence its rank equals the number of matching
distinct partial-sum pairs.

Theorem 4 settles the basic calculation exactly. When the expected number of
decompositions of one target is \(O(1)\), this target-specific rank is
\(O(1)\) on average. A proof must use a stronger measure or track adaptive
search branches; single-target matrix rank alone is insufficient. Route A
should therefore be treated as a diagnostic, not the principal proof route.

### Route B: a direct-sum theorem

Prove that answering \(B\) independently uniform targets with a shared [pcc:unverified]
preprocessing object costs, up to subpolynomial factors, at least the smaller
of explicit candidate materialization and \(B\) target-specific searches.
Combining this with the exact expected relation count
\(B|\Omega|/n\) would recover the square-root frontier.

This is the cleanest route, but it needs an explicit circuit or data-structure
model; unrestricted arithmetic-circuit direct-sum theorems are out of reach.

### Route C: polynomial-calculus monomial growth (materializing systems only)

Work in the Boolean quotient

\[
A=\mathbb F_p[b_{ij}]/(b_{ij}^2-b_{ij}),\qquad \dim A=2^{mk}=N^m.
\]

Prove that deriving a target-specific leaf assignment, or deriving \(1\) for [pcc:unverified]
an insoluble target, requires a graded component containing
\(p^{1/2-o(1)}\) square-free monomials.

Degree alone is insufficient: a degree-\(d\) component contains
\(\sum_{i\le d}\binom{mk}{i}\) square-free monomials, and a useful theorem must
locate \(d\) relative to \(mk\), not merely show \(d=\Omega(\log p)\).
Proposition 11 proves that this route cannot lower-bound rational circuits [pcc:unverified]
with inversion gates: their expanded Boolean normal form may be dense while
their straight-line representation is logarithmic.  Route C remains relevant
only after fixing a proof system that explicitly charges monomial
materialization.

## 7. Falsification program

The barrier conjecture should be attacked, not protected. Look for:

1. **Target-linearization:** a change of variables making the target enter only
   through a low-dimensional linear sketch.  Proposition 23 gives the exact
   low-degree shift \(x(T-P)\), but not a way to avoid scanning the other
   partial side.
2. **Low partition rank:** an exact or one-sided-error decomposition of the
   split-tree relation tensor below the collision frontier.
3. **Reusable partial elimination:** a curve-specific data structure of size
   \(p^\beta\) with query/recovery time \(p^\gamma\), whose complete relation
   and linear-algebra balance is below \(p^{1/2}\).
4. **Hidden group compatibility:** a finite-set map not induced by a
   bounded-degree \(\mathbb G_a\), \(\mathbb G_m\), or isogeny morphism, but
   transporting enough convolution to elliptic addition.
5. **One-sided randomized filters:** cheap filters with a proved [pcc:unverified]
   false-positive rate whose verification does not restore square-root work.

Every candidate must be evaluated by

\[
T_{\rm total}=T_{\rm FB}+T_{\rm pre}
+\frac{M}{\rho hP_{\rm rel}}T_{\rm query}+T_{\rm LA}.
\]

## 8. Experimental protocol

Experiments are diagnostic only. For each sampled curve and parameter tuple
\((p,N,m)\), record:

- factor-base size after rational lifting;
- ordered tuple count and support size of the \(m\)-sum distribution;
- collision probability, maximum fibre, entropy, and total-variation distance
  from uniform;
- the exact leaf--target incidence rank, equal to the support size;
- partial-sum supports and target-specific partial-permutation ranks for a
  balanced cut;
- Macaulay column counts and observed ranks by degree;
- preprocessing and target-specific wall time separately.

Do not fit an asymptotic exponent from fields below the regime where
\(\log p\), \(k\), and \(m\) all vary.

The accompanying experiments/sumset_profile.py computes this combinatorial
profile on small prime fields without Sage.

## 9. Decision gates

### Gate 1: model closure

Show that Boolean F4/F5 or XL elimination, tensor-network contraction, and
bounded-width arithmetic branching programs embed in ARQP with polynomial
overhead, while product trees, fast gcd/resultant algorithms, and Proposition
12 embed with their soft-linear costs.

### Gate 2: first adaptive lower bound

Theorem 18 supplies the exact multi-target preprocessing/query direct-sum
bound for static-provenance product trees with arbitrary adaptive query
branching.  Theorem 24 supplies the corresponding square-root direct sum for
degree-explicit algebraic selectors with adaptive branch choice.  Theorem 26
removes the degree charge for compact single-valued global maps in an explicit
branch catalogue.  The remaining Gate 2 extension is to succinct multivalued
correspondences recovered through genuine algebraic root operations,
finite-domain coordinate formulas, and control DAGs that implicitly generate
exponentially many output branches.  Theorem 28 closes symbolic straight-line
recovery that remains in the target function field, while Proposition 29
shows that compact finite-field root extraction need not define a generic
section.  A width or monomial lower bound counts only if a simulation theorem
charges the remaining compact representations.  Theorems 21--22 and
Proposition 23 further show that this extension must cover joint or
target-dependent witness search, rather than only exact labels,
false-positive verification, or target-shifted scans.
`[pcc:unverified]`

### Gate 3: square-root closure

For the static-provenance subclass, Theorem 18 combines the adaptive bound
with exact relation-output requirements and gives
\(p^{m/(2m-1)-o(1)}\), hence \(p^{1/2-o(1)}\), total cost.  Gate 3 remains open
for implicit candidate generators and the full ARQP. `[pcc:unverified]`

### Gate 4: algorithmic inversion

If a lower-bound step fails because of an explicit low-rank or reusable
representation, turn that witness into an algorithm and audit its exponent.

## 10. Near-term paper-quality deliverable

Theorem 18 supplies the first item below for the static-provenance subclass,
Theorem 24 extends it to degree-explicit algebraic selector catalogues, and
Theorem 26 handles compact single-valued global maps without a degree charge.
The next publishable extension should be one of:

1. an extension of the exact direct-sum theorem from explicit provenance to a
   closed class of compact split-\(S_3\) candidate generators;
2. a polynomial-calculus monomial lower bound of \(p^{1/2-o(1)}\) for
   insoluble random targets, explicitly scoped to materializing proof systems;
   or
3. a concrete data structure with proved preprocessing/query exponents that [pcc:unverified]
   violates the conjectured trade-off.

Theorem 1 makes “global low tensor rank” an unpromising escape, while
Propositions 11 and 12 show that expanded support is not a general circuit
measure.  Theorems 21--22 close exact and one-sided target-independent
separate labels, Proposition 23 audits the simplest target-dependent
coordinate filter, Theorem 26 closes compact global single-valued maps, and
Theorem 28 closes straight-line rational recovery from correspondences.
Proposition 29 shows why finite-field root extraction is a distinct case.
Proposition 30 further rules out per-leaf factor-base hit bounds for arbitrary
finite-domain circuits.  Research should now concentrate on the joint
all-leaf addition constraint, finite-domain branching, implicit branch
generation, and multi-target witness recovery.
