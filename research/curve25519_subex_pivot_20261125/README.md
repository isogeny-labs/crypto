# Curve25519 subexponential pivot

Date: 2026-11-25.  This note closes the fixed-arity pair-image route for the
stated **total expected** `r^o(1)` goal and records a focused audit of the
growing-arity tensor recursion.  It does not claim a new ECDLP algorithm or an
unconditional proof of the external contraction theorem.

## Decision

The operational pair-image result remains useful as a finite algebraic
diagnostic, but an `o(d^2)` target-conditioned common-root routine is no longer
the next exponent-moving target by itself.  Fixed arity fails before its
per-target algebra is charged.

Let `B` be the number of signed factor-base elements, let `m` be a fixed
relation arity and let the prime subgroup have order `r`.  At most

```text
1 + B + ... + B^m
```

group elements have a decomposition of arity at most `m`.  A uniform target
therefore succeeds with probability at most `O(B^m/r)`.  Producing `Theta(B)`
rows needs at least

```text
Omega(r / B^(m-1))
```

target attempts, independently of the decomposition oracle's cost.  Total
subexponential work also requires `B = r^o(1)` (the matrix has `Theta(B)`
columns), so every fixed `m` gives `r^(1-o(1))` attempts.  Symmetry can change
constant factors and polynomial powers of `B`; it cannot clear this counting
gate.  The surviving routes must use growing arity or change the target
distribution/factor-base correspondence in a way that invalidates the uniform
coverage model.

For the measured `m=3` split, the ordinary pair table gives the more concrete
model

```text
setup + relation search = Theta(B^2) + Theta(r/B)
```

before linear algebra: `Theta(r/B^2)` targets and a `Theta(B)` scan per target.
It is minimized at `B` of order `r^(1/3)`, for `Theta(r^(2/3))` work.  The exact
squarefree degree-`B^2` pair image measured by
`EXP-C25519-PAIRIMAGE-GATE-001` explains the setup term.  Fast special
resultants can make construction quasi-linear in their degree, but emitting a
degree-`B^2` polynomial still emits `Theta(B^2)` field coefficients.  Fast
modular composition applied to the natural tensor quotient likewise sees
dimension `Theta(B^2)`, not `Theta(B)`.

These are upper-model and counting conclusions, not an unconditional lower
bound for every coordinate-aware relation generator.  In particular, a
nonuniform target generator with proved useful-row density would need its own
analysis. [pcc:unverified]

## Focused D.4.6--7 child-call audit

The growing-arity route already has the conditional prime-field rate

```text
2^O(b/m + m*b/sqrt(log b)),  m = Theta((log b)^(1/4)),
```

hence `2^O(b/(log b)^(1/4))`, provided the general Boolean tensor contraction
theorem D.1 is correct and applies with uniform construction/entry costs.  This
route covers fixed-cofactor prime-field curves, so it is the relevant
asymptotic family for Curve25519's prime subgroup.  Its constants do not imply
a practical attack at 255 bits.

The source checked here is OpenAI/math commit
`42907c9e999cc00d6abe7a50bfc1023250edd1b9`, file
`build/quantitative-contraction.tex`, SHA-256
`671642d7b53154d92344e8c4be0240f18929632ebcfdf73b07c3e9939b873860`.
The local check covered the transition in D.4.6--7 from one mode tuple to its
group children:

1. A crossing mode has one target group and one source group.  Coefficient
   extraction therefore separates over target-group row products, while its
   substituted linear form uses only source-group column bits.
2. Multiplying the group contractions reconstructs each retained mode
   monomial exactly.  Terms of weight at least the working precision vanish
   modulo `2^p`, so enumerating only lower-weight tuples is valid.
3. A new packet entry is evaluated from at most `L` affine row factors plus
   source-form powers.  With polynomially many terms per factor, the direct
   coefficient traversal costs `exp(O(L log s))` time and polynomial space.
   The source guard `L <= s^(1/1000)` places this inside the theorem's
   `exp(O(s^(9/10)))` entry allowance.
4. A new description contains packet-local data, the chosen mode tuple,
   interpolation data and creation precision.  It contains no callback to a
   parent evaluator.  Retained tensors use immutable creation-time
   descriptions and canonical lifts, so entry routines do not compose with
   recursion depth.

No contradiction was found in this local transition.  The native
`hashed-ecdl-tensor` controls were replayed with Rust only:

```text
cargo test --locked --release \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --lib contraction::recursive::coupled_controls -- --nocapture

5 passed; 0 failed
```

Those controls cover parallel/correlated ports, quadratic incoming modes,
outgoing form powers, two taper levels and a nested child whose older packet
keeps its creation precision.  The fourth control forces three nested active
contractions, records a nonempty child visit at depth three, and matches the
independent direct count.  It uses the production deformation, child,
interpolation and recovery routines with the smaller diagnostic taper
`u=1,a=1,L=2`; the `a=2,L=4` preflight exceeded 60 seconds and is not retained
as a routine gate. The executed callback tree now also serializes and
independently verifies one schema-2 factorization certificate at every active
visit through depth two: 1, 20, and 412 certificates, respectively, totaling
390,237 bytes. The three unique depth records have working precisions
`3,67,131`, one mode, two tuples, and tuple-log2 ceilings `15,60,85`.
These are bounded exact controls under non-strict diagnostic recursion; the
one-edge child does not meet the source shrink premise, so this is not a live
strict history or a proof of the asymptotic induction. `[pcc:unverified]`

A production-path control closes the corresponding finite representability
question: `Setup::new(usize::MAX,0)` cannot return recursive parameters at any
depth. Universally, every allocated graph has
`lambda<=usize::BITS<128*1024`, so the source schedule has root `u=0`.
Consequently no larger allocated Rust fixture can become the missing strict
history. The next bridge must use an implicit/symbolic graph representation
and receive a fresh refinement and cost audit. `[pcc:unverified]`

A heterogeneous attempt with a two-taper `u=2,a=2,L=4` root and two compact
nested levels remained live past 90 seconds and was interrupted without a
result.  Thus the fast two-taper one-level and three-level one-taper controls
have not yet been composed in one retained finite test. [pcc:unverified]

The finite counting and pair-split formulas are independently executable in
`crates/hashed-ecdl-tensor/src/bin/curve25519-subex-gate.rs`.  It uses exact
integers for both the existing order-131203 miniature and Curve25519's 253-bit
prime subgroup; its unit controls include the exact Curve25519 power-of-two
optimum.  The standalone crate keeps this gate independent of unrelated root
library build failures on the sparse research branch.

The exact finite output puts the scale in one place:

| group | order bits | best power-of-two `B` | pair-split work bits, before LA | ceil(work / sqrt(r)) |
| --- | ---: | ---: | ---: | ---: |
| order-131203 miniature | 18 | `2^5` | 13 | 15 |
| Curve25519 prime subgroup | 253 | `2^84` | 170 | 8,796,093,022,208 |

The full-size probe at `B=2^16` has an exact fixed-arity target-attempt lower
bound of 220 bits.  This is a coverage bound, while the 170-bit pair-split row
is the stated uniform-occupancy cost surrogate.  Both remain far above the
253-bit group's square-root scale before charging linear algebra.

## New target

The next acceptance gate is a **source-bound active-call cost certificate**,
not another fixed-arity solver tweak.  For every actually constructed child it
must bind, in one induction:

- the checked leaf, degree, packing and shrink guards;
- inherited and creation precision, including rational intermediate widths;
- old vertices, packet vertices, isolates, mode tuples and description bits;
- construction work and the number/cost of packet-entry evaluations; and
- the depth-first branching product through all active depths.

The target theorem is a fixed-constant bound of the form

```text
total bit work <= (s + description_length)^C
                  * 2^(C*s/sqrt(log s))
```

with `C` independent of the recursion depth and input instance.  A precise
counterexample to one induction premise is equally decisive.  Passing this
gate would retire the largest remaining conditional step between the existing
growing-arity ECDLP reduction and an independently supported asymptotic
subexponential result; failing it would identify the exact theorem repair
needed.

### Active-call certificate component

The first finite component of that gate is now executable in
`tensor-active-call-audit`.  For current edge count `M`, packet count `K`,
packet length `L=2^a`, active valuation `i`, interaction distance
`D=L/2^i`, retained levels `u` and precision `p`, it checks `KL<=3M` and
certifies

```text
n = K*2^(i+1),             n*D <= 6M
log2(number of mode tuples) <= 5*ceil(sqrt(n*p)).
```

The second inequality follows from the weighted generating function: for at
most `n` variables of each positive weight, evaluation at `exp(-theta)` gives
`ln N <= theta*p + 2n/theta`; choosing `theta=sqrt(n/p)` and converting to
base two is below the stated integer constant.  Consequently, whenever the
precision induction supplies `p <= c*M*D/u` for a fixed `c`, this finite bound
is `O(sqrt(c)*M/sqrt(u))`, exactly the branching exponent required in D.4.7.

The certificate also bounds all group-child invocations over the active nodes
and interpolation samples, the new packet-description slots, and the
coefficient-choice leaves of a packet entry.  The default exact fixture
`M=64,K=8,a=u=2,i=0,p=128` reports:

```text
mode tuple log2 bound                  230
whole-call group-child log2 bound      248
packet description slots            20456
packet-entry choice log2 bound          44
ceil(p*u/(M*D))                          1
```

Seven native dimension/work controls pass, including exact enumeration of
small weighted tuples.  This closes the local exponent algebra conditional on
the packet-capacity and precision premises.  The following components bind
those premises to recursive histories and price the remaining arithmetic.

The standalone factorization certificate now carries the schema-2 tile period
`tau` and independently validates the same local counts. Nonempty mode columns
must occupy one residue modulo `tau`; duplicate `(packet,weight,residue)` keys
are rejected. Exact independent tuple reconstruction then enforces

```text
Q <= K*tau*(p-1),
log2(T) <= 5*ceil(sqrt(K*tau*p)).
```

The bounded positive controls cover 100 three-edge, 132 four-edge, and 364
five-edge serialized certificates. This joins the finite factorization
evidence to the active-call dimension ledger without trusting the producer's
reported `Q` or `T`. It does not yet join the separate packing and precision
premises to every node of an arbitrary recursive history. `[pcc:unverified]`

An object-level cross-check now makes the packing boundary explicit. Of those
596 partition certificates, 285 satisfy `K L<=3M` and match the independent
BigUint `NodeDimensions` result field for field; 311 fail that source premise
and remain algebraic controls only. The next gate therefore targets actual
recursive histories, not a larger fixed-edge certificate census.
`[pcc:unverified]`

### Uniform source-history bridge

The capacity and normalized-precision premises are now bound to the successful
source path at the scalar-invariant level.  Production packing calls
`pack_labels(..., enforce_bounds=true)`, which rejects unless
`K*L<=3M`.  The remaining precision bridge combines the finite-history
bound already proved in
`research/hashed_ecdl_precision_history_20261123` on `origin/main`.
[pcc:unverified]

```text
p_d <= 81*s*2^A,
```

with two checks enforced by the actual recursive source: an evaluated child
has `8*M_(d+1)<=M_d`, and a call is active only above the root-fixed leaf
cutoff `floor(s/h)`, where `h=floor(sqrt(lambda))` and
`lambda=ceil(log2(s))`.  Therefore every possible active depth satisfies

```text
8^d < h.
```

For the source scale `u_d=floor(lambda/(2^17*2^d))`, this implies the stronger
bound `u_d>=32` whenever `lambda>=2^26`.  If `d<=4`, the threshold gives
`u_d>=2^(26-17-4)=32`.  If `d>=5`, activity gives
`lambda>2^(6d)` and hence

```text
u_d >= 2^(5d-17) >= 2^d >= 32.
```

Thus `2^d<=u`.  Since `lambda<2^(17+d)(u+1)` and `u+1<=2u`, it follows that
`h<512u`.  The base cases at `u=32`, followed by factor-at-most-two induction,
give the first inequality below.  For the second, the ratio
`(A+2u+1)/2^A` decreases for every integer `A>=0`, so `A=0` is worst; its
`u=32` base case and the same induction give

```text
81*h*u <= 2^(u+1),
8*u*(A+2*u+1) <= 2^(A+u+1).
```

These are exactly the inherited-precision and evaluation-loss absorption
conditions for the existing finite-history endpoint.  Together with the
native taper term they give the uniform local bound

```text
p_next*u <= 34*M*2^(A+2*u-i)
```

at every possible active child history above the threshold.  This is an
unverified producer derivation composed from separately retained components,
not a new kernel-checked refinement theorem. [pcc:unverified]

The exact-integer Rust certificate is `tensor-history-audit`.  At the first
claimed threshold `lambda=2^26`, it enumerates the only five depths not
excluded by shrinkage:

| depth | `8^d` | `u_d` | inherited absorption | evaluation absorption |
| ---: | ---: | ---: | :---: | :---: |
| 0 | 1 | 512 | pass | pass |
| 1 | 8 | 256 | pass | pass |
| 2 | 64 | 128 | pass | pass |
| 3 | 512 | 64 | pass | pass |
| 4 | 4096 | 32 | pass | pass |

The negative control `lambda=2^23` reaches `u=16` and `u=8`; both fail
the inherited absorption check, so the certificate distinguishes the missing
margin rather than accepting every schedule.  The focused native suite now
passes 14/14 controls. [pcc:unverified]

`SYMBOLIC_HISTORY_CERTIFICATE.md` now separates this transcript from its
producer. A standalone module reconstructs the five-depth schedule, both
precision absorptions, geometric path comparisons, packet-entry absorption,
and final constants from a stable 7,355-byte JSON fixture. Schema 4 additionally
checks the chain from `K L<=3M`, `tau D=2L`, and `p u<=34 M D` to
`n p<=204 M^2/u` and tuple exponent constant `80`. Every depth now binds the
normalized family `M=uL`, `K=3u`, and `p_i=34L(L/2^i)` at both coordinate
endpoints. The independent verifier reconstructs and checks the exact
identities `KL=3M`, `tau D=2L`, `nD=6M`, `pu=34MD`, and
`npu=204M^2`; the closed-form tile/distance schedule covers the full coordinate
range. It imports neither the producer dimension audit nor
contraction/packing/factorization code. Schema, missing-depth,
precision-transition, normalized-dimension, source-constant, and final-constant
mutations are rejected. This is an independent normalized scalar replay, not
graph realization or a proof of the external theorem.
`[pcc:unverified]`

On the finite side, the standalone factorization module now provides the same
scalar interface for an accepted schema-2 object. Given `(a,u,i)`, it
reconstructs `(M,K,L,tau,D,p)`, requires uniform packet length, checks
`KL<=3M` and `nD<=6M`, reports or requires `pu<=34MD`, and recomputes the
mode/tuple ceilings. All 285 source-admitted certificates in the 596-case
bounded corpus agree
field-for-field with the separate BigUint audit; the other 311 remain explicit
capacity-failing algebraic controls. This connects serialized evidence to the
symbolic dimension vocabulary without claiming graph provenance.
`[pcc:unverified]`

`PACKING_PROVENANCE_CERTIFICATE.md` now closes the next finite part of that
boundary. A separate schema-1 object serializes the labeled bipartite edge
skeleton and packet orientation patterns. Without importing graph or packing
code, its verifier reconstructs every label class, padding/copy index, packet
slot, and active endpoint group, then requires exact agreement with the
schema-2 factorization certificate. Every producer path in the 596-case corpus
and the executed diagnostic performs a serialize/parse/round-trip/replay gate.
A dedicated admitted three-edge control also replays the actual greedy
conditional-expectation label choice, first-minimum tie breaking, variance
inequality, packet capacity, and every-coordinate group balance. This proves a
complete finite path from an edge skeleton to packet/group objects, not a
uniform arbitrary-size theorem, asymptotic graph realization, or the external
contraction theorem. `[pcc:unverified]`

The same certificate now charges the schema-2 factorization-description
envelope. After the tuple factor, it bounds every remaining JSON/canonical
binary factor by
`Delta=2L+16(lambda+a+u+1)` and checks
`Delta*h*ceil(sqrt(u))<=2^(lambda-1)` at all five depths. Active-node size then
gives `Delta<=M/sqrt(u)`, using the final spare local unit:
`80+1+4+5+6=96`. This keeps the composed constants `290` and `148480`
unchanged while making description serialization explicit. The argument is
still conditional on graph realization and the external theorem.
`[pcc:unverified]`

This threshold is asymptotic: it means a root edge count on the order of
`2^(2^26)`, far outside the live `usize` representation and irrelevant to
practical 255-bit security.  It closes the uniform capacity/precision bridge
for the theorem path, not the complete cost theorem.  Ring operations,
interpolation, Hermite rational numerators/denominators, description bit
widths, and their composition over the recursive call tree are priced in the
next component.

### Arithmetic widths and recursion-tree composition

Those remaining source-cost terms now have an executable coarse certificate.
For one active call, let `J` be the total retained jet coefficients and
`q_max` the maximum tapered multiplicity.  The exact dimension identities
give

```text
J <= 8*M*L,                 q_max <= 4*M.
```

Every stored extension coefficient is canonical below `2^p), and the direct
ring multiplication has temporary width at most `2p+1`.  In Hermite
recovery, grid numerators and denominators are smaller than `2L`.  Thus one
rational multiplier has at most

```text
B = (J+q_max)*(a+1)+1
```

bits in its numerator and denominator.  Even allowing the output accumulator
to use the product of all `J` term denominators gives the conservative bound

```text
W = J*B + p + ceil(log2(J+1)) + 1.
```

This covers the unreduced common denominator; normalization by
`num_rational` can only reduce the represented values.  The direct
interpolation, series, reciprocal, quotient, and rational-operation counts are
also explicit polynomials in `M,L,p` and the fixed extension degree.  A
native control constructs the real small grids and rational multipliers and
checks every intermediate numerator and denominator against `B` and `W`.
[pcc:unverified]

The default arithmetic fixture `M=64,K=8,a=u=2,i=0,p=128`, extension degree
32, reports:

```text
stored coefficient bits                         128
ring multiplication temporary bits              257
binomial integer bits                            257
rational multiplier bits                       3649
rational accumulator bits                   3503179
direct interpolation ring operations      2172538584
recovery ring operations                    14206128
recovery rational operations                   63312
```

The large numbers are safe envelopes, not observed runtime or calibrated
machine operations.

For the call tree, `nD<=6M` and `p*u<=34MD` give
`np<=204M^2/u`.  The weighted-tuple bound plus the active-node, sample, and
radix factors is therefore at most `96M/sqrt(u)` bits per active call.
Along any root-to-leaf path,

```text
M_d <= s/8^d,
u_0 <= 4^(d+1)*u_d,
sum_d M_d/sqrt(u_d) <= 3s/sqrt(u_0).
```

Hence the product of active branching factors has exponent at most
`288s/sqrt(u_0)`.  Direct leaf enumeration adds one unit.  Packet-entry
coefficient traversal adds another: using `a<=4u_0`, its exponent is at most

```text
2^(4u_0) * (8+2lambda+5u_0) <= s/sqrt(u_0)
```

above the same threshold.  Packet descriptions, vertices including isolates,
ring/interpolation work, and the rational widths above are fixed-degree
polynomial factors; packet descriptions contain no recursive evaluator
callback.

The resulting producer-level source-cost envelope is

```text
bit work <= (s + description_length)^O(1)
            * 2^(290*s/sqrt(u_0))
         <= (s + description_length)^O(1)
            * 2^(148480*s/sqrt(lambda)).
```

The second line uses `u_0=floor(lambda/2^17)>=lambda/2^18`.  The constant is
deliberately loose and depth independent.  The exact schedule certificate
reports constants 96, 3, 288, 290, and 148480, with leaf, packet-entry, and
all depth comparisons passing at `lambda=2^26`.  The focused native suite
passes 12/12 controls. [pcc:unverified]

This reaches a subexponential **source-cost certificate**, not a verified
Curve25519 break.  It composes the retained finite-history result with the
actual Rust representation and control flow, but has not been independently
refined against every Rust execution or kernel-checked as one theorem.  The
general contraction theorem and its prime-field ECDLP reduction remain
external assumptions; the live `usize` backend still takes the documented
finite-size fallback on every representable input. [pcc:unverified]

### Prime-field reduction and total-work ledger

The outer ECDLP reduction has now been checked against the pinned Bochum v0.2
primary source at commit `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`;
the read PDF has SHA-256
`82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11`.
The relevant chain is Theorem 4 and Lemmas 9--13.

For `b=ceil(log2(p))` and
`m=ceil((log2(b))^(1/4))`, put
`L=max(ceil(r^(1/m)),10m)`.  The random affine hashes give each fixed target
a decomposition with seed-averaged probability at least `2/5`.  Therefore
at least `1/4` of seeds cover at least `1/5` of targets.  For one such
seed, 20`(N+1)` independent targets collect `N+1` relations with probability
at least `3/4`.  Because decomposition and row-dependency selection are
deterministic functions of the seed and targets, the final coefficient `V`
remains uniform; `V!=0` with probability `1-1/r`.  The resulting exact
per-trial lower bound is

```text
3(r-1)/(16r) > 1/6.
```

The production Rust path matches the required information flow:
`solve` samples independent `u,v`, passes only
`R=[u]P+[v]Q` and the public bases to deterministic `Counter::witness`,
uses the first deterministic row dependency, retries on too few rows or
`V=0`, and replays the recovered scalar.  It enumerates each base exactly
by inverse hash and parity-selected square root, and gives a point appearing
in two slot bases two distinct matrix columns.  Its configured finite trial
cap returns a resource-limit error rather than a false answer; the theorem
schedule retries fresh trials and has expected trial count at most six.
[pcc:unverified]

The new native `tensor-reduction-audit` prices that schedule with exact
integers.  Its default Curve25519 field/subgroup row reports:

| item | exact or bit-size bound |
| --- | ---: |
| field/subgroup bits | 255 / 253 |
| theorem slots `m` | 2 |
| target membership mass `L` | 85070591730234615865843651857942052865 |
| safe columns using `s<=4p` | 5444517870735015415413993718908291383340 |
| expected prefix-contraction calls | 149 bits |
| conservative dense-elimination bit work | 412 bits |
| unit-constant relation-phase exponent surrogate | 392 |
| unit-constant linear-algebra exponent surrogate | 399 |

The last two rows deliberately omit the contraction theorem's hidden constants
and are not runtime estimates.  They show that the construction has no
practical claim at 255 bits even before those constants.

Asymptotically, one decomposition network has `O(mb)` edges, polynomial
typed description and degree `O(b^(3/4))).  Prefix self-reduction uses
`O(mb)` contractions per relation query.  The complete charged terms are

```text
factor-base construction          2^O(b/m + log b)
number of contraction calls       2^O(b/m + log b)
one contraction                   2^O(mb/sqrt(log b))
dense linear algebra              2^O(3b/m + log b)
target generation/recovery        polynomial per call.
```

With `m=ceil((log b)^(1/4))`, setup, relation finding, linear algebra, and
target recovery are all bounded by

```text
2^O(b/(log b)^(1/4)) = r^o(1)
```

for fixed cofactor and `log r=b+O(1)`.  This is a total expected-work
composition, not an online-only or free-oracle calculation.
[pcc:unverified]

The achievement remains conditional in two important senses.  The combined
Rust/source cost argument is a producer audit rather than an independent
refinement proof, and the Bochum reduction assumes the external tensor
contraction theorem.  The native typed encoders and 183-test suite exercise
the required information-flow and polynomial-description claims, but do not
kernel-check the whole asymptotic chain. [pcc:unverified]

## Primary references and local context

- Akshima et al., pinned Bochum v0.2, Theorem 4:
  <https://github.com/bochum-crypto/subexponential-ecdl/blob/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf>
- OpenAI/math, pinned D.1/D.4 source:
  <https://github.com/openai/math/blob/42907c9e999cc00d6abe7a50bfc1023250edd1b9/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/build/quantitative-contraction.tex>
- Existing lead assessment:
  `docs/leads/2026-11-17/subexponential-ecdlp-goal/REPORT.md`
- Independent-lemma and acceptance map:
  [`FORMALIZATION.md`](FORMALIZATION.md)
- Tapered recovery identity and precision obligation:
  [`HERMITE_IDENTITY.md`](HERMITE_IDENTITY.md)
- Mode-to-group packet factorization identity:
  [`GROUP_FACTORIZATION.md`](GROUP_FACTORIZATION.md)
- Symbolic versus live implementation boundary:
  [`IMPLEMENTATION_BOUNDARY.md`](IMPLEMENTATION_BOUNDARY.md)
- Pair-image degree gate:
  `research/curve25519_pair_image_gate_20261125/README.md`

