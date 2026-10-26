# Generalized GHS search and full-cost controls (frozen before candidate runs)

Date: 2026-10-24. Revision: `v1`. Source base: `a53a7e2bfa20080a424b9a2484a16932b6e3c116`.

## Questions and candidate rules

1. Enumerate every proper subfield of each selected ordinary binary curve and every nonzero Hess parameter `c` when the field has at most 16 bits. For larger fields, record a finite, deterministic parameter budget and report its fraction of `2^N-1`. Compute exact order polynomials over `F_2`, the MMT descent gate, genus, and the subfield model. Candidate isogenies require explicit rational kernels and point maps; a modular-polynomial root alone is not eligible.
2. For every candidate reaching a map check, save the exact source modulus, `a,b`, subgroup order and generator, target point, descent parameter, curve equation, and the image of the generator and target. Check `[r]P=[r]Q=0`, `Φ(P)≠0`, `[r]Φ(P)=[r]Φ(Q)=0`, and known-answer additivity. The algebraic degree argument and Lean theorem supply the general group implication only when the geometric composition certificate is established.
3. Execute a complete destination attack with relation collection, matrix elimination, individual descent, recovery, and source scalar replay. Use the same previously unseen target for source rho. Charge construction, search, isogeny, map, factor base, relations, linear algebra, target descent, and verification to cold one-target work; show reusable setup separately. Preserve failures and exhausted budgets. A screen or a generic Jacobian DLP is not an index-calculus run.
4. For each exact candidate, check primary papers, public challenge lists, standards, and this repository before assigning `previously_reported`, `related_family_known`, or `prior_work_unknown`. `potentially_new_vulnerable` additionally requires a verified executable map and a measured full attack below matched rho across the required cost band. Otherwise report a structural lead with the failing gate.

## Controls and budgets

- Binary execution matrix: degree 6 with modulus `0x43`, degree 8 with `0x11b`, then degree 10/12/14/16 where the constructor and group operations work. Use a=0 and a=1 twists and non-subfield `b` values; both trace-killed and nonzero-image controls. The exact candidates are selected before timed solves and retained even on failure. Primary reference is signed-Frobenius rho when its automorphism acts on the exact subgroup; otherwise negation rho. The ordinary binary curves screened here are generally not Koblitz, so negation rho is expected.
- Seed = 20261024. One target per run. Development limit: 20,000 relation trials and 20,000 individual descent trials; a larger run requires a new protocol. Require independent scalar replay. Use field operation counts as the portable primitive unit, with field degree attached; addition, multiplication, square and inverse must all be emitted. The `0,0.1,1` GAE band prices memory/hash/table operations in both arms. On macOS, wall measurements are L0 descriptive and cannot support a runtime speed claim.
- For isogeny searches, degree set `{2,3,5,7}`, no invented codomain edge. Charge walk failures and kernel construction to search cost. For odd cubic curves, require a Frobenius-stable maximal isotropic kernel, a rational genus-three model, and an explicit quotient map before any attack run.
- Stop an individual run on a verified scalar, matrix rank exhaustion at the cap, or the stated trial cap. Do not substitute a different curve or target after observing a failure. Exact source and result hashes, CLI commands, status, and host manifest go into each receipt.

## Evidence and interpretation

The primary cold comparison uses identical source group, generator, target, host and accounting period. Report per-phase work, `S = cost/sqrt(r)`, setup plus online total, a matched rho row, unpriced counters on both arms, and the three-weight band. A measured ratio is quoted only when both answers verify and every cost is in one calibrated unit. Retain null totals where a backend cannot expose the required counters. A component or bounded run is reported at its actual scope.
