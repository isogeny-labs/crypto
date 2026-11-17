# Amendment 2 — work-row custody and bootstrap seed

**Filed 2026-11-16 03:24 UTC, before affected data.** The combined K13 panel
still has no submitted execution. [Protocol v2](protocol-v2.json) is retained
byte for byte with canonical freeze hash
`441c2fd187f759d3e5cccaf78c89ede1057ab0d92bd9a44cbd8055ba6b0c228a`.
[Protocol v3](protocol.json) is frozen at
`b4631c0451d9647c3dfaa5d20ae6d56eeb37e1ab833d5ce22653a64991d8be2e`.

## Pre-data failure reproduced

A 560-record synthetic panel with the required ten arms, eight workloads,
seven rounds and qualifying wall fields, but **zero** `ecbench work` rows,
returned `panel_complete=true` and `all_four_f456_rules_met=true`. The output
listed all 560 missing work records without letting that gap prevent the
mechanical verdict. Source inspection also found that A/A used bootstrap seed
`202611151603`, while the five decisive comparisons used offsets through
`202611151608`. The frozen decision rule names one seed,
`202611151603`, for the target-cluster bootstrap. These are local synthetic
controls and source observations [pcc:unverified], not panel measurements.

## Exact correction

- Require one `ecbench work` result for every unique execution record ID, with
  no extra or duplicate work rows, before the panel is complete. A recorded
  `Err` work result remains an incomplete **secondary** cost observation and
  does not by itself invalidate complete wall timing; absent or malformed work
  results fail the panel admission gate.
- Use seed `202611151603` for A/A and each of the five comparison intervals.
  The 20,000 target-cluster resamples, percentile endpoints, all-pair rule and
  Bonferroni adjustment stay as frozen.
- Include valid NUMA node zero in the analysis output's node list. Its
  admission behavior was already corrected by Amendment 1.

The analyzer SHA-256 is
`96164aebfb626d9add02d0dfb1cecbd0f86644f3e6c229bc2336d60178825bc2`.
Four adversarial tests in [test_analyze.py](test_analyze.py) exercise absent,
complete, extra and duplicate work rows and the single-seed rule; all pass
locally [pcc:unverified]. `experiment_protocol.py validate` reports zero
findings for v3. The ten arms, eight public targets, 560 executions, methods,
factor base, process-wall metric, thresholds, stopping rule and budget gate are
unchanged. The existing `panel.dot`/SVG and design PDF show the same decision
path; no quantitative graph or curve fact changes in this amendment.

The next admission step is a pushed GitHub publication receipt binding v3,
both preserved versions, the corrected script and exact source. Native replay,
per-record work, independent review and the monthly cost gate still apply.
