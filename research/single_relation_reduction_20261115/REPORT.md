# One checked relation from a random labelled base

This assesses a published alternative to the auxiliary-fibre route. It removes
auxiliary-curve setup and the final relation matrix, but supplies no new fast
relation selector. The unconditioned finite-group coverage argument now has a
complete Lean endpoint. Elliptic conditioning, the conditional cost and native
program behavior remain separate hand/implementation obligations. No numerical
call is made. [pcc:unverified]

PCC-20261115-3dfae3 records the hand assessment and remains unverified.

## 1. Interface and finite existence bound

Let G=<P> have odd prime order r, let Q=[k]P, and assume the usual valid-input
ECDLP promise. G may be a subgroup of a larger elliptic group. Choose m>=3 and
b>=1, with m disjoint lists of b *records*. Draw independent uniform labels
a_ij,b_ij in F_r and set R_ij=[a_ij]P+[b_ij]Q. These points are independent
uniform in G: for every fixed second label, the first translates bijectively.
Labels stay outside the selector. This is ideal independent randomness, not a
claim about a fixed native seed. [pcc:unverified]

Count only positive zero-sum tuples, choosing one record from each part. Let
L=b^m and Z be their number. A fixed tuple sums to O with probability1/r.
Two distinct tuples differ in a part; each has a different random point in
that part which the other tuple does not use. Conditional on all other
points, these two free points determine their two sums independently.
Consequently the two events have joint probability1/r^2, even when the
tuples overlap in other parts. Therefore

    E Z = L/r,
    E Z^2 = L/r + L(L-1)/r^2.

Cauchy--Schwarz on Z times its nonzero-support indicator gives

    Pr[Z>0] >= (E Z)^2/E Z^2 = L/(r+L-1).

For L>=r this is at least1/2. Signed tuples can only increase existence;
they are not counted as pairwise independent in this calculation. The
calculation establishes existence, not an algorithm for finding a tuple.
[pcc:unverified]

## 2. Clean abscissas and private labels

For a short Weierstrass curve in characteristic>3, reject a sampled base if
it contains O or any two records have the same abscissa. Let B=mb. One record
is O with probability1/r. Given one point, at most two subgroup points have
the same finite abscissa, since they must be equal or opposite. A union bound
therefore gives

    Pr[bad] <= B/r + 2*binom(B,2)/r = B^2/r = epsilon.

It follows without an independence assumption that

    Pr[clean and Z>0] >= L/(r+L-1)-epsilon.

If L>=r and epsilon<=1/4, resampling whole bad bases has expected count<=4/3,
and a retained clean base is covered with probability at least1/4. Rejecting
duplicates must not be modelled as leaving the points independently uniform;
the displayed unconditional subtraction is what justifies the conditioned
bound. [pcc:unverified]

For each record, write its hidden scalar as u_ij=a_ij+k*b_ij. The change of
variables (a,b)->(u,b) is a bijection. Conditional on *all* public points, all
second labels remain independent uniform. This persists after point-only
base rejection, oracle randomness, failure and relation selection. A selector
returns one record per part and signs c_i in{1,-1}, with the actual group sum
checked equal to O. The coefficient vector on records is nonzero because
parts are disjoint. Thus, conditional on the point-only selected relation,

    alpha=sum c_i*a_i,   beta=sum c_i*b_i,
    alpha+k*beta=0,      Pr[beta!=0]=1-1/r.

Recover k=-alpha/beta and verify [k]P=Q; otherwise restart. No supplied base
logarithm or final B-by-B elimination is needed. A selector that examines
private labels does not satisfy this argument. [pcc:unverified]

One tempting shortcut fails exactly: choose m-1 labelled points, then set the
last point and both its labels to the negatives of their sums. The forced
zero-sum relation has alpha=beta=0. It is guaranteed to exist but recovers
nothing. This dependent construction does not satisfy the independent-base
sampling law above. [pcc:unverified]

## 3. The complete conditional procedure and its charged cost

Handle Q=O directly. Put n=ceil(log2 r), choose m=max(3,ceil(sqrt(n))) and
b=ceil(r^(1/m)). These specify integer operations; no parameter tuning from
results is allowed. L>=r and B=2^(O(sqrt(n))). Test the exact inequality
4B^2<=r. If it fails, use exhaustive scalar search; this occurs only for a
bounded set of r because B^2/r tends to zero. This finite fallback does not
need an unspecified asymptotic cutoff. [pcc:unverified]

For the other inputs, repeatedly sample a clean base, invoke a label-free
selector with a known per-call cap T, check its signed relation, recover and
verify the scalar as above. A capped unknown or failed base is a charged
restart. If this selector is complete on covered bases, each clean trial
succeeds with probability at least(1/4)(1-1/r), hence at least1/6 for r>=3.
If it is selective, instead require a known lower bound theta on its valid
answer probability under the clean-base distribution. Existence alone does
not establish theta for that algorithm. [pcc:unverified]

Sampling costs O(B log r) group operations plus sorting/canonical point checks.
Building the m degree-b membership polynomials naively costs O(m b^2) field
operations. If the selector returns only abscissas, charge a 2^m sign search
and actual group checks; all known ordinates are in F_p. Thus a safe expected
bit-cost envelope, with N the input bit length, is

    O((2^(O(sqrt(n)))*poly(N) + T(N))/[theta(N)*(1-1/r)]).

Known effective subexponential T and theta^-1 would meet the requested
complexity class. They are not supplied here. A generic meet-in-the-middle
selector costs b^ceil(m/2) up to signed-list and polynomial factors, retaining
an exponential scale in n; it does not fill this gap. [pcc:unverified]

## 4. Why the present all-column solver is not a fast direct implementation

The direct APS system uses a degree-b polynomial f_i(X_i) for each part.
Consider the current critical-pair producer's actual Index architecture,
after hypothetically removing its fixed field/variable/degree caps. It loads
original input polynomials only at a degree bound D>=b and materializes all
monomials of degree<=D in m variables before reduction. Hence its column
allocation is at least

    M=binom(m+D,m) >= binom(m+b,m) >= b^m/m! >= r/m!.

The middle inequality follows from
m!*binom(m+b,m)=product_(j=1..m)(b+j)>=b^m. At m=O(sqrt(n)), log2(m!)=o(n),
so that named allocation alone has size2^(n-o(n)). Its existing fixed D<=6
admission is not a growing implementation. Raising it cannot make this
complete direct index subexponential in n. When log p=Theta(log r), it is
also exponential in full input length. This is not a lower bound for every
sparse, circuit, Gröbner or dedicated selector, nor a solving-degree theorem
for the earlier auxiliary norm system. [pcc:unverified]

The 2026 deterministic root-finding theorem takes a univariate polynomial and
charges its degree, together with elliptic preprocessing. Here the membership
roots are already known. It does not construct the missing multivariate
selector or make an implicitly huge eliminant free. No claim that its ideas
could never help a different construction is made. [pcc:unverified]

## Evidence and sources

This is source review and hand mathematics, not a measured scaling study.
SCOPE.md and checks/ retain exact source identities. APS2018 equations7--11
are the prior framework; its section3.2 leaves the selector cost unestimated.
The existing native explicit-relations GPU service is an exact generic
meet-in-the-middle backend and claims no subexponential contraction. Shared
supporting Lean laws and the full unconditioned coverage endpoint are checked
separately. Elliptic interpretation, asymptotics and programs are not formalized.
No new native panel is run while checkpoint access is unavailable.

Shared support sourcead0b10055fe01cc8fba761382a0481491c947bb5 passes its
second build and canonical local gate:111theorems,0failures,0trusted native
exceptions. Its first build failed for missing explicit imports; declarations
and proof bodies were unchanged. The three theorem targets are
CryptoLean.SingleRelation.mixed_label_fiber_card, recover_from_relation and
direct_index_size, with the supporting semantic fibre equivalence. Exact
logs/source bindings are copied under checks/shared-laws/. These prove the
stated field/cardinality and arithmetic facts, not the entire probabilistic
reduction or an implementation. [pcc:unverified]

Shared PR11 merged as33c3451feeb45ad1785c14aeac97090ea45023dc. Its hosted
run43738335731 executed zero steps and the annotation explicitly reports
the same account billing lock. No hosted pass is asserted. [pcc:unverified]

## A1. Full finite-group coverage formalization

At shared source2e4b08b719452f49bfef91e5c31329f2e251b445, the theorem
CryptoLean.PartiteCoverage.coverage_count works over an actual finite additive
commutative group G, with m,b>0. It constructs tuple sums on the assignment
space Omega=(Fin m times Fin b)->G. Single and distinct-pair sum maps are
surjective, giving exact event counts and both moments. Finite Cauchy--Schwarz
on the actual support proves the following, with r=|G|, L=b^m and N=r^(mb).
[pcc:unverified]

    L*N <= (r+L-1) * |{x in Omega : exists a, sum_i x(i,a(i))=0}|.

Dividing by N gives the uniform coverage lower bound L/(r+L-1). The formal
statement uses integer cardinalities; no probability measure notation or
assumed moment axioms replace the actual events. Prime order is unnecessary
for this counting result. This endpoint checks unconditioned coverage; it
constructs no relation selector. [pcc:unverified]

The canonical repository-wide source/axiom gate audits134theorems,0failures,
0trusted-native exceptions. This module contributes16audited declarations,
including two definition equations. Complete gate/source hashes and all six
build logs are copied under checks/coverage-laws/. Builds001,004and005failed
for type inference/rewrite normalization; builds002,003and006pass their
respective source versions. No endpoint was weakened or limit raised.
PCC-20261115-950598 remains unverified; local proof checking is distinct from
a signed PCC verdict. [pcc:unverified]

Shared PR13 merged as865577bef93b64880cf6aba434f13c5b624488ac. Hosted
run43741500945 executed zero steps in both jobs; its annotation explicitly
reports the account billing lock. The local source/axiom pass is the evidence
here, with no hosted pass asserted. [pcc:unverified]

## A2. A concrete coordinate partial join

[SELECTOR-JOINS.md](SELECTOR-JOINS.md) checks fixed point-independent slope
buckets y-lambda*x as a candidate first merge for a birthday tree. For each
known curve point and slope, at most two other distinct-abscissa curve points
lie on the line. Therefore J first merges with list length b and s slopes
have admission probability at most 2Jb^2s/r; after clean-base conditioning it
is at most 8Jb^2s/(3r). With W fresh retained bases, the union bound is
8WJb^2s/(3r). At the stated balanced parameters and subexponential s,W this
is 2^(-n+o(n)). This rejects that small fixed-slope schedule's useful yield.
Adaptive slopes and other join algorithms are outside this conclusion.
The probability/cost argument is a hand derivation; actual curve-fibre counts
are a separate shared formal target. No native numerical case is run.
[pcc:unverified]

## A3. Implicit intervals and the uniform-list assumption

[INTERVAL-JOINS.md](INTERVAL-JOINS.md) checks the proposed near-certain
retention of old abscissa constraints for independent uniform subset points.
For a finite group subset S with nontrivial character sums bounded by M,
the actual triple count satisfies |T(S)-s^3/r|<=M*s. Shared source5b7a9d2
checks this generic conditional endpoint; its canonical source/axiom gate
audits149theorems/0failures/0trusted-native exceptions. All four builds and
pre-edit/final assembly probes are retained in checks/interval-laws/.
Claims730397/ab6d22 are unverified. [pcc:unverified]

For a full prime-order elliptic group in characteristic>3, the cited
Kohel--Shparlinski x-coordinate and interval estimates give
M=1+4sqrt(p)(1+ln p). At interval density1/b with b=2^(O(sqrt(log2 r))), the
return probability is asymptotic to1/b. This refutes near-certain retention
in that independent uniform model. The curve analytic input and actual
sampling law remain external/hand bridges; no conclusion for all interval,
correlated-list or adaptive selectors is claimed. [pcc:unverified]
