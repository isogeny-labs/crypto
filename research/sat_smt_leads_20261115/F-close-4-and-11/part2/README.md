# Part 2 — close-out of idea 11 (choosing V to maximise the early Hilbert/defect)

Pilot, not claim data. Toy sizes, one core under `nice` on a shared 4-core box, 2026-11-15. Every
number is an **unverified pilot assertion**, tagged `[pcc:unverified]`. No pcc claim, benchmark
record, leaderboard row, isolab run, replay certificate or matched-rho arm. Protocol frozen before
the runs: `PROTOCOL.md`.

## Questions

(a) Is `w(V) = dim(V.V) = 2l-1` (the polynomial-span value the linear 2+1 baseline already uses)
the *minimum* over all l-dim F2-subspaces V of F_{2^n} for prime n? (b) Does a structured
(polynomial-span) V lower SAT solve cost vs random V beyond the A/A floor, with >=16 bases per arm?
(c) The same contrast for the linear 2+1 scan's leaf count and candidates-per-scan, where
yesterday's only separation appeared. (The 2026-11-14 pilot ended "kill"/"inconclusive": no search
went below 2l-1 but the claim rested on a cited bound, and the SAT arms had a 1.6-1.9x A/A floor
with only 4 bases.)

## (a) 2l-1 is the minimum — exhaustive (up to scaling) + argument

**Scaling reduction (argument):** `w` is invariant under `V -> cV`, so every V is equivalent to one
containing 1; enumerating l-dim subspaces that contain 1 is therefore exhaustive for the minimum.

**Computation** (`run_part2.py defect`, `defect_min.json`): in every covered cell the minimum of
`w` equals `2l-1`, with **no V below it**. `[pcc:unverified]`

| cell | coverage | min w found | 2l-1 |
|---|---|---|---|
| l=2, n=3,5,7,11,13 | exhaustive (3..4095 subspaces) | 3 | 3 |
| l=3, n=7 | exhaustive (651 subspaces ∋1) | 5 | 5 |
| l=3, n=11 | exhaustive (174251 subspaces ∋1) | 5 | 5 |
| l=4, n=7 | exhaustive (1395 subspaces ∋1) | 7 | 7 |
| l=3,n=13; l=4,n=11,13; l=5,n=17; l=6,n=19 | 5000 random + poly/Frob/trace0 | = 2l-1 (poly); random_min ≥ 2l-1 | 2l-1 |

Positive control: subfields of F_{2^15} give `w = l` (3 and 5) against random medians 6 and 14, so
the statistic detects genuine low-defect structure when it exists — and finds none below 2l-1 for
prime n.

**Argument / reference (clearly not re-derived here):** the F2-linear Cauchy-Davenport analogue
(Hou-Leung-Xiang, *J. Number Theory* 97 (2002); Bachoc-Serra-Zemor) gives `dim(V.V) >= 2 dim V - 1`
when the field has no subfield of intermediate degree, in particular for prime n. The computation
above confirms it and its tightness in the covered cells.

**(a) result: holds in every covered cell — there is no V below the polynomial span to synthesise.**

## (c) Linear 2+1 scan: leaves vs candidates `[pcc:unverified]`

16 S (polynomial-span) + 16 A + 16 B (random) bases, 8 shared targets, non-decomposable
(decomposable similar; `results.json`). Ratio S/A with A/B as the A/A floor; permutation p on
per-base geometric means.

| unit | l | S/A (95% CI) | A/B floor (95% CI) | perm S-vs-A |
|---|---|---|---|---|
| leaves | 3 | 1.162 | 0.969 | 0.125 |
| leaves | 4 | 0.964 | 1.281 | 0.641 |
| leaves | 5 | 1.010 | 0.941 | 0.871 |
| candidates | 3 | 0.675 [0.53, 0.87] | 1.116 [0.91, 1.39] | 0.006 |
| candidates | 4 | **0.100 [0.083, 0.120]** | 1.496 [1.20, 1.91] | 0.000 |
| candidates | 5 | **0.016 [0.014, 0.020]** | 0.966 [0.83, 1.12] | 0.000 |

- Leaf count is base-independent (S/A ≈ 1, inside the A/A floor), as predicted.
- Candidates-per-scan separate massively: the polynomial span yields ~1 candidate/leaf while random
  V yields exponentially more (S/A 0.10 at l=4, 0.016 at l=5), far outside the A/A floor, p=0.000.
- **This is exactly the mechanism the repository's linear 2+1 baseline already exploits** (the S
  arm *is* that baseline's own base, w = 2l-1). Per the protocol it is the pre-registered "known
  mechanism, nothing new to synthesise" — reported for completeness, not a new result.

## (b) SAT cost structured vs random — **underpowered (not achieved)** `[pcc:unverified]`

Target: l=4,5, >=16 bases/arm, >=8 targets/base. Under box contention (load ~7 on 4 cores, each
SAT solve ~3-25 s) only **1-2 complete bases per arm at l=4** were obtained before the time cap;
l=5 SAT was not attempted (planned contingency). The partial (l=4, non-decomposable): S/A 0.815,
A/B 1.126 with 1-2 bases — **too few for a comparison or A/A floor**. It neither shows nor excludes
a 2x effect. This leaves (b) exactly where the 2026-11-14 run left it: S/A inside a wide A/A floor
(SAT3 S/A 0.65-1.73 vs A/B 0.67-1.64 at l=3,4 there), i.e. no S-vs-random SAT difference beyond a
floor that could not be narrowed here.

## Decision (pre-registered rule)

Rule: **nothing to synthesise** if (a) holds everywhere AND (b) shows no S-vs-random difference
outside the A/A 95% range; **alive** if a V below 2l-1 exists OR structured V lowers SAT conflicts
≥2x outside A/A; else **inconclusive**.

- **(a) is decisive: no V below 2l-1 exists in any covered cell (exhaustive up to scaling at l=2,
  n≤13; l=3, n≤11; l=4, n=7; sampled elsewhere), so the "alive via (a)" path is closed.** The best
  base is the polynomial span the baseline already uses.
- **(c)** confirms the only separation is that known baseline mechanism (candidates-per-leaf), not a
  new synthesizable structure.
- **(b) could not be powered** on the contended box; on the evidence available (here + yesterday)
  there is no SAT difference outside the A/A floor, but the ≥16-base strengthening the skeptic asked
  for was not achieved.
- **Verdict: "nothing to synthesise" for the structural/defect claim (a is decisive); the SAT
  sub-clause (b) remains underpowered.** Not "alive". Reported honestly as a negative on the core
  question with one sub-clause unresolved.

## Not covered / deviations

- (a) exhaustive omitted for l=3,n≥13 and all l≥4,n≥11 (10^7+ Python subspaces): covered by the
  5000-sample + the HLX/BSZ argument instead.
- (b) SAT: the ≥16-base / l=4,5 design was not reached (contention); only 1-2 complete bases/arm at
  l=4. SAT per-cube capped at 200000 conflicts. l=5 SAT not run.
- Permutation tests (20000 relabels) and bootstrap CIs are reported for (c); (b) has too few bases
  for either.
- One curve (a=0,b=1), one solver (CaDiCaL), defect defined on V.V and the m=2 leaf system (not a
  descended S4), as yesterday.

## Rerun (smallest cells)

```
cd research/sat_smt_leads_20261115/F-close-4-and-11/part2
python3 run_part2.py defect        # part (a): exhaustive min of dim(V.V) + samples (~15 s)
python3 run_part2.py l21           # part (c): L21 leaves/candidates, 16 bases/arm (~10 s)
python3 run_part2.py sat 4 13      # part (b): SAT l=4 (slow; interleaved S,A,B, cap externally)
python3 analyze.py && python3 figure.py
```
