# Projective Semaev chain finite sweep — frozen protocol

Date: 2026-11-28. Draft PR #5365. This protocol is committed before the first run.
Standing: exploratory native finite correctness and work-count sweep only
[pcc:unverified]. It does not measure F4, a PDP oracle, relation rank, or an
end-to-end discrete-logarithm speedup.

## Question and prior work

For genuine `E(F_p)` relations, does adding the identity chart to an
`F_p`-projective S3 chain recover the exact finite target set across
Kummer factor bases as `m` and tower depth vary? How often does the
affine-only chain miss a valid target? Count the explicit S3 and reference
group operations under a cold per-target rule.

The construction and exceptions are in
`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` §§2.2, 18,
`summation_split/FINDINGS.md` (projective/Fp2 polynomial-root projection),
and `summation_split/SECTOR_COMPLETE.md` (full-sector circuit). The cold
m=7 left-chain limitation is in
`research/pplus_dickson_m7_elimination_20261127/REPORT.md`.
Primary context: Semaev, ePrint 2004/031 (recursive summation
polynomials), and Petit–Kosters–Messeng, PKC 2016 (composed factor-base
maps). This sweep makes no novelty claim. Research-KB and Conductor tools
were unavailable in the selected environment; the cited repository
sources were the fallback.

## Frozen public panel

All choices are deterministic; there is no PRNG, master seed, reroll,
instance selection after outcomes, or timing-based inclusion rule.
Seed manifest: `deterministic-enumeration/v1`, random streams: none.

- Curves: `y²=x³+2x+3` over `p ∈ {97,193,257}` (all nonsingular).
- Kummer depths: `t ∈ {2,3,4}`; `2^t | p−1` in every cell.
- Summands: `m ∈ {4,5,6,7}`. Total: 36 cells.
- Base: `V=2·μ_(2^t)`. Choose the least integer `h≥2` for which
  `h^((p−1)/2^t)` has exact order `2^t`; enumerate its powers.
  Keep only abscissae with a rational curve lift; include both signs
  except a zero ordinate.
- Targets per cell, in order: the first three liftable abscissae in
  `[0,p)` outside `V`, then `x([m−2]P)`, where `P` is the first
  factor-base point in ascending `x,y` order with `[m−2]P≠O`.
  This last target has the planted witness `P,−P,P,…,P`
  (m leaves total). If no such `P` exists, record a censored target
  with the reason and retain the cell. Target construction is evaluator
  work and is not supplied to either decision arm.
- Do not substitute parameters or discard misses, invalid targets,
  errors or censored rows.

## Paired cold arms and exactness

For every target and arm, independently rebuild the Kummer base and
curve-lift mask. No table, state, symbolic preparation, trace, or solver
output is reused across targets.

1. Projective arm: propagate reachable partial-sum abscissae in
   `P¹(F_p)`. A finite-to-finite transition scans `v∈F_p` and tests
   `S3(u,x,v)=0`. Add `v=∞` if `u=x`; from `u=∞`, return `v=x`.
   Retain the identity chart at every depth.
2. Affine control: the same finite transitions, dropping every
   infinity state and its outgoing transitions.
3. Independent reference: propagate signed curve points using a
   separate affine group law and test the final `x` against the target.

The reference and projective verdicts must agree on all uncensored
targets. Any mismatch, arithmetic error, or overflow is a retained
failure and stops the sweep. The affine control may miss; it must never
be used as the truth oracle. Record positive and negative targets,
the per-step reachable state counts, infinity reached, S3 evaluations,
identity-chart transitions, base candidate checks and reference
group-addition calls. A replay of the full deterministic sweep must
match every non-timing output byte for byte.

## Limits and execution

This is a finite-root propagation diagnostic, not an F4 experiment.
Its S3 counts do not model Gröbner matrix work or imply a
`p^{o(1)}` bound. The Fp-projective sector is exact for genuine
rational-point witnesses, but not for every formal `S_(m+1)`
root on arbitrary `F_p` abscissae: the latter may require Fp2 charts.
No raw wall-time comparison or speed claim is admitted.

Use a native implementation; no Python research code. Rust is the repo
default, but the selected environment has no Rust toolchain or checkout.
The bounded local fallback uses GCC C and will be labeled L0 diagnostic.
EKS Spot and the repository's native F4 backend are unavailable here.
Compile with warnings and sanitizers for a separate verification build,
retain source and executable hashes, command, raw rows, host/compiler
metadata and any failures. If the native implementation or exactness
check cannot be completed, report the blocker instead of a result.

## Post-primary addendum: fixed-leaf identity stress panel

Frozen before this second run. The primary 144 target cases all matched the
reference, and the affine arm had no target-verdict misses despite reaching
infinity in every cell. This separately labeled diagnostic deliberately fixes
leaf order to test that chart. It is an adaptive follow-up, not part of the
original 144-case panel or a fresh validation set.

Use the same 36 `(p,t,m)` cells. In each cell choose `P` as the first
factor-base point in ascending `x,y` order and `Q` as the first point
with a *different* abscissa. Fix the ordered leaf abscissae to
`(x(P),x(P),x(Q),…,x(Q))`, with `m−2` copies of `x(Q)`.
The target is `x([m−2]Q)`, planted by the signed witness
`P,−P,Q,…,Q`. If fewer than two lifted abscissae exist or the target
is infinity, retain a censored row with a reason. There is no search for
a pair giving an affine miss, no replacement cell, and no reroll.

Run the same `F_p`-projective and affine S3 root propagation on the
fixed ordered leaves, and an independent signed group-addition reference.
Rebuild the base independently for each arm and cell. Record the
projective/affine/reference verdicts, infinity-step mask, S3 counts,
and group-addition count. Any projective/reference mismatch stops the
run and is retained. A separate sanitizer build and byte-identical
deterministic replay check are required. No wall-time or F4 claim follows.
