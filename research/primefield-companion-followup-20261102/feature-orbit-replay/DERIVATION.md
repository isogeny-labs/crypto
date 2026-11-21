# Orbit-equivariant character residues

2026-11-07, T-1815. Written derivation and deterministic native producer checks; no timing result or independent signed verdict.

## Prior work and assumptions

Read feature-mode-set-pullback/{DERIVATION.md,pullback.rs}, the weighted-mode derivation, existing polynomial backend and CONTINUATION source/replay notes. KB MCP unavailable; fallback official reference: [FLINT nmod_poly conventions](https://flintlib.org/doc/nmod_poly/index.html) for composition, modular arithmetic and inflation. The underlying quotient-ring substitution is standard algebra; no literature-first claim is made. The implementation uses the existing native backend.

Let r>=7 be a caller-certified prime, h=(r-1)/2, m>=2 divide r-1, and zeta have exact order m. Let L be the monic squarefree lifted polynomial from the prior component, whose roots are a union of complete multiplicative zeta-orbits, optionally including zero. For every j, L(zeta^j X) is a nonzero scalar times L(X). This remains true after the prior zero-multiplicity correction: exponents of L are all0 mod m without a zero root and all1 mod m with a zero root.

## Residue transport

Define R_a(X)=(X+a)^h mod L(X), with degree less than N=degree(L). Since substitution X->zeta^(-j)X preserves the ideal (L),

    R_(a*zeta^j)(X) = zeta^(jh) R_a(zeta^(-j) X).

The right side already has degree below N, so this is equality of canonical remainders, not merely equality on tested points. Its coefficient of X^i is scaled by zeta^(j(h-i)). The character factor zeta^(jh) is necessary when d=(r-1)/m is odd. At zero roots and source endpoints, Euler's criterion still uses chi(0)=0; no division by a source value occurs.

For one orbit compute R_a with one modular power, then transport its coefficients. Precompute factors zeta^(h-i) with one inverse and one character power; advance to each next orbit member with coefficient-wise multiplication. Initial target offset0 uses one separate modular power. Every supplied offset block is checked for exact orbit order and nonzero starting offset. The full modulus is checked for canonical coefficients, monicity, cap and scaling stability before residue transport. Caller certifies r; root order is verified.

## Correctness after refinement

Retained signs refine a sequence G_0=L, G_(t+1)=gcd(G_t,R_a-s). Every G_t divides L, so the fixed-modulus residue remains a valid residue modulo G_t. Hence the candidate root set after each prefix equals ordinary replay using (X+a)^h mod G_t. The code never assumes G_t retains orbit stability. This distinction is essential: generally it does not.

The adapter retains the prior sign order, full history, image transition, additional-query schedule, budgets and final point verification. Only the retained replay computation changes. Scalar labels remain test-only. Original4563 cohort, public source domain and cloud policy are unchanged.

## Costs and tradeoff

For q retained complete orbits, initial lifted degree N and one target sign, the new route uses q+1 modular character powers at the original modulus (apart from the initial ordinary sign step), plus O(q*m*N) coefficient updates and q*m+1 gcd constraints. Factor preparation costs O(N+log r) field operations. Gcd arguments are bounded by degree N; shrinking moduli can reduce subsequent gcd work. Storage adds O(N) words for original modulus, current residue and rotation factors; no q*m*N residue table is materialized. Source calls do not change.

The reference computes q*m+1 character powers but on shrinking candidate degrees. Therefore the reduction in exponentiation count is not a total field-operation or elapsed-time comparison: one fixed-degree exponentiation can cost more than several late small-degree exponentiations. Keep both paths available until an explicitly frozen cost comparison determines a selection rule. Extra ordinary offset queries still use the original replay path.

## Checks

Validation01 passed, then its orbit.rs and outputs were snapshotted before replacing per-rotation inversions by fixed coefficient factors and adding malformed-input controls. Validation02 is final. The validator pins compiler inputs and stores raw warnings/output. generate.py/generation.json bind the exact previous implementation, preserved reference and generated adapter.

On the same small image subsets as the previous pullback checks:5,838 exact transported coefficient vectors equal direct modular exponentiation;5,838 refined prefixes equal ordinary replay.1,110 wrong-character-phase controls differ as expected;356 subsets include zero. Nonstable modulus, malformed orbit ordering and insufficient-cap controls reject. These checks use independent direct powering for the transported residue comparison, and prior exact root-set enumeration for the inherited pullback checks.

On all63 inherited public fixtures, both adapters recover the same scalar and route, with identical source-call, extra-query, lifted-degree, image-count and retained-count fields. Each candidate is independently checked by the public point equation. Each adapter uses511 source calls. The candidate records78 retained residue powers for180 retained constraints. These are deterministic algorithm-path counts, not a statistical or timing experiment. The preceding mode-only46/63 and pulled63/63 controls still run unchanged. No full-release rerun or full formalization is claimed.

## Next

A deterministic selection rule must account for the degree profile, coefficient rotations and gcds, rather than counting powers alone. One structural option is to rebuild a smaller orbit-stable envelope after whole image fibers disappear; that envelope construction has its own cost and is not implemented here. Source costs remain separate. Preserve both implementations for a future frozen comparison, rather than silently selecting the fused path universally.
