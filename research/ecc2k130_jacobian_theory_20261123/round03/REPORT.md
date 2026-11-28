# Genus131: elliptic complements, gluing degrees and a norm263 candidate

2026-11-23. Scope: J(C)/F2 isogenous to A131(E0)*E, where E is elliptic.
Written deductions and native arithmetic only; all PCC standings are
unverified. No curve or subgroup correspondence is constructed. [pcc:unverified]

## Findings

1. The complementary elliptic trace-2 is impossible: it would give
N1(C)=4>N2(C)=2. The other four traces satisfy all closed-place
nonnegativity conditions; the finite replay through19 plus the uniform
bound in section2 establish the latter statement as written mathematics.
[pcc:unverified]

2. Exact real-Weil resultants restrict the degree d of the associated
optimal map C->E to a divisor of the integers in section3. These are
necessary gluing constraints, not constructions. [pcc:unverified]

3. If the A-factor stays maximal atell=146505763881528721, traces0/1 are
excluded by the split-support Artin obstruction. For trace2, d must be
divisible by Q=1213355897362451. Losing maximality atell remains a separate
escape, requiring an ell-divisible isogeny from the natural A. [pcc:unverified]

4. If C contains the natural A itself as its A-subvariety and d<Q, the
remaining possibility is E=E0 with d=131*263=34453. Its polarization
requires a totally positive generator of the specified norm263 ideal
(263,u+1) in K+=Q(u), u=pi+2/pi. This necessary condition is not yet
decided. The lower quadratic norm obstruction passes. [pcc:unverified]

These results do not exclude arbitrary isogenous A-factors, large-degree
maps or carriers of genus132–300. The original existence/map question
remains open.

## 1. Exact input and native arithmetic

The source A, E0, alpha,beta,pi,zeta and K are those pinned in the parent
REPORT.md. E0:y²+xy=x³+1 has trace-1. Let s0=2,s1=-1 and
s_(m+2)=-s_(m+1)-2s_m. Native C++17/GMP computes s131=-22283658519494248867,
constructs the integer polynomial

    P_A(X)=(X^262-s131*X^131+2^131)/(X²+X+2),
    P_A(X)=X^130*h_A(X+2/X),

and checks every coefficient of these identities by exact subtraction.
For each t=-2,-1,0,1,2, a second path evaluates P_A at a formal root
gamma of X²-tX+2 and checks

    Norm(P_A(gamma)) = 2^130 * h_A(t)^2.

The reduced real-Weil resultant against X-t is exactly |h_A(t)|:
polynomial remainder gives h_A(t) in the ideal, and evaluating any
integer combination at t shows every constant in that ideal is a
multiple of h_A(t). This is not the larger full-Weil resultant above.
[pcc:unverified]

All five elliptic classes occur. The checker enumerates the32 general
Weierstrass coefficient tuples over F2, applies the discriminant criterion
and counts points. One smooth fixture for each trace is retained in
summary_final.txt. PARI2.19.1, built from the official SHA-pinned source,
independently replays H degree130, rational irreducibility,130 real roots
and all five evaluations. It is a second implementation, not independent
human review or a signed execution proof. [pcc:unverified]

## 2. Point counts: one exclusion and four all-degree passes

For m not divisible by131, Tr(F^m|A)=-s_m; for multiples it is130*s_m.
If the elliptic complement has trace t, let b0=2,b1=t and
b_(m+2)=t*b_(m+1)-2b_m. Then

    N_m=2^m+1-Tr(F^m|A)-b_m,
    N1=2-t,   N2=6-t².

For t=-2 this predicts four F2 points but two F4 points, contradicting
inclusion of rational points under field extension. [pcc:unverified]

For the four survivors, exact divisor subtraction gives nonnegative
closed-place counts B_m through262, with integer results throughout.
The smaller finite certificate places_first19.tsv suffices for the
following post-screen analytic extension to every m. [pcc:unverified]

A genus131 Weil polynomial has |Tr(F^m)|<=262*2^(m/2). Put k=floor(m/2).
For m>=20, the sum over all possible proper-divisor degrees is bounded by

    sum_(j=1..k) N_j
      <= 2^(k+1)-2+k+1048*2^ceil(k/2).

Here sum 2^ceil(j/2) <=4*2^ceil(k/2). Also

    N_m >= 2^m+1-262*2^ceil(m/2).

For m>=20 we have floor(m/2)>=10, m-k-1>=9,
m-ceil(k/2)>=15, and k<=2^(m-10). Consequently

    262*2^ceil(m/2) + 2^(k+1) + k + 1048*2^ceil(k/2)
      <= (9528/32768)*2^m < 2^m.

The fixed exponent inequalities are monotone beyond20; k<=2^(m-10)
follows at20 and by induction. Thus N_m exceeds the sum of all N_j with
proper-divisor degrees. All N_j are nonnegative: checked through19 and
positive from the Weil bound above thereafter. In the Mobius expression
m*B_m=sum_(a|m) mu(a)N_(m/a), discarding positive proper-divisor terms
therefore gives a positive lower bound N_m-sum_(j=1..k)N_j. Hence B_m>0
for all m>=20. [pcc:unverified]

Integrality for all m follows from the integer formal power series
P_C(T)/((1-T)(1-2T)): recursively extracting its Euler factors gives
integer exponents. The exact m<=19 certificate and this tail argument
show that none of the four surviving classes is excluded by negative
closed-place counts in any degree. This necessary zeta-function condition
is not sufficient for curve existence. [pcc:unverified]

## 3. Exact finite gluing-degree bounds

Write J for the hypothetical polarized Jacobian. Howe2022 Proposition3.7
provides intrinsic subvarieties A' and E', with A' isogenous to A and E'
of trace t, and a graph kernel Delta in A'*E'. The induced polarizations
have kernel Delta. On E' the polarization is [d], so Delta=E'[d].
Proposition3.10 and Theorem4.15 imply d divides D_t=|h_A(t)|, and give the
corresponding degree-d curve map C->E'. A Jacobian of genus>1 is
geometrically indecomposable, so d=1 is excluded. [pcc:unverified]

| t | D_t factorization | status |
|---|---|---|
|-2|523*183691462360100297|excluded by point counts|
|-1|131*263*146505763881528721|remaining|
|0|142248971*156652511177|remaining|
|1|142248971*156652511177|remaining|
|2|14149*1213355897362451|remaining|

All listed factors are prime and all D_t are squarefree. primes.cpp
retains trial checks or full-factorization Lucas certificates; the parent
check.cpp supplies ell's certificate. The products agree with the exact
integer evaluations, not decimal approximations. [pcc:unverified]

Since d divides one of these odd integers, the map is separable. The
degree bound concerns the optimal elliptic complement chosen by the
polarized decomposition; composing it with arbitrary elliptic isogenies
can produce other map degrees outside this list. [pcc:unverified]

## 4. Consequence of split versus inert gluing support

Assume A' stays maximal at all primes overell. As in round02, choose a
conjugation-stable coefficient order S=R+263^k O contained in End(A'),
where R=Z[pi,2/pi], O=O_K and k is sufficiently large. The nonzero Artin
class of A's natural polarization survives in B(S), because O/S is
supported only at263 and that rational prime splits completely in K.
Polarization-class invariance along the S-linear isogeny A->A' forces
any polarization on A' to have that nonzero class. [pcc:unverified]

If every prime supporting Delta is split in K/K+, and S is maximal at
those primes, its self-dual module is a sum of conjugate pairs of simple
modules. Each pair is zero in B(S). Thus such a kernel cannot be the
polarization kernel on A': it would have class zero, a contradiction.
This is the split-support obstruction used here. [pcc:unverified]

For t=0 or1, both resultant primes are1 modulo131 and quadratic residues
modulo7, hence split completely in K. Neither is263. Thus these traces
require A' nonmaximal atell. Every isogeny A->A' then has ell-divisible
degree, by the integral Tate-module argument in round02. [pcc:unverified]

For t=2,14149 splits completely in K. The other prime Q is-1 modulo131
and3 modulo7; its Frobenius in K is conjugation, so primes above Q are
inert over K+. Therefore, if A' is maximal atell, d cannot be14149 and
must be divisible by Q=1213355897362451. [pcc:unverified]

For t=-1, d=263 has only split support at the coefficient-order singular
prime. This case is not covered by the preceding simple split-support
argument without an extra local check. At the selected real-Weil root
u=-1, h_A'(-1)=47 modulo263. Thus the two conjugate Weil roots alpha,beta
are simple unpaired roots, and R is locally maximal on both corresponding
O-prime factors. Delta=E0[263] is supported exactly on those two factors.
Its class is a conjugate pair, zero in B(S), even if S is nonmaximal at
other263 factors. Hence d=263 is excluded under A' maximal atell as well.
[pcc:unverified]

This extra localization is necessary: it would be incorrect to assume
that every263-supported kernel is split at a singular coefficient order
without checking the contracted primes. The native derivative47 and
round02 collision labels identify the two simple factors used here.
[pcc:unverified]

## 5. The remaining natural-A candidate below Q

Now assume A' is the natural A itself, not just an arbitrary isogenous
representative, and d<Q. The preceding conditions leave E'=E0 with

    d=131  or  d=131*263=34453.

For d=131 the O-module ker(lambda_A) has cardinality131² and is the
unique residue module O/P131. In particular zeta-1 kills it. The
polarized automorphism (zeta,1) of A*E0 therefore fixes the graph kernel
and descends to J. Torelli supplies an order131 curve automorphism.
Riemann-Hurwitz at genus131 forces quotient genus1 and two geometric
ramification points. An F2-defined tame order131 action requires every
ramification point degree to be divisible by ord_131(2)=130; two points
are impossible. Therefore d=131 is excluded. [pcc:unverified]

For d=34453, let lambda0 be the natural polarization on A, with kernel
E0[131]. Any other polarization is lambda0*c for a totally positive
c in K+. At131 the kernels of both polarizations have O-length1, so c
has valuation0. At263 the new kernel is E0[263], with Frobenius roots
alpha and beta, supported on the conjugate pair singled out by u=-1.
At all other primes the kernel is trivial. Hence

    (c)=p263=(263,u+1)O_(K+),    Norm_(K+/Q)(c)=263,
    c is totally positive.

The simple root H'(-1)=47 modulo263 identifies p263 as a single prime
of norm263 in the maximal order. Thus the degree34453 natural-A route
requires this particular ideal to be narrow principal. Merely producing
some element of absolute norm263 would not suffice. [pcc:unverified]

The real quadratic subfield F=Q(sqrt(917)) gives a useful necessary norm
test, but it passes: (395+13*sqrt(917))/2 is an algebraic integer, positive
in both embeddings, of norm263, because395²-917*13²=1052. The two conjugate
choices cover the two primes in F. This is not a lift to K+ and does not
decide narrow principality of p263 in degree130. [pcc:unverified]

Even a positive generator would establish only the required induced
polarization condition. A compatible gluing, geometric indecomposability,
Torelli realization as a curve and an evaluated subgroup correspondence
would still be required. None has been supplied here.

## 6. Sources, execution and remaining dependencies

Sources checked: KB unavailable (no loaded tools or executable); fallback
was the exact parent reports and primary Howe2022 arXiv2110.04221
Propositions3.7/3.10, Theorem4.15, together with the parent Howe1996
Artin-obstruction proof. Official PARI documentation for bnfisprincipal,
bnfcertify and bnfinit was read before preparing the next calculation:
https://pari.math.u-bordeaux.fr/dochtml/ref/General_number_fields.html .
A provisional bnf class group cannot certify nonprincipality without
removing its GRH assumption. A returned generator can instead be checked
by exact ideal arithmetic and real-embedding signs. [pcc:unverified]

The optimized native screen, UBSan replay and static PARI polynomial
replay agree. Full geometric/CM/gluing Lean bridges are not formalized;
no unsigned replay is represented as a signed PCC or Lean proof.
The all-degree tail bound is a written argument added after the finite
screen, not a preregistered finite-search prediction. [pcc:unverified]

The raw places.tsv is112897 bytes. Artifact upload failed because this
executor lacks AWS credentials. It remains local, gitignored, with its
SHA-256 and regeneration command in RAW_STATUS.md. The finite certificate,
summary, factor certificates, source and field polynomial are published;
full raw remote archival is explicitly unfinished. No policy threshold
was bypassed by renaming, compression or sharding.

For a larger principality run, the repo's isolab-eks skill and cloud cost
controls require current shared budget/capacity readback before launch.
The GitHub-authenticated alternative was inspected: read-only telemetry
run45219644370 failed OIDC assumption with Not authorized to perform
sts:AssumeRoleWithWebIdentity; budget reads were skipped. Local AWS STS
also has no credentials. No cloud class-group job was launched, and no
negative principality conclusion is drawn from this access limitation.

A separate fixed-input native certificate attempt was made on the already
active executor, capped at30 seconds,512MiB PARI stack and1GiB address
space. It exited124 before bnfinit(H,1) returned, so no class vector or
generator was obtained. This was not a paid cloud launch or a runtime
comparison. Inputs, output and exit status are retained. It gives no
evidence of nonprincipality. The next materially different implementation
step is an explicit real-cyclotomic integral basis, rather than repeating
the same power-order initialization with a longer timeout. [pcc:unverified]

The original genus130–300 curve/map objective remains unresolved.
Next discriminating obligation: decide whether p263 has a totally positive
generator, with an exact witness or a certified negative answer. A failure
of this one candidate would not exclude other isogenous representatives,
larger cover degrees or genus132–300 carriers. Existing cost charts stay
unchanged: no destination solver or measured cost was added.
