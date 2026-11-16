# PROTOCOL: wall-time gap between the `lin21` SAT arm and the plain 2+1 scan (pilot)

Written 2026-11-16 (about 02:55Z) before any measured run; committed alone before the first
measured row. Pilot at toy sizes in a shared 4-core container with another CPU-heavy agent.
No benchmark record, leaderboard row or pcc claim; every result line will be tagged
`[pcc:unverified]`. Wall times are indicative; the A/A band is reported with them.

## Question

The pilot `research/sat_native_lin21_20261116/` showed that the `lin21` encoding makes
`LinCdcl` (examples/pdp_lin_cdcl.rs) perform the linear 2+1 scan in about 1.5 * 2^l conflicts,
but 1,400 to 1,900 times slower in wall time than the plain native scan in the same file.
Can a copy of the solver with a different XOR engine, deciding the same thing with the same
search skeleton, bring the wall time to within 30x of the plain scan?

## Frozen code (sha256)

- `examples/pdp_lin_cdcl.rs` 996770f4b4ecbc84c451203f6edce5cb17ac40b44892def19d06ca63bc0d2f47: the original solver, included UNCHANGED through `#[path]`; arm `lin21`
- `research/sat_native_lin21_20261116/lin21.rs` 2631df21b8177ac645ac31568f3ec2bbf37c9762bdcc8692a21f08865925e3f1: lines 16..862 are extracted verbatim into `base_lin21_verbatim.rs` (c9282e389106fa7ecb8e2246c20bcfc0f94c8b43b975b71f662bb951b9998891) and compared by `run.sh` at every build: field, curve, pair oracle, plain scan, `Lin21Instance`, model checks
- `fastcdcl.rs` ef88dca6020ceaefe1b14341a864037cd4e82787062330676ebd140aefcb3c05: the modified copy, arm `lin21-fast`; its header lists every change (C1..C9)
- `l21fast.rs` 05548b1ea15d8573e2e48922fbe769e3a3db2c58992bdf087fc8739098e3e816: harness, selftest
- `prof.sed` 70726bbd20d55243ca432d70128302b286872143cc6b0ac175309df76c50afc3 and its output `lincdcl_prof.rs` 764a12c7bc41f0f3b91ff8ce8704ae023ea0e85b6bf4623ecea037f2481a9058: the original plus timers and counters only
- `summary.rs` 34d8e71203d462cc9a2b38184baf1015cb7fd7ba2ab8bc02eb2d84f2c7a3b19a (tables, decision, SVG), `run.sh` 97ddbac354ff3a2bf3b2f29b7d7f52e4c564656a1581f86d7a6078a0034e4512 (thin orchestration)
- Target files of the lin21 pilot, reused unchanged (seed 20261116), sha256 first 8 hex:
  l5 466cdaaf, l6 e66b6ec6, l7 9d0a7a84, l8 2fe17d90, l9 898f54f7, l10 298ef9c7, l11 b8ec6015,
  l12 b6bdd9d1.

Build: `rustc -O` (rustc 1.97.0, standard library only, default x86-64 target, no cargo). All
computation, tables and the figure are Rust; `run.sh` only builds and starts processes.

## Step 1: profile of the original solver (seed 777, not a measured seed)

One non-decomposable target per size, `lin21` in the profiling copy (same conflicts, decisions
and GJE calls as the unmodified solver on every target where both ran).

| l | wall s | conflicts | GJE calls per conflict | row XORs per conflict | words per conflict | gje_eliminate | gje_scan_explain | gje_build | gate_inject | analyze | clause_prop | backtrack |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 9 | 1.12 | 773 | 5.2 | 5 306 | 61 752 | 39.6 % | 41.4 % | 9.7 % | 2.9 % | 2.9 % | 0.5 % | 1.2 % |
| 10 | 2.76 | 1 527 | 5.1 | 7 511 | 94 271 | 43.3 % | 37.7 % | 8.9 % | 2.9 % | 2.8 % | 0.9 % | 1.7 % |
| 12 | 15.76 | 6 150 | 5.0 | 10 390 | 154 426 | 42.3 % | 40.3 % | 6.7 % | 2.6 % | 2.5 % | 2.3 % | 1.7 % |

`gje_scan_explain` is the conflict/unit scan with the eager `explain` calls: at l = 12,
815 739 explanations with 56.5 million literals (69 per explanation, sorted and deduplicated).
Watch lists, learning and branching are below 3 % each.

Ceilings stated before the changes (if the phase cost went to zero, nothing else changing):
re-elimination and rebuild (C1, C2, C5: gje_eliminate + gje_build, 49 %) alone at most 2.0x;
eager explanation (C4, 40 %) alone at most 1.7x; both together at most about 9x, because gate
injection, analysis, clause propagation and backtracking (about 11 %) remain. Getting from
1,400x to 30x of the scan needs 47x, so those remaining phases must shrink too: value-folding
gates (C6) for gate injection, and unit-cut reasons (C4) for analysis and clause size.

## The modified solver (`lin21-fast`), frozen

Changes C1..C9 are listed in the header of `fastcdcl.rs`: one reduced row-echelon matrix per
decision level, kept across decisions and copied at a decision (C1, C5); assignments as unit
rows (C2); bit-packed rows with a column index and range-limited XORs (C3); lazy explanations
cut at unit literals, as an OR-mask of unit slots (C4); gate linearisation that multiplies or
divides in the field when a block is fully assigned and substitutes assigned bits otherwise
(C6); highest-index pivot (C7); XOR engine started in `solve()` (C8); in-place watch lists
(C9). Not changed: first-UIP learning on every conflict, all learnt clauses kept, backjumping,
priority/activity/lowest-index branching, phase saving, no restarts. Not implemented: Gray-code
order over x1, a learning-free leaf (both change the search), multi-word field blocks.

Tuning was done on seed 777 only (l = 7, 9, 10, 11, 12; 2 to 4 non-decomposable and 1 to 2
planted targets per size): three engine versions (v1 exact combination masks and row scan;
v2 value-folding gates; v3 column index and unit-cut masks), four pivot settings (static/dynamic
x high/low, `FAST_PIVOT`), and a popcount-free unit test. The measured runs use the defaults in
the frozen file. No change after the first measured row.

## Setting, targets, arms

Setting exactly as in the lin21 pilot: F_{2^n}, n = 3l + 2, l = 5..12, curve
y^2 + xy = x^3 + x^2 + 1, V = elements < 2^l, problem ALG0, the same admission rule. Targets:
every line of the pilot's `raw/targets_l<l>.jsonl` (16 non-decomposable uniform targets per
cell, the decomposable uniform draws met on the way, 4 planted; 168 targets).

One process per target runs, in this order: `scan` (plain 2+1 scan, median of 21 repeats),
`lin21-fast`, `lin21` (unmodified solver, the baseline, rerun here), `lin21-fast` again
(`lin21-fast-b`, the A/A control, interleaved with the baseline). The profiling copy
(`lin21-prof`, a counter source and not a timed arm) runs last on every target at l <= 9 and on
the first four targets (idx 0..3) of each larger cell. One measurement process at a time; a
second process may generate extension targets meanwhile. 600 s cap per process; a capped
process is reported as censored.

Extension cell, run only if the main run is finished and time remains: l = 13 (n = 41), targets
from the pilot's own `lin21 targets --l 13 --seed 20261116 --uniform 6 --planted 2`, same arms.
It is reported separately and is not part of the rule below.

## Checks (exactness)

- `selftest` at every build: exhaustive over all rational targets outside V at
  (n, l) = (11, 3), (8, 3), (7, 3), sampled at (14, 4), (17, 5), (20, 6): pair oracle = scan =
  original `lin21` = `lin21-fast`; every `lin21-fast` model in V^3 and algebraically valid, and
  on admissible targets lifting to points that sum to the target.
- Every measured row: verdict equal to the pair oracle's (`oracle_sat` in the target file),
  hence equal across `scan`, `lin21`, `lin21-fast`, `lin21-fast-b`; every SAT model in V^3,
  liftable, with a signed sum equal to the target, chain equations satisfied.
- `lin21-prof` equals `lin21` in conflicts, decisions, propagations and GJE calls.
- A run with any disagreement, failed model check or censored row cannot give "gap closes".

## Metrics

Per target and arm: conflicts, wall time of `solve()` (scan: median of 21 repeats), row
operations and the 64-bit words they touch (scan: 2 words per row XOR; `lin21-fast`: variable
range + mask word, with column-index XOR words and checkpoint-copy words listed separately;
`lin21`: variable + combination words, from the profiling copy). Per cell, over the
non-decomposable uniform targets: medians; the ratio of medians lin21-fast / scan and
lin21 / lin21-fast (first `lin21-fast` run); paired per-target ratios with [min, max]; the A/A
ratio second / first with [min, max].

## Decision rule (pre-registered, evaluated on l = 9, 10, 11, 12)

- "Gap closes": the median wall of `lin21-fast` (first run) is <= 30 times the median wall of
  the plain scan at every l in 9..12, with verdicts identical to the oracle on every row and
  no censored row.
- "Partly": not the above, and `lin21-fast` is >= 10 times faster than the original `lin21`
  (ratio of medians) at every l in 9..12.
- "Fails": `lin21-fast` is < 3 times faster than the original at l = 12.
- Anything else: inconclusive.

If 30 lies inside [ratio * A/A min, ratio * A/A max] at some l, the outcome is reported as
decided by the rule but within the A/A band.

## Predictions (written after the seed-777 tuning runs, before any measured row)

1. lin21-fast / scan, ratio of medians: in [17, 27] at each l = 9..12 (seed 777, one to four
   targets: 20 to 23 at l = 9, 20 to 23 at l = 10, 22 at l = 11, 20 to 26 at l = 12 depending on
   load). At small l the fixed start-up of the solver weighs more: 25 to 60 at l = 5..8.
   Expected rule outcome: "gap closes", with a margin that one loaded cell could remove.
2. Slope of log2(lin21-fast / scan) against l over l = 9..12: in [-0.10, +0.10] bits per unit
   of l, i.e. no trend toward parity. Reasons to expect a small positive slope beyond l = 12:
   learnt clauses are never deleted (clause propagation share grows with 2^l), row width grows
   with n, and the checkpoint copy grows with n^2.
3. lin21 / lin21-fast: 55 to 90 at l = 9..12.
4. Conflicts: equal to the original's on at least 90 % of the non-decomposable targets (the
   seed-777 targets: all equal); on decomposable targets within 5 % in sum.
5. Gauss work in words (rows + column index) of lin21-fast: 9 to 15 times the scan's, and the
   original's 25 to 35 times lin21-fast's.
6. A/A: per-cell median second/first in [0.9, 1.1]; single targets may deviate by 40 %.
7. l = 13, if run: ratio to the scan in [17, 30].

## Not covered

Other curves, fields, subspaces or n/l ratios; m > 3; the chained `lin` encoding; external
solvers; restarts or clause deletion; Gray-code enumeration; a learning-free leaf; multi-word
fields (n > 62, the ECC2K-130 stretch) unless time remains after the main run, in which case it
is labelled exploratory; any size at which the scan itself matters for an index-calculus cost;
the gap to Pollard rho (unchanged by this pilot: the best arm in the file is still the scan).
