# Protocol: the hybrid optimum, a fully charged bracket, and F4's cost per unknown

**Registered 2026-10-18, before any measured run of this round.** Conductor task
`T-3`. Round 3 of the split/hybrid PDP study (round 1: PR #1709,
`research/pdp_split_dreg_20261011`; round 2: `research/pdp_split_slots_20261012`).

## What round 2's exploration showed (sizing, not results)

`research/pdp_split_slots_20261012/EXPLORATION.md`:

- Guessing 8 of the Boolean unknowns instead of 4 cut F4's XOR count per
  target 10–128× at ℓ = 4–5, every branch charged. The round-2 arms
  (g = 4) were far from the hybrid optimum.
- On the closure, F4's XORs per branch grow by **2.7–8.7× per residual
  unknown** across k = 4…13 (n = 23, ℓ = 4…7), and the factor rises with
  k. Enumeration grows by 2× per unknown. In that regime the hybrid
  optimum is full enumeration, whose fully charged relative is the
  pair-table oracle `mitm-frobenius`.
- The closure of the dimension-4 subspace has yield at n = 17 where the
  subspace alone had none; ℓ = 3 is too sparse at n ≤ 19.

## Questions

- **Q1 (hybrid).** Does g = 8 beat g = 4 end to end, on both bases?
- **Q2 (bracket).** Is the fully charged pair table on the *same closure
  base* cheaper than the best F4 arm's priced S on every curve?
- **Q3 (exponent).** Is F4's growth per residual unknown above 2 at every
  ladder step with k ≥ 7, up to k = 16?
- **Q4 (invariant base).** At n = 15, does a Frobenius-invariant subspace
  of the same dimension give more decompositions per system than the
  closure, at the same per-system cost?

## Part A — ecbench (spec-only; the oracle is round 2's, unchanged)

Curves: `icv1-f2m13-t181-515ee569` (ℓ = 4), `icv1-f2m17-tm101-00378d4e`
(standard ℓ = 5, closure ℓ = 4), `icv1-f2m19-tm797-9c54981b` (ℓ = 4).
Eight public one-target workloads, one round, target seed `202610181101`,
measurement seed `202610181102`, L0 (counts only).

| arm | method |
|---|---|
| `rho-strong`, `rho-frob` | as in round 2 |
| `std-unsplit-g4`, `std-unsplit-g8` | `koblitz-standard-subspace` × `pdp-split:plan=3,slots=one` |
| `clo-unsplit-g4`, `clo-unsplit-g8` | `koblitz-closure` × `pdp-split:plan=3,slots=all` |
| `clo-mitm3` | `koblitz-closure` × `mitm-frobenius:m=3` — the fully charged bracket on the same base |
| `inv-mitm2` (n = 17 only) | `koblitz-orbit:divisor=0;1` × `mitm-frobenius:m=2` — the repository's 4.27× incumbent, re-measured on these targets |

Figures as in round 2: recorded S (F4 arms are lower bounds), priced S
at the pinned XOR ratio (a sensitivity that **overstates** the gain at
high g, since F4's per-call work is outside the elimination-only count),
and the table arms' recorded S, which is complete. Specs
`SPEC-A-n13.json`, `SPEC-A-n17.json`, `SPEC-A-n19.json`.

## Part B — the cost-per-unknown ladder (stage driver, closure, n = 23)

`K_0 / GF(2^23)` (`icv1-f2m23-t5197-69e76b73`), `--base closure`, plan `3`,
two targets, two slot tuples per target, seed `20261018`, `--max-free 24`,
`--budget-secs 120` per branch for ℓ ≤ 8 and `600` for ℓ = 9 (a timed-out
branch censors its run; it is reported, never dropped):

| series | cells | g | residual k |
|---|---|---|---|
| ℓ-series | ℓ = 4, 5, 6, 7, 8, 9 | 8 | 4, 7, 10, 13, 16, 19 |
| g-series | ℓ = 7 | 8, 10, 12, 14 | 13, 11, 9, 7 |

Per step, `pdp_split_analyze --ladder` reports mean XORs per branch and
the growth factor per residual unknown `(c₂/c₁)^{1/(k₂−k₁)}`. The
exploration's ℓ = 8 point took 7 min per target; ℓ = 9 is expected at
1–3 h per target and may censor.

- **P3′.** The growth per unknown on the k = 16 → 19 step is below the
  k = 13 → 16 step's (the trend of a bounded solving degree), and still
  at or above 2.

## Part C — invariant base at n = 15 (ecbench, spec-only)

`K_0 / GF(2^15)` (`icv1-f2m15-tm275-2d22ff5d`, r ≈ 2^9.6) is the only
Koblitz size ecbench builds that has Frobenius-invariant subspaces of
dimension 4 and 5 (2-cyclotomic cosets of 15 have sizes 1, 4, 4, 2, 4).
Four public targets, `max_trials = 2000` (the bases are tiny), seeds
`202610181301` / `202610181302`. `koblitz-orbit`'s divisor indexes the factorisation
`x^15 − 1 = (x+1)(x²+x+1)(x⁴+x+1)(x⁴+x³+1)(x⁴+x³+x²+x+1)`; `ecbench fb`
gives, for dimension 4, 1 / 31 / 21 points (indices 2 / 3 / 4) and, for
dimension 5, 3 / 33 / 43 points (`0;2` / `0;3` / `0;4`). Arms: `pdp-split`
unsplit at g = 4 and 8 on `koblitz-orbit:divisor=3` (dim 4, 31 points,
2 columns), `divisor=4` (dim 4, 21 points, 3 columns), `divisor=0;4`
(dim 5, 43 points, 6 columns), `koblitz-closure:dimension=4` (122 points,
5 columns), and the standard subspace at ℓ = 4 (11 points, 6 columns) and
5; `mitm-frobenius:m=3` on `divisor=0;4` and on the closure; both rho
arms. **rho is degenerate at this size** (its setup dominates), so
Part C compares systems per relation, F4 cost per system and recorded S
across bases, not S against rho. Spec `SPEC-C-n15.json`.

## Predictions (written first)

- **P1.** On every curve and base, `g8`'s median priced S is below 0.25×
  `g4`'s.
- **P2.** On every curve, `clo-mitm3`'s recorded S is below the cheapest
  F4 arm's priced S on the same curve (median over workloads).
- **P3.** Every ladder step with k ≥ 7 has growth per unknown ≥ 2.
- **P4.** At n = 15 and equal dimension, the invariant base's relations
  per system are at least the closure's, at XORs per system within 2×.
- **P5.** No F4 arm's median priced S is below strong rho's median S on
  its curve. **Falsification target:** any curve where one is.
- **P6.** Every ecbench run verifies; every stage run is complete and
  exact.

## Inadmissible after any measured run

Changing curves, dimensions, arms, seeds, guess counts, trial caps or
pricing; dropping censored or exhausted runs; quoting priced S as
recorded S or any figure as a speedup; claims about deployed curves,
asymptotics beyond k = 16, or ECC2K-130.

## Order and provenance

Round 3 runs after round 2's resumed runs finish (`run_after_round2.sh`
waits for `all done` in round 2's `progress.txt`), on the same release
binaries (`BINARIES.sha256` of round 2; this round adds only the
analyzer's `--ladder` mode, which reads records). The builds of the
analyzer and the exploration sweeps ran while round 2's resumed runs were
in progress; counts are unaffected, wall times are descriptive.
