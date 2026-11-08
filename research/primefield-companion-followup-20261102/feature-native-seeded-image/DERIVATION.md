# Initial character selects phase branches

Let r be an odd prime, m>=3, 4m divide r-1, h=(r-1)/2,
n=(r-1)/(4m), L=h/m=2n. Let b be a nonzero public radius,
k a scalar, x=k/b, V=x^m, and Z=k^m. These variables describe
correctness; the runtime adapter receives a point and an existing sign source,
not k. The elliptic curve base field and scalar field are distinct prime fields.

The initial source query returns s=chi(k). The identity endpoint s=0 is handled
by the existing point verification before initialization. Otherwise s is 1 or -1.
Euler criterion and multiplicativity give

    V^(2n) = x^h = chi(k)*chi(b) = s*b^h.

The canonical representation already partitions nonzero candidates by V^n=u,
where u ranges over 1,-1,i,-i and i^2=-1. Therefore a compatible branch must
satisfy u^2=s*b^h. Exactly two of the four choices satisfy this equation: 1,-1
for normalized sign 1, or i,-i for normalized sign -1. Select them before
orientation.branch, squarefree reduction, canonical initialization and phase gcd.
No new source query is needed. This is an application of the existing character
identity, with no novelty claim for that identity.

The same condition in the materialized image coordinate is Z^L=s. For the
monic squarefree old image polynomial G(Z), the seeded polynomial is

    gcd(G(Z), Z^L-s).

The old phase branches have disjoint V roots. Removing the incompatible two
removes exactly roots violating the character equation; multiplication by b^m
carries them to Z. The nonzero right-hand side also excludes Z=0. Subsequent
filters are intersections and do not change this identity. Repeated radii and
both capacity paths use the same anchored phase labels.

This is conditional on the existing source returning the specified scalar
character and on the predecessor phase/canonical representation invariants.
The finite checks test those inherited mechanisms on fixtures; this document
is not a full formal refinement proof. A deliberately wrong sign must exclude
the true nonzero image, making the source contract explicit.

## Implementation and work

SeededImage adds a checked initial_character argument; invalid values including
zero are rejected. The optional recover_seeded_image adapter passes the first
query result, keeps offset-zero history, and preserves endpoint, call-cap,
capacity fallback, replay and final elliptic-point checks. No default changes.

The branch selector reduces the initial phase-polynomial attempts from four to
two. It adds normalization arithmetic and four scalar square comparisons. This
version computes b^h once more for normalization; it does not claim that step
is free. Initial ReciprocalMode.build still runs before branch selection and
constructs its long mode0 representation. Source extraction, setup, allocation,
coefficient generation, quotient operations, canonical filtering, lifting,
replay, inversions and verification remain charged. Earlier pruning can change
routes and call totals; equality with the old route is not an acceptance rule.
No elapsed-time or total-work ranking is asserted.

## Sources checked

- ../feature-native-powered-image/{powered-image.rs,powered-adapter.rs}: the
  initial query was present but its character was only replayed later.
- ../feature-native-signed-reciprocal/DERIVATION.md: epsilon=u^2 and the scalar
  field versus base field distinction.
- ../feature-native-powered-image/lib.rs: actual source sign extraction and
  calibration; this remains a material cost dependency.
- Ben Lynn, Quadratic Residues, https://crypto.stanford.edu/pbc/notes/numbertheory/qr.html
  (read 2026-11-08): odd-prime Euler criterion and character multiplicativity.
- KB unavailable: MCP absent; scripts/research-kb status reports no executable.
  Exact predecessor files and the primary mathematical exposition were read.

Next address initial mode0 construction, which occurs before this branch saving,
and evaluate full work only under a common frozen comparison protocol.
