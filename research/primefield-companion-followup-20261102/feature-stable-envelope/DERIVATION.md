# Minimal stable envelopes for orbit replay

2026-11-07, T-1818. Written algebraic construction and deterministic native checks. No total-cost comparison or independent signed verdict.

## Context and assumptions

Read prior feature-orbit-replay/NEXT_ENVELOPE.md, DERIVATION.md, orbit.rs and the native polynomial gcd/division backend. KB MCP unavailable. Official fallback: [FLINT nmod_poly index and conventions](https://flintlib.org/doc/nmod_poly/index.html). The main index was accessible; a section click failed. Polynomial lcm, substitution and invariant closure are standard operations; no novelty claim is made for them. This component supplies a capped construction and integrates it with the existing replay adapter.

Work over caller-certified prime F_r, r>=7; m>=2 divides r-1 and zeta has exact order m. Let monic G divide the stable monic parent L. In the recovery path L is squarefree and splits over F_r. A monic rotation of A is T_z(A)=z^(-deg A) A(zX). Its roots are the inverse-z translate of those of A; a full cyclic orbit is independent of that orientation.

## Doubling construction and minimality

Define H0=G. If Ht is already fixed by T_zeta, stop. Otherwise set

    H_(t+1)=lcm(H_t,T_(zeta^(2^t))(H_t)).

Before early stopping, Ht is the monic lcm of rotations with exponent indices0..2^t-1. Rotation commutes with monic lcm, so the next step covers0..2^(t+1)-1. Once2^t>=m the interval contains a complete period. Therefore at most ceil(log2 m) lcm stages produce

    H=lcm_(j=0..m-1) T_(zeta^j)(G).

Every stable monic multiple of G contains each rotation and hence H. Thus H is the unique minimal stable monic multiple of G. Early stopping is valid for the same reason: a stable intermediate contains G, while every intermediate divides H. The constant polynomial1 is fixed immediately. If G contains X, rotation leaves that root and its multiplicity unchanged; no special division at zero is needed here because the preceding pullback already corrected zero multiplicity.

Since G divides stable L, every rotation of G divides L. Every intermediate and final envelope therefore divides L; degree never exceeds N=degree(L). This invariant is checked after each implemented lcm, not inferred from an allocation estimate alone.

## Capped implementation

Check canonical coefficients, monicity, exact root order, parent stability and divisibility G|L. The input G degree and each prospective lcm degree are checked against the requested output cap. Compute D=gcd(A,B), calculate degree(A)+degree(B)-degree(D) before multiplication, divide B exactly by D, then multiply A*(B/D). This avoids first allocating the unreduced product A*B. Reject a nonzero division remainder or failed parent divisibility. The parent is an existing input: an output cap does not make parent validation/free-standing polynomial division cost disappear.

A conservative closure cost is O(m + (M(N)log(N+1)+log r)log(m+1)) field operations for this implementation, plus backend inversion accounting. The O(m) term includes repeated exact-order checking. The log r term includes exponentiation-based monic rotation normalization; gcd scalar inversions use the existing Euclidean backend. Logarithmically many gcd/division stages, rotation and validation are all charged. Divisibility checks and modular inversions are explicitly retained. Early stability exits can avoid stages but require their own coefficient check. O(N) coefficient arrays coexist with the installed gcd engine's workspace. Degree profiles are structural diagnostics, not a substitute for complete operation or time accounting.

## Dynamic replay

Start with the ordinary target-sign refinement. Before every complete retained orbit, build the minimal stable envelope of the current candidate polynomial inside original L. Compute one character residue modulo that envelope; coefficient rotations supply the other orbit residues. Current candidates divide the envelope, so the previous residue-transport proof applies and every gcd refinement agrees with fixed-parent replay. The candidate state is never enlarged to the envelope: only the modulus for computing residues changes.

Retained order, query schedule, budgets, direct-phase route, additional-sign path and public-point verification are unchanged. Dynamic envelope construction is an optional path beside fixed-parent and ordinary replay. The code retains upfront validation and per-block closure checks; neither is hidden in the profile of lcm stages. Envelope rebuilding may cost more than the exponentiation work it saves, especially when no whole image fiber disappears.

## Validation design

Validation01 passed and is preserved, with its exact envelope.rs, before adding early stable exits, an m=0 guard and exceptional-history checks. Validation02 is final. generate.py binds the preserved earlier implementation and generated dynamic adapter. validate.py compiles optimized native Rust with overflow checks, executes deterministic assertions and records dependency hashes and raw warnings/output.

For r7,11,13 and every eligible m, check every scalar subset against an independent root-set orbit-union reference. Compare the entire monic coefficient vector, not only evaluations. Check G|H, exact root count, intermediate degree bounds, stage bound and output cap failure below a strictly larger envelope. Include empty set, zero-containing sets, full closures and a non-divisor rejection. Reference enumeration is test-only.

Additional complete histories for every scalar in these fields include zero target/source values and compare dynamic replay to fixed replay. The inherited residue/prefix and pullback reference checks also remain active. The public fixture panel compares63 targets across the same four configurations, preserving all output routes and query fields. Exact counts and per-orbit degree profiles are retained in validation-02/check.stdout.

Original4563 cohort and source domain unchanged; no cloud execution, new timing study or full release rerun. Written minimality proof is not a full formalization, and producer checks are not a signed independent verdict.

## Next construction

The envelope must pay for its gcd/division/rotation work. Derive a degree-aware replay selector or a lower-cost image-presence certificate before a frozen full-cost comparison. Preserve unchanged-envelope cases as controls. Source setup and query cost remain separate from decoder improvements.

## Final exact results

47,488 exhaustive scalar subsets;23,744 contain zero;87,238 closure stages;47,196 intermediate output-cap rejections;16,935 envelopes equal the full parent (retained no-shrink controls).119 complete small-field histories match fixed replay, including zero values. All63 public fixtures match scalar/routes/query fields:511 source calls,78 retained powers. Across51 orbit blocks,26 envelope degrees decrease and25 remain unchanged;40 lcm stages are recorded for those blocks. Upfront validation and stability checks remain additional work.15 compiler-input hashes and generation hashes match.
