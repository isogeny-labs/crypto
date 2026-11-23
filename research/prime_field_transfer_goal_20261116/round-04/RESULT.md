# Source-selected transfer through a concrete CM Weber cover

The coordinate-input recognizer now produces exact compatible global Weber
roots for every nonidentity order19 input on the F79 fixture, and for three
of four declared public order61 inputs on the F1531 fixture. Their images
land on anomalous curves over the smaller fields F19 and F61, respectively.
The classical target solver and at most six source comparisons recover
the scalars. Claim **PCC-20261116-fb2ab0**, unsigned. [pcc:unverified]

The implemented mechanism uses classical CM, rational reconstruction and
LLL. The original [goal](../GOAL.md) remains unchanged: a novel efficient
method for arbitrary prime-field curves is still an open requirement.
This round supplies an input-selected common-field class on the stated
fixtures; it does not supply an efficient general map. Field degree and
the dense lattice dimension grow with the subgroup order. No time,
performance, EKS or independent execution experiment was performed.

## Requirements and evidence

| Requirement | Status and exact evidence |
| --- | --- |
| Evaluate from public coordinates, without supplied scalar | Implemented: `recognize(model,input,N)` in v4/recognizer.cpp; input scalar labels occur only in the order19 auditor |
| A compatible global class, not two local fits | Accepted candidates satisfy exact C(beta)=0 in K[W]/C and the fixed source-place condition |
| Smaller and structurally weaker target | Concrete F79→F19 and F1531→F61 anomalous targets; classical Smart decoding on accepted images |
| Input coverage | All18 order19 points at tested8/16/32 digits;3/4 order61 public points at32 digits; incomplete elsewhere |
| Formal proof | Nine named literal root/quotient/residue endpoints; CM, oriented specialization and native algorithms remain outside the formal proof |
| Efficient arbitrary-curve method and novelty | OPEN; no height/convergence theorem or compressed field evaluation implementation |
| Independent execution and complete comparative costs | OPEN; unsigned native development evidence only |

All numeric outcomes in this table refer to the recorded qualification,
not signed verdicts. [pcc:unverified]

## Concrete cover and field

Let K=Q(omega), omega²+omega+1=0, u∈{3,5,7}, b=2u−1 and
r=3u(u−1)+1. On the common characteristic-zero model y²=x³+B, take
the degree-r endomorphism cover [(u−1)+b*omega]. Its kernel H has order r
under the classical CM interpretation: the Eisenstein norm of its
coefficient is r. Quotienting its nonzero points by the six CM units gives
the Weber coordinate w=x³/B and degree d=(r−1)/6.

The native constructor uses the exact rational functions
x([n]T)=x*f_n(w), y([n]T)=y*g_n(w), and the monic common factor

    C(W)=gcd(num(f_(u−1)−omega*f_b), num(g_(u−1)+g_b)).

It retains the x/y numerators, exact divisibility quotients, degree and
denominator coprimality checks. Division-polynomial version03 computes the
same models and recognition output as version02 at the two completed
fixtures. This is separate producer arithmetic evidence, not a Lean
formalization of division polynomials or CM kernels. [pcc:unverified]

| u | source p | r / target characteristic | source cofactor | shared B | d |
| --- | --- | --- | --- | --- | --- |
|3|79|19|4|659|3|
|5|1531|61|25|2|10|
|7|not constructed|127 planned|not qualified|not obtained|21 planned|

For each completed model, exact small-field counts admit the chosen sextic
twists. The source prime is the first prime p=c²r+c+1 with2≤c≤32. The
target has exactly r points over Fr and is anomalous. Fixed embeddings send
omega to (1+c(1−u))/(cb) at p and −u/b at r, and W to the chosen generator's
Weber value at each prime. Auxiliary irreducible reductions occur at
ell=7,omega=2 and ell=13,omega=3. [pcc:unverified]

At u3 the actual kernel is

    C(W)=W³−(36+24omega)W²−48(1+omega)W+(-128+192omega)/19.

Its reduction at7 is W³+3W+5 and is irreducible. A monic polynomial with
these coefficients integral at that selected prime cannot factor over K
without factoring its reduction: use the valuation-ring Gauss argument.
This gives the written field bridge. Only the finite-field cubic
irreducibility itself is formalized here. The source and target reductions
factor with roots {58,67,46} over F79 and {2,14,17} over F19.
[pcc:unverified]

## Input-selected evaluation

For input Q the recognizer validates subgroup membership, lifts the fixed
generator's W and the input's w(Q) to roots of C at p^N, and constructs an
integer congruence lattice. Versions01–03 search a constant-denominator
power-basis representation, dimension2d+1. Version04 searches

    beta=A(W)/B(W),
    deg A≤floor((d−1)/2), deg B≤d−1−floor((d−1)/2),
    A(w(P))−w(Q)*B(w(P))=0 mod p^N,

in dimension2d+2. Exact LLL uses delta3/4 and a10000-step cap. The program
checks Gram–Schmidt/Lovász conditions and a unimodular basis-change
certificate, then inspects only reduced basis rows. It performs no scan
of orbit labels, subgroup points or combinations of rows. A source-zero
denominator is rejected. After auxiliary homogenized screening it computes
beta=A*B⁻¹ in the exact global quotient and requires C(beta)=0 plus the
fixed source evaluation. LLL shortness alone never accepts an image.

There are d+1 numerator/denominator coefficients over a d-dimensional
K-space, so a rational representation exists by linear algebra; that fact
does not guarantee small coefficients or LLL recovery. The checked native
LLL algorithm and its coverage are not formally proved. [pcc:unverified]

The target is the residue of this same beta at the fixed r-place. A
denominator divisible by r is handled with additional local digits and
certified valuation cancellation. For example the constant coefficient of
C has denominator19 but is integral at the selected omega=7 place after
lifting omega. Its conjugate place has a pole. There is no field map
Qp→Fr or Fp→Fr in this construction.

## Why the scalar is recoverable

The written structural argument uses the actual CM kernel H and its
oriented reductions. At these selected good places H is étale and reduces
injectively to the respective order-r subgroups. Choose a global T whose
Weber coordinate is W. Its two reductions agree with the reference P,R
up to CM units. For Q=[k]P, the unique global root whose simple source
residue is w(Q) is the Weber class of [k]T after that source-unit choice.
Its target residue is therefore w([k]R); unit choices disappear in w.

Decode a target point S with w(S)=w([k]R). Since the nonzero H-points have
free six-unit orbits, S=[epsilon*k]R for epsilon in {±z0^j:0≤j<3}, where
z0=(1−u)/b modulo r. Smart's classical anomalous-curve solver obtains
k0=epsilon*k. Checking at most six candidates against the source P,Q
selects k modulo r. The core does not receive k. The Weber coordinate
itself is not an additive group map. The proof depends on the written CM
and oriented-specialization bridge; the Lean polynomial endpoints do not
establish that bridge. [pcc:unverified]

As a concrete public-input execution, the u5 input Q=(746,1002) yields
target Weber value25 and recovered scalar35 at N32; the distinct public
input (21,818) yields42 and57. The scalar is output by target decoding,
then checked on the source with OpenSSL. These observations are native
producer results in qualify-04-u5-public-0/2.jsonl. [pcc:unverified]

## Coverage, controls and retained failures

| Model / recognizer | N1 | N2 | N4 | N8 | N16 | N32 |
| --- | --- | --- | --- | --- | --- | --- |
| order19 constant denominator |6/18|6/18|6/18|6/18|18/18|18/18|
| order19 rational Padé |6/18|6/18|6/18|18/18|18/18|18/18|
| order61 constant denominator |not scheduled|not scheduled|1/4|1/4|1/4|1/4|
| order61 rational Padé |not scheduled|not scheduled|1/4|1/4|1/4|3/4|

These are fixed finite coverage counts, not estimated success probabilities
or minimum-precision results. There are nine setup calls: five complete
models including regression repetitions, four hit60-second limits. All16
qualification calls exit0, including zero-acceptance public calls; a
completed process is not a successful map. No u7 public bin ran because no
u7 model was obtained. All versions and failed precisions remain available.
[pcc:unverified]

Controls reject a two-prime CRT chimera that passes both local tests but
is not a global C-root; a genuine different global root that fails the
reference source gate; the conjugate orientation (a local pole); beta+1;
and k+1 on the source. OpenSSL multiplication checks accepted source/target
relations. These checks do not constitute independent execution proof.
[pcc:unverified]

Native issuer03 produces four degree6 IDC1 component records for the two
nontrivial order19 roots and rejects all four altered identities. Earlier
issuer/compiler/preparation/Lean failures and their exact source snapshots
are retained. Two74783-byte first-build stderr logs remain local with
tracked hashes/excerpts; their full bytes are not published in Git because
of the data policy. No direct upload, pre-compression or PNG re-addition.
[pcc:unverified]

## Formal scope and costs

The [frozen six-field proof map](PROOF-MAP.md) describes the existing
Mathlib quadratic-polynomial, ideal quotient and irreducibility structures.
Final declarations use WeberRootBoundaryV3 and WeberResiduesV2; binding IDs
are recorded in receipts/round-04/lean-claims-01.json. The failed earlier
generated sources are archived as text, not reported as proofs. Literal
global-root multiples imply vanishing in the principal-ideal quotient by
existing Mathlib laws. CM theory, the field/specialization interface,
native Newton/LLL/Smart algorithms and complete scalar reduction remain
outside the formal proof.

| Charged stage | Actual dependency, not measured time |
| --- | --- |
| Setup | exact CM kernel rational functions/GCD, small fixture admission, auxiliary irreducibility, field model |
| Each public input | subgroup validation, p-adic lifts to N digits, congruence lattice construction |
| Recognition | exact rational LLL in2d+1 or2d+2 dimensions, including unsuccessful rows/precisions |
| Certification | auxiliary field arithmetic, dense global inversion/composition modulo degree-d C, source check |
| Target | denominator cancellation, finite-field roots, classical Smart solve |
| Recovery / checks | up to six source multiplications and target/source OpenSSL validation |

No complete timing, memory or strongest-tool comparison is recorded. This
implementation explicitly materializes a degree-d field. That is a cost
of this implementation, not a lower bound against a compressed algorithm.
Next research obligation: a compact representation/evaluator with a
height/precision bound, full costs and a method extending beyond these
generated CM curves. The uncovered order61 input and missing order127
model are concrete qualification limits.

## Reproduction and literature

From the repository root, compile
`round-04/v4/recognizer.cpp` with C++20, `-O2 -Wall -Wextra -Werror`, GMP
and OpenSSL. Run `qualify receipts/round-04/models/u3-v3.model -1`, then
`qualify receipts/round-04/models/u5-v3.model I` for I=0,1,2,3, each under
the60-second cap. Commands are relative to this study after compilation.
Use new output names; original recorders refuse replacement. The setup
constructor source versions and immutable pre/post manifests are retained
for exact reproduction and failure inspection.

Primary sources and repository controls are linked in PROTOCOL.md and the
three amendments: original LLL preprint, Bourdon–Clark CM Weber/ray-class
statements, Lauter–Stange normalized/unnormalized EDS distinction,
FLINT3.3.1 polynomial-GCD source, CT84/115/116/119/124. KB unavailable;
no claim that the search establishes novelty. Conductor and Cargo are also
unavailable. Mathlib4.24 is pinned to
f897ebcf72cd16f89ab4577d0c826cd14afaafc7. No study-local generic law
replaces a missing crypto-lean CM interface.
