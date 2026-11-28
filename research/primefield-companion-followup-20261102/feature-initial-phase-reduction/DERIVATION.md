# Initial mode0 by phase-domain powering

Assume caller-certified prime scalar field r, m>=3, 4m|(r-1),
h=(r-1)/2 and n=(r-1)/(4m). The source and elliptic-curve base field retain
all predecessor restrictions. A nonzero initial sign chooses two quartic phases.

Write W=V+V^-1. Define D_0=2,D_1=W,D_(j+1)=W D_j-D_(j-1).
Then D_j(W)=V^j+V^-j. Construct D_n without reading binomial coefficients using

    D_(2j)=D_j^2-2,
    D_(2j+1)=D_j D_(j+1)-W,
    D_(2j+2)=D_(j+1)^2-2.

A recursive pair returns D_n,D_(n+1). Its recursion depth is logarithmic;
its polynomial degrees grow to n+1, so logarithmic depth is not logarithmic
field work or storage. This is the established Chebyshev/Dickson identity.

For each compatible phase u, first form the squarefree part of
D_n(W)-u-u^-1. In the quadratic algebra V²-WV+1=0, impose V^n=u using the
existing canonical fiber filter. This retains the phase domain before applying
any mode0 observation. The squarefree step is required at ramified points;
we do not assume D_n-u-u^-1 itself is squarefree.

Over this phase quotient compute

    (X-1)^h mod (X^m-V) = sum_(e=0)^(m-1) F_e(V) X^e.

The component values equal the original binomial sections modulo the phase
ideal. With A=m*b^h and the public observations f0,f_plus,f_minus, filter by
A*F0(V)-f0, then A²*V*F1(V)*F_(m-1)(V)-f_plus*f_minus. Reduction commutes
with passing to each smaller quotient. Thus old representatives remain valid
after mode0 shrinks the support. The resulting root support equals the seeded
streaming initializer, which imposed the same three phase/mode equations in
a different order. Normalization to Z=k^m and later updates are unchanged.

This replaces ReciprocalMode.build and its coefficient reader on the successful
powered path. It reuses values for both initial mode0 and signed-product
filtering, rather than recomputing sections through absorb. The returned state
has rounds=1, zero=false and an orientation object retaining only r,n.

## Capacity and work

Before constructing D_n, check 2*m*d<=slot_cap using checked arithmetic,
where d=floor(n/2)+1 for normalized sign +1 and d=n for sign -1.
The inverse-orbit argument below gives the maximum squarefree phase degree, so this is a bound
on logical output coefficients for one section evaluation. It is not peak RAM:
Dickson pairs, products, canonical buffers, copies, accumulator/base/output and
other temporaries coexist. A limit selects the existing seeded streaming
initializer; invalid-domain and arithmetic errors propagate. The adapter's
eligibility fallback remains separate.

Let B(h)=bit_length(h)-1+popcount(h). Each surviving initial phase computes
B(h) section-algebra products, with the existing dense m² and m(m-1)/2 wrap
quadratic products per product. Phase polynomial construction, squarefree gcd,
canonical filtering, scalar inverses and source calls are charged. A quadratic
product includes polynomial products and modular reductions. No coefficient
scan occurs in initial_sections, but degree n still scales with r/m. Initial
RecoveryModes setup and later streaming fallbacks can still perform coefficient
work. This patch makes no total-work or timing superiority claim.

The preflight uses the phase degree before observations, rather than a hoped-for
small degree after mode0. Removing a coefficient scan is not the same as
bounding initial candidate representation independently of r.

## Sources and proof scope

- ../feature-native-seeded-image/reciprocal.rs: Clenshaw coefficient construction
  and phase-domain/lift definitions.
- ../feature-native-seeded-image/{seeded-image.rs,powered.rs,canonical.rs}:
  sign restriction, residue sections and canonical intersections.
- NIST DLMF18.5.1, https://dlmf.nist.gov/18.5.E1, read2026-11-08:
  D_n(W)=2*T_n(W/2). The integer polynomial identities specialize to odd
  characteristic. No novelty claim for this algebra.
- KB unavailable: MCP absent, scripts/research-kb status has no executable.
  Exact predecessor files and the primary reference were used.

The written argument is conditional on the existing source and canonical-state
invariants. Finite Rust checks compare to the predecessor, separately form
character gcds, and check direct Dickson recurrence. No new Lean runtime
refinement proof or independent signed verdict is claimed.

## Exact phase-domain degree and capacity refinement

The conservative cap can be sharpened without a measurement. Over the algebraic
closure, V^n=u has n distinct roots because n<r. For u=1 or -1 the set is
closed under inversion. Each nonfixed inverse pair contributes one W value;
fixed points can only be V=1 or -1. Consequently the squarefree W degrees are

    u=1:  (n+2)/2 when n is even, (n+1)/2 when n is odd;
    u=-1: n/2 when n is even, (n+1)/2 when n is odd;
    u=+i or -i: n.

In the last case D_n(W)=0 combines the two inverse phase sets, with 2n distinct
V roots and no fixed point. Their W image therefore has n points. The later
canonical phase constraint picks the orientation, not a smaller W support.

For normalized sign +1, the maximum degree needed by section outputs is thus
floor(n/2)+1; for sign -1 it is n. The final revision uses this phase-aware logical-slot
preflight; validation01 preserves the preceding conservative 2mn version.
Even with the sharper output bound,
D_n and its doubling temporaries have degree n; reducing those allocations is
a separate construction task. This degree argument explains where a further
capacity improvement can occur and where the current representation still
scales with the subgroup order.

## Mode0-only retention bound

There is also a representation-specific constraint on a mode0-first strategy.
Fix a nonzero scalar character and a radius b. Among the nonzero scalar points
with that character there are h/m=L distinct m-th-power images. The sign
orbit is constant under multiplying k by an m-th root of unity, so mode0 is
well-defined on those images. Remove the endpoint orbit k in b*mu_m if it
belongs to the chosen sign: let delta=1 in that case, otherwise delta=0.
The remaining L-delta images each produce a sum of m nonzero signs.

There are at most m+1 such sums. They are distinct as residues when all occur,
since 2m<r. Pigeonhole therefore gives a mode0 fiber containing at least
ceil((L-delta)/(m+1)) distinct images. An explicit squarefree polynomial
representing that fiber must have at least that degree. This is a worst-fiber
bound for a representation retaining all candidates after mode0 alone. It is
not a bound on typical fixtures, an arbitrary compressed representation, the
additional signed modes, or the full recovery algorithm. It motivates combining
constraints before large explicit intermediate representations. The current
initializer computes signed sections together but still constructs the phase
domain and applies mode0 first.

## Next constructor: avoid the large phase polynomial for positive sign

Let S_j(W)=U_j(W/2), with S_0=1,S_1=W and S_-1=0. DLMF18.5.2
(https://dlmf.nist.gov/18.5.E2) gives the reciprocal expression for S_j;
together with D_j it yields the polynomial identities

    D_(2t)-2 = (W²-4)*S_(t-1)²,
    D_(2t)+2 = D_t²,
    D_(2t+1)-2 = (W-2)*(S_t+S_(t-1))²,
    D_(2t+1)+2 = (W+2)*(S_t-S_(t-1))².

Since n is prime to r and less than r, the distinct-root inverse-orbit count
above proves that their monic squarefree parts are, respectively,
(W²-4)*S_(t-1), D_t, (W-2)*(S_t+S_(t-1)), and
(W+2)*(S_t-S_(t-1)). The even formulas have t>=1; the odd formulas include
n=1 through S_-1=0. Ramified W=±2 points are included explicitly, not divided
away. These expressions match the derived phase degrees.

They suggest a further positive-sign constructor that never builds D_n and
never needs a squarefree gcd of its degree-n specialization. It would build
D_t or S_t directly at about half the degree. This is not implemented in the
current patch. The ±i phase domain still needs D_n; a change there requires
an additional representation argument. No arithmetic saving from the proposed
constructor is included in this patch's qualification.
