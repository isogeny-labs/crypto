# Hyperelliptic IC thread through ecbench

Round sixteen of the hyperelliptic index-calculus thread
(`research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`, "Round
sixteen"): the thread's 18 instances (genus 2, 3 and 4 curves over
`p < 256`, prime subgroup orders `r` from 1321 to 16 790 591) run through
ecbench's generic `rho.plain` and `rho.negation` on the new hyperelliptic
family.

- Spec: `spec-r16.json` (sha256 `60fb0a9e…35afa7f`), 8 planted targets
  per curve, 2 rounds after 1 warm-up, counted operations only (L0).
- Binary: ecbench built from commit `575fea48`, sha256 `a267fe64…823fef`.
- Card: `round16/CARD-HEIC-ROUND16-1.yaml`; receipt
  `round16/run1/receipt.json` (completed, 28 commands exit 0).
- Session: `sessions/r16` (`ECBS1h15dfd26783e9`, 864 records, 864
  verified); audit `sessions/r16.audit.json`: `audit OK: 864 records, 864
  verified, 576 replays reproduced, 0 problems` (`--replay-all`; receipt
  sha256 `92d88c3c71de0d0f5acaf7722f35e21984f75956d56576f095f3544474df5b8e`).
- Comparison: `sessions/r16/comparisons/rho-plain__rho-neg.json`.

Slugs in the tables carry `#J`, not `r` (`icv1-hypg{g}p{p}-n{#J}-…`); they
are unregistered (`ecbench plan` notes it) and are named in prose here by
genus, `p` and `r` only.

## Result

| | registered test | result |
|--|--|--|
| E1 | every run verifies; `--replay-all` identical | **holds**: 864/864 verified; 576 replays reproduced, 0 problems |
| E2 | median per-curve `S(rho.negation)/S(rho.plain)` over the 7 instances with `r ≥ 10⁵` ≤ 0.80 | **holds at w = 0**: 0.646 |
| E3 | median `S(rho.negation)` on those 7, beside round fifteen's median folded `S_walk` 1.008 | descriptive: 1.273 |

Per-curve ratios on the 7 instances (95% intervals, linearised t):

| g | `p` | `r` | rho.negation / rho.plain |
|--:|--:|--:|--|
| 3 | 61 | 124 459 | 0.776 [0.488, 1.235] |
| 3 | 101 | 364 747 | 0.776 [0.490, 1.228] |
| 3 | 151 | 1 180 351 | 0.613 [0.391, 0.962] |
| 3 | 211 | 4 620 611 | 0.604 [0.473, 0.772] |
| 4 | 31 | 239 753 | 0.646 [0.476, 0.876] |
| 4 | 41 | 333 041 | 0.783 [0.681, 0.900] |
| 4 | 61 | 16 790 591 | 0.627 [0.490, 0.801] |

**§5a band.** `rho.plain` leaves no work unpriced. `rho.negation` leaves
`canonicalisations_uncharged = 166 244` over its 288 measured runs.
Pooled over all 18 curves, `rho.negation / rho.plain` is 0.840 at
`w = 0`, 0.895 at `w = 0.1` and 1.394 at `w = 1` GAE per canonicalisation:
**unresolved inside the band**, so the two arms are not ranked. E2 is the
registered test at `w = 0` and is reported as that.

**Decision (as registered).** E2 holds, so ecbench's `rho.negation`
becomes the thread's rho reference and the thread's own walk
(`hyperelliptic_ic_bench`) is retired as a reference (class
**accounting**). Every later IC/rho comparison in the thread goes through
an ecbench session with both arms, priced across the band.

**E3.** ecbench's `S` counts every charged group operation including
set-up (branch table and start points), which round fifteen's `S_walk`
excludes; at these sizes set-up is a visible share, so 1.273 against
1.008 is not a ranking.

Host: 4 vCPU Xeon @ 2.30 GHz (see `sessions/r16/host.json`); level L0;
no wall-time figure is read. No validator report, so no
`benchmarks/README.md` row (validation gate).
