# Riemann–Roch jet certificate for the inherited F8 order-117 fixture

Date: 2026-11-23. Tracking: [PR #5027](https://github.com/isogeny-labs/crypto/pull/5027), after [merged #4977](https://github.com/isogeny-labs/crypto/pull/4977) and [merged #4979](https://github.com/isogeny-labs/crypto/pull/4979). Status: **same-coordinator, different-algorithm finite-field mathematical check; no signed independent verification**. [pcc:unverified]

## Exact new gate

The earlier [explicit Mumford fixture](../iteration10/source_reconstruction_followup/HASSE_Q8_EXPLICIT_GENERATOR_REGRESSION.md) reports `D=(u=x+z,v=z²+z)` as an order-117 element of the genus-two Jacobian and checks it with Cantor arithmetic. This note constructs a **different kind of certificate**: Riemann–Roch jet-matrix nonmembership for `[9]D` and `[39]D`. It does **not** call the previous group-addition oracle, trust its scalar-multiple table, or infer a native Rust pass.

Let `K=F₂[z]/(z³+z+1)` and encode `a+bz+cz²` by `a+2b+4c`. Take

```text
C/K: y²+y=x⁵+x+1
P=(z,z²+z)=(2,6),   infinity the unique point at infinity
D=[P-infinity] in Jac(C)(K).
```

The on-curve check is exact: `(z²+z)²+(z²+z)=z²=z⁵+z+1`. The curve is smooth because `∂(y²+y-x⁵-x-1)/∂y=1`, and its genus is two because the right side has degree five.

### 1. Order bound without divisor addition

For `q=8`, the finite Artin–Schreier trace condition gives two `y` solutions precisely when `Tr_{F_q/F₂}(x⁵+x+1)=0`. In `F₈=F₂[z]/(z³+z+1)`, the six qualifying element encodings are `2,3,4,5,6,7`, giving `N₁=#C(F₈)=13`. In the independent field presentation `F₆₄=F₂[w]/(w⁶+w+1)`, 40 of 64 elements qualify, giving `N₂=#C(F₆₄)=81`. The binomial polynomial defines the same curve over `F₂`; the different field presentations do not have to share literal encodings.

With `L_C(T)=1+a₁T+a₂T²+q a₁T³+q²T⁴`, `a₁=N₁-q-1=4`, and the quadratic Newton identity gives `a₂=16`. Hence

```text
#Jac(C)(F8) = L_C(1)
            = 1+4+16+8*4+8²
            = 117 = 3²*13.
```

Equivalently `#Jac=(N₁²+N₂)/2-q`. No cyclicity assumption is required: by Lagrange, `117D=0`.

### 2. Exact jet criterion for kD=0

At the rational point `P`, `t=x-z` is a uniformizer, because the curve equation has derivative one in `y`. In `K[[t]]` uniquely write

```text
x(t)=z+t
y(t)=z²+z + sum_(i>=1) a_i*t^i
c(t)=f(z+t)-f(z)=(z⁴+1)*t+z*t⁴+t⁵
a_i = c_i + (if i is even then a_(i/2)² else 0)
```

where `c_i` are the coefficients of `c(t)` and arithmetic is in `K`. The last recurrence follows by equating coefficients in `y(t)²+y(t)=f(z+t)`. No root approximation or floating-point arithmetic is used.

At infinity, `v∞(x)=-2` and `v∞(y)=-5`. For each integer `k>=5`, a standard, complete basis of `L(k∞)` is

```text
{x^j : 2j <= k}  union  {x^j*y : 2j+5 <= k},
dimension k-1.
```

Form the `k × (k-1)` matrix `M_k` whose columns are the `t^0,...,t^(k-1)` jets of the displayed basis elements, in the displayed order. Then

```text
[k]D = 0  iff  kernel(M_k) has a nonzero vector.
```

Indeed a nonzero kernel vector is a function with a zero of order at least `k` at `P` and a pole of order at most `k` at infinity, so its principal divisor must be exactly `kP-k∞` by the degree-zero law. The converse follows from the function certifying linear equivalence. Constants do not cause spurious kernels.

### 3. Small exact certificates and conclusion

Using the above basis and coefficient recursion, finite-field Gaussian elimination gives:

| k | matrix dimensions | rank | determinant of the leading (k-1) × (k-1) minor |
| ---: | ---: | ---: | ---: |
| 9 | 9 × 8 | 8 | **1** in F8 |
| 39 | 39 × 38 | 38 | **5** in F8 |
| 117 | 117 × 116 | 115 | 0 (full jet rank 115) |

Both leading minors at `k=9` and `k=39` are nonzero. Consequently `9D!=0` and `39D!=0`, **without executing Cantor addition**. Since `ord(D)|117=9·13`, the first inequality forces a factor 13 and the second forces the full factor 9; therefore

```text
                         ord(D) = 117.
```

The rank defect at `k=117` is an additional consistency check, not an independently signed proof. This gives a second algebraic path to the same explicit generator's order, conditional only on checkable finite-field determinant arithmetic. The source-alone claim is unverified in the project's PCC ledger. [pcc:unverified]

### 4. Reproducibility and test boundary

A disjoint, standard-library-only Rust verifier is [`rr_jet_order117.rs`](rr_jet_order117.rs), **Git blob `0fd14e5a5bbf109fc07c20b3b9811bf489ceea5c`**. It implements independent GF8/GF64 bit polynomial arithmetic, trace counts, the local Artin–Schreier jet recurrence and exact Gaussian elimination; the tests require `(N₁,N₂)=(13,81)` and ranks `8,38,115`. To execute after source freeze: `rustc --edition=2021 --test rr_jet_order117.rs -o rr_jet_test && ./rr_jet_test`. This command **has not been run** because this invocation has no rustc/cargo toolchain. The mathematical results were rechecked with a separate same-coordinator Python GF(2^m) enumeration and finite-field matrix implementation, not with the repository's cryptographic solver; it is **not** independent-worker or signed verification.

This is a small algebraic correctness test. It does not alter or qualify the production `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs`, the reviewed both-parent repair candidate, the inherited `divisor_order_bsgs` logic, any Rust workflow, the immutable claims, or the sibling research. In particular, it does not prove an asymptotic relation-finding speedup for ordinary prime-field ECDLP.

### 5. Full cost and follow-up

The jet oracle at parameter `k` uses `k-1` basis elements, `k` output coefficients each, and dense elimination bounded by `O(k³)` field operations and `O(k²)` storage. These are **verification** costs only, with no implication for general ECDLP relation-query complexity. At `k=117`, the entire fixed toy oracle fits in a tiny matrix; it is neither the original Rust `divisor_order_bsgs` run nor an admitted timing experiment.

The next dependent gate remains separately reviewed root-module source reconstruction preserving both merge parents, then an original-root regression using the existing public Mumford `D`. The independent checker should confirm the exact field/curve encoding and test old `None` versus corrected `Some(117)`, with source-bound logs. Concurrent work on Prime31's exact empty-inclusive `S₃` pair query must remain separate; no new solver primitive was selected here.

## Literature / ownership / delivery accounting

Checked the repository's merged #4977 source, its explicit-generator note, current `AGENTS.md`, `docs/ACTIVE.md`, live #4979 state, and the named [Prime31 gap](../../../problems/prime31-exact-pair-spectrum-query.md). The 2026 Dinur–Golovnev [3SUM-indexing study](https://drops.dagstuhl.de/storage/00lipics/lipics-vol374-icalp2026/html/LIPIcs.ICALP.2026.78/LIPIcs.ICALP.2026.78.html) and classic Riemann–Roch/Artin–Schreier facts were considered; no 3SUM indexing algorithm or DLP-free additive encoding is inferred here. The shared KB tools and native child-agent dispatch are not loaded in this runtime, so the primary repository notes and external primary literature serve as fallback, not a comprehensive novelty audit.

Roles: one coordinator completed the algebra and an alternate arithmetic validation; zero native child agents were available. The Rust implementer for a *new relation solver* was not started because no such algorithm was admitted. The independent validation assignment is reduced to source-frozen preparation, **not** a signed independent-verifier verdict. All work was local arithmetic and GitHub source/evidence publication, with no EKS, cloud benchmark, paid compute, user message to others, merge or Space mutation. A normal exact dated session-log append was not possible through the connector because the current huge `docs/session-logs/2026-11-23.md` returns empty content; preserve that existing stream, and use the separate [handoff](HANDOFF_20261123.md) instead of overwriting it.
