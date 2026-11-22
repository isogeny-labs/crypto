# Bounded image-set pullback and retained-sign recovery

2026-11-07, T-1813. Derived set identity with native producer checks; no independent signed verdict or full formal proof.

## Prior work and scope

The weighted-mode component already compiles selected character coefficients and recovers a phase from a singleton image. The retained-fiber component already lifts one image and replays sign constraints. This component combines them and extends the lift to a bounded set of images before uniqueness. It preserves the old implementation and its 17 unresolved public cases as controls.

Read feature-weighted-mode-derivation/DERIVATION.md, feature-native-fold-fiber/single.rs and feature-native-hgcd-roots/{polynomial,gcd}.rs. KB MCP unavailable. Official reference read: [FLINT nmod_poly](https://flintlib.org/doc/nmod_poly/index.html), conventions, modular arithmetic/gcd and inflation interfaces. Inflation, modular powering and polynomial gcd are standard operations, not novelty claims. This code uses the existing native backend rather than FLINT. All arithmetic is in prime fields; scalar r>=7 and base p are caller-certified primes. Existing source restrictions remain unchanged.

## Exact pullback lemma

Let m>=2 divide r-1, d=(r-1)/m, and let F(Z) be a monic divisor of Z(Z^d-1) over F_r. Its roots are a subset S of the attainable mth-power images, each of multiplicity one. Define epsilon=1 if F(0)=0 and0 otherwise. Then

    L(X) = F(X^m) / X^((m-1)*epsilon)
    degree(L) = m*degree(F) - (m-1)*epsilon.

L is monic and squarefree, and its roots are exactly all x in F_r with x^m in S. For a nonzero image z, X^m-z has m distinct F_r roots because z is an mth power and r does not divide m. Distinct images have disjoint fibers. At z=0, X^m has only the distinct root0; the explicit division changes its multiplicity from m to1. If S is empty, L=1. These statements prove the formula and squarefreeness without enumerating roots.

The internal pullback function requires the maintained divisor invariant; it does not certify arbitrary untrusted F. It checks canonical coefficients, monicity, no repeated zero factor, arithmetic overflow, and degree cap before allocating the lifted vector. The public recovery path obtains F exclusively by gcds starting from Z(Z^d-1), preserving the invariant.

For center0, each retained observation (a,s) means chi_r(k+a)=s. Refine L with

    gcd(L, (X+a)^((r-1)/2)-s).

The modular power is evaluated modulo L first. Euler's criterion, including chi(0)=0, proves that this retains exactly the consistent roots. A sequence of such gcds preserves squarefreeness and the true scalar. The singleton root is extracted from the resulting monic linear polynomial and checked by [k]P=Q. No factorization, candidate list, discrete-log table, or group collision search is used.

## Transition and charged work

After every complete prescribed orbit, apply the weighted-mode image constraints. First try the existing singleton phase extractor. Otherwise compute the prospective lifted degree using checked integer arithmetic. If it exceeds the cap, remain in image space; no lifted allocation occurs. When it fits, allocate the sparse inflation, replay all retained signs including the initial target sign, drop image/transcript storage, then make at most the configured number of extra sign queries. The transition is one-way and deterministic. Zero source values immediately yield a candidate which is verified. Exhausted budgets return unresolved; they are not extended automatically.

The adapter uses center0, radii1,2,3,4, lifted-degree cap64, canonical extra offsets0..min(17,r)-1, and source-call cap128. Radius count<=4 and extra count<=32 are checked. Source validity is checked before group operations. Scalar labels appear only in fixture construction/assertions. The zero target returns via verification. No image root or scalar root enumeration occurs in runtime.

Let n=degree(F) and N=m*n-(m-1)*epsilon<=cap at transition. Inflation costs O(N) field words and assignments. Each retained or additional sign uses O(M(N)log r + M(N)log N) field operations for modular powering and gcd, with inversion and backend costs explicit. Replaying h retained signs costs the sum over shrinking degrees and makes zero source queries. Retained history is O(h), with h bounded by the source-call budget and the orbit schedule. Image-space compiler/state storage and the existing source setup remain separately charged; the scalar degree cap does not cap all image or compiler memory. Source-call counts exclude source setup and final group verification. No elapsed-time or total-cost superiority is claimed.

## Evidence

Generated modes.rs preserves the previous weighted implementation, renames its main to previous_checks, and includes pullback.rs. generate.py/generation.json bind source and output hashes. validate.py compiles with overflow checks, runs assertions and records dependency hashes. All validation-01 outputs are retained, including warnings.

Small algebraic reference: for primes7,11,13,17,19,31 and eligible m with at most8 image values, enumerate every image subset in the reference checker only. Compare the exact lifted root set and polynomial degree to independent field enumeration. Replay four ordinary sign prefixes for each nonempty subset. Results:712 subsets,356 containing zero,2756 replay prefixes,689 cap rejections. The empty set and repeated-zero rejection are included. Degree equals distinct root count in every lifted and refined reference.

The unchanged previous adapter still records46/63 recoveries and17 unresolved under four radii. The integrated adapter records63/63, including all20 frozen F509/r479/m2 cases. Fixture identities and prescribed scalars are inherited exactly from the weighted component: two full13-case F7/r13 panels (m3,m6), frozen20 F509/r479/m2,17 fixed F701/r683/m11 cases. There are27 transitions from more than one image; maximum lifted degree60. Native totals:511 source calls,180 retained-sign replays,115 extra queries. Additional call-cap1 controls run on all63 targets and reject nonidentity recovery. Each successful scalar is checked against its point. These counts belong to different control flows and do not establish a timing comparison.

Original4563-case cohort unchanged. No cloud execution, new statistical experiment, broad release rerun, or independent signed review. The scalar-image degree reduction and bounded set extraction are verified at this component's scope; the public source still has its earlier restricted domain and charged per-query cost.

## Next construction

Avoid repeated modular exponentiation when replaying many retained signs: investigate a batched residue computation or subproduct organization that reuses the final low-degree modulus without materializing degree-r polynomials. Independently examine orbit source batching and domain extension. Any schedule/cap optimization needs a frozen comparison protocol before performance measurements. The current fixed schedule and prior failures remain immutable.
