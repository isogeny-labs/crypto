# Direct oriented phase supports

T-1877. Producer assertion; independent signed verdict pending. [pcc:unverified]

## Initial folding identity

Assume the predecessor's caller-certified prime/order conditions and4m|(r−1). Write h=(r−1)/2=2mn and F0(V)=sum_(j=0)^(2n) a_j V^j. The signed-binomial symmetry gives a_j=a_(2n−j), with a0=a_(2n)=1. The prior reciprocal construction formed F0(V)=V^n H(V+V^-1), built H and a Dickson polynomial D_n, then recovered oriented candidates through phase filtering and lifting.

Instead fix u in{1,−1,i,−i}, i²=−1, and reduce directly in F_r[V]/(V^n−u). A term a_j V^j contributes a_j*u^floor(j/n) to coefficient j mod n. Read only j=2n,...,n from the existing bounded coefficient stream; for j>n also insert its mirrored term at2n−j, and insert the center only once. This uses n+1 reader deliveries and four degree-below-n residue arrays, without constructing H, D_n or the affine Clenshaw transform.

For normalized observation c=f0/A, the oriented phase support is

G_u(V)=gcd(V^n−u, R_u(V)−c),

where R_u is the folded residue. If the equation is nonzero and nonconstant, compute V^n−u modulo the equation by binary modular exponentiation and take the gcd; do not expand the sparse phase polynomial. A nonzero constant equation returns1. An identically zero equation returns V^n−u explicitly: that branch genuinely contains all n phase roots, and this implementation charges its n+1 coefficient allocation.

V^n−u is squarefree because n<r, u≠0. Its roots lie in F_r: n divides r−1 and u^((r−1)/n)=u^(4m)=1. The four phase supports are disjoint and exhaust the nonzero mth-power image V^(4n)=1. Handle V=0 separately using the exact C0/C2 zero conditions. These statements preserve phase orientation directly; inverse partners need no later disambiguating lift.

## Persistent scalar update and recovery

The adapter anchors V=t0 Z at the first radius. Later modes evaluate at(t/t0)V. Each branch stores its monic G_u directly in V. Evaluate C0 and the fused F1/F2 pair modulo G_u and replace G_u by gcd(G_u,equation). Reuse the existing generic scalar component and power/reader machinery, through an explicit scalar_variable constructor. This constructor does not create a CanonicalFiber or assert a quadratic relation for V; only scalar quotient arithmetic is used.

After each radius, the union is the product of the disjoint phase supports, with an optional zero factor. Substitute V=t0 Z and normalize. This is exactly the same candidate polynomial as the canonical reciprocal representation, though the stored representation differs. The new adapter has the same radius, source-call, image and pullback caps. Direct materialization checks its actual branch degrees; reciprocal lifting may have a different temporary-work envelope outside the checked panels. No universal routing equivalence is claimed from the finite tests.

## Costs and limits

Folding uses O(n) field work after the same n+1 coefficient deliveries, replacing construction of the reciprocal H/D_n and its balanced matrix transform. It retains four n-entry residue arrays during initialization. Each phase still requires a gcd and, for nonconstant folded equations, modular exponentiation costing O(M(d)log n) for equation degree d below n, with the actual division engine charged. The gcds and dense support storage remain; this is not an implicit representation of all future constraints.

Representing a branch directly in V can be larger than an unresolved reciprocal support in W. It removes quadratic-pair updates, selected-root CRT/inversions and reciprocal lifting from this adapter, but changes degree-dependent arithmetic. Do not infer a universal memory or timing advantage. Source grids, order checks, initial observation construction, coefficient stream setup/advancement, phase search, ring contexts/caches, inversions, allocations/capacities, all source calls and final point verification remain charged. The initial-support degree parameter and the zero-equation expansion are explicit. Original4,563cohort and prior adapters/evidence remain unchanged. No cloud or timed workload is run.

## Controls

For supplied small primes and admissible orders, compare folded residues with dense polynomial reduction, and compare G_u for every c in F_r with both dense gcd and the predecessor reciprocal branch/lift. Check squarefreeness, zero/nonzero-constant cases and initial degree admission. The state panel compares complete candidate polynomials and each old phase's lifted support with the direct representation over four radii. Two public panels compare the fused reciprocal and direct adapters under caps64/16. Root/scalar enumeration is confined to checker fixtures.

See RESULTS.md and validation01 for observed counts, source pins, failures and current delivery scope.

## Sources and next direction

Reviewed feature-native-fused-inverses/{reciprocal.rs,canonical.rs,fused-adapter.rs}, the existing palindromic identity and generic scalar evaluator. Official FLINT polynomial quotient/division conventions: https://flintlib.org/doc/nmod_poly/index.html. Direct reduction modulo a binomial is standard polynomial arithmetic, not a novel transfer. KB MCP unavailable and scripts/research-kb status reports no executable; exact predecessor and primary documentation provide the fallback.

Next investigate whether multiple initial observations permit a low-degree constraint before any degree-n support is formed. The present construction removes the reciprocal transform and lift, but still constructs folded residues and computes their gcds; avoiding that remaining degree-dependent work requires a separate construction, not an extrapolation from these checks.
