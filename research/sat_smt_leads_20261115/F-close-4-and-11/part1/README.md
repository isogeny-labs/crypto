# Part 1 — close-out of idea 4 (XOR cubes vs literal cubes)

Pilot, not claim data. Toy sizes, one core under `nice` on a shared 4-core box, 2026-11-15. Every
number is an **unverified pilot assertion**, tagged `[pcc:unverified]`. No pcc claim, benchmark
record, leaderboard row, isolab run, replay certificate or matched-rho arm. Protocol frozen before
the runs: `PROTOCOL.md`.

## Question

Does fixing k coordinates of `e1 = x1+x2+x3` ("XOR cube") simplify the m=3 decomposition SAT
instance more than fixing k literal bits of x1 ("literal cube"), on the same CNF/solver? And does
handing the solver the symmetrised system in `(e1,e2,e3)` help? (The 2026-11-14 pilot ended
"inconclusive" because the sub-1 ratio at l=5 was a cube-order artefact with no A/A floor, only
k∈{2,5} ran at l=5, and the symmetrised system was never given to a solver — SKEPTIC.md of that
directory.)

## What was run

- Arms (CaDiCaL 1.5.3, pysat, per-cube 50000-conflict budget): `lit` (x-bit ANF CNF, cube on x1
  bits); `xor` (**same CNF and solver**, cube on e1 bits); `sym` (the symmetrised `(e1,e2,e3)` CNF,
  cube on e1 bits). Every CNF forbids x_i = 0. Cube order randomised with 3 order seeds per arm
  (the A/A floor = within-`lit` ratio across order seeds).
- Cells **at full preregistered power**: l=3 (n=11) and l=4 (n=13), 8 UNSAT + 4 SAT targets each,
  all k = 1..l, 3 order seeds, 3 arms. Raw: `raw_l3.jsonl` (324 rows), `raw_l4.jsonl` (432 rows).
- l=5 (n=17) was attempted but **not completed**: each UNSAT solve costs ~16–25 s and the box was
  contended (load ~5 on 4 cores, 5 agents), so a full l=5 cell (~3 h) and even a reduced lit/xor
  cell did not fit the "single run < 10 min" / ~40 min Part-1 budget. See "Not covered".
- Validation (`run_part1.py --validate`, all passed): planted SAT target is an S4 zero and lifts to
  curve points summing to +-R; the set of x-index triples returned as models of **both** the `lit`
  CNF and the `sym` CNF (enumerated with blocking clauses) **equals the group-law brute-force
  decomposition set** over (V\{0})^3 (SAT==enumeration and UNSAT==enumeration; x=0 and the point at
  infinity are excluded consistently); the symbolic e-form equals S4 numerically.

## Results `[pcc:unverified]`

Best-k conflict ratio vs `lit`, pooled over instances (geometric mean; 95% basic bootstrap over
instances). A/A = within-`lit` cube-order floor. Primary population: UNSAT (full refutation).

| l | pop | N | xor/lit best-k (95% CI) | A/A floor (lit, 95% CI) | sym/lit best-k (censored) |
|---|-----|---|-------------------------|-------------------------|---------------------------|
| 3 | UNSAT | 8 | **0.951 [0.896, 1.010]** | [0.966, 1.034] | 5.04 (0) |
| 4 | UNSAT | 8 | **0.975 [0.955, 0.993]** | [0.993, 1.007] | 2.15 (0) |
| 3 | SAT | 4 | 1.600 [1.400, 1.920] | [0.829, 1.205] | 10.35 (0) |
| 4 | SAT | 4 | 2.094 [1.227, 3.574] | [0.872, 1.145] | 6.47 (0) |

Figure: `figure.svg` / `figure.png`. Full machine-readable results: `results.json`.

- **UNSAT:** `xor` is ~5% below `lit` at l=3 (CI includes 1.0) and ~2.5% below at l=4 (CI
  [0.955, 0.993]). The A/A floor is extremely tight (cube order moves `lit` by ≤3.4% at l=3,
  ≤0.7% at l=4), so at l=4 the 2.5% `xor` edge sits a hair *below* the floor — a real but trivial
  effect. Its upper bounds (1.01, 0.993) are nowhere near the 0.8 "alive" threshold.
- **SAT:** `xor` is markedly *worse* than `lit` (1.6× at l=3, 2.1× at l=4), well outside the floor.
- **sym (the skeptic's missing arm):** the symmetrised `(e1,e2,e3)` system is **2–10× harder** for
  the solver than the plain x-bit CNF across both populations (UNSAT 5.0×/2.2×, SAT 10.3×/6.5×), and
  it is the only arm that hits the conflict budget (SAT pilot solve ran to 1.45M conflicts before
  the cap was added). Giving the solver the symmetrised system makes things worse, not better.
- Consistent with yesterday's secondary reading that `xor` cubes also cost ~1.7× the propagations
  and wall time of `lit` cubes at equal conflicts.

## Decision (pre-registered rule)

Rule: **kill** if the xor/lit best-k 95% CI includes/exceeds 1.0 at the largest l OR lies inside the
A/A interval; **alive** if its upper bound < 0.8 at the two largest l; else **inconclusive**.

- **Formal verdict: inconclusive — by the letter, at the largest size powered (l=4).** At l=4 the
  xor/lit CI neither includes 1.0 nor overlaps the (very tight) A/A band, and its upper bound 0.993
  is far from 0.8; at l=3 the CI *does* include 1.0 (kill condition met there).
- **Substantive reading: no viable lead.** The "alive" criterion is missed by ~8×; the whole UNSAT
  effect is ~2.5%, it reverses to 1.6–2.1× *worse* on SAT, costs ~1.7× propagations/time, and the
  symmetrised arm is decisively worse. This is a practical kill; it reads "inconclusive" only
  because a ~2.5% effect is resolvable against an almost-zero-width A/A floor and because the
  preregistered "largest l" (l=5) could not be powered on the contended box.
- Corroboration: the 2026-11-14 skeptic's shuffled-order l=5 runs gave xor/lit ≈ 1.01 (geomean),
  i.e. at l=5 the effect vanishes or reverses.

## Not covered / deviations

- **l=5 not completed** (the central deviation): infeasible within the budget under box contention;
  the decision rests on full-power l=3 and l=4 plus yesterday's l=5 shuffled-order corroboration.
  Because of this the rule's "largest l" is l=4 here, not l=5.
- Per-cube 50000-conflict budget added before the kept runs (bounds a single run; `lit`/`xor` never
  approach it; `sym` is censored when it exceeds it). `sym` dropped at the attempted l=5.
- One solver (CaDiCaL), one curve (a=1,b=1), toy sizes, encoding cost (full 2^{3l} ANF) uncounted
  and equal across arms, so no arm here is comparable with the 2^l linear-scan baseline on total
  work.

## Rerun (smallest cell)

```
cd research/sat_smt_leads_20261115/F-close-4-and-11/part1
python3 run_part1.py --validate                     # brute-force checks (~20 s)
python3 run_part1.py 3 11 8 4                        # l=3 full cell (324 rows)
python3 analyze.py && python3 figure.py             # results.json, figure.svg/png
```
