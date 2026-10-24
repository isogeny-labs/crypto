# R6.1: direct S5 systems and the `ic` plug-in for the chained x-frame descent

Round 6 of `RESEARCH_ISOGENY_FIBERS_ECDLP.md`, design in `round6/PROGRAM.md`
§R6.1, following round-4 lead 16.1. These are toy Koblitz curves only. Nothing
here is a sub-rho claim, and nothing bears on a deployed curve. All numbers
come from Rust (`examples/r6_1_direct_s5.rs` and the library plug-in). Python
only plots the recorded values.

## Question, hypothesis, falsifier (from PROGRAM.md, fixed before the runs)

* **Question.** Does a direct S5 change the m = 4 picture? Does the new
  `chained-x` oracle plug-in change the crossover verdict? Is the round-4
  panel vs `ic bench` hit-rate gap real?
* **Hypothesis.**
  - The direct symmetrised S5 (A(w) + s·B(w), 126 + 56 orbit coefficients)
    is feasible wherever the chained symmetrised system is, and it is cheaper
    per relation.
  - It still never beats a feasible m = 2 arm at the same n.
  - The hit-rate gap is a budget-accounting difference.
* **Falsifier.** A direct-S5 arm whose total S is below the best m = 2 arm at
  the same n.

## Library change (own commit, in this PR)

`src/cryptanalysis/ic_framework/plugins.rs` gains the `chained-x` oracle,
`ChainedXOracle`:

* It wraps `koblitz_index_calculus::groebner_decompose` (m − 1 chained Semaev
  S3 links, m = 2, 3, 4) for any base whose abscissae form an F2-subspace.
* It refuses layouts above 64 unknowns in `prepare`.
* Its cost is charged in the same word-XOR unit as `symmetrised` and
  `inherited-f4`.
* Lifting goes through the framework's `lift_abscissae`.
* `trace=1` keeps a per-call record: outcome, reductions consumed, splits and
  XORs.

Two unit tests cover it:

* every relation re-sums, every completed refusal agrees with enumeration,
  and every call is accounted for;
* the arity and layout guards refuse.

I did **not** register it in `src/bin/ic/bench.rs`, because the permission
covered `ic_framework/` only. The end-to-end runs here call the same
`run_pipeline` that `ic bench` calls, with the same calibration, target source
and verification. Adding a `"chained-x" => ChainedXOracle::new(m, inst)` arm
to `bench.rs` is a 3-line follow-up.

## Commands

```
bash research/isogeny_fibers_20261010/round6/r6_1_direct_s5/run.sh all
# interpolate:  r6_1_direct_s5 interpolate --n 29 --a 1
# discrepancy:  r6_1_direct_s5 discrepancy --n 23 --a 0 --node-budget 4096 --panel-targets 96 --max-trials 300
# panel (m=4):  r6_1_direct_s5 panel --n N --a A --targets 24 --arms … --node-budget 4096 --cell-wall-s 600 --pre-cap-s 240
# e2e:          r6_1_direct_s5 e2e --n N --a A --node-budget 4096 --arms ARM --arm-wall-s 600
```

Each `results/*.log` repeats its command line. Settings shared by every run:

* Engine `InheritedF4 { max_degree: 3 }` with the default split rule. The
  driver refuses to start if an engine environment knob is set.
* `node_budget = 4096` reductions per solve, for every arm.
* Each process runs under `timeout 900` (the 15-minute cell cap).
* 22 cells were run. 6 more were skipped and are marked `NOT RUN` in their
  logs (see Limits).
* The host is a shared 4-core machine, so all wall times are **contended**.

## 1. Interpolation (measured)

`results/interpolate.json`. The polynomials were interpolated by F2 linear
algebra over sampled relations on `K_1/F_{2^29}`. Each was then verified on
200 fresh relations and 200 random tuples at n = 13, 17, 19, 23, 29 and 37,
covering both a.

| polynomial | unknowns | rank | nullity | support (orbits / monomials) | Boolean degree in the 4 summands | samples | field mults | wall |
|:--|--:|--:|--:|:--|--:|--:|--:|--:|
| sym S4 (control) | 20 | 19 | 1 | 6 / 18, **equal to the library's `symmetrised_terms(3)`** | 4 | 9 | 2 385 | <0.01 s |
| x S4 (control) | 70 | 69 | 1 | 6 / 24, **equal to `plain_terms(3)`** | 6 | 11 | 22 176 | <0.01 s |
| **sym S5**, A + sB | 182 (126 + 56) | 181 | **1** | 24 / 196 | **9** | 15 | 2.5e5 | <0.01 s |
| **x S5** | 1 287 | 1 286 | **1** | 44 / 729 | **12** | 53 | 1.4e7 | 0.10 s |

* Every verification relation vanished: 200/200 at each n and a.
* No random tuple vanished: 0/200 in each case.
* Both S5 are unique, with F2 coefficients, and do not depend on the field
  size.
* So the interpolation itself is cheap. The cost is all in Weil-restricting
  the degree-9 or degree-12 polynomial (table 3).

## 2. Discrepancy protocol (run first)

`results/discrepancy_n23.json`. Setup: n = 23, a = 0, the invariant ℓ = 12
base (divisor 0;1, |F_u| = 4 049), symmetrised S4, budget 4096. The same
oracle function ran under both drivers. Every call logged the target, the
reductions consumed, the XORs and why it ended.

* **The panel's 24 targets were reproduced from its seed rule.**
  - Panel path: found 1/24, with 23 budget-limited. This is exactly the round-4
    number.
  - Framework oracle on the same targets: identical outcome and identical
    word-XOR count on 24/24.
  - Extended to 96 targets: found 14/96 (0.146). Both paths agree on 96/96
    outcomes and 96/96 XOR counts.
* **The pipeline, i.e. `ic bench`'s code:** 300 trials, 66 relations
  (0.220), 234 budget-exceeded. The logarithm was recovered and verified,
  S = 4.31e4.
* **All 300 pipeline targets were replayed through the panel path:**
  - outcome agreed on 300/300, XOR count equal on 300/300;
  - all 300 targets lie in the prime-order subgroup;
  - found calls consumed 23 to 505 reductions (mean 198);
  - every budget call consumed exactly 4 096.

**Finding.** Matched per target, the two drivers do the same computation with
the same budget, so there is no bug. The rates differ only because of which
targets were drawn, i.e. sampling.

* 1/24 against 66/300: Fisher exact two-sided p = 0.036, so the 24-target
  panel draw was an unlucky one.
* 14/96 against 66/300: p = 0.14, consistent.

The hypothesis said "budget accounting". It is refined: the budgets and their
accounting are identical, and the gap is target sampling on a mostly
budget-limited arm.

## 3. m = 4 panel (measured; every verdict gated, 0 gate failures)

There is one random non-invariant `V ∋ 1` per curve with ℓ = ⌈(n + log2 24)/4⌉,
shared by both frames, and 24 subgroup targets.

* `x-chained` runs **through the new plug-in**.
* `sym-chained` is the round-4 control.
* `XOR/rel` is the engine's word XORs per relation found. It is a lower bound,
  because the symbolic build is not counted.

| n, a | ℓ | arm | unknowns | found / refuted / budget | XOR / relation | notes |
|:--|--:|:--|--:|:--|--:|:--|
| 13, 0 | 5 | x-chained (plug-in) | 46 | 24 / 0 / 0 | **1.48e6** | |
| 13, 0 | 5 | sym-chained | 43 | 15 / 0 / 9 | 7.75e6 | |
| 13, 0 | 5 | **sym-S5** | **17** | 9 / 0 / 15 | **1.38e9** | one-time build 30.7 s; per-target build ~0.09 s |
| 13, 0 | 5 | **x-S5** | 20 | 3 / 0 / 3 (6 attempts) | 5.24e10 | one-time build 166 s; cell wall guard stopped it after 6 targets |
| 17, 1 | 6 | x-chained (plug-in) | 58 | 23 / 0 / 1 | 2.24e7 | |
| 17, 1 | 6 | sym-chained | 55 | 2 / 0 / 22 | 1.38e8 | |
| 17, 1 | 6 | sym-S5 | 21 | — | — | **one-time build did not finish in a 15-min cell (exit 124)** |
| 17, 1 | 6 | x-S5 | 24 | — | — | one-time build stopped at the 240 s cap: 148 products, 6.3e5 terms in a coordinate |
| 19, 0 | 6 | x-chained (plug-in) | 62 | 23 / 0 / 1 | 2.79e7 | |
| 19, 0 | 6 | sym-chained | 59 | 0 / 0 / 24 | — | infeasible |
| 23, 0 | 7 | x-chained, sym-chained | 74, 71 | refused (> 64) | — | as in round 4 |
| 19, 23 | 6, 7 | sym-S5, x-S5 | 21–28 | not run | — | skipped: ℓ ≥ 6, and the n = 17 build already exceeded the cell |

The direct S5 has the fewest unknowns (17 against 43–46 at n = 13). It is
still the costliest arm:

* At n = 13, sym-S5 costs 177× sym-chained and 930× x-chained per relation.
  Its hit rate is also lower (9/24 against 15/24), because 15 of its 24 calls
  ran out of budget.
* From ℓ = 6 its target-independent Weil restriction, a degree-9 polynomial in
  21 unknowns, does not finish inside a cell. The x-S5 (degree 12) is worse.
* So, contrary to the hypothesis, the direct S5 is **not** feasible wherever
  the chained symmetrised system is. At n = 17 the chained sym system ran
  (2/24 found), while the direct S5 could not be built.
* At n = 23 the direct S5 layout (25 / 28 unknowns) is under the 64 cap where
  the chains are not, but it could not be built in time either. The 64-cap
  question is therefore moot.

## 4. End to end (measured, `run_pipeline`, the logarithm recovered and verified in every row)

Every row uses the same random-V bases with cofactor-projected columns and
the seed 123212651130.

| n, a | arm | ℓ | columns | trials / relations | budget hits | S |
|:--|:--|--:|--:|:--|--:|--:|
| 13, 0 | **mitm m = 2** | 7 | 36 | 32 / 20 | — | **186** |
| 13, 0 | chained-x m = 3 (plug-in) | 6 | 30 | 26 / 26 | 0 | 429 |
| 13, 0 | chained-x m = 4 (plug-in) | 5 | 12 | 13 / 13 | 0 | 659 |
| 13, 0 | **direct sym S5, m = 4** | 5 | 8 | 24 / 9 | 15 | **7.61e5** |
| 17, 1 | **mitm m = 2** | 9 | 240 | 186 / 114 | — | **543** |
| 17, 1 | chained-x m = 3 (plug-in) | 7 | 59 | 57 / 51 | 0 | 1 401 |
| 17, 1 | chained-x m = 4 (plug-in) | 6 | 26 | 31 / 25 | 6 | 6 699 |
| 19, 0 | **mitm m = 2** | 10 | 501 | 361 / 239 | — | **1 658** |
| 19, 0 | chained-x m = 3 (plug-in) | 8 | 123 | 106 / 106 | 0 | 5 869 |
| 19, 0 | chained-x m = 4 (plug-in) | 6 | 42 | 46 / 43 | 3 | 12 680 |

The graph is `s_by_arm.svg` / `.png`, with its data in `plot_data.csv`.

* With the plug-in, the chained x arms now run in the `ic` pipeline. They are
  the best algebraic arms: at every n, S is ordered m = 2 < m = 3 < m = 4.
* None of them beats mitm m = 2. The best ratio is 2.3× (n = 13, m = 3).
* The direct sym S5 is the only S5 arm that ran end to end. At n = 13 its
  S = 7.6e5 is 4 090× the best m = 2 arm.
* So the crossover verdict of 17.1 stands with the stronger arm plugged in:
  at m = 3 and m = 4 there is no crossover against m = 2 at any measured n.

## Verdict

**Mixed: the falsifier is not met, and the hypothesis is partly refuted.**

1. **Falsifier not met.** No direct-S5 arm has S below the best m = 2 arm.
   At n = 13 it is 7.6e5 against 186. At n ≥ 17 it cannot be built in a cell.
2. **Hypothesis part refuted.** The direct S5 is neither cheaper per relation
   (1.4e9 against 7.7e6 XOR/relation for the chain at n = 13) nor feasible
   wherever the chain is (n = 17).
3. **Discrepancy explained.** Matched target by target, the two drivers
   agree exactly (396/396 outcomes and XOR counts: 96 panel targets and 300 pipeline targets). The 0.04 against 0.22 gap
   is target sampling (p = 0.036). There is no bug.
4. **The plug-in works.** `chained-x` m = 3 and m = 4 recover the logarithm
   end to end at n = 13, 17 and 19. This does not change the verdict.

## Limits

* The curves available limited the grid:
  - `koblitz_instance` has no usable-subgroup curve for n = 15 (either a), or
    for a = 1 at n = 13, 19, 23. The grid is therefore n ∈ {13, 17, 19, 23}
    on one a per n (a = 1 only at 17), not the full a = 0 / 1 grid.
  - The interpolation used n = 29 because n = 31 has no instance.
* Cells not run:
  - Six cells are marked `NOT RUN` in their logs and recorded as such:
    direct S5 at n = 19 and 23 in the panel, and sym-S5 end to end at
    n = 17 and 19. They were skipped after the n = 17 build overran a whole
    cell, to stay within the 24-cell / 15-minute budget.
  - "Not buildable" is a contended wall-time observation, not a counted bound.
* The symbolic build is not counted in XORs, as in round 4. For the direct S5
  that build is the dominant cost, so the S5 XOR figures understate it.
* The x-S5 cell at n = 13 ran 6 targets before its 120 s wall guard stopped
  it.
* There are 24 targets per panel cell. The end-to-end rows are single runs.
* No matched rho was run. The round-4 rho references on these curves are
  S ≈ 1–2, and every row here is above them.
* Two environment problems:
  - Conductor is not installed in this environment, so `conductor check`
    could not run.
  - The shared `CARGO_TARGET_DIR` (`r6target`) gave a stale `libcrypto_lib`
    rlib that another worktree had built (`ChainedXOracle` unresolved). The
    builds here used a private target dir, `scratchpad/r6_1target`.

## Next step

Register `chained-x` in `src/bin/ic/bench.rs` (3 lines). A direct S5 would
only be worth retrying with a build that never expands the full degree-9
polynomial. One option is a resultant or summation chain evaluated on the
subspace, or the factored form A + sB applied through the solver's
specialisation instead of being Weil-restricted up front.

## Files

* `examples/r6_1_direct_s5.rs`: the interpolation, the discrepancy driver,
  the panel and the end-to-end commands.
* `run.sh`: the exact commands. `results/*.json` and `*.log`: the raw
  records. `results/host.txt`: the host.
* `plot.py`, `plot_data.csv`, `s_by_arm.svg` / `.png`: the graph.
* `report.typ`, `report.pdf`: the report.
