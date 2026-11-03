# Protocol: two orbit columns under the lazy member index (2026-10-18)

Frozen before any claim data. Class: **engineering** (a constant; same algorithm,
different base size). Follows `research/ecbench_lazy_orbit_members_20261018/`
(PR #2158), whose oracle `mitm-frobenius-lazy-counted` is its subject-matter
prerequisite; this branch is stacked on that PR's head.

## 1. Question

The lazy oracle removes the `|F|(n−1)` member-index set-up, which grew with the
number of columns `c`, so the cost-minimising `c` may fall. The 2026-10-14
protocol chose `c` per curve under the *eager* oracle (13: 1, 17: 1, 19: 2, 23: 3,
29: 1, 31: 2, 37: 8, 41: 90). Is `c = 2` cheaper than that frozen choice
once the oracle is lazy?

## 2. Change under test

Only `columns`: `ic-lazy-c2` (`compact-orbit-scan:columns=2`) against
`ic-lazy-c<c0>`, `c0` the frozen choice above, both with
`mitm-frobenius-lazy-counted:m=2` and `targets=stride-renew`. Curves with
`c0 = 2` (n = 19, 31) are the same method as the candidate and are excluded, as
is n = 41 (`c0 = 90`: two columns cannot be a competitor, and its sessions run
for hours). The panel is therefore n = 13, 17, 23, 29, 37.

## 3. Exploration (not evidence)

Scratch sessions of 2026-10-18 on target seeds 2026101870+n+a (kept only in the
scratchpad) swept c = 2, 3, 4, 5, 6 under the lazy oracle, mean S (eager c = 3
for reference): n23: c2 1.652, c3 1.307, c4 1.255, c5 1.378, c6 1.587;
n31: c2 1.213, c3 1.226, c4 1.445; n29 a1: c2 2.867, c3 4.336, c4 6.144;
n17 a1: c2 2.119, c3 2.680, c4 3.484. `c = 1` was not tried there. The claim
uses fresh target seeds 202610182001–202610182009.

Predictions, stated before the data: c2 is **cheaper** than the frozen choice
where that choice is c > 2 and the base is small relative to the walk (n = 29
a1 and n = 17 a1 are c0 = 1, so c2 against c1 is open), **dearer** at n = 23
(exploration: c2 > c3) and at n = 37 (c0 = 8).

## 4. Sessions

One session per curve (`make_specs.sh` → `docs/ecbench/specs/claim-koblitz-lazy-c2-a<a>-n<n>.json`):
32 public targets × 3 rounds (+1 warm-up), n = 37: 16 × 3; alternate order; L0
(`--cpus none --allow-busy`; counts stand, wall times are descriptive). Arms, all
`*_full`: `rho-frob` (reference), `rho-frob-aa` (A/A), `rho-strong`,
`ic-lazy-c<c0>` (baseline), `ic-lazy-c2` (candidate). `run.sh` runs, replays
(`verify --replay-all`), tabulates and saves the comparisons.

## 5. Hypotheses and decision rules

Every record must verify, every replay match, `lower_bound = false`, and
`rho-frob-aa ÷ rho-frob` contain 1 per curve.

- **H1 (per curve).** `ic-lazy-c<c0> ÷ ic-lazy-c2` (paired by target, cluster
  bootstrap 95 %): c2 is an improvement on a curve iff the lower end > 1, a
  regression iff the upper end < 1, else no difference.
- **H2 (distance to rho).** `ic-lazy-c2 ÷ rho-strong` and the baseline's are
  reported per curve as the distance remaining; not a gate. No ranking against
  rho unless the interval excludes 1 and survives AGENTS.md §5a's band.
- **H3 (what changes).** Relations (`trials`) and rank differ by construction
  (a different base); phase totals are reported, and the net `S` decides.

## 6. What cannot be claimed

A per-curve choice of `c` made after the data is a tuning result on these seeds
and needs fresh seeds before it is a rule. No exponent, ECC2K-130 or deployed-
curve statement follows.
