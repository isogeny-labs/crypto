# Round 03 — a compact CM model with a smaller anomalous target

The original GOAL.md remains active and unchanged. This is a new candidate
setup and development correctness suite. It does not supply or presume a
source-point-to-target-point evaluator.

## Sources checked before implementation

KB unavailable: no research-KB MCP; `scripts/research-kb status` exits127.
Read repository CT105, CT115, CT116, CT119, CT122–124, the open
lift-reduce-second-prime problem, and the CM/anomalous lead-sweep results.
The CM height sweep tests coverage of a small reduced set; it does not test
this common-model construction. CT116 warns that a labelled orbit selector
has already recovered the source logarithm. CT124 rejects two independent
local root checks as a global torsion certificate. No novelty/absence claim.

Primary sources read:

- Leprévost–Monnerat–Varrette–Vaudenay, *Generating Anomalous Elliptic
  Curves*, §§2–4: https://www.monnerat.info/publications/anomalous.pdf.
  It explains Smart's local solver and CM norm construction, but explicitly
  excludes the special j=0 equations from its displayed generic formula.
  Here six sextic twists are checked directly.
- Morain, *Elliptic curves, primality proving and some Titanic primes*,
  Theorems1–3 and their corollaries (1991):
  https://smf.emath.fr/sites/default/files/2023-08/AST_1991__198-199-200__245_0__sample.pdf.
  Use the Pocklington factor criterion and the elliptic certificate with a
  prime order exceeding (N^(1/4)+1)^2; require nonzero reduction at every
  prime divisor, not just nonzero modulo N as a composite ring element.
- Galbraith, *Mathematics of Public Key Cryptography*, chapter25 §§25.1:
  https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch25.pdf,
  for isogeny kernels and degree/composition. Chapter26 covers ordinary
  pairings, not a cross-characteristic map. The requested MIT lecture URL
  was inaccessible; no result is attributed to that unread lecture.
- Existing Mathlib Zsqrtd.norm_def and multiplication laws, pinned4.24.

## Candidate and mathematical obligations

Let u>1, b=2u−1, r=3u(u−1)+1, and p=4r+3. Require r,p prime.
In K=Q(omega), omega²+omega+1=0, set

    pi = u+b*omega,  pi_bar=1−pi,  alpha=1+2*pi_bar.

Classical CM gives trace(pi)=1, norm(pi)=r, trace(alpha)=4,
norm(alpha)=p. Choose a sextic twist y²=x³+Bp over Fp with order4r,
and one y²=x³+Br over Fr with order r. Construct an ordinary integer
B0 by CRT; use E:y²=x³+B0 over K. Coefficients are materialized integers
of logarithmic bit length, rather than2^r.

Pin omega at the source by zeta_p=(3−2u)/(2b) mod p and at the target
by zeta_r=−u/b mod r. Check each is a nontrivial cube root of unity.
The point endomorphisms alpha and pi should reduce to the respective
Frobenius maps. On the source r-subgroup pi(Q)=Q; on E(Fr) pi(R)=R.
The conjugate cube-root orientation must fail those fixed-point tests.

This is the residue-characteristic-r case excluded by the earlier generic
prime-to-characteristic target assumption. The proposed global subgroup is
ker(pi_bar), which should be étale at the chosen prime over r. Proving
the CM/isogeny interpretation and choosing the SAME global point at the two
primes remain separate from merely passing two local equations.

## Fixed development fixture selection

Use m in {1,12,24,48,96,112}. For m=1 use d=1; otherwise scan odd
1<=d<=1000001, in increasing order. Put u=d*2^m+1. Choose the first
candidate for which u,r,p pass the declared certificate qualification.
Keep an empty/timeout bin as a failure; do not silently replace it.

GMP probable-prime screening selects candidates only. For u, verify the
Pocklington criterion using the fully factored u−1=d*2^m. For r use
F=3*d*2^m dividing r−1, require F²>r, and check modular powers and
gcd conditions for every prime divisor of F. Small factors of d are
trial-division certified. Witness search is bounded to2..256.
For each twist allow up to16 distinct generator labels before rejecting the
twist, so a single point killed by the source cofactor is not mistaken for
an impossible curve order. Public test points use separate fixed labels
including the selected twist index. For each source public index, try up
to16 attempt labels and accept the first nonzero cofactored point.

For p use the selected source curve and a nonzero affine point P of order r,
with gcd(discriminant,p)=1. Independently recompute the full Jacobian chain,
requiring primitive projective coordinates at each step and unit Z for each
nonidentity intermediate, with a primitive infinity result at rP. Check the
integer bound r>(ceil(p^(1/4))+1)^2. This is a standard elliptic primality
certificate, not a new primality algorithm or a fully formal ECPP checker.

Traverse the six sextic twists using powers of the first g in2..10000
which is a quadratic nonresidue and noncube. Choose the first source twist
with nonzero [4]T of order r, and the first target twist with a nonzero
point of order r. Points come from SHA256 of fixed labels containing m,
field side and twist index; increment x until its curve ordinate exists,
choose the smaller square root. Use a native modular square-root routine.
Source cardinality4r follows from the certified prime, r-torsion and the
unique multiple in Hasse's interval; if not unique, count the small field
directly (in particular p=79). Check target cardinality r the same way.

## Common-field and public-point controls

For t=0..5 consider B=B0+t*p*r, without changing either finite fibre.
For each, scan auxiliary primes ell<=257 with ell≡1 mod3, good reduction
and ell!=r. Count E(Fell) exactly. Determine its CM Frobenius a+v*omega
from trace/norm and zero differential at the chosen cube-root embedding.
Let z0=(1−u)/b mod r. Require a+v*z0 to have exact order r−1, using
the complete factorization r−1=3*u*d*2^m. Choose the first (t,ell) witness.
An order-(r−1) Frobenius would certify a transitive orbit on nonzero
ker(pi_bar), hence the common cyclic point field over K. If no witness
exists in these bounds, mark that certificate missing; keep the fixture.
The CM-to-Galois implication is a sourced written argument, not a Lean
implementation or a global point-recognition algorithm.

Test four independently hash-derived source points after cofactor4 and
four target points, with different labels. Their unknown scalars are not
supplied. For r=19 also validate all18 nonidentity scalar multiples,
with scalars confined to the auditor. Check point membership, r-torsion,
pi fixedness and the z0 eigenvalue of omega. OpenSSL separately checks
source/target scalar arithmetic.

At precision p² and r², Hensel-lift the coordinate and cube-root embeddings
on the SAME integer CM model. Apply alpha once on the source lift and pi
once on the target lift. Prediction: these contraction steps give r-torsion
to that precision. They still do not identify which target point corresponds
to an arbitrary source Q. No table of matched source and target points.

## Actual target solver, with the canonical-lift control

Implement the classical Smart map on E(Fr). For a chosen lift over Z/r²,
lift y at the same x, compute [r]R in Jacobian coordinates, then

    lambda(R)=((-X*Z/Y mod r²)/r) mod r.

Infinity has lambda zero. A nonzero formal-kernel output has nonunit Z
and unit Y; the primality certificate's unit-Z check does not apply to
this final output over the residue ring.

Use lambda(S)/lambda(R) to recover a target scalar and check [k]R=S
with OpenSSL. The solver takes target coordinates, never a target scalar.
On the integer j=0 CM lift, predict lambda is zero (the canonical-lift
exception). For the actual solver, try a4=tau*r with tau=1..4 and the
unchanged a6=B; choose the first nonzero generator lambda. Preserve any
failure of that prediction and do not replace the solver silently.

Check the target map's addition law on the small fixture and public input
relations, and reject k+1 as a scalar. Target public points are NOT claimed
images of the independently generated source public points. An empty
source-to-target output count must remain explicit.

## Execution, evidence and interpretation

This is bounded LOCAL DEVELOPMENT correctness and certificate qualification,
under the local unit-test exception in isolab-eks. No timings, cold-cost
comparison, speedup, benchmark/leaderboard row, EKS admission or cloud spend.
Each fixture call has a60-second limit. At most one original call per bin
and two focused diagnostic calls before another protocol decision.

Use native C++/GMP/OpenSSL. Reuse the frozen round-two group implementation
without editing it. Keep build, pre-call and post-call manifests distinct;
copy read-only source and mode0555 executable qualifiers and hash before
calls; refuse pre-call replacement. Retain errors, partial outputs and
timeouts. Protocol is producer-frozen, not independently preregistered.

Promote only the actual setup/target-solver observations. Dense field degree
does not itself bound compressed evaluation. A cheap target and common model
do not satisfy the original arbitrary-point transfer goal. Next gate: compute
the compatible target residue of the input-selected global point in that
fixed common field, without an orbit label, exhaustive subgroup table or
source DLP. Formalize feasible concrete endpoints and identify missing bridges.
