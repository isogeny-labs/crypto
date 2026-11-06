# Counted-S results, protocol v2 (DEC-20261031-77e2b3)

Status: **unvalidated** (no VALIDATION.json, no independent validator yet). Not a leaderboard or benchmarks/ row.
Source: `decision.json` (sha256 eb5d791a…), produced by the frozen `analysis/decide.py`. S3: `s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/v2/`.

## Integrity
- `ecbench verify --replay-all`: C-10-14 (96/96), C-16 (32/32), B (288 records), 0 problems. A/A determinism: 0 mismatches on C-aa, B-aa-rho, B-aa-selectors.
- Rebuilt binaries are NOT bit-identical to those recorded in the original C-10-14 (679cdc2f…) and B (a133714d…) sessions; replays reproduce the counts.

## Track C: whole-point tensor IC (ic.tensor_contraction_v1, "tensor-m2") vs rho.negation, counted S
Ratio S(tensor)/S(rho), at 0 / 0.1 / 1 GAE per unpriced op. Order holds across the whole band on every curve.
| curve | S ratio vs rho | vs ic-mitm |
|---|---|---|
| fp10 | 4.3e3 / 5.0e3 / 9.5e3 | 259 / 304 / 630 |
| fp12 | 2.7e4 / 3.1e4 / 5.0e4 | 3.0e3 / 3.5e3 / 7.2e3 |
| fp14 | 1.7e5 / 1.9e5 / 3.0e5 | 2.1e4 / 2.4e4 / 4.6e4 |
| fp16 | 6.7e5 / 7.6e5 / 1.2e6 | 1.2e5 / 1.4e5 / 2.2e5 |
The gap to rho widens roughly 4x per 2 bits of group size (exponential toy, as predicted by H1). Distance remaining to matched rho: 3.6 to 6.1 orders of magnitude, growing.

## Track B: native hashed arms (rho-neg reference; native S is a lower bound)
- complete cells: native-selectors fp10 (1.3e3 / 1.5e3 / 2.4e3 x rho), native-partial fp10 (5e2 / 5.5e2 / 9e2) and fp12 (1.8e3 / 1.9e3 / 2.8e3); ic-mitm all four curves (14.8 down to 5.2 x rho).
- all other native cells are incomplete (exhausted executions kept as unknown; 110 of 288 exhausted); no verdict, no ratio claimed. native-source and native-width never produced a verified pair.
- P1 and P4 supported_within_scope per decide.py. P2/P3 are not emitted by the frozen script.

## Scope / caveats
- Finite-size fallback only (lambda<=64, root_u=0): the recursive subexponential kernel was never exercised. Nothing here says anything about the asymptotic claim.
- Counted units only; no wall-time claim. Exhausted cells are unknown, not failures of the method.
- Verdict: no tested arm beats rho at any size; measured improvement over the IC baseline: none.
