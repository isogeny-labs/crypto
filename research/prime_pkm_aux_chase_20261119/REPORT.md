# Prime PKM / T₂-torus chase — smoke report

**Date:** 2026-11-19 (America/Los_Angeles)  
**Branch:** `cursor/prime-pkm-aux-tower-3858` ([crypto#4820](https://github.com/isogeny-labs/crypto/pull/4820))  
**Standing:** all numbers below are `[pcc:unverified]` machine measurements from a local release binary.  
**Coverage:** Full discriminating numbers for P0-1c / P1-3a are in `RUN_SMOKE.log` (includes M31). A machine-readable snapshot is `RESULTS_P0_P1.json` (fast-smoke shape: bits≥16 and M31 omitted; P2-5 marked `smoke-skipped`). A full `--suites all` re-run may still be in flight and will replace `RESULTS.json` when it finishes.

## Verdict (discriminating)

| Claim (workspace#28 / ACTIVE) | Smoke result |
| --- | --- |
| ParamS5 specialization ≫ grid interp | **Hold.** 264×–832× fewer Fp mults/target; 0 mismatches on alg-k20 and M31, m=2..4. |
| Trace+inherit+descent ≥20× S(w≈0.1) on alg rungs / M31 | **Miss.** Best algebraic-rung S(0.1) ≈ **2.05×** (k=16). Wall clocks look larger (≈30×) because Before pays a slower path, but priced S does not clear 20×. |
| Descent kills `grid()` | **Mixed.** Algebraic m=2 rungs improve; several Dickson m=3 rows recorded **negative** (“descent did not cut grid”). |
| m=8 KM3-SPEC-LADDER under Lean growth floor | **Does not kill** under all-/one-output floors (c=1/14 Kummer; c=1/28 T₂ Cayley). Kill only under sharp LFD / direct leading-ideal assumptions (c≈1). |

## P0-1c highlights

### ParamS5 audit
- alg-k20 / M31, m=2,3,4: mismatches=0 throughout.
- Spec vs grid: ParamS3 ≈264×, ParamS4 ≈668×, ParamS5 ≈832×.

### Algebraic rungs (after = trace+inherit+descent)
| bits | S(0.1) speedup | wall speedup | verified |
| --- | --- | --- | --- |
| 12 | 1.16–1.17× | ≈3× | yes |
| 14 | 1.63× | ≈30–34× | yes |
| 16 | 2.05× | ≈30× | yes |
| 12 m=3 mode=sym | **0.93×** (regression) | 0.93× | yes |

### Dickson / T₂
- M13 m=4 sym: ≈2.5× ops / 4.5× wall.
- M31 m=3 chain: ≈2.0× ops / **3.23×** S(0.1) — best torus S seen; still ≪20×.
- M31 m=5 chain: ≈1.02× ops (flat).
- Negative grid findings on M17 m=3 and M31 m=3 chain.

## P1-3a (m=8 desk ladder)

Lean `rational_prime_growth` / Cayley half-degree transfer: `kill_all=false`, `kill_one=false` at w=1 and w=2.  
Sharp LFD ≥ t (c=1.0) and direct `y_j² → y_{j+1}+2` (c=1.0): kill under both floors.  
Interpretation: the m≥8 sub-ρ story remains **conditional on degree slopes that Lean growth does not currently justify**.

## P2-5 / Panel A

Not completed in this smoke (process SIGTERM during P2-5). Re-run with suite flags once available, or overnight without a kill budget.

## Next actions

1. Treat ≥20× S(w) as **not yet demonstrated** on alg-k12..16 / M31 with current After flags; do not claim ratchet win.
2. Debug Dickson m=3 “descent did not cut grid” negatives (basis root descent gap vs grid fallback).
3. Keep m=8 work focused on proving a sharper degree slope (or closing the lead).
4. Finish P2-5 / Panel A under a long-lived runner; attach full `RESULTS.json`.
