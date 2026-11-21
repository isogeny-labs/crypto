# Prime-field transfer goal: component targets and input-selected examples

Latest: [round06 modular construction and growing public inputs](round-06/RESULT.md)
and [report](reports/round-06.pdf). The new degree21/order127 Weber model is
constructed, and fixed public-input tests recover4/4 order61 scalars at64/128
digits and3/4 order127 scalars at32/64. All four order127 processes time out
during128 work; no128 result is inferred. The route returns a Weber class
before target solving and source candidate checks. Claims37e825/e0d875 carry
the PCC-20261116 prefix and are unsigned. Twelve finite split-prime Lean
endpoints and the unchanged77-theorem gate concern coefficient reconstruction,
not the full CM/native transfer. Original goal remains active. [pcc:unverified]

Earlier component: [round04 source-selected Weber roots](round-04/RESULT.md)
and [two-page report](reports/round-04.pdf). Coordinate-input rational Padé
recognition accepts all18 order19 inputs from F79 to anomalous F19 at the
tested8/16/32 digits and3/4 declared public order61 inputs from F1531 to
F61 at32 digits. The remaining public case and all earlier failures stay
recorded. Nine named Lean endpoints cover literal global-root, quotient
and residue certificates; the CM/specialization/native bridge remains a
written argument. Claims fb2ab0/be69df carry the PCC-20261116 prefix and
are unsigned. [pcc:unverified]

The [round03 result](round-03/RESULT.md) and
[two-page report](reports/round-03.pdf) cover compact CM models and
smaller anomalous targets on six generated fixtures. The native target-only
Smart solver supplies the target component used by the new small examples.
The efficient arbitrary-curve evaluator remains open. Claims are unsigned.
[pcc:unverified]

The earlier [round02 local lift](round-02/RESULT.md) and its frozen receipts
remain unchanged.

The round constructs and checks one compact curve-model component, rejects an
independent-lift shortcut on an actual order-five source subgroup, and tests
the precision needed by a proposed target encoding. **An efficient
arbitrary-point transfer has not been constructed. The research goal stays
active.** No novelty or practical reduction is claimed. [pcc:unverified]

Read [GOAL.md](GOAL.md) for the unchanged acceptance criteria and sources,
[PROTOCOL.md](PROTOCOL.md) and [AMENDMENT-01.md](AMENDMENT-01.md) for the
pre-call plans, and [PROOF-MAP.md](PROOF-MAP.md) for the exact Lean boundary.
The editable diagrams and inspected report are in [reports/](reports/).

## Candidate target and model component

An elliptic curve with split multiplicative reduction of type I_r at 2 has
a component quotient isomorphic to Z/rZ. Cremona gives a classical explicit
component map kappa from an already available local point. This target is a
component quotient of a Neron model, not the point group of an elliptic curve
over F2. It therefore does not have the genus-one Hasse point-count limit.
The standard component map and Tate uniformization are prior art.
[pcc:unverified]

For any odd source prime p and a positive r, an integral model can be
represented by five small integers c1,c2,c3,c4,c6 and the binary exponent r:

\[
A_1=1+2^{r+1}c_1,\quad A_i=2^{r+1}c_i\ (i=2,3,4),\quad A_6=2^r c_6.
\]

Use c1=(a1−1)2^(−r−1) modulo p, ci=ai 2^(−r−1) modulo p for i=2,3,4,
and the odd representative c6 congruent to a6 2^(−r) modulo p. These
coefficients reduce to the specified source equation. At 2, the node has
distinct tangent lines y and y+x. In the usual discriminant formula, b2 is
odd, v2(b8)=r, and every term other than −b2²b8 has valuation greater than r.
Thus v2(Delta)=r and c4 is odd; the classical Tate-algorithm interpretation
is split type I_r. This paragraph is a written general construction, not a
formal implementation of Neron models. [pcc:unverified]

The binary size of this symbolic description is polynomial in log(p)+log(r).
Expanding its conventional integral coefficients takes at least r bits. The
native constructor only evaluates source projections or bounded expansions;
it does not materialize cryptographic-sized 2^r or provide a point lift.
[pcc:unverified]

| Source-model fixture | r | Local invariant check |
| --- | --- | --- |
| p=17, y²=x³+2x+2 | 19 | Expanded model: v2(Delta)=19, c4 odd |
| p=257, y²=x³+23x+11 | 83 | Expanded model: v2(Delta)=83, c4 odd |
| p=37, y²+y=x³−x²−10x−20 | 7 | Expanded model: v2(Delta)=7, c4 odd |
| Public secp256k1 model | Standard 256-bit subgroup order | Source projection only; modular exponentiation agrees with OpenSSL |

All three bounded fixtures reject both a source-coefficient mutation and a
c6 parity mutation. The 256-bit check concerns only the model's compact
projection, not an ECDLP instance solve, a target point, a complexity
measurement or evidence of a general transfer. Native observation claim
`PCC-20261116-4d7a4e` remains unsigned. [pcc:unverified]

## Independent rational lifts fail to preserve the source relation

Use the rational curve

\[
\mathcal E:y^2+xy=x^3-64x+288,\qquad S=(4,8),\quad T=(2,12).
\]

At 2 it has c4 odd and v2(Delta)=5. In Cremona's consistent tangent
orientation, kappa(S)=2 and kappa(T)=1 in Z/5Z. The fixed prime scan selects
p=59, where #E(F59)=65 and the cofactor is h=13. Exact rational addition and
separate modular addition give

\[
P=\operatorname{red}_{59}([13]S)=(11,28),\quad
Q=\operatorname{red}_{59}([13]T)=(11,20)=-P,
\]

with both P and Q of order five, while

\[
\kappa([13]S)=1,\qquad\kappa([13]T)=3.
\]

A scalar-preserving homomorphism sending P to 1 must send Q to −1=4, so
these proposed labels fail. More explicitly,
U=[13](T−[4]S) reduces to O at 59 but has kappa(U)=4. Its exact rational
coordinates are retained in the raw native output. This demonstrates the
failure of the required kernel inclusion for these independent lifts.
It does not rule out a differently restricted, compatible lift.
[pcc:unverified]

Lean checks the nonsingular source curve, literal points, Q=−P, both
orders and the conclusion that no additive homomorphism from this source
point group to Z/5Z has the proposed labels. It uses Mathlib's `map_neg`
as stated. The rational lifting and component-map interpretation are native
checks plus the written classical argument; they are not presented as a
fully formal Neron construction. Claim `PCC-20261116-a3e9e1` is unsigned.
[pcc:unverified]

## Low absolute precision also loses target labels

The informed successor is to extract only the component label of a consistently
selected torsion branch. Its target encoding must still retain that label.
The separate classical Tate control has period q=2^r and auditor parameters
u=2^k for 1≤k<r. Each represents an order-r torsion point with component
label k. These k values are known to the target-side auditor; no source
evaluator uses them. This is not the exact same global CRT model above.
[pcc:unverified]

At absolute precision N, all labels N≤k≤r−N have both coordinates zero
modulo 2^N. The native control checks the truncated Tate formulas against the
reduced equation and retains every collision. All nine predeclared cases
r in {19,83,127}, N in {3,5,8} agree with the predicted counts. At N=5:

| r | Nonzero auditor labels | Labels encoded as (0,0) | Distinct coordinate pairs |
| --- | --- | --- | --- |
| 19 | 18 | 10 | 9 |
| 83 | 82 | 74 | 9 |
| 127 | 126 | 118 | 9 |

This rejects fixed low absolute precision as a faithful encoding of these
targets. An encoding carrying valuations separately, or a direct label
extractor, remains outside this test. No general algorithmic lower bound or
source-point transfer follows from the collisions. [pcc:unverified]

The target-control observation claim is `PCC-20261116-3bd2b2`, unsigned.

## Verification and reproduction

The native programs require a C++17 compiler, GMP and OpenSSL. From this
directory:

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Werror component_candidate.cpp \
  -lcrypto -lgmpxx -lgmp -o /tmp/component-candidate
/tmp/component-candidate
g++ -std=c++17 -O2 -Wall -Wextra -Werror target_precision.cpp -o /tmp/target-precision
/tmp/target-precision
cd lean
lake build
lake env lean AxiomAudit.lean
```

From the repository root:

```sh
tools/lean/proof-gate.sh research/prime_field_transfer_goal_20261116/lean
```

The unchanged local proof gate audits 16 theorems, zero failing and zero
trusted native exceptions. The earlier 15-theorem gate is retained separately.
Lean is pinned to 4.24.0 and the resolved mathlib v4.24.0 revisions.
`IDC1hcad63e1a0fbc75b5` certifies the denominator-cleared nodal polynomial
identity used by the target control, with an exact Lean proof pointer.
Its C++ issuer accepts 32 points and rejects one perturbed control; this is
a producer check, not independent Rust replay or a signed PCC verdict.
[pcc:unverified]

Raw failed builds, successful outputs, source hashes, probe, axiom audit and
gate receipts are in `receipts/round-01/`. The initial compiler lacked Boost;
the implementation instead uses the already installed GMP. Later Lean
failures were resolved by the existing division law with explicit nonzero
denominators, without increasing limits. Root Cargo is absent; no root Rust
suite pass or performance timing is claimed. [pcc:unverified]

## Next candidate and exact outstanding obligation

Fix one source place, target place and generator orientation for the entire
input subgroup. Seek a compressed algebraic r-torsion representation in which
the component value of the unique source-compatible torsion point can be
computed directly. The selector must accept encoded Q without its scalar,
without choosing independent local branches for P and Q and without expanding
a degree-r or degree-r² global field by default. A short root-of-circuit
description is not an implementation of cross-prime branch matching.

The next discriminating action is to specify that component extractor over
the actual torsion algebra and test all public inputs on one fixed setup,
with wrong-place and wrong-orientation controls, before any large-order
timing run. Full lifting cost, faithful target encoding and end-to-end target
solving advantage remain open requirements. A plan, a decoder or the rejected
candidate does not complete the goal.

Existing performance panels and the earlier example diagrams are unchanged:
this round reports correctness and encoding obstructions, with no measured
speedup or new benchmark/leaderboard row.
[pcc:unverified]


Round05 follow-up: the [coefficient-metric trial](round-05/RESULT.md) has unchanged fixed-bin coverage; its inspected [one-page report](reports/round-05.pdf) preserves the null result and feasible real-matrix Lean certificate. The round04 source-selected examples above remain the current coverage result. PCC-20261116-fc5aeb is unsigned. [pcc:unverified]
