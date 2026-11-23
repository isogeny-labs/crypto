# Prime31 Dickson composition oracle

Research checkpoint dated November 22, 2026, America/Los_Angeles. PR [#4928](https://github.com/isogeny-labs/crypto/pull/4928).

## Outcome and exact scope

The candidate evaluates the resultant of a Dickson fibre and a target-dependent S3 quadratic using a denominator-free trace/norm recurrence. It keeps the inner factor-base membership constraint as a composition of quadratic maps. A rational-lift lemma allows that full fibre to be used even though only some of its abscissae lift to the elliptic curve. The new Rust component passed its five development tests, and 41 unchanged library tests passed through the qualified dependency adapter. The diagnostic rejects a gain against the strongest elementary algebraic control in the tested cell: at B=31 the recurrence uses 17,923 field multiplications versus 8,816 for sparse product. These are component derivations and development observations, with no signed PCC standing or claimed asymptotic speed advantage. [pcc:unverified]

The strongest immediate controls matter: expanded Horner evaluation is unnecessarily expensive, but a product of sparse quadratic factors and a direct group-complement scan are substantially better comparators. The recurrence's logarithmic layer count does not remove the cost of its polynomial coefficients. [pcc:unverified]

## Benchmark curve and inherited implementation

The unchanged source curve is

```text
p = 2147483647
E: y^2 = x^3 + 847514846 x + 2087444724
n = 2147446831
G = (625037715, 2100627251)
```

The native source records this as an ordinary curve with prime full group order and cofactor one. Those are inherited facts, not a new point-counting certificate from this iteration. The implementation imports `Arithmetic`, `Base`, and `PRIME31` from the existing Rust crate; it does not substitute a deliberately weak curve. [pcc:unverified]

Primary source: `research/prime31_ic_20261115/native/lib.rs` at `26cc445bdc6b0b3518535d5f7b8b4ec6cc08fb19`. The earlier exact quotient oracle is `examples/prime31_s3_quotient_gcd.rs` and its accompanying `S3_QUOTIENT_GCD_ORACLE_20261115.md`.

## Why these equations

Work over a field of characteristic different from 2 and 3. Write `f(x)=x^3+a*x+b`, and let the public target be `R=(r,s)` with `s != 0`. Define

```text
q_X(Y) = S3(X,Y,r) = d(X) Y^2 - t(X) Y + u(X)
d(X) = (X-r)^2
t(X) = 2[(X+r)(X*r+a)+2b]
u(X) = (X*r-a)^2 - 4b(X+r).
```

The discriminant is `t^2-4du = 16 f(X) s^2`. Thus a rational S3 solution at a rational non-2-torsion target forces the other coordinates to lift rationally. The exceptional specialization `X=r` is linear, not a reason to divide by zero or discard a valid relation. [pcc:unverified]

The useful consequence is that the composed membership polynomial can be left intact. Let `F` be the product over a split rational coordinate fibre, and `H` the product over its liftable subset. For the stated target domain, a common zero of `F(X), F(Y), S3(X,Y,r)` already has both coordinates in the liftable subset. One can use the smaller outer modulus `H(X)` while retaining `F(Y)` as the inner membership predicate. [pcc:unverified]

## A full-fibre construction for this prime

Use the Dickson recurrence

```text
D_1(Z) = Z
D_(2n)(Z) = D_n(Z)^2 - 2
N = 2^k
F_k(Z) = D_N(Z) - c.
```

The norm-one torus in the quadratic extension has order `p+1=2^31`. If `eta` has exact order `2^31`, `e` is odd, and `1 <= k <= 29`, the choice `c=eta^(eN)+eta^(-eN)` gives a split squarefree degree-N fibre over the prime field. Its roots are the traces of one N-element coset, whose inverse coset is disjoint. This is torus structure, not a proper subfield of the prime field. The existing `Base::build` implements a bounded depth range and supplies the exact terminal and full coordinate fibre. [pcc:unverified]

The abstract construction must reject the repeated-root boundary `c=+2` or `c=-2`; in particular the stated primitive-torus recipe reaches `c=-2` at `k=30`. The number of liftable fibre elements is an observed cardinality, not always exactly half of N. [pcc:unverified]

## Denominator-free resultant recurrence

Initialize three polynomials in the outer coordinate:

```text
Q_0 = d,       T_0 = t,       U_0 = u.
```

At every level compute

```text
Q_next = Q^2
T_next = T^2 - 2 U Q - 4 Q^2
U_next = U^2 - 2 T^2 + 4 U Q + 4 Q^2.
```

After k levels set

```text
G_k = U_k - c T_k + c^2 Q_k
J   = gcd(H, G_k mod H).
```

The exact polynomial identity is `G_k(X)=Res_Y(F_k(Y),S3(X,Y,r))`. All recurrence operations can be reduced modulo the outer membership polynomial without changing the final residue. The recurrence remains defined when `d=0` and in quotient rings with zero divisors. [pcc:unverified]

### Derivation

When `d != 0`, let `z1,z2` be the roots of the quadratic q in an algebraic closure, with multiplicities, and put `v_i=D_(2^j)(z_i)`. If `tau=v1+v2` and `nu=v1*v2`, applying `v -> v^2-2` gives

```text
tau_next = tau^2 - 2 nu - 4
nu_next  = nu^2 - 2 tau^2 + 4 nu + 4.
```

Now take `Q_j=d^(2^j)`, `T_j=Q_j*tau`, and `U_j=Q_j*nu`. Clearing the powers of d gives exactly the displayed recurrence, and

```text
U_k - c T_k + c^2 Q_k
  = d^N [D_N(z1)-c] [D_N(z2)-c].
```

This is the resultant away from `d=0`; the identity extends polynomially to the diagonal. Taking the monic F as the first argument fixes the specialization and sign convention even when q drops degree. [pcc:unverified]

For `X=r` and `s != 0`, `q_r(Y)=-4s^2(Y-x(2R))`. Since N is even for the supported positive depths,

```text
G_k(r) = (4s^2)^N F_k(x(2R)).
```

The tangent relation is therefore retained exactly. At depth zero the unsimplified monic-first product convention must be used; the implementation has a separate sign control. [pcc:unverified]

## Recover signs without square roots

For a rational zero `(x,z)` of the S3 equation, use

```text
lambda = ((x-r)(z-r) - (3r^2+a)) / (2s)
P = (x, lambda*(x-r)-s)
Q = (z, lambda*(z-r)-s).
```

The centered S3 identity yields `lambda^2=x+z+r`, and

```text
f(X) - (lambda*(X-r)-s)^2 = (X-r)(X-x)(X-z).
```

Thus the two recovered points lie on E and sum to R with the correct intersection multiplicities. The formula handles `x=r`, `x=z`, and the order-three tangent case; it needs one inverse of `2s` per target and no inverse of `x-r`. It remains subject to a final source-curve group check and factor-base sign/index normalization. [pcc:unverified]

The affine method excludes `s=0` and the identity target. A rational 2-torsion target can admit twist-coordinate S3 solutions and needs its own lifting rule. The identity target uses inverse pairs. These exclusions are explicit API conditions, not dropped benchmark failures.

## Cost analysis and falsification controls

Let N be the degree of the full fibre, B the dimension of the chosen outer quotient, and C(B) the cost of modular polynomial multiplication including reduction. The recurrence uses four modular products per level and costs `O(C(B) log N)`, plus setup, the final combination, gcd, root selection, sign recovery, and relation validation. At most a constant number of B-coefficient arrays is needed by an ordinary implementation, apart from multiplication scratch and extracted output. [pcc:unverified]

| Method | Algebraic kernel cost | Interpretation |
| --- | --- | --- |
| Expanded inner Horner, naive dense products | O(N B^2) | Existing weak symbolic control |
| Dickson norm recurrence, degree-sensitive schoolbook products | O(B^2 [1+log^+(N/B)]) | O(B^2) when N/B is bounded |
| Sequential product of known quadratic factors | O(N B) | Stronger elementary algebraic control |
| Balanced product tree, fast multiplication | approximately O(M(N) log N) when B is comparable to N | Prior quasi-linear comparator; implementation comparison pending |
| Group-complement scan | O(B) group operations plus charged membership work | Required relation-search control |

These are derived operation models, not measured speed ratios. The loose bound O(B^2 log N) must not be presented as a tight exponent: state degrees grow geometrically before reaching the outer modulus, so only O(1) layers are saturated when N/B is bounded. In that regime the schoolbook recurrence and sparse product are both quadratic. A degree-sensitive fast arithmetic implementation has a kernel bound O(M(B)[1+log^+(N/B)]); setup, gcd and extraction still need their own bounds. There is no established exponent advantage over the group-complement scan. [pcc:unverified]

The polynomial degrees before reduction are at most `2^(j+1)`. If the reduced residues become dense, explicitly materializing them costs linear space in B. A circuit with logarithmically many layers can still have expensive root intersection. The next mathematical obligation is an output-sensitive algorithm for intersecting the succinct fibre and resultant circuits that avoids both root enumeration and dense degree-B materialization. [pcc:unverified]

## Development evidence and validation boundaries

The new source is checked against three independent descriptions of its output: the sparse product of the full fibre's quadratic factors, the previous normalized expanded Horner construction on its valid domain, and exhaustive signed source-curve addition on small development fixtures. Full-fibre and lift-filtered support, target-coordinate collisions, repeated indices, inverse pairs, degree drops, and a corrupt-resultant negative control are part of the checks. Exact commands, outputs, hashes and the dependency deviation are retained under `evidence/`.

The original locked dependency set could not be resolved from the local cache. A separate cached-dependency adapter uses the original library source bytes and a separate lockfile; the upstream lock is unchanged. These are development checks, not a repository-wide original-lock pass, independent external replay, an admitted benchmark, or signed PCC verification. [pcc:unverified]

No comparative wall-time figure or complete ECDLP cost is asserted. Kernel counters exclude setup and extraction until the prospective protocol is implemented. The largest individual polynomial allocation is not peak live memory. Canonical IC leaderboards and benchmark plots remain unchanged because this iteration supplies no admitted performance row.

### Exact development observations

The final source SHA-256 is `b76cd5a1efeff77bc065554d89acc9b53be051e6b17811cca589c2dc93a018b1`. Five example tests cover 25 original-curve fixtures at depths 2 through 6. A separate 12-case correctness instrument records full resultant coefficient equality, support equality, signed relation equality and oriented recovery. The observed library count is 41 passed tests. These receipts are producer evidence, with a separate agent source audit and root certificate replay; they are not external signed validation. [pcc:unverified]

For the regular depth-six cell, N=64 and the actual liftable base has B=31:

| Instrumented kernel | Quotient product calls | Field multiplications |
| --- | ---: | ---: |
| Dickson norm recurrence | 24 | 17923 |
| Sparse full-fibre product | 64 | 8816 |
| Filtered-base Horner comparison | 81 | 126511 |
| Full-fibre Horner coefficient control | 145 | 254913 |

These are deterministic source-instrumented kernel counts, excluding setup, extraction and group checks. Both Horner controls include a denominator-clearing unit rescaling for coefficient equality; the previous normalized-GCD oracle can omit that rescaling, so this table does not establish its complete cost or a speed ratio against it. [pcc:unverified]

The recurrence uses about 2.03 times the sparse-product multiplication count in this cell. All three final recurrence polynomials have 31 nonzero coefficients, 93 retained coefficient slots altogether. That observed density rejects bounded sparsity for these fixtures, not all compact circuits. The next iteration therefore targets root intersection without explicit dense residues. [pcc:unverified]

Four `identity.certificate/v1` records accompany the centered S3, discriminant, trace-step and norm-step identities. Each is evaluated over `2^61-1` with 32 certificate points, with an additional 64-point root replay and separate checked-integer coefficient expansion. Each record has `proof:null`; none certifies the function-field theorem, Lean verification or PCC standing. [pcc:unverified]

## Second iteration and the degree obstruction

The next proof calculation studies the two functions `u(P)=D_N(x(P))` and `v(P)=D_N(x(R-P))` on the entire elliptic curve. For a nonzero target of odd order, their generated function field is the full elliptic function field. Consequently the irreducible algebraic relation between u and v has exact degree `2N` in each variable. The readable proof and its conditions are in [NEXT_PROOF.md](NEXT_PROOF.md); it has been independently reviewed within this session but is not Lean- or PCC-verified. [pcc:unverified]

The proof uses the disjoint single-pole divisors `2N[O]` and `2N[R]`. A nontrivial common subcover would either be an unramified genus-one cover with a totally ramified pole, or a genus-zero cover forcing a power-of-two multiple of R to vanish. Both contradict the hypotheses. Separability follows from the power-of-two map degree in odd characteristic; global tameness is not assumed. [pcc:unverified]

This rules out a fixed-bidegree scalar-Dickson replacement for the full relation. It does not rule out succinct circuits, sparse high-degree equations, factor-base-restricted tests or a faster root algorithm. In particular it is not a lower bound on finite-field ECDLP. The successor must exploit one of these still-open representation possibilities, with the exact extraction and full-cost requirements retained. [pcc:unverified]

## Third iteration: the missing empty-subtree operation

The exact Dickson tree has a useful conditional branch bound. If Z is the set of z projected answers in an N-element fibre, an exact subtree-emptiness oracle gives `1+2 sum_(j=1)^k |D_(2^j)(Z)|` node tests, bounded by `O(1+z[1+log(N/z)])`, with one test when z=0. Child-generation square roots and the oracle's arithmetic must also be charged. The complete argument is in [NEXT_ROOT_INTERSECTION.md](NEXT_ROOT_INTERSECTION.md), claim `PCC-20261123-87f011`, standing unverified. [pcc:unverified]

This branch bound does not supply the oracle. The scalar norm transform `G_(j+1)(T)=Even_j(T+2)^2-(T+2)Odd_j(T+2)^2` retains degree exactly 2N, or 2N-1 when the target abscissa is in the fibre. Standard recursive norm evaluation still visits N leaves on an empty query. If each certified empty block costs its dimension, the disjoint empty blocks cost at least a constant times N-z altogether. These are obstructions for the stated representations and algorithms; they do not lower-bound every symbolic method. [pcc:unverified]

The next precise target is an exact empty-root certificate using `O(N^(1-epsilon) polylog(p))` charged work on a declared growing family and target distribution, then an output-sensitive extraction bound. Target-dependent preparation and amortized tables count. No such sublinear emptiness predicate was constructed in this iteration. Once projected roots are available, ordinary quadratic recovery and the oriented witness formula are already inexpensive per returned root; general-p square-root cost is explicit in the note.

## Asymptotic objective and the next experiment

A fixed prime is a benchmark, not an asymptotic family. Any exponent claim must quantify a growing family of prime fields, the available smooth divisor of `p+1`, fibre construction, representation, and all solving costs. Infinitely many Mersenne primes must not be assumed. [pcc:unverified]

Under the usual random-sum heuristic, an m-summand factor base with B representatives has success probability on the order of `B^m/(m! n)` before saturation, up to the declared sign/orbit conventions. A conventional full-rank pipeline must price roughly `n/B^(m-1)` decomposition attempts, linear algebra, target recovery, and setup. Rank deficiencies and duplicated relations can increase this cost. The one-dependency Amadori–Pintore–Sala architecture has different accounting and is a required separate baseline, not silently ruled out by this model. [pcc:unverified]

The next admitted component experiment is frozen in `PROTOCOL.md`. It compares complete per-query costs and output sets across increasing fibre depths, records dense support at each recurrence layer, and includes unsuccessful natural targets. If this recurrence loses to sparse product or group scanning, retain that outcome and pursue the missing succinct common-root algorithm rather than increasing the same dense computation indefinitely.

The longer-term integration target remains actual unknown-index higher-arity relations, useful independent rank, and one cold public-target scalar recovery using the maintained Rust pipeline. Complete comparisons must include the strongest applicable index-calculus baseline and optimized negation-map Pollard rho on matching instances and resources.

## Prior art and overlap audit

The linked S3 chain is already Semaev 2015, equation (4). Composed low-degree factor-base maps and generalized prime-field root finding are already Petit–Kosters–Messeng 2016. Known linear combinations with one dependency are already Amadori–Pintore–Sala 2018. Low first fall degree does not establish low solving degree, as Kosters–Yeo explains. [pcc:unverified]

Repository searches also found existing discriminant factorization, oriented coordinates, quotient/GCD elimination, Dickson fibres, Brent–Kung acceleration, and quasi-linear resultant controls. The narrow contribution under investigation is the full-fibre lifting equivalence combined with denominator-free Dickson evaluation and explicit exceptional-point coverage. No global novelty claim is made. The KB tools and wrapper were unavailable; searches were bounded and cannot prove an absence of earlier work. [pcc:unverified]

Live extraction work in PRs #4692, #4812 and #4822 and the rank/RHS work in #4901 are dependencies and overlap exclusions. PR #4829 maintains point-origin summation-root machinery. This new lane does not replace those implementations or overwrite their branches.

### Concurrent hourly-task checkpoint

The existing hourly task has also advanced [PR #4924](https://github.com/isogeny-labs/crypto/pull/4924), covering symmetric radical moments, a monic-in-product S3 quotient, and a rank-four balanced factor tree. Its exact monic quadratic and quasi-linear symbolic-construction model are mandatory comparison material for this lane. The two branches are linked for continuation; the new norm recurrence is not a claim to have originated quotient/GCD extraction or fast product trees. [pcc:unverified]

At this read, GitHub run [45086381347](https://github.com/isogeny-labs/crypto/actions/runs/45086381347) and job 137405842381 report `success`, with API head SHA `aa27edacb477ddc55bccd23b86d90584ce438cd1`. Its later `NATIVE_F4_RUST_RUN74_20261123.md` at branch head `fb850179c3847fd6950dbd91015c4e3d7c6da62d` instead names checkout `b8a594ea514ff44cadb5342986c68b1eac6e22ed`. Resolve that provenance discrepancy against the actual checkout log before inheriting exact-source assertions. Its reported F4 and x-only extraction figures are not pooled with this lane's diagnostics or promoted to signed PCC standing. [pcc:unverified]

### Primary literature

1. Igor Semaev, *New algorithm for the discrete logarithm problem on elliptic curves*, 2015: https://arxiv.org/pdf/1504.01175
2. Christophe Petit, Michiel Kosters and Ange Messeng, *Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over prime fields*, 2016: https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf
3. Alessandro Amadori, Federico Pintore and Massimiliano Sala, *On the discrete logarithm problem for prime-field elliptic curves*, 2018: https://pure.tue.nl/ws/files/91158650/609.pdf
4. Michiel Kosters and Sze Ling Yeo, *Notes on summation polynomials*, 2015: https://arxiv.org/pdf/1503.08001

### Repository sources checked

- `research/prime31_ic_20261115/native/S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md`
- `research/prime31_ic_20261115/native/S3_QUOTIENT_GCD_ORACLE_20261115.md`
- `research/trace_incidence_20261116/accelerated/README.md`
- PR #4742, resultant/multipoint evaluation controls
- PR #4824, torus comparison tracking draft, closed without merging
- PR #4829, maintained prime summation-root implementation
- PR #4901, projective S3 and authentic RHS/rank controls

## Persistent state and access dependencies

The existing hourly Summation polynomial task is the continuation vehicle; the prompt must point to this PR and the exact state file. A Space creation request for `Prime Field Summation Polynomial Research` has an uncertain outcome after repeated read timeouts and no write receipt. Reconcile that exact name before retrying creation. No Space save is reported as confirmed.

The local environment exposes no Conductor binary, research KB tools, EKS MCP or AWS CLI; the GitHub command-line client has no authenticated host. Repository access through the connector remains available. No paid job was launched, and no benchmark admission, checkpoint or budget rule is waived. Local development tests remain permitted by the EKS skill. Lean, lake and elan are absent; formal verification remains a recorded dependency.
