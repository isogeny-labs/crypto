# Signed-product recovery across radius charts

Assume the existing prime scalar field r, exact-order zeta of order m>=3, and 4m|(r-1). Let h=(r-1)/2, L=h/m, n=(r-1)/(4m), and sigma=(-1)^h (here sigma=1). The preceding signed-reciprocal construction establishes the weighted-product equation for nonzero normalized scalar x:

    f_1 f_(m-1) = m^2 sigma epsilon F_1(V) F_1(V^-1),
    V=x^m, epsilon=V^L=x^h.

Public inputs are the target point and the existing source's signs. Hidden scalars and coefficient/root enumeration occur only in correctness fixtures.

## The scaling obstruction and its resolution

Fix the initial radius b0 and anchor V0=(k/b0)^m. At radius b, let s=(b0/b)^m, so V=sV0. Existing canonical fibers use W0=V0+V0^-1 and relation V0^2-W0V0+1=0. Old-chart conjugation maps sV0 to s/V0, which is generally not (sV0)^-1. Consequently taking the old-chart norm of F_1(sV0) is not the desired signed product.

The correct two evaluation bases in the old quotient are

    B_plus = s V0,
    B_minus = s^-1 (W0-V0) = (s V0)^-1.

Their product is one. Their trace is

    B_plus+B_minus = s^-1 W0 + (s-s^-1) V0.

This trace depends on the orientation V0 unless s=s^-1. Thus later constraints generally have both components A(W0)+V0 B(W0); forcing the equation to be scalar in W0 would lose the relevant orientation information.

`transport.rs` evaluates the same F_1 coefficient stream at both bases with a balanced recursion. Each coefficient is consumed once per active branch evaluation. Internal nodes combine each child value with the other child's base power. The returned product equals F_1(sV0)F_1((sV0)^-1) modulo the anchored quadratic quotient. For s=1 it uses the existing norm implementation directly.

## Phase and canonical-state update

Each anchored branch maintains V0^n=u for u in {1,-1,i,-i}. Since L=2n,

    epsilon_current = (sV0)^L = s^L u^2.

The implemented equation is m^2 sigma epsilon_current times the transported product minus f_1 f_(m-1). The scalar factor is known from the radius and the stored phase; it is not obtained from k. A valid radius makes it +1 or -1, which the equation helper checks.

The mode0 equation is applied first for subsequent radii, using the existing scaled mode0 evaluator. Then `CanonicalBranch.keep` applies the new two-component equation, retaining both orientations where appropriate and a selected root where the equation determines one. Its existing modular-inverse/CRT representation and lift retain phase compatibility. Neither W nor epsilon alone is treated as a recovered scalar.

The zero candidate is handled outside the inverse chart: f0=A and f1=f_(m-1)=0, with A=m*b^h. The public adapter already checks the identity and every zero-sign endpoint directly. Individual modes and final point verification remain available after image materialization. A zero product is not assumed to distinguish sign branches.

## Recovery adapter

`signed-adapter.rs` derives from canonical-adapter.rs with exact scoped substitutions: type/counter/method renames and observations [sum(signs), f_plus, f_minus]. The sum is mode0 because m divides h. It preserves source queries, radius limits, call cap, history replay, pullback, optional extra signs, eligibility fallback and final public point verification. Candidate pruning is different, so equality of routes or source-call totals is not asserted. It is an optional path, not a default policy switch.

## Cost obligations

For L coefficients and s!=1, the pair evaluator consumes L coefficients and performs 4(L-1) quadratic products for its two balanced value/power recursions, then one product to combine evaluations. It also computes and checks one scalar inverse of s per active branch evaluation. The current implementation does not share that inverse across branches. For s=1, existing one-mode evaluation and norm apply. Interval-product setup, stride inversions, coefficient-buffer allocations, recursive quotient allocation/copies, mode0, canonical filtering, lifting, replay, source queries and point checks still count. One stream per active branch is not one stream for the entire recovery.

No matched timing or total-work result is inferred from these correctness controls. Next compare candidate retention and full work with the current mode2 path under one frozen protocol; the pre-existing width-panel draft binds older sources and must not be reused as though it covers this adapter.

## Sources and proof map

- ../feature-native-signed-reciprocal/DERIVATION.md and Norm.lean: signed product and conditional norm.
- Predecessor canonical-image.rs, canonical-adapter.rs, canonical.rs, quadratic.rs and persistent.rs: orientation-preserving state, lift, public replay and quotient operations.
- NIST DLMF18.5.1 https://dlmf.nist.gov/18.5.E1, reviewed in T1896: standard reciprocal polynomial representation. No novelty claim.
- KB unavailable: MCP tools absent; scripts/research-kb status reports no executable. Exact source fallback used.
- Transport.lean, Lean4.34.1 (commit5045d0056413266e57c625dcd7c365b10e377c52; bundled Std only), proves `SignedTransport.inverse_pair` and `SignedTransport.transported_trace` over arbitrary commutative rings with explicit inverse/trace assumptions. Axioms: propext, Classical.choice, Quot.sound. No sorry/custom axioms; separate lean4checker replay not run.
- Full finite-field weighted projection, stream implementation and recovery refinement are not formalized by these lemmas. Their written derivation and finite controls remain distinct evidence.
