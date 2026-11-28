# Is rho-neg's sub-½ 16–26 exponent an accounting artifact?

Frozen 2026-10-21, before any number below was computed.  Branch
`claude/ic-acct-2`.  Analysis only: no new measurement; every input is a
committed hold-out session of `research/bitop_ladder_20261019`.

## Background

On the bitop ladder's hold-out panel, `rho-neg` (`rho.negation`) fitted in
`bitop/v1` over 16–26 bits has an exponent in additions of 0.413
[0.388, 0.447] (`research/bitop_ladder_20261019/analysis/bounds/rho-neg-16-26.json`):
the positive control "a generic walk scales as r^½" tripped.  Two features
of the accounting could produce that without the walk itself being
sub-½:

1. **Set-up in the total.**  The total counts rho's set-up (`G + Q`, the
   jumps `[a_j]G + [b_j]Q`, the start stride: O(jumps · log r) additions),
   which does not grow like √r and is a larger share at small r.
2. **Two configurations in one fit.**  `rho_jumps_for` (ic_boundary.rs)
   gives 8 jumps for 16–22-bit r and 16 from 23 bits, so 16–26 mixes two
   walk configurations.

Separately, `ecbench` classified a phase named `setup` as reusable across
targets for every method, crediting rho and kangaroo with reuse they do not
have (their set-up uses the target).  This branch makes the classification
target-aware (`work::is_reusable_phase`; committed blocks are re-split from
their phases by `work::target_split`) and adds `bound fit --cost
total|per-target|search`.

## Instrument

`target/release/ecbench` built from this branch (commit recorded in
README.md).  Cost bases (`work::CostBasis`, doc comment):

- `total`: every gate of the record's bitop/v1 work block;
- `per-target`: the gates a further target pays again under the
  target-aware rule (for rho.negation and kangaroo.vow this equals `total`
  by construction; stated here in advance);
- `search`: for generic walks/tables the phase `search` only (not set-up,
  internal verification or `natives`); for IC (`ic.*`), `relations` +
  `linear_algebra`.

## Inputs

`research/bitop_ladder_20261019/sessions/holdout-P{16..40}`, unmodified.

## Frozen analysis plan

For each arm in {`rho-neg`, `bsgs-neg`, `kangaroo`, `ic-pair-rule`}, each
basis in {total, per-target, search} and each range in {16–22, 23–32,
16–26, 33–40} (sessions `holdout-P<lo>` … `holdout-P<hi>`):

    ecbench bound fit --dir … --arm <arm> --unit bitop/v1 --cost <basis> \
      [--param-rule "<bitop ladder rule>"]   # ic-pair-rule only, same text as
                                             # research/bitop_ladder_20261019/analyse.sh
      --out analysis/bounds/<arm>-<basis>-<lo>-<hi>.json

with the default resamples (2000) and seed.  The statistic is
`fit.alpha_in_additions` and its 95% interval `fit.alpha_in_additions_ci95`.
A fit the tool refuses is reported as refused with its message, not
re-run with other options.

Amortization: `ecbench amortize --dir holdout-P16 … holdout-P40 --arm
ic-pair-rule --reference rho-neg` twice: with `--stored-split` (the split
the committed blocks carry, i.e. the classification before this branch)
and without (target-aware).  Reported per curve: both arms' reusable and
per-target gates, `cold_ratio`, `per_target_ratio`, `break_even_window`.

IC/rho ratios: per curve, `cold_ratio` (total, IC over rho) and
`per_target_ratio` (per-target IC over per-target rho) from the
target-aware amortize output, and `per_target_ratio` from the stored-split
output; numbers only.

Check: rho-neg's `jumps` counter per session, to confirm 8 for 16–22 and
16 for 23–32 on this panel.

## Positive control

For each generic arm (`rho-neg`, `bsgs-neg`, `kangaroo`) and each
fixed-jump range (16–22, 23–32): the **search**-basis
`alpha_in_additions_ci95` contains 0.5.  Six checks; all must pass.

## Verdict rule (applied mechanically)

1. If any positive-control check fails: **inconclusive** (control tripped),
   naming the failing checks; no attribution is made.
2. Otherwise, for rho-neg over 16–26:
   - if the total-basis interval excludes 0.5 and the search-basis
     interval contains 0.5: **yes**: the sub-½ total exponent is an
     accounting artifact of counting non-search work (set-up, verification,
     natives) in the total; the walk scales as ½;
   - if the search-basis interval also excludes 0.5: **no**: the sub-½
     exponent persists in the search phase over 16–26 (consistent with the
     jump-count change affecting the walk itself, since each fixed-jump
     range passes);
   - if the total-basis interval contains 0.5 on this binary: **no sub-½
     exponent to explain** (and the discrepancy with the committed bound is
     reported).
3. The per-target basis cannot change rho-neg's exponent (equal to total
   by construction) and enters the verdict only as a check that it does
   equal total.

Nothing in this study is an improvement of index calculus: it is an
accounting correction and a diagnostic (CLAUDE.md, "Relabelling and
accounting changes are not improvements").
