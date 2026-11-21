# PROTOCOL — Part 2 close-out of idea 11 (choosing V to maximise early defect)

Written 2026-11-15 **before** any measured run of this directory. Controlled rerun of the
2026-11-14 idea-11 pilot (`research/sat_smt_leads_20261114/11-smt-factor-base-synthesis`), which
ended "kill" by the producer and "inconclusive" after the skeptic because (i) no search arm went
below w = dim(V.V) = 2l-1 (the polynomial-basis span the linear baseline already uses) and the
"nothing lower exists" claim rested on a cited bound, not a search; and (ii) the SAT arms could
not show "no difference" — the A/A floor was a factor 1.6-1.9 with only 4 bases per arm.
Pilot standing only: toy sizes, one core under `nice`, no isolab, no replay certificate,
no matched-rho arm. Every number is `[pcc:unverified]`. Field/curve/subspace/L21/CNF code is
yesterday's `fb.py`, unchanged.

## Setting

m=3 point decomposition, curve y^2+xy = x^3+1 (a=0,b=1), factor base x(P) in an l-dim F2-subspace
V of F_{2^n}. Primary defect statistic w(V) = dim_F2 span{v_i v_j : i<=j} (lower = larger defect).
The polynomial span c<1,t,...,t^{l-1}> has w = 2l-1 (products are c^2 t^k, k=0..2l-2); the linear
2+1 scan (`linear21.rs`) already uses this base. Reference baseline named for context only: the
linear 2+1 scan at 2^l leaf solves.

## (a) Is 2l-1 the minimum of dim(V.V)?

**Scaling reduction (argument, used to make enumeration exhaustive):** for any nonzero c,
dim((cV).(cV)) = dim(c^2 (V.V)) = dim(V.V), so w is invariant under V -> cV. Every nonzero V
contains some v != 0; v^{-1}V then contains 1 and has the same w. Hence the minimum of w over all
l-dim V equals the minimum over l-dim V that contain 1, and enumerating the latter is exhaustive
for the minimum.

**Computation:** for each covered cell, enumerate every l-dim F2-subspace V containing 1
(canonicalised by reduced echelon basis, deduplicated) and record the histogram of w; report
min w and whether any V has w < 2l-1. Covered cells (prime n):
l=2: n = 3,5,7,11,13; l=3: n = 7,11,13; l=4: n = 7 (exhaustive). Larger cells where full
enumeration does not fit the budget — (4,11),(4,13),(5,17) — are covered by a large random +
structured sample (>=5000 random V, plus poly/Frobenius/trace-zero families) reporting min w only.

**Argument / reference (clearly not re-derived here):** the F2-linear analogue of the
Cauchy-Davenport theorem (Hou, Leung, Xiang, *J. Number Theory* 97 (2002); Bachoc-Serra-Zemor)
gives dim(V.V) >= 2 dim V - 1 whenever the ground field has no subfield of degree between 1 and n
exclusive, in particular for prime n (with 2l-1 <= n). This is the source of the 2l-1 lower bound;
the computation above confirms it and its tightness in the covered cells. Marked as argument, not
as an independent proof.

**Decision input (a):** "holds" in a cell iff no enumerated/sampled V has w < 2l-1.

## (b) SAT cost: structured (polynomial-span) V vs random V

Cells l=4 (n=13) and l=5 (n=17), m=3, chained CNF `cnf_m3`, CaDiCaL 1.5.3 (pysat), conflicts
primary. Arms per cell: S = 16 polynomial-span bases, A = 16 random bases, B = 16 random bases
(A/A control, A vs B). >=8 targets per base: 8 shared random curve-point targets per cell, solved
against every base; ground-truth decomposability (from `l21`) splits each (base,target) into
decomposable / non-decomposable, reported separately. A single cube solve is capped at a
200000-conflict budget (`solve_limited`); censored instances are excluded from the geometric mean
and counted. l=5 SAT is run only if a probe instance finishes under ~30 s and the cell fits the
budget; otherwise l=5 SAT is omitted and stated (L21 still covers l=5).

## (c) Linear 2+1 scan: leaf count and candidates per scan

Same S/A/B bases and shared targets, cells l=4 (n=13) and l=5 (n=17) (and l=3 (n=11) for a third
size). Units: `leaves` (= 2|factor base|) and `cands` (sum over leaves of the (s,p) solution-set
size handed to the split), plus the fraction of leaves decided with <=1 candidate. This is where
yesterday's only separation appeared, so it is measured with the same >=16 bases / >=8 targets.

## Metric and statistics

Per arm, per cell, per unit, split decomposable / non-decomposable: geometric mean of the unit
over all (base,target) instances; and the per-base geometric mean (16 values per arm). Ratio S/A
with A/B as the A/A noise floor. **Permutation test** on the base-level geometric means:
difference of mean(log per-base gm) between two arms, 2-sided p from 20000 random label swaps of
the 32 base values (S vs A, and A vs B for the floor). Slopes = least squares of log2(geomean) on l.

## Decision rule (pre-registered, from the task card)

- **nothing to synthesise** if (a) holds in every covered cell (no V below the polynomial span)
  AND (b) shows no S-vs-random difference outside the A/A 95% range;
- **alive** if a V below the polynomial span exists, OR structured V lowers SAT conflicts by >= 2x
  outside the A/A range;
- else **inconclusive**.
(The L21 candidate contrast (c) is reported for completeness; per the task it is the known
mechanism the baseline already uses and is not itself a synthesis result.)

## Validation before measured runs (smallest cell)

`validate.py` from yesterday is re-run (field/curve identities; at (l,n)=(3,11) brute-force triple
enumeration vs L21 and SAT3 agree, every SAT model and L21 solution lifts to curve points summing
to +-R, x=0/2-torsion handled by the t!=0 clause; subfield positive control w=l at n=15).
w = 2l-1 for the polynomial span is checked directly in every cell.

## Budget / deviation

One process, one core, `nice`. (a) and (c) are cheap. (b) SAT at l=5 is run only if it fits;
any omission or base/target reduction is recorded in results.json and stated in README.md.
