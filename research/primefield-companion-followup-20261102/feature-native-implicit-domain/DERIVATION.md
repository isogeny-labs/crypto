# Implicit image-domain intersection

2026-11-07, T-1850. Derived polynomial representation change, native implementation and deterministic software checks. This changes the scalar-field image state in recover_traced; it does not change the coordinate-field source kernel or the frozen prospective trace collector.

## Located cost and selected change

The preceding adapter allocates d+2 coefficients for U(Z)=Z^(d+1)-Z, d=(r-1)/m, on every nonidentity target before observing the first radius constraint. This is an unconditional O(r/m) initialization/allocation in that representation. It occurs before the fiber-degree cap is applied and even on paths that later return an endpoint or exhaust no radii. The stored polynomial represents zero plus all m-th-power images in F_r, for caller-certified prime r and m dividing r-1.

The new ImageDomain retains only r,m,N=d+1 and an absent dense vector initially. A zero constraint leaves this state implicit. A nonzero constant produces [1]. For a nonconstant canonical constraint C of degree c, compute T=Z^N mod C by binary modular powering, then G=monic_gcd(C,T-(Z mod C)). Store G densely and use the preceding gcd route for every later constraint. Raw constraint length is capped before copying as in the previous gcd API. Existing phase, fiber, replay and public-point verification semantics are preserved.

## Exact identity and invariant

If U=QC+R, an element divides both U and C if and only if it divides C and R. Thus gcd(U,C)=gcd(C,U mod C), up to normalization. Binary powering inductively maintains the usual residue of the requested power modulo C. Subtracting the residue of Z gives U mod C. Monic normalization therefore yields exactly the preceding initial gcd, including nonmonic or repeated-root C. No root enumeration or factorization occurs in this construction.

The state invariant is either implicit U, or the monic gcd of U and every received constraint. Gcd with zero changes nothing; gcd with a nonzero constant is1. Induction establishes equivalence for every finite sequence of admitted constraints, not just the finite fixtures. U has simple roots: at zero U'=-1, and at a nonzero root U'=d, nonzero in F_r because 1<=d<r. Thus its divisor states remain squarefree even if a constraint has repeated roots.

If dense state exists, pullback uses the unchanged helper. For implicit state,

    U(X^m) / X^(m-1) = X^(md+1)-X = X^r-X.

The implicit pullback has degree exactly r. It checks r<=cap and checked length arithmetic before allocating r+1 coefficients. If cap<r it returns Limit without constructing U or its pullback. Identity state never masquerades as a small candidate set. The original API's constant and linear state decisions are retained through degree() and linear_root().

## Charged costs and material limits

Implicit initialization is O(1) storage/work for the existing word-size domain. The dense U vector and its O(N) zero filling disappear. At first nonconstant C, residues have degree<c and each top-level multiplication result has degree<=2c-2. Binary powering uses exactly floor(log2 N) squaring/reduction stages and popcount(N) multiply/reduction stages in this implementation, followed by subtraction and gcd; initialization also reduces Z modulo C. An upper bound in existing kernel costs is their sum times the cost of multiplication and reduction at these degrees, plus final gcd. Under fast polynomial arithmetic this is the usual logarithmic-exponent modular-power construction, but the code's actual Newton/half-GCD costs must be retained for quantitative claims. No universal field-operation or elapsed-time dominance over directly dividing dense U is asserted.

Top-level temporary polynomial dimensions depend on c, not a separately expanded U. Constraint construction, gcd/reduction recursion and the returned dense state remain charged. This is not a claim that all recovery memory is O(c), since compiled mode coefficients and source grids are separate live objects.

The first constraint c0 in Modes has degree floor(h/m), h=(r-1)/2, whenever m<=h. Its leading coefficient is nonzero: all binomial coefficients binom(h,j) are nonzero modulo prime r since h<r, and the public scale m*b^h and change-of-variable scale are units. Only the constant coefficient is modified by the observation. Therefore on this branch c=floor(d/2), and the first stored gcd degree is at most c. The exceptional m=r-1 gives degree-zero c0; zero/constant handling above covers it. This does not remove the separate O(r/m) mode-coefficient construction.

The coordinate-field source separately constructs a grid with width/block count on the order of sqrt(r); its max_lanes guard and constructor calibration remain. This change neither removes that dependency nor enables full large-order public-source recovery by itself. The purpose is to eliminate an unnecessary expanded domain while preserving the exact recovery state.

## Native validation

validation-01 passes8,600 constraint intersections against explicit U/gcd and dense pullback references over r7,11,13,17,31,97 and every divisor m>=2 of r-1. Constraints include zero, arbitrary small coefficients, repeated constraints, linear and constant termination, with monic/nonmonic cases.2,192 zero-constraint stages retain an implicit state;3,113 cap checks reject materialization. Counts include sequential intersections per fixture, not independent sampled experiments.

A separate large-domain helper check uses prime r=2^61-1 and m=2, hence N=2^60. It constructs no U vector, retains implicit state after zero input, rejects pullback cap64 before allocation, and intersects C=Z-1 exactly to [r-1,1]. This is a polynomial helper check, not a large-order ECDLP recovery.

All inherited algebra/image/envelope/workspace controls pass. The altered public recover_traced agrees with the earlier dense recover_enveloped path in617 additional targets:5,929 calls, zero unresolved; scalar results, routes and recorded source-call/replay/image statistics agree. The63 inherited public comparisons use511 calls and agree. No cloud or timing experiment, full Cargo rerun, or change to original4,563-case cohort.

Reproduce via `python3 validate-recovery.py NEW_OUTPUT_DIRECTORY`. Receipt pins exact rustc command, compiler, source dependency hashes, exit and stdout digest. Reference scalar enumeration is confined to validation; runtime uses public points and polynomial constraints. Existing source modules are copied unchanged by generate.py; source/generation hashes retain that provenance.

## Sources and next task

KB unavailable (no MCP, wrapper executable missing). Read preceding weighted source and Modes/trace-adapter code, reference pullback, scalar gcd implementation and primary [FLINT modular exponentiation documentation](https://flintlib.org/doc/nmod_poly/index.html#arithmetic-modulo-a-polynomial). This is standard gcd reduction and modular powering, no novelty claim. Current code inspection identifies the domain allocation; no workload measurements are inferred from the prospective collector's software tests.

Next investigate a compact or streamed representation of the mode coefficients/first constraint itself. This change moves the domain intersection to the constraint's size; the next scaling opportunity is to avoid building a large constraint before that intersection. Continue accounting for the source grid independently. Prior trace protocol remains immutable and targets the preceding collector revision. No canonical research graph/hypothesis status changes.
