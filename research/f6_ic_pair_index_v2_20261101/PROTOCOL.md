# F6-IC with a pair-index closure, under accounting v2: protocol

Frozen before any measured run (2026-11-01). Owner request: "A and b" in
the session that asked to chase F6-IC's gap to rho.

## Question

F6-IC's one-fixed-summand closure (`close_one_fixed_legacy`) scans the base
at every closed node and charges each point addition. In the 2026-10-05
ladder that was 35,670 additions per run at m = 13 against a whole inherited
F4 relation phase of 720 GAE, which made its `S` 54.75× strong rho. The
library already has `groebner_decompose_f6_ic_pair_index`: the closure is
answered from a pair-sum index of the base, built once per run (its
|F|(|F|+1)/2 additions charged), with one charged addition per lift.
`ecbench` now exposes it as `pdp3-koblitz:m=3,engine=f6-ic-pair`.

- **Q1 (primary).** Does `f6-ic-pair` lower `S` against `f6-ic` on the same
  targets? Statistic: per-target `log₂(S(f6-ic)/S(f6-ic-pair))`, median and
  range. Rule: **improves** when the median is > 0 at both sizes and the
  paired bootstrap 95% interval of the median excludes 0; **no change**
  otherwise.
- **Q2.** Against the inherited-F4 baseline and the m = 3 root finder, under
  AGENTS.md §5a: price every arm's unpriced native work at 0, 0.1 and 1 GAE
  per operation (`ecbench band`); rank a pair only where the order holds at
  every weight.
- **Q3 (context, distance remaining).** `S(arm)/S(rho-strong)` per target,
  median and range, for every IC arm. Not a gate.
- **Instrument check.** `f6-ic-pair` must make the same trials and relations
  as `f6-ic` on every target (the closure decides the same branches). A
  mismatch is reported per target and does not void the round.

## Workloads (unchanged from the ladder, so targets are identical)

| size | curve | base | target seed | measurement seed | timeout |
| --- | --- | --- | --- | --- | --- |
| m = 13 | Koblitz E_0, n = 13 | `koblitz-standard-subspace:dimension=5` | 202610051301 | 202610051302 | 3600 s |
| m = 19 | Koblitz E_0, n = 19 | `koblitz-standard-subspace:dimension=7` | 202610051901 | 202610051902 | 3600 s |

8 public targets per size, 3 rounds, 1 warmup, alternating order,
isolation L2 required. Specs: `SPEC-n13.json`, `SPEC-n19.json` (this
directory). The seeds are the ladder's `SPEC-n13/n19.json`, so the targets
are the ladder's.

## Arms

| arm | method |
| --- | --- |
| `rho-strong` (reference) | `rho.signed_frobenius_strong`, 32 lanes, dp 4 |
| `rho-strong-aa` (A/A) | the same, run again |
| `ic-f4` (baseline) | `pdp3-koblitz:m=3,engine=inherited-f4,degree=3,node_budget=8192` |
| `ic-f6` (incumbent F6) | `pdp3-koblitz:m=3,engine=f6-ic,…` |
| `ic-f6-pair` (candidate) | `pdp3-koblitz:m=3,engine=f6-ic-pair,…` |
| `ic-rootfind-m3` (context) | `pdp3-koblitz:m=3,engine=rootfind` |

All IC arms use the same base as the ladder at each size.

## Accounting

`ecbench.gae` as on `main` at the frozen commit: accounting version 2
(inherited F4's solver work charged eagerly). Each run is a fresh process,
so the pair index is built and charged once per run. Wall time is a
practicality note only.

## Route

isolab-eks on EKS Spot (AGENTS.md §9a), the frozen commit's image, L2
admission with replay and A/A. Every measured run is replayed
(`ecbench verify --replay-all`) before analysis.

## Stop and report

The round reports whatever completes. A size with fewer than 6 complete
targets per arm reports no decision for that size. Interrupted sessions are
kept and named. No speedup over rho is claimed by this protocol; the gap to
rho is reported as the distance remaining.

## Development smoke (not a measurement)

Two m = 13 targets, one round, L0, local container, before freezing:
`f6-ic-pair` verified on both, with trials and relations equal to `f6-ic`;
charged additions 1,790 and 1,525 against 50,304 and 33,984. Kept out of
every result.
