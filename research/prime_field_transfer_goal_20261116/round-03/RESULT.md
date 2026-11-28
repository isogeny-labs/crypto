# Compact CM models and smaller anomalous targets

The fixed development suite constructs six compact integer models joining a
source with order4r over Fp, p=4r+3, to an anomalous target with order r over
the smaller prime field Fr. The target-only Smart solver recovers scalars
from coordinates and OpenSSL replays every answer. Each fixture also has a
numerical full-order auxiliary Frobenius witness for the proposed common
kernel field. Native claim **PCC-20261116-99cae8 is unsigned**.
[pcc:unverified]

| m | selected d | subgroup bits | source bits | integer B bits | auxiliary ell |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1 | 5 | 7 | 10 | 7 |
| 12 | 751 | 45 | 47 | 91 | 13 |
| 24 | 1537 | 71 | 73 | 143 | 13 |
| 48 | 13275 | 125 | 127 | 251 | 67 |
| 96 | 27015 | 224 | 226 | 448 | 19 |
| 112 | 110385 | 260 | 262 | 519 | 67 |

These are generated j=0 fixtures, not arbitrary input prime-field curves.
All six predeclared bins are attempted and completed, with no excluded,
empty or timed-out native bin. Counts and bit lengths are recorded values,
not performance measurements. [pcc:unverified]

## The common model and kernel

Put u=d*2^m+1, b=2u−1, r=3u(u−1)+1, p=4r+3. The native certificate
checks require u,r,p prime. In K=Q(omega), omega²+omega+1=0, define

    pi=u+b*omega,   pi_bar=1−pi,   alpha=1+2*pi_bar.

Then trace(pi)=1, norm(pi)=r, trace(alpha)=4 and norm(alpha)=p.
Indeed 2*pi=1+b*sqrt(-3), alpha=2−b*sqrt(-3), and
1+3b²=4r. The Lean endpoints check the exact polynomial identity and
the literal small/large scaled norms using existing Mathlib laws. They do
not define pi by truncating division in Z[sqrt(-3)].

Choose sextic twists y²=x³+Bp over Fp of order4r, and y²=x³+Br over Fr
of order r. Set B by CRT, with the protocol's six bounded variants if
needed. All actual fixtures select variant0. Thus the ordinary integer
coefficient has O(log p+log r) bits; no2^r coefficient is expanded.
[pcc:unverified]

The characteristic-zero cover [pi_bar]:E→E has degree r under the
classical CM norm interpretation. Let H=ker(pi_bar). This is a cyclic
order-r subgroup in characteristic zero. Fix the source and target cube
roots as

    zeta_p=(3−2u)/(2b) mod p,   zeta_r=−u/b mod r.

For the selected twists the intended Frobenius elements are alpha and pi.
At p, alpha=1 on H, giving its rational source subgroup. At the chosen
prime over r, pi_bar has differential1, so H has étale reduction even
though the residue characteristic equals its order r. It reduces to
ker(1−Frob)=E(Fr). This uses the selected étale half of ordinary r-torsion;
the connected half would not give an injective specialization.

These statements use the classical CM endomorphism, good-reduction and
Frobenius identifications. The native checks evaluate them on the tested
points; the complete CM interpretation is a written argument, not a
formal Lean construction. The six-field PROOF-MAP.md records that boundary.

## Common field: what the auxiliary witness supplies

For each model an auxiliary split prime ell≤257 has good reduction.
Exact point counting determines the trace t. The native record supplies
Frob=a+v*omega with 2a−v=t, a²−av+v²=ell, and zero differential at
the selected cube root. Its eigenvalue on H is

    lambda_ell=a+v*z0 mod r,   z0=(1−u)/b mod r.

The full factorization of r−1 certifies that lambda_ell has order r−1.
The small count is recomputed by a separate double loop and the selected
endomorphism is tested on an auxiliary rational point. [pcc:unverified]

Under the CM identification above, the Galois action on H is a subgroup
of F_r^× containing an element of order r−1. It is therefore all of
F_r^×. Every nonzero H-point generates the same cyclic Galois point
field L/K of degree r−1. Its abscissa subfield has degree (r−1)/2.
This establishes the written common-field consequence of the witness;
it does not provide an expanded defining polynomial or an embedding
conversion algorithm. Dense degree alone is not a compressed-evaluation
lower bound.

Fixing primes of L above the two selected K-primes makes both reductions
of H group isomorphisms. Their composition

    phi = red_r ∘ (red_p|H)^(-1)

then preserves every scalar: phi([k]P)=[k]phi(P). The missing operation is
evaluation of this composition from the encoded arbitrary source Q. An
abstract inverse on H does not compute the compatible target residue.
The program contains **no source-to-target evaluator and reports zero
source-to-target outputs**. It does not choose a branch by recovering k,
building a matched subgroup table, or asserting that independently sampled
local points are the same global point.

## Actual target solver and local checks

For an anomalous target, lift a point to a curve over Z/r², compute
[r]P=(X:Y:Z) and take lambda(P)=((-XZ/Y mod r²)/r) mod r. A nonzero
lambda(generator) gives k=lambda(input)/lambda(generator). This is the
classical Smart attack, not a new scalar solver. All four public target
inputs per fixture are independently hash-derived and have no supplied
input scalar. OpenSSL checks [k]R=S and rejects k+1.
[pcc:unverified]

The j=0 CM lift is canonical at the target: its lambda vanishes on every
tested target point. The declared noncanonical lift a4=r, a6=B gives a
nonzero generator lambda in all six cases. Merely changing B would retain
j=0; changing a4 is essential for this control. Addition of public target
points also respects lambda. [pcc:unverified]

At precision p²/r², one CM contraction (alpha on the source, pi on the
target) gives local r-torsion lifts on the same integer model, preserving
the respective residue. There are42 source input checks,42 target input
checks,84 local projections and42 canonical-zero controls. These totals
include all18 nonidentity points at r19 plus four public inputs per side
per bin. The18 scalar-labelled points are a small-fixture auditor only;
the24 public source inputs remain unmapped. [pcc:unverified]

u and r use native Pocklington witness arithmetic. p uses the standard
elliptic primality criterion: a point of prime order r exceeds the
integer bound (ceil(p^(1/4))+1)². Every nonidentity chain point has unit Z;
infinity is primitive; the discriminant is coprime to p. These checks avoid
mistaking a projective point vanishing at a prime factor for an elliptic
primality certificate. Full primality/Hasse soundness is not formalized.
The native source MOV embedding degree is recorded separately; no source
hardness or generic applicability follows from these fixtures.

## Formalization and retained failures

Nine named endpoints certify small/large norms, norm multiplicativity,
small orientations and two integer polynomial identities. The unchanged
repository gate audits33 theorems, zero trusted native exceptions. Four
of the additional audited statements are anonymous existing-law assembly
probes. Explicit axiom output uses standard foundations only.
See ../receipts/round-03/lean-claims-01.json and PROOF-MAP.md.
Claim standing remains unsigned. [pcc:unverified]

Build01 of the target program failed on five indentation warnings. These
were fixed before arithmetic; build02 passes. Before the first native call,
build03 also corrects final formal-kernel Z handling and fixes public-point
admission labels. All three build logs survive. An initial identity-issuer
include renamed an old main with implicit return, producing a compiler
warning; its failed source/log are retained. No warning gate is disabled.
One preparation command used the wrong cwd; the failed Lean assembly log
survives, and corrected assembly02 passes. [pcc:unverified]

Qualifier sources are read-only, the executable is mode0555, and build,
pre-call and post-call records are separate and refuse replacement. No
timing, performance/scaling experiment, cloud spend, independent execution
proof, signed replay or leaderboard promotion is claimed. Cargo, Conductor
and KB are unavailable; full Rust workspace tests were not run.

## Requirements and next technical action

| Original requirement | Status / evidence |
| --- | --- |
| Smaller field and explicitly solvable target | implemented on the six generated fixtures; unsigned native evidence |
| Compact shared model and branch-field certificate | implemented model and auxiliary arithmetic; written CM bridge |
| Unknown-scalar source input |24 public source inputs checked locally; not transported |
| Efficient source→target evaluator, no hidden DLP/table | OPEN; no such evaluator supplied |
| Arbitrary prime-field curves | OPEN; generated CM family only |
| Novel map and full charged target advantage | OPEN; no map-cost or end-to-end comparison |
| Lean proof | partial: nine concrete arithmetic endpoints; missing structural bridges listed |

Next investigate an oriented common-field evaluation procedure from Q's
source residue, with source-relation checks and CT124 chimera controls.
Price field construction, branch selection, precision, target solve and
verification separately. A local Frobenius eigenvalue pins the kernel,
but leaves every nonzero point in it as a possible branch. Neither the
auxiliary full-orbit witness nor Smart's target logarithm resolves that
input-specific correspondence.

## Sources and preservation

See PROTOCOL.md for primary links and searched repository sources. Smart's
attack, CM norm construction, Pocklington and elliptic primality criteria
are prior art. KB unavailability does not support a novelty claim.

PR4388 merged externally at6ebf026274395b5de50ddbc8d0523a72ba016a7b
(03:21:51Z); its monitor is stopped. The read-only audit retains both parent
journal multisets and finds all18 frozen inputs, five claims and the old
PNG artifact manifest unchanged. The old published1193-row ledger is not
a prefix after1188 common rows; the incoming1190-row prefix survives.
No external ledger history is repaired. The broad research goal remains
active. [pcc:unverified]
