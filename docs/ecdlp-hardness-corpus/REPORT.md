---
title: "ECDLP Hardness: Theorem, Bound, Heuristic, and Attack Graph"
subtitle: "A claim-level corpus and research-obligation map"
date: "1 November 2026"
geometry: margin=22mm
fontsize: 10pt
toc: true
toc-depth: 2
colorlinks: true
linkcolor: blue
urlcolor: blue
header-includes:
  - |
    \usepackage{longtable}
    \usepackage{booktabs}
    \usepackage{microtype}
    \usepackage{needspace}
---

# Executive result

The assembled corpus contains **82 claims, 119 typed logical edges, 49 source
records, and 12 prioritized research targets**. It separates five propositions that are
often collapsed into “elliptic curves are secure”:

1. the selected subgroup is large and avoids known exceptional attacks;
2. generic algorithms need birthday-order work in the declared oracle model;
3. preprocessing, memory, parallelism, and multiple targets are charged consistently;
4. no known coordinate-aware classical algorithm beats the best generic method on the
   named family;
5. the adversary is classical and the implementation exposes only valid subgroup
   operations.

Only parts 1--3 have strong theorem support. Part 4 remains an empirical and
literature-backed assumption for standard prime-field curves; part 5 is a system model.
There is no known unconditional superpolynomial classical lower bound for ECDLP on a
named standard curve in the unrestricted computation model.

The graph does identify a disciplined way to “go after” the assumption. A classical
sub-rho attack must exploit at least one capability excluded from the generic lower
bounds: coordinates/algebra, special curve structure, target-independent setup under a
different accounting model, or a stronger oracle such as a cheap endomorphism. The
most credible active branch is coordinate algebra, but it does not become an attack
until factor-base generation, relation yield, polynomial solving, rank, sparse linear
algebra, individual descent, memory, and verification all fit one sub-rho cost bound.

![Seven-layer overview of the corpus. The full 82-node graph is in `graph.svg`.](overview.pdf){ width=100% }

# Scope and terminology

The corpus covers classical and quantum ECDLP, CDH/DDH reduction boundaries,
generic-group lower bounds, preprocessing, conventional algorithms, exceptional-curve
attacks, pairing transfers, Weil descent, summation-polynomial index calculus,
prime-field algebraic proposals, isogeny transport, quantum resources, and the first
protocol/implementation boundary layer.

The phrase in the request may refer to either of two different techniques, so both are
included:

- **index calculus:** factor base, relation collection, sparse linear algebra, and
  descent;
- **xedni calculus:** Silverman's “index backwards” lift to a rational curve followed
  by a Mordell--Weil dependence search.

Xedni is not a synonym for Semaev index calculus. The two branches have different
mechanisms and different obstructions.

This report is a research map, not a security proof for a product and not a claim of a
new ECDLP break. Exact records are in `corpus.json`, `corpus-extension.json`,
`claims.tsv`, and `sources.tsv`.

# Load-bearing theorems and bounds

| Claim | Exact form retained in the graph | Hypotheses / model | What it does not prove |
|---|---|---|---|
| Hasse | `#E(F_q)=q+1-t`, `|t|<=2 sqrt(q)` | elliptic curve over a finite field | prime order or hardness |
| Pohlig--Hellman | reduce DLP modulo the group order to prime-power components | known group order/factorization | hardness of the largest prime component |
| BSGS | `O(sqrt(r))` group operations and storage | generic cyclic group | a memory-light optimum |
| Pollard rho | expected `O(sqrt(r))`; random-mapping repeat constant about `sqrt(pi r/2)` | heuristic random walk; useful collision and restart conditions | an unconditional wall-time constant |
| Shoup 1997 | generic DLP success `O(m^2/p)` after `m` oracle queries; constant success needs `Omega(sqrt(p))` | random encoding; `p` largest prime divisor; generic oracle | coordinate algorithms, pairings, polynomial systems, cheap extra maps |
| CGK 2018 advice | `S T^2 = soft-Omega(epsilon N)` | prime-order generic group; `S` advice bits; `T` online queries; success `epsilon` | uncharged or coordinate-augmented advice outside the interface |
| CGK 2018 setup | `P T + T^2 = Omega(epsilon N)` | paper's generic preprocessing experiment | a many-user conversion without matching experiments |
| Smart trace-one | polynomial/linear-basic-operation attack when `#E(F_p)=p` | anomalous prime-field curve; p-adic lift | ordinary non-anomalous curves |
| MOV / Frey--Rueck | transfer order-`r` ECDLP to a multiplicative group over an embedding field | nondegenerate pairing; suitable extension | that small `k` alone determines cost |
| GHS | conditional `O(q^(2+epsilon))` on specified characteristic-two families with fixed composite extension degree | transfer kernel does not kill the large subgroup; paper's family | arbitrary binary or prime-field curves |
| Gaudry fixed extension | quoted `soft-O(q^(2-2/n))` for fixed `n>=3` under PDP hypotheses | extension field; decomposition/solver/rank assumptions; hidden exponential dependence on `n` | a prime-field result obtained by putting `n=1` |
| Diem 2011 | expected `exp(O(max(log q,n^2)))` in the paper's input model | specified extension-field sequence and randomized construction | a uniform subexponential attack on named prime curves |
| Shor | polynomial-time DLP in finite abelian groups | scalable fault-tolerant quantum computation | practical physical resources on one architecture |
| JKSST xedni analysis | one-shot `<= C0/p`; uniform conditional hazard gives `E[T]=Omega(p)` | selected-lift height/Lang; reduction, usefulness, bounded cofactor, adaptive recurrence | an unconditional lower bound on every lifting algorithm |

The logical asymmetry is essential. Generic lower bounds are rigorous but narrow;
unrestricted classical hardness is broad but assumed.

# Assumption lattice

## DLP, CDH, and DDH are not interchangeable

An ECDLP oracle solves CDH, so `CDH <= DLP`. A CDH oracle decides DDH, so
`DDH <= CDH`. Therefore:

- CDH hardness implies DLP hardness;
- DDH hardness implies CDH and DLP hardness;
- DLP hardness alone does **not** imply CDH hardness;
- CDH hardness alone does **not** imply DDH hardness.

Pairings make the distinction concrete: a source group can have easy DDH while CDH is
still assumed hard (a gap-DH setting). Protocols must therefore be attached to the
assumption their proof actually uses, not labeled merely “ECDLP-based.”

## Parameter screens are necessary, not sufficient

NIST SP 800-186 makes the conventional exclusions explicit: a large prime-order base
point, small cofactor, non-anomalous order, and large embedding degree. Twist orders
and public-point validation matter at the system boundary. Passing this checklist
removes named attacks; it does not prove unrestricted ECDLP hardness.

The security parameter is the prime subgroup order `r`, not simply the field size.
Under the generic rho estimate, a random full-width subgroup of approximately `2^256`
offers roughly a `2^128` operation exponent before constants and multi-target effects.
This is an upper-cost estimate for known attacks, not a lower-bound theorem in the
unrestricted model.

# Generic algorithms and preprocessing

## Upper and lower bounds nearly meet inside the model

BSGS and rho supply `O(sqrt(r))` generic upper bounds; Shoup supplies the matching
`Omega(sqrt(r))` query scale for constant-success generic attacks on a prime-order
subgroup. The result explains why a purely black-box rearrangement can improve
constants, memory, or parallelism but not the square-root exponent.

van Oorschot--Wiener distinguished-point search gives near-linear wall-clock
parallelism while total collision work remains birthday order. In its notation, for
state space `N`, useful-collision probability `u`, `m` processors, distinguished density
`theta`, and iteration time `t`, the detection-time model is

`T = (sqrt(pi N/(2u))/m + 1/theta) t`.

That expression carries assumptions: a shared random-looking mapping, controlled
endpoint storage/communication, and a useful collision. It is not a universal hardware
constant.

## The first concrete audit result

`GENERIC_INTERFACE_AUDIT.md` traces every current decomposition oracle into source.
The audit uses four interfaces:

- **G0:** standard opaque generic group;
- **G1:** G0 plus fully charged target-independent advice/preprocessing;
- **GA:** G0 plus a cheap known-scalar automorphism;
- **C:** coordinate and algebraic access.

The initial conclusions are:

- subtract, MITM, k-sum, rootfind-MITM, and pair enumeration are generic table
  algorithms once their base is supplied;
- the roots of specialized `S3(x_R,x_F,X)` are exactly `x(R-F)` and `x(R+F)`, so
  arity-two root finding does not create a new algebraic search space;
- coordinate batching can change the implementation constant, and smallest-`x`
  factor-base generation remains representation-specific;
- Koblitz/GLV Frobenius folding needs GA for a same-cost claim: a generic simulator
  can compute `[lambda]P`, but not for the price of one native Frobenius map;
- F4/F5/SAT, PKM tower, Weil descent, and lattice small-root solvers remain in C and
  outside Shoup/CGK.

This closes a bookkeeping ambiguity without closing the actual algebraic attack surface.

# Index calculus as an obligation graph

## End-to-end cost identity

For factor-base size `B`, arity `m`, subgroup order `r`, and one decomposition attempt
cost `C_PDP`, a useful first-order ledger is

`T_total = P_setup + R_trials C_PDP + T_filter + T_LA + T_descent + T_verify`.

The mean ordered decomposition multiplicity over targets is exactly `B^m/r` by double
counting. Approximating target success by

`p_dec ~= 1-exp(-B^m/r)`

adds a Poisson/independence heuristic. In the sparse regime, collecting about `B`
relations costs roughly

`R_trials ~= B / p_dec ~= r / B^(m-1)`.

If `B=r^beta`, the relation-trial exponent is `1-(m-1)beta`. Conventional sparse
linear algebra contributes about `B^2`, exponent `2 beta`, before block/rank constants.
Ignoring decomposition-solver cost and balancing these terms gives

`beta=1/(m+1)`, total exponent `2/(m+1)`.

This tempting algebra is exactly where unjustified claims arise. For `m>3` it appears
sub-rho only because `C_PDP` has been treated as free. A generic enumeration/table
oracle pays back that apparent gain through setup, memory, or per-target work; Shoup
forbids an end-to-end generic contradiction. A coordinate solver can escape Shoup, but
then its solving degree, matrix dimensions, rational roots, and memory must be bounded.

## Independent obligations

The graph isolates the following nodes instead of hiding them inside one “index
calculus heuristic”:

1. factor-base cardinality and construction cost;
2. algebraic-closure solution count and rational-point yield;
3. target-to-target variance and exceptional specializations;
4. zero-dimensionality of the descended PDP system;
5. actual Groebner solving degree, not merely first fall;
6. Macaulay matrix dimensions, sparsity, and peak memory;
7. relation independence and full factor-base rank;
8. sparse linear algebra and filtering;
9. individual logarithm/descent;
10. setup reuse, advice bits, and online success;
11. final verification in the original group.

The live prime-field route is PKM/tower or another genuinely algebraic factor-base
predicate. Small measured degree plateaus are useful falsifiers and engineering data,
but do not establish bounded degree at cryptographic scale.

## Extension and binary fields

Gaudry/Diem-style gains are real on specified extension-field regimes. They are not
evidence for a prime-field attack. Over binary fields, summation-polynomial Weil
descent produces Boolean/multivariate systems whose cost depends on actual
first/last-fall and solving degree. GHS is a different descent—to a curve/Jacobian—and
depends strongly on the resulting genus and transfer kernel.

For the repository's degree-131 Koblitz target, existing structural work severely limits
Frobenius-stable linear subspaces and makes the classified GHS genus enormous. These
are scoped negatives; nonlinear or non-invariant algebraic bases and unresolved m=4
solver behavior remain separate questions.

# Xedni calculus

Silverman's xedni proposal takes several random combinations of `P` and `Q`, lifts
them to rational points on an elliptic curve over `Q`, and hopes for a Mordell--Weil
linear dependence. Reducing such a dependence modulo `p` gives the desired scalar
relation.

Jacobson, Koblitz, Silverman, Stein, and Teske analyze this mechanism directly. Their
displayed `C0/p` success bound and linear waiting-time obstruction depend on a chain:
selected-point height versus a global minimal discriminant, Lang's height gap,
Mordell--Weil packing, good reduction and subgroup sampling, selection of a relation
with nonzero denominator, a bounded-cofactor conversion from subgroup order `q` to
field prime `p`, and a history-uniform hazard statement for repeated adaptive trials.
Under that complete chain the runtime is `Omega(p)`, asymptotically worse than rho.
Their experiments also find suitable lifts prohibitively unlikely at practical sizes.

A 2026 kernel-checked Lean audit proves the finite core without erasing those bridges:
exact coefficient-box counts, a packing theorem with fiber multiplicity, cyclic-log
extraction, fixed-direction useful-event probability `(q-1)/q^2`, the Hasse/cofactor
conversion, and the conditional hazard-to-`Omega(p)` theorem. It also gives explicit
counterexamples to several overbroad readings. Passing the proof gate establishes
these endpoint theorems, not their correspondence to the actual lift algorithm.

The repository studied a different, frozen function-field lifting family. For that exact
family it derives

`P_lift(p) = (1-2/p+p^-3)/(2p^3) = Theta(p^-3)`.

The equality is exact within its family, not a universal theorem about every lifting
construction. Together the results strongly disfavor the historical and frozen routes,
but the graph keeps the end-to-end JKSST bridge as an explicit open obligation rather
than calling every conceivable lifting strategy unconditionally closed.

# Special-family and transfer attacks

The graph treats known attacks as conditions, not folklore exceptions:

- **Pohlig--Hellman:** small prime factors are easy; price the largest prime subgroup.
- **Anomalous curves:** `#E(F_p)=p` admits additive/p-adic transfer attacks.
- **MOV/Frey--Rueck:** low embedding degree transfers ECDLP to finite-field DLP.
- **Supersingular curves:** have very small embedding degree in the classical ECC
  setting.
- **GHS:** can beat generic ECDLP on specific composite binary extensions, subject to
  its kernel and genus conditions.
- **Cheon:** auxiliary powers can lower cost under divisibility conditions; this is an
  augmented-input problem, not plain ECDLP.
- **Invalid-curve/small-subgroup attacks:** leak implementation scalars by leaving the
  intended group; they do not solve valid-input ECDLP.
- **Isogeny transport:** random reducibility within an endomorphism-ring scope does
  not provide a free coordinate-aware factor-base transport across arbitrary levels.

# Quantum boundary

Shor removes asymptotic ECDLP hardness on scalable fault-tolerant quantum computers.
Concrete estimates form a moving, architecture-dependent ladder:

- Roetteler et al. (2017) give one logical construction bounded by
  `9n+2 ceil(log2 n)+10` logical qubits and
  `448 n^3 log2(n)+4090 n^3` Toffoli gates.
- Haener et al. (2020) improve arithmetic with windowing, pebbling, and inversion
  changes, emphasizing width/depth/T-count tradeoffs.
- Gouzien et al. (2023) estimate 9 hours and 126133 cat qubits for a 256-bit ECDLP
  under a specific 2D repetition-cat architecture, `10^-5` loss ratio, and 500 ns cycle.
- A September 2026 preprint reports a secp256k1 circuit of about 1450 logical qubits
  and 40 million Toffolis, compiled to 19397 physical trapped-ion qubits, 25.7 days,
  and estimated 63% success on its proposed architecture.

These are not mutually contradictory security levels. They optimize different circuits
and assume different error correction, routing, factories, clocks, and success budgets.
The last item is retained as a primary-read preprint, not treated as independently
replicated.

\needspace{18\baselineskip}

# Research priorities

| Priority | Target | Acceptance criterion in brief |
|---:|---|---|
| 1 | Concrete generic-interface audit | every live method classified by setup, online operations, advice, and exact escaping predicate |
| 2 | Algebraic solution-count falsifier | exact/certified counts for structured and random controls under one convention |
| 2 | End-to-end prime-field IC ledger | one symbolic maximum over setup, relations, solver, rank, LA, descent, memory, and verification |
| 3 | PKM solving-degree growth | uncensored larger rungs plus checked Hilbert-series prediction |
| 4 | Binary m=4 degree ladder | matched larger cells and shared cost-unit conversion |
| 5 | Coordinate transport theorem | explicit interface and proof or counterexample after the withdrawn overclaim |
| 5 | Xedni bridge-composition audit | instantiate selected lifts, height, reduction, usefulness, cofactor, and adaptive hazards for one explicit family |
| 6 | Automorphism preprocessing model | GA oracle theorem or cost-preserving reduction with orbit/stabilizer accounting |
| 7 | Named-curve quantum ladder | verified circuits and architecture/error assumptions |
| 8 | Multi-target/nonuniform model | target distribution and all advice/setup units matched to CGK |
| 9 | Pairing target-field census | exact `q,r,k`, target representation, and finite-field DLP cost for named curves |
| 10 | Protocol assumption graph | proof notion, exact assumption, model, validation rule, and reduction loss per protocol |

The best immediate theoretical target is the automorphism/preprocessing bridge; the
best attack-oriented target is the end-to-end prime-field algebraic ledger plus the
solution-count falsifier. The former can close misuse of existing theorems. The latter
can reveal whether a proposed coordinate base changes recognizability/solver cost or
only rearranges generic yield.

# Evidence and limitations

Every externally proved claim must cite at least one `primary_read` source under the
validator. Every primary-read record carries a SHA-256. Inaccessible primary papers are
marked metadata/secondary and are not used as sole evidence for an external theorem.
Internal formal/analytic claims remain `proved_internal_unreviewed`.

The literature search is deep and branch-complete for load-bearing claims, but it is not
an assertion that all ECDLP publications have been read. The most important remaining
primary verification targets are original MOV/Frey--Rueck, Gaudry/Joux--Vitse, and
protocol-specific proof papers. `LITERATURE_SEARCH.md` records those limits and the
discovery ledger.

The full graph is necessarily dense. `overview.svg` is the readable layer map;
`graph.svg` is the claim-level audit artifact; `claims.tsv` and the JSON corpus are the
preferred exact interfaces.

# Conclusion

ECDLP security is not one theorem. It is a stack:

`valid subgroup and curve screens`

`+ generic birthday lower bounds in explicit models`

`+ no verified special-family or coordinate-aware faster attack`

`+ protocol reduction and implementation validation`

`+ a classical-adversary qualifier`.

The rigorous generic results are strong enough to tell us where not to spend time:
relabeling group subtraction as root finding, building larger generic sum tables, or
claiming a free preprocessing/automorphism quotient cannot by itself establish a
sub-rho exponent. The remaining credible research surface is narrower and sharper:
coordinate predicates whose algebraic solution cost scales favorably, exact transport
interfaces, or a new model-changing mechanism. The corpus turns each of those into a
falsifiable obligation rather than an undifferentiated hope.

# Selected sources

- Victor Shoup, [Lower Bounds for Discrete Logarithms and Related Problems](https://doi.org/10.1007/3-540-69053-0_18), EUROCRYPT 1997.
- Henry Corrigan-Gibbs and Dmitry Kogan, [The Discrete-Logarithm Problem with Preprocessing](https://doi.org/10.1007/978-3-319-78375-8_14), EUROCRYPT 2018.
- Paul van Oorschot and Michael Wiener, [Parallel Collision Search with Cryptanalytic Applications](https://doi.org/10.1007/PL00003816), Journal of Cryptology 1999.
- Nigel Smart, [The Discrete Logarithm Problem on Elliptic Curves of Trace One](https://doi.org/10.1007/s001459900052), Journal of Cryptology 1999.
- Pierrick Gaudry, Florian Hess, and Nigel Smart, [Constructive and Destructive Facets of Weil Descent](https://doi.org/10.1007/s00145-001-0011-x), Journal of Cryptology 2002.
- Claus Diem, [On the Discrete Logarithm Problem in Elliptic Curves](https://doi.org/10.1112/S0010437X10005075), Compositio Mathematica 2011.
- Jean-Charles Faugere et al., [Using Symmetries in the Index Calculus for ECDLP](https://doi.org/10.1007/s00145-013-9158-5), Journal of Cryptology 2014.
- Michael Jacobson et al., [Analysis of the Xedni Calculus Attack](https://doi.org/10.1023/A:1008312401197), Designs, Codes and Cryptography 2000.
- Peter Shor, [Polynomial-Time Algorithms for Prime Factorization and Discrete Logarithms](https://doi.org/10.1137/S0097539795293172), SIAM Journal on Computing 1997.
- NIST, [SP 800-186: Elliptic Curve Domain Parameters](https://doi.org/10.6028/NIST.SP.800-186), 2023.
- Martin Roetteler et al., [Quantum Resource Estimates for Computing ECDLP](https://arxiv.org/abs/1706.06752), 2017.
- Thomas Haener et al., [Improved Quantum Circuits for ECDLP](https://doi.org/10.1007/978-3-030-44223-1_23), 2020.
- Elie Gouzien et al., [Performance Analysis of a Repetition Cat Code Architecture](https://doi.org/10.1103/PhysRevLett.131.040602), 2023.

The complete source ledger, including hashes and verification status, is
`sources.tsv`.
