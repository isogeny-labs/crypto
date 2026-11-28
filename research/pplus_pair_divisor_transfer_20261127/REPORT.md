# Rank-15 Dickson pair-divisor transfer

## Outcome

The degree-two Dickson pair-divisor recurrence did not yield a subquadratic
one-target contraction of the rank-15 S5 polar in either frozen
representation. Exact fixed-target specialization materially reduces the
initial polar, but the next norm expands again: 2,025 sparse coefficients become
85,833 after one pair fold, and the second fold reaches the prospective
262,144-slot cap. The explicit quadratic-tower DAG grows
`209 -> 3,843 -> 44,838` nodes, then reaches its cap before depth three.
[pcc:unverified]

The frozen verdict is `INDETERMINATE_CAP`, not an algorithmic pass and not a
universal lower bound. The bounded evidence excludes a hidden win in the
canonical sparse map and in the strongest obvious four-conjugate sharing DAG;
another arithmetic-circuit representation could in principle behave
differently. [pcc:unverified]

No `A+A` set, pair list, pair-domain scan, quartic divisor algebra, generic
Groebner solve or cross-target amortization was used. This lane also does not
assume an available S3 zero-query oracle: S3 enters only through the published
resultant identity used to construct the direct S5 polar.

## The tested transfer

For an unordered pair with elementary invariants `(s,p)`, reduce the Dickson
polynomial modulo `q(T)=T^2-sT+p`:

```text
D_(2^j)(T) mod q = u_j*T+v_j,
u_0=1, v_0=0,
u_(j+1)=u_j^2*s+2*u_j*v_j,
v_(j+1)=v_j^2-u_j^2*p-2.
```

The off-diagonal pair scheme is `u_t=v_t=0`. Under one Dickson doubling level,

```text
S=s^2-2p-4,
P=p^2-2s^2+4p+4,
p^2=P+2S+4.
```

The four sign lifts of a lower off-diagonal pair have invariants

```text
(s,p), (-s,p), (d,-p), (-d,-p),  d^2=s^2-4p.
```

For a polynomial `f`, the exact four-lift transfer is therefore

```text
E_f(q,p)=f(z,p)f(-z,p), z^2=q,
N(f)(S,P)=E_f(S+2p+4,p) E_f(S-2p+4,-p), p^2=P+2S+4.
```

Odd remainders are rejected. The protocol separately specifies anti-podal and
repeated-leaf charts; the bounded run stops before a complete chart product is
formed.

The target object is the actual symmetric S5 polar

```text
H_r(A,B)=nu_4(c(A))^T C_r nu_4(c(B)),
c2=s^2-4p,
c1=-2(s(p+a)+2b),
c0=(p-a)^2-4bs.
```

The candidate constructs `C_r` through the padded-degree Semaev/Sylvester
resultant, not by interpolating sampled values.

## The novel tower arm

Four separately cloned calls could hide avoidable duplication, so the second
representation evaluates the entire input circuit in

```text
R[w,z]/(w^2-(P+2S+4), z^2-(S+2w+4))
```

on basis `(1,w,z,wz)`. Both quadratic multiplications use Karatsuba, but every
base-ring addition and multiplication is emitted as an ordinary DAG node. The
norm is then

```text
g=a^2-(S+2w+4)b^2,  f=a+bz,
N(f)=g0^2-(P+2S+4)g1^2,  g=g0+g1*w.
```

This is a real sharing attempt: the four conjugates travel as components of one
quadratic tower. It is not an opaque extension-element node; all components,
edges and operations are charged. The observed growth shows that the actual
polar does not collapse under the frozen simplifications. [pcc:unverified]

## Exact bounded results

The formal-target series is diagnostic. The two headline series specialize
the target before transfer and independently rebuild every source object, so
neither target receives work from the other.

| series | H sparse slots | first-norm slots | next sparse status | H DAG nodes | norm-1 DAG | norm-2 DAG | next DAG status |
|---|---:|---:|---|---:|---:|---:|---|
| formal `r` | 16,752 | none | cap at norm 1 | 221 | 4,261 | 61,309 | cap at norm 3 |
| `G` | 2,025 | 85,833 | cap at norm 2 | 209 | 3,843 | 44,838 | cap at norm 3 |
| `[17]G` | 2,025 | 85,833 | cap at norm 2 | 209 | 3,843 | 44,838 | cap at norm 3 |

For each fixed target, the first sparse norm recorded 160,473,912 field
additions and 160,660,740 field multiplications. The DAG state factor from
norm one to norm two is 11.667447307; the corresponding arithmetic-node factor
is 11.583026800. Both are observations from two bounded depths, not fitted
asymptotic exponents. [pcc:unverified]

The two fixed targets have different coefficient-map digests even though their
support counts and DAG topology coincide. That equality of counts is not
cross-target reuse.

## Independent checking and custody

The v3 verifier is separately expressed from the candidate. It:

1. reconstructs H by enumerating all 720 permutations of the 6-by-6 Sylvester
   determinant, while the candidate uses subset dynamic programming;
2. independently specializes both public targets and reconstructs all five
   emitted sparse polynomial cells coefficient-for-coefficient;
3. parses and recounts all nine emitted DAGs;
4. performs 45 complete polynomial-versus-DAG evaluations; and
5. performs 54 direct predecessor-circuit tower norms and compares them with
   successor DAG values.

It reported `exactness_ok=1` and zero errors. `verify_evidence_v3.sh` rebuilt
the verifier/analyzer and reproduced their retained outputs byte-for-byte.
These are deterministic public replays, not a zero-knowledge proof, a trusted
execution receipt or a signed PCC verdict. [pcc:unverified]

The experiment uses `seed_manifest=null`: there is no random stream. Source
manifests 1--3 preserve each source transition; evidence directories preserve
the original formal packet, stronger verification-only packet and per-target
packet. Stored gzip files use deterministic headers and package manifests hash
the exact retained bytes.

## What the result establishes—and does not

Within the frozen canonical representations:

- target specialization does not uncover a below-four fold;
- carrying all conjugates in one quadratic tower does not keep the actual polar
  compact; and
- increasing the cap would only extend the same already-super-four bounded
  sequence, not supply the missing recurrence proof. [pcc:unverified]

The result does **not** establish an arithmetic-circuit lower bound for every
representation. It also stops before the exceptional charts, rational lift/sign
filter, first-pair contraction and exact witness/miss certificate. Therefore
`coverage_complete=0`, `recurrence_rho_lt_4=0`, and no m=7 integration or
speedup claim is authorized.

## PKM and Amadori assessment

Petit--Kosters--Messeng leave the root-finding/polynomial-system cost as
`T(E,m,L)`. Amadori--Pintore--Sala's m=7 comparison assumes comparable solver
costs, while their experiments cover m in `{3,4,5}`. Neither paper supplies a
subquadratic pair-spectrum query or an m=7 extractor that closes the missing
step here. [pcc:unverified]

Fast special-resultant and structured-linear-algebra results can be
quasi-linear in a retained output or quotient dimension. Reordering the S5
products into a degree-`Theta(n^2)` pair aggregate therefore remains quadratic
in the leaf count and violates this lane's gate; it is not rescued by the rank
15 feature map. [pcc:unverified]

## Best next lead: torus-binomial support gate

The next representation change worth testing is the lower-priority backup from
the direct-S5 report, now sharpened into a proof-first gate.

Write each Dickson leaf as

```text
x=z+z^-1,  D_n(x)=z^n+z^-n,
```

so the leaf condition is the binomial `z^(2n)+1=0`. Substitute four such
Laurent variables directly into the target S5 polar and clear only the
monomial denominator. Before implementing any eliminator, compute the exact
Laurent support and mixed volumes of the binomial-plus-polar system under the
two-pair split.

Promotion requires an explicit per-target elimination recurrence with support,
state and arithmetic `O(rho^t poly(t))`, `rho<4`. The arm stops immediately if
any intermediate Newton polytope, sparse resultant or quotient has
`Omega(n^2)` lattice width/volume or coefficients. It may not materialize the
root-of-unity pair grid, use a multidimensional FFT over all pairs, or share a
transform between targets.

This gate is more discriminating than another factor-base swap: it asks whether
the torus substitution turns the actual S5 polar into a sparse binomial
elimination below its generic mixed volume. If the mixed-volume screen is
already quadratic, the torus backup closes without Rust execution. If it is
subquadratic, the next implementation is a sparse resultant/subproduct circuit
with the same public replay and chart obligations.

## Sources

The prospective protocol records the full literature and internal-source
review. The primary sources are Semaev 2004
(<https://eprint.iacr.org/2004/031.pdf>), Petit--Kosters--Messeng 2016
(<https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>),
Amadori--Pintore--Sala 2018
(<https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf>),
Bostan--Flajolet--Salvy--Schost on special resultants
(<https://cs.uwaterloo.ca/~eschost/publications/BoFlSaSc05.pdf>) and
Bostan--Jeannerod--Schost on structured linear systems
(<https://cs.uwaterloo.ca/~eschost/publications/tcs.pdf>).

The shared research KB was unavailable in this runtime; the review used those
primary papers and the committed internal controls named in `PROTOCOL.md`. No
novelty claim is made from the unavailable search service.
