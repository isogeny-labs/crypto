# Protocol: rank-15 Dickson pair-divisor transfer

Date frozen: 2026-11-27 UTC. Branch:
`codex/pplus-pair-divisor-transfer-main-20261127`. Draft PR: #5298.
Standing: prospective deterministic algebra/structure experiment,
`[pcc:unverified]`.

This protocol is frozen before candidate source, transfer-state observations or
a decision outcome exist. It authorizes one bounded Rust symbolic experiment.
It does not authorize a relation census, an m=7 integration, a timing or
speedup claim, or a computation shared across unrelated targets.

## 1. Question and hard boundary

Let `n=2^t` and `F_n(X)=D_n(X,1)`, with

```text
D_0=2, D_1=X, D_(2m)=D_m^2-2.
```

The completed direct-S5 study found an exact rank-15 pair polar but no
subquadratic zero-entry query. This study asks the narrower next question:

> Can the degree-two Dickson pair-divisor recurrence contract the actual
> target-parameterized rank-15 S5 polar with `o(n^2)` live state and arithmetic
> for one target, while returning an exact witness or miss?

Both time and live target state must be subquadratic in the leaf count. A
compact description of a degree-`Theta(n^2)` object, an opaque norm/resultant
gate, a low matrix rank, or an empirical slope is not a pass. The candidate
must expose an ordinary field-arithmetic circuit and a closed bound

```text
T_t, S_t <= C * rho^t * t^k,  for fixed C,k and rho < 4,
```

so `T_t,S_t = o(4^t)=o(n^2)`. All work used for a target is charged to that
target. Circuit topology may be public algorithm source, but no evaluated
field element, divisor, table, transform or transcript may be reused from a
different target.

The following are forbidden in the candidate and any promoted construction:

- an explicit elliptic `A+A` set, ordered or unordered `Theta(n^2)` pair list,
  or scan of that domain;
- a pair-sum, pair-spectrum or Chow polynomial with `Theta(n^2)` coefficients,
  even if a later algorithm is quasi-linear in its output degree;
- the `binom(n+3,4)` quartic-divisor algebra, an `n^4` S5 kernel, or a generic
  Groebner-basis handoff;
- an opaque `norm`, `resultant`, `subresultant`, `multipoint`, `oracle` or
  reusable parameterized-subcircuit node whose hidden state is not charged;
- a new factor base, generic k-sum table, or amortization across DLP targets.

## 2. Sources checked before design

The shared research-KB MCP is not loaded and `scripts/research-kb` is not
executable in this checkout. The bounded fallback below was used, so no
literature-wide novelty claim is made.

Primary sources:

- Semaev, *Summation polynomials and the discrete logarithm problem on
  elliptic curves* (2004), the zero criterion and resultant recursion,
  <https://eprint.iacr.org/2004/031.pdf>.
- Petit--Kosters--Messeng, *Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields* (PKC 2016), especially the
  composed selector and unresolved solve term `T(E,m,L)`,
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
- Amadori--Pintore--Sala, *On the discrete logarithm problem for prime-field
  elliptic curves* (FFA 2018). Its m=7 comparison assumes a solver cost; its
  reported experiments stop at m=5,
  <https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf>.
- Bostan--Flajolet--Salvy--Schost, *Fast Computation of Special Resultants*.
  Its composed-operation costs retain the quotient/output dimension,
  <https://cs.uwaterloo.ca/~eschost/publications/BoFlSaSc05.pdf>.
- Bostan--Jeannerod--Schost, *Solving structured linear systems with large
  displacement rank*. Its structured solve is still quasi-linear in the
  ambient dimension,
  <https://cs.uwaterloo.ca/~eschost/publications/tcs.pdf>.

Repository controls:

- parent-lane `research/pplus_dickson_s5_structure_20261127/{PROTOCOL.md,REPORT.md}`
  at `origin/codex/pplus-dickson-s5-structure-20261127`;
- parent-lane `research/pplus_dickson_norm_gcd_20261127/{PROTOCOL.md,REPORT.md}`;
- `research/prime31_summation_compression_20261122/iteration10/
  S3_PENCIL_CONTRACTION.md`;
- `research/prime31_summation_compression_20261122/iteration10/
  quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`;
- `research/prime31_summation_compression_20261122/iteration13/
  {EXPLICIT_KSUM_FRONTIER.md,RANK_STAR_CONTRACTION.md}`;
- `research/prime31_summation_compression_20261122/iteration13/
  OFFSET_PARITY_QUARTIC_NORM.md`;
- `research/symmetric_pair_20261116_v93b/PROOF_AND_REVIEW.md`; and
- `problems/prime31-exact-pair-spectrum-query.md`.

These sources determine the scope. PKM and Amadori do not provide the missing
m=7 extractor. Special-resultant and displacement algorithms do not remove a
quadratic ambient/output dimension. Low polar rank does not index a zero
component, and the actual pair support cannot be assumed to inherit a second
uniform Dickson tower. [pcc:unverified]

## 3. Exact pair scheme and its charts

Work over `Fp`, `p=2^31-1`, where two is invertible. For

```text
q_(s,pair)(T) = T^2 - s*T + pair,
D_(2^j)(T) mod q = u_j*T + v_j,
```

use exactly

```text
u_0 = 1,                         v_0 = 0,
u_(j+1) = u_j^2*s + 2*u_j*v_j,
v_(j+1) = v_j^2 - u_j^2*pair - 2.
```

For distinct leaves, `u_t=v_t=0` is the off-diagonal unordered pair scheme,
of size `binom(n,2)`. It does not include repetitions. The candidate must keep
three chart types explicit:

```text
O: x != y and x != -y       (lower pair is off diagonal),
A: y = -x                   (anti-podal; lower pair is diagonal),
D: y = x                    (repeated-leaf diagonal).
```

Under `X=x^2-2`, `Y=y^2-2`, upper invariants `(s,pair)` map to

```text
S = X+Y = s^2 - 2*pair - 4,
P = X*Y = pair^2 - 2*s^2 + 4*pair + 4,
pair^2 = P + 2*S + 4.
```

Every lower off-diagonal pair has the four sign lifts

```text
(s,pair), (-s,pair), (d,-pair), (-d,-pair),  d^2=s^2-4*pair.
```

Thus `binom(2m,2)=4*binom(m,2)+m`; the final `m` is chart A and may not be
dropped. For unordered pairs with repetition,
`binom(2m+1,2)=4*binom(m,2)+3m`; the additional two lifts over each lower leaf
are the two repeated pairs in chart D.

## 4. Canonical four-lift transfer

Let `f(s,pair)` have any commutative coefficient ring. Define its exact even
product

```text
E_f(q,pair) = f(z,pair)*f(-z,pair) with z^2=q,
```

by collecting powers of `z`, rejecting any surviving odd power. The four-lift
norm is

```text
N(f)(S,P) = E_f(S+2*pair+4,pair)
            * E_f(S-2*pair+4,-pair)
            with pair^2=P+2*S+4.
```

The product before the last substitution must be even in `pair`; a surviving
odd power is an exactness failure. This formula is the only authorized O-chart
transfer. A symbolic `Norm` node is not an implementation of it.

For a lower leaf `X` and an auxiliary `z^2=X+2`, define the exceptional charts

```text
A_f(X) = f(0,-X-2),
D_f(X) = f(2*z,X+2)*f(-2*z,X+2) with z^2=X+2.
```

Let `L_t(g)=product_(D_(2^t)(X)=0) g(X)` and let `Q_t^O(f)` be the product of
`f` over distinct unordered pairs. Then the exact chart recurrence is

```text
Q_(t+1)^O(f)   = Q_t^O(N(f)) * L_t(A_f),
Q_(t+1)^all(f) = Q_t^O(N(f)) * L_t(A_f*D_f).
```

`L_t` must itself be exposed as ordinary additions and multiplications, with
all coefficient slots or circuit nodes charged. Hiding it in a resultant is a
failure. These identities are checked independently against enumeration only
on the small verifier cells; enumeration is never a candidate query.

## 5. Actual target polar and contraction order

For curve `y^2=x^3+a*x+b`, one pair has

```text
c2 = s^2 - 4*pair,
c1 = -2*(s*(pair+a)+2*b),
c0 = (pair-a)^2 - 4*b*s.
```

Let `nu4(c0,c1,c2)` be the 15 monomials of total degree four. The exact
target-parameterized polar is

```text
H_r(A,B) = nu4(c(A))^T * C_r * nu4(c(B))
         = S_5(x1,x2,x3,x4,r),
```

where `A=(s,pair)` and `B=(s',pair')`. `C_r` is constructed from Semaev's
padded-degree resultant recursion; it is not inferred from sampled values.
The independent verifier constructs the same symmetric polynomial through a
separately written Sylvester determinant and checks every coefficient.

The authorized elimination order is

```text
G_(t,r)(A) = Q_t^all(B -> H_r(A,B)),
Z_(t,r)    = Q_t^all(A -> G_(t,r)(A)).
```

Over the full Dickson fibre, `Z_(t,r)=0` exactly when some pair-pair entry is
zero. This product identity is a correctness definition, not permission to
construct its `Theta(n^2)` first-pair polynomial or its `Theta(n^4)` expanded
product. Multiplicativity of resultants is a negative control: as soon as a
reordering creates an aggregate of degree or live dimension `Theta(n^2)`, that
arm returns `PAIR_AGGREGATE_STOP`.

The full-fibre predicate is not yet the rational ECDLP predicate. A promoted
oracle must also treat nonlifting abscissae, both rational y signs, repeated
leaves, target infinity and all degree drops. An algebraic zero with no
rational signed witness returns `INDETERMINATE`, not `EMPTY`.

## 6. Two charged representations

Both representations start from the actual `H_r`, with `r` retained as a
formal input. Fixed target specializations are controls, not a source of
cross-target preprocessing.

### A. Canonical sparse coefficients

Use a sorted sparse map from complete exponent tuples to nonzero `Fp`
coefficients. Expand every addition, multiplication and substitution; combine
equal monomials and delete exact zeros. No coefficient may itself contain an
unexpanded polynomial, norm or function call.

One state unit is one nonzero coefficient slot, including its exponent tuple.
Peak live state is the sum of slots in every simultaneously live input,
temporary and output; an input is released only after its last consumer.
Arithmetic counts every field add/multiply used to produce the canonical map.

### B. Hash-consed ordinary arithmetic DAG

Allowed nodes are only `input`, field `constant`, binary `add`, binary `mul`
and unary `neg`. Squaring is a multiplication. Fixed-input nodes may fan out.
Every instantiation at different substituted inputs has fixed incoming wires
and must be materialized; a reusable function/template call is forbidden.

The builder sorts commutative operands, folds field constants and applies only
the prospectively listed identities `x+0=x`, `x*0=0`, `x*1=x`,
`-(-x)=x` and identical-node hash-consing. Any later algebraic rewrite is a new
arm and must be frozen before use. One state unit is one live arithmetic node;
edges, peak nodes and operation counts are all emitted. Garbage collection may
discard a node only after its last consumer.

The sparse arm detects coefficient cancellation; the DAG arm tests whether a
non-expanded circuit actually avoids four fresh instantiations per level.
Neither representation may count an output handle while omitting the state it
names.

## 7. Fixed public inputs, caps and deterministic cells

Use registered curve `icv1-fp31-tm3989-ae9db972`:

```text
p = 2147483647
E: y^2 = x^3 + 2147483644*x + 21
group order = 2147487637
G     = (1391443431, 177979044)
[17]G = (1960828035, 1167167028)
```

There is no random stream; the seed manifest is exactly `null`. Dickson roots
use the parent's deterministic backwards-square-root order: the smaller square
root is branch zero and its negative is branch one. Positive controls at
`t=2,3,4` use the parent's frozen rule: the sum of the lexicographically first
signed four-leaf multiset with repetition whose sum is finite and nonzero.

Cells:

| component | depths | targets |
|---|---|---|
| one-step formula and chart identity | `t=0..4` | formal `r`, `G`, `[17]G`, positive control |
| sparse second-pair transfer | `t=1..6` until cap | formal `r`, then both fixed targets |
| DAG second-pair and attempted first-pair transfer | `t=1..6` until cap | formal `r`, then both fixed targets |
| independent direct enumeration | `t=1..3` only | `G`, `[17]G`, positive control |
| rational signed replay | `t=2..4` only | same fixed and positive controls |

Before allocating a next state, the candidate computes a checked conservative
allocation bound. A representation stops before allocation if it would exceed
`262144` coefficient slots, `262144` DAG nodes, `1048576` DAG edges, or 192 MiB
resident memory. The emitted status is `INDETERMINATE_CAP`, unless a symbolic
recurrence already proves the frozen quadratic stop below. A cap is never
reported as an empty relation set or as evidence of subquadratic growth.

## 8. Prospectively fixed decision rule

### Exactness gate

All are required before any structural conclusion:

1. candidate and verifier agree coefficient-for-coefficient on `H_r`;
2. the four-lift formula has no forbidden odd remainder and agrees with direct
   four-sign evaluation on every small cell;
3. O, A and D chart products equal the independently enumerated products;
4. sparse and DAG representations evaluate identically on all common cells;
5. every algebraic zero and rational signed witness agrees with direct S5 and
   curve addition, while exceptions return `INDETERMINATE`; and
6. the analyzer recomputes every state and operation count from the emitted
   representation rather than trusting producer totals.

Any failure gives `INVALID_STOP`.

### Algorithmic pass

`ALGORITHM_PASS` requires all of:

1. an explicit exact recurrence for both peak state and arithmetic with a
   proven constant base `rho<4`, allowing only polynomial factors in `t`;
2. the recurrence covers the second-pair contraction, first-pair zero test,
   exceptional charts, rational lift/sign filtering, and witness recovery or
   a finite miss certificate;
3. source construction plus one complete target stays within that recurrence,
   with no target-shared evaluated state;
4. four growing cells agree exactly with the independent verifier; and
5. no forbidden aggregate or opaque node occurs.

A fitted slope below 1.9, low memory on small cells, rank 15, or an `O(log n)`
pair-membership circuit cannot replace the recurrence proof.

### Stops

- `CANONICAL_QUADRATIC_STOP`: an exact derivation exhibits
  `Omega(4^t/poly(t))` live coefficient slots or ordinary DAG nodes for the
  frozen transfer, including any alleged fold. This conclusion is scoped to
  the tested transfer representation, not a universal arithmetic-circuit
  lower bound.
- `PAIR_AGGREGATE_STOP`: the construction first forms a degree/dimension
  `Theta(n^2)` pair aggregate.
- `BOUNDED_NO_FOLD`: bounded exact cells show four-way growth but do not prove
  an asymptotic lower bound.
- `INDETERMINATE_CAP`: the prospective cap is reached without a proof.
- `INVALID_STOP`: an exactness or coverage check fails.

Only `ALGORITHM_PASS` permits a separately preregistered large run or m=7
integration. Every other status stops this arm before a relation census.

## 9. Rust source custody and proof model

Use a dependency-free standalone Rust crate with separate binaries:

- candidate: constructs `H_r`, sparse transfer and the ordinary DAG, emitting
  full canonical representations and prospective-allocation stops;
- verifier: separately constructs the Sylvester/Semaev polynomial, enumerates
  only the frozen small cells, parses and reevaluates candidate objects, and
  checks signed curve witnesses; and
- analyzer: recomputes counts, checks forbidden-node absence and applies only
  Section 8's decision rule.

Candidate and verifier may share only a text schema and the public constants,
not Rust modules. Source hashes are committed before the first scientific run.
The run wrapper retains stdout, stderr, exact commands, compiler version,
source manifest and hashes of every artifact.

All inputs are public and `seed_manifest=null`. A witness is a public tuple of
leaf indices and signs; a miss certificate is the exposed transfer circuit and
its nonzero terminal value. Independent deterministic replay is therefore the
appropriate proof model. It is **not** a zero-knowledge proof, trusted-execution
receipt or signed PCC verdict. Any result remains `[pcc:unverified]` until an
independent signed PCC verdict covers it; hashes alone prove only byte custody.

No timing or fastest-implementation claim is in scope. If `ALGORITHM_PASS` is
ever reached, a later frozen protocol must add same-target cold comparisons and
the fastest external implementation before a speedup claim.
