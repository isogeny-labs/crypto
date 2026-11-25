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
analysis.

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

3 passed; 0 failed
```

Those controls cover parallel/correlated ports, quadratic incoming modes,
outgoing form powers, two taper levels and a nested child whose older packet
keeps its creation precision.  They are bounded exact controls, not a proof of
the asymptotic induction.

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
the packet-capacity and precision premises.  It does not yet bind those
premises to every actual recursive child or price ring/rational bit
operations.  Those history/refinement obligations remain the next gate.

### Uniform source-history bridge

The capacity and normalized-precision premises are now bound to the successful
source path at the scalar-invariant level.  Production packing calls
`pack_labels(..., enforce_bounds=true)`, which rejects unless
`K*L<=3M`.  The remaining precision bridge combines the finite-history
bound already proved in
`research/hashed_ecdl_precision_history_20261123` on `origin/main`,

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

For the source scale `u_d=floor(lambda/(2^17*2^d))`, this implies
`u_d>=24` whenever `lambda>=2^26`.  The proof is elementary: if
`u_d<=23`, then

```text
lambda^5 < (2^17)^6 * 24^6 < 2^130,
```

contradicting `lambda^5>=2^130`.  The same depth inequality gives

```text
h^5 < (2^17)^3*(u+1)^3,
h <= 512*(u+1)                    for u>=24.
```

The base cases at `u=24`, followed by factor-at-most-two induction, yield

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
passes 9/9 controls. [pcc:unverified]

This threshold is asymptotic: it means a root edge count on the order of
`2^(2^26)`, far outside the live `usize` representation and irrelevant to
practical 255-bit security.  It closes the uniform capacity/precision bridge
for the theorem path, not the complete cost theorem.  Ring operations,
interpolation, Hermite rational numerators/denominators, description bit
widths, and their composition over the recursive call tree remain to be
priced with a depth-independent constant.

## Primary references and local context

- Akshima et al., pinned Bochum v0.2, Theorem 4:
  <https://github.com/bochum-crypto/subexponential-ecdl/blob/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf>
- OpenAI/math, pinned D.1/D.4 source:
  <https://github.com/openai/math/blob/42907c9e999cc00d6abe7a50bfc1023250edd1b9/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/build/quantitative-contraction.tex>
- Existing lead assessment:
  `docs/leads/2026-11-17/subexponential-ecdlp-goal/REPORT.md`
- Pair-image degree gate:
  `research/curve25519_pair_image_gate_20261125/README.md`

