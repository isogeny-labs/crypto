# R30: uniform recovery scaling and actual ordinary scalar monodromy

Observations and exact algebra only. No IC cost, relation-rank or attack verdict.
The original F2 modular-factor transfer and arbitrary-class reverse evaluator
remain open. The active research goal continues beyond this round.

## Integral recovery scales as an identity

Generalize the actual R22 n=5 coefficient pattern to
m=(2+w)+7r+(6+2w)r^3 and7I, over Z[w],w^2+w+2=0,r^n=1.
Native SCALE30 checks19 odd lengths through n1021, including every one of the
1020 augmentation basis vectors at that size. Each has least recoverable
positive scalar1. Ten even controls have least scalar7: evaluation r=-1,w=3
mod7 annihilates both maps and not1. The explicit7I map supplies7.

This is a generalized algebraic family, not geometric cover maps at larger n.
The inherited regular-cycle maps responsible for R21's denominator5 obstruction
are different maps. No R21 obstruction is contradicted.

There is a universal integral formula. Put
S=sum_{i=0}^{n-1}(-r^3)^i, a=12S, e=w-3, b=1+2r^3,
h=17+a(r+r^3). For odd n, (1+r^3)S=2 and e^2=7(1-w), giving

    B1=a-e*a^2*b,
    B3=(1-w)*a^2*b^2-h*(1-e*a*b),
    B1*m+7*B3=1.

Lean proves this for every odd n, including lengths divisible by7. Native
records use balanced mod7 lifts to retain compact certificates, and verify
the exact integral identity plus every augmentation basis vector separately.
Maximum absolute CM coordinate among the recorded compact certificates is6;
that is a recorded finite-list value, not a universal height theorem.

## Actual ordinary fibers, including the larger degree436 field

Use the R29 selected pairs, with the original Sutherland Phi11 integer data
and R13's kernel/transport formulas. Native common-neighbor factor products,
7/11 division factors, stable kernels, commuting squares and BOTH complete
49-level transported triples are checked. Independent ENUM30 confirms all
common-j and parameter polynomials split completely and simply over the
retained fields: no omitted nonrational candidate remains in this finite
ordinary middle enumeration.

| Base field | Complete compatible middles | Kernel color pairs |
|---|---:|---|
| F2^12 | 2 | (3,4), (4,3) |
| F2^436 | 1 | (3,3) |

Colors are powers of doubling modulo sign in F11*/{+/-1}; all six kernel
colors are nonzero, so all scalar fibers have orbit length5. Their finite
algebras have absolute F2 dimensions60 and2180. Full explicit degree2180
coordinates are unnecessary for these kernel-algebra certificates. This is
larger finite-algebra geometry on the existing n=5 ordinary lane, distinct
from the formal n1021 recovery test.

The particular ordinary middle/kernel construction is not an identification
with the proper C61 incidence carrier. Its moduli/collector binding remains a
written bridge. Complete signed principal divisors and exceptional fibers
remain separate obligations.

## Why the closed orbit does not finish the colored transfer

DESCENT30 binds all six recorded colors and checks the determinant relation
(firstColor+secondColor)%5=baseDegree%5. In a Frobenius cycle of length5d,
the deck step is d*k^-1 modulo5d. All coordinates satisfy F^d=D^k.
F^5 has5 components, each of lengthd; deck acts regularly on those components.
Thus after F32 constant extension an individual component has a nonzero
formal augmentation difference, while the sum of all five has difference0.
Both positive controls and corrupted-monodromy rejection pass.

This resolves a permutation-module distinction. It does not prove an
individual divisor class nonzero: that needs actual divisor/Picard evaluation.
The degree460 BASE divisor cancellation and an inert F2 scalar orbit cannot
be used alone as a zero-column certificate for an individually colored F32
collector divisor. Retain actual component/descent data in the reverse map.

## Evidence and next work

Five native attempts, all accepted; two Lean compile/audit runs accepted.
Nine named Lean theorems,13 audited declarations including4 generated,
0failures/0trusted-native. No older source, failed input or canonical archive
is changed. A standalone checkpoint probe without credential export failed
before a trial; scoped temporary ambient credentials allowed durable gates.
No new EKS, Diem, waiting-freeze action, fifth Conductor claim or merge.

Next: identify individual F32 component divisors and coupled descent, evaluate
their actual Jacobian images, then collect new projected columns. Apply the
same kernel method to ordinary degrees284/83. New prospective leads in
LEADS.md include the adaptive exponent family for even n and canonical/Cartier
constraints before elimination. Actual larger-n geometric carriers still need
construction, identities, descent and intended Koblitz-subgroup binding.
