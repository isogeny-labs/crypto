# PROTOCOL — Part 1 close-out of idea 4 (XOR cubes vs literal cubes)

Written 2026-11-15 **before** any measured run of this directory. Controlled rerun of the
2026-11-14 idea-4 pilot (`research/sat_smt_leads_20261114/04-xor-cubes-symmetric`), which ended
"inconclusive" because (per its SKEPTIC.md) the sub-1 ratio at l=5 was a cube-order artefact, only
k=2 and k=5 ran at l=5, there was no A/A floor, and the symmetrised (e1,e2,e3) system was never
handed to a solver. Pilot standing only: toy sizes, one core under `nice`, no isolab, no replay
certificate, no matched-rho arm. Every number is `[pcc:unverified]`.

## Setting

m=3 point decomposition R = P1+P2+P3, x(Pi) in an l-dimensional F2-subspace V of F_{2^n},
curve y^2+xy = x^3+x^2+1 (a=1,b=1), target S4(x1,x2,x3,x(R)) = 0 after Weil descent to the 3l
coordinates of (x1,x2,x3). Reference baseline named for context only: linear 2+1 scan at 2^l leaf
solves. Field/curve/S4 code is yesterday's `pdplib.py` unchanged; the symbolic (e1,e2,e3) form is
yesterday's `symbolic_e.py`/`symbolic_e.json` unchanged.

## Arms (differ only in CNF and in which variables are cubed)

- `lit`: CNF = x-bit ANF of S4 (yesterday's encoding, one AND var per nonlinear monomial, XOR rows
  cut to 3-literal chunks, y_j <-> x1_j^x2_j^x3_j), cube = all 2^k assignments of x1_0..x1_{k-1}.
- `xor`: **same CNF and same solver as `lit`**, cube = all 2^k assignments of y_0..y_{k-1}
  (y_j = bit j of e1 = x1+x2+x3).
- `sym` (the extra arm the skeptic asked for): CNF = the symmetrised system in (e1,e2,e3) — x-bits
  plus e2 (n bits), e3 (n bits) defined by their descent, and S4 written in (e1,e2,e3) from the 11
  symbolic monomials (bit-degree <= 2); cube = all 2^k assignments of the e1 bits y_0..y_{k-1}.

Every CNF additionally forbids x_i = 0 (one clause per summand), so twist/2-torsion x-only zeros
at x=0 cannot be returned; the point at infinity is not a factor-base element. Solver: CaDiCaL
1.5.3 (python-sat), default options, one incremental solver per (instance, arm, k, order-seed),
all 2^k cubes solved (no early exit). Each individual cube solve is capped at a 50000-conflict
budget (`solve_limited`); if a cube exhausts it the run is marked `censored` and stopped, and its
conflict total is a lower bound. This bounds a single run to a few seconds. The cap was
added before the kept measured runs after a pilot `sym` SAT solve ran to 1.45M conflicts; `lit`
and `xor` never approach it (the hardest single UNSAT refutation is ~33k at l=5).

## Cube order and the A/A floor

For each (instance, arm, k) the 2^k cubes are solved in a **seeded-random order**, with order
seeds {101, 102, 103} (3 per arm). The A/A floor is the within-`lit` ratio across order seeds
(seed 101 vs 102, 101 vs 103, 102 vs 103), pooled; it measures how much cube order alone moves one
fixed arm. The same is reported within `xor`.

## Cells and targets

- l=3 (n=11), l=4 (n=13): **8 UNSAT + 4 SAT** targets each (full preregistered power).
- l=5 (n=17): reduced slice — see "Budget / deviation". Each l=5 UNSAT solve costs ~19 s, so the
  full 12x3x3x5 design is ~3 h and does not fit the shared box; the reduced l=5 cell is marked as
  such and the two decision sizes are the two largest l actually powered.

Targets: UNSAT = random curve-point x with no S4 zero in (V\{0})^3 (redrawn until so), x(R) not in
V; SAT = planted R = +-P1+-P2+-P3 with distinct x(Pi) in V\{0}, x(R)!=0 and not in V. Per-target
seed = 1000*l + index (UNSAT indices 0..7, SAT indices 50..). SAT and UNSAT reported separately;
UNSAT (full refutation) is the primary population.

## Metric and statistics

Primary: total conflicts summed over the 2^k cubes. For each instance, each arm's **best k** =
the k minimising mean-over-seeds total conflicts. Per instance, ratio = xor(best-k) / lit(best-k)
(and sym/lit). Pooled across instances by geometric mean; 95% interval by basic bootstrap
(10000 resamples of instances) on the log ratio. A/A floor: same statistic on lit seed-pair
ratios. Secondary: decisions, propagations.

## Decision rule (pre-registered, from the task card)

- **kill** if the pooled xor/lit conflict ratio at each arm's best k has a 95% interval that
  includes or exceeds 1.0 at the largest l, OR lies inside the A/A interval;
- **alive** if its upper bound is < 0.8 at the two largest l;
- else **inconclusive**.
The `sym` arm is reported the same way (ratio and interval vs `lit` and vs its A/A floor); it does
not change the lit-vs-xor verdict but settles the skeptic's open point.

## Validation before measured runs (smallest cell l=3)

Done in `run_part1.py --validate`: (1) planted SAT target's tuple is an S4 zero and lifts to real
curve points summing to +-R; (2) the set of x-index triples returned as models of each CNF (lit
and sym), enumerated with blocking clauses, equals the group-law brute-force decomposition set
over (V\{0})^3 (so SAT==enumeration, UNSAT==enumeration, x=0/infinity handled consistently);
(3) the symbolic e-form equals S4 numerically (yesterday's check, re-asserted).

## Budget / deviation

One process, one core, `nice`. l=3 and l=4 at full power. l=5 reduced to the largest slice that
fits ~25 min (recorded in results.json as the actual UNSAT/SAT/seed counts); all k=1..5 kept.
Any shortfall against the 8+4x3-seed target is stated in README.md.
