# Zero-core genus-2 complete cold comparison

The exact specialization passes Z1; its complete cost fails Z2 on both
fixtures. All six new targets recover with identical gate/zero/positive/A/A
stopping signatures. The fastest existing control is compact on all six.

| Prime | Best existing / zero (median) | Positive / zero (median) | A/A max | Zero setup fraction (median) |
|---|---:|---:|---:|---:|
| 1009 | 0.245253 | 1.018057 | 1.091063 | 57.42% |
| 16381 | 0.362194 | 1.136459 | 1.035648 | 32.39% |

Three independent targets per fixture, indices 17–19. Observed ranges and
individual costs in nanoseconds are retained in SUMMARY.json and CELLS.json.
No confidence interval or statistical superiority claim. Local sequential ARM
L0, common fixture/known-answer setup excluded equally, all method setup and
final verification charged. Maximum primary ratio over targets is 0.380818;
none favors zero-core. Positive-core gains do not multiply earlier results.

## Registered workload and terminal state

COLD-PROTOCOL.md committed before worker source; source a500c14f9a was built
in an isolated directory and checkpointed before execution at
20261115T120848Z-a500c14f9a20. Two reused family-0 genus-2 fixtures,
14 existing controls plus zero/positive/zero-AA; 102 independently cold timed
children and six full gates. All 108 exit successfully with no cap/error/missing
cell. Fixed candidate limit 32768 trials, timed/gate limits 30/60 seconds,
aggregate cap 600 seconds. Actual aggregate child wall 2378128298 ns.
The registry command wall is 2.549 s including collection overhead.

Z1 requires scalar, target, trials, smooth-count, raw-row-hash and terminal
status equality of all four candidate/gate outputs, plus every returned
scalar correct and every same-target workload identical. All checks pass.
Z2 requires all six recoveries and both fixture medians >=10; it fails.
Expected scalar remains in the outer harness; engine uses public inputs.

## Mathematical consequence and next cost boundary

The partition argument remains valid: full coefficient-aware weighted
elimination of degree-at-most-two genus-2 Gaudry rows preserves the target
projection and earliest determination. Finite prefix checks support that
implementation. This is an established-method specialization, not a new
exponent or mathematical novelty claim. It does not automatically extend to
degree-three principal rows or genus 3/4.

The storage reduction from the earlier gate yields only modest positive/zero
runtime ratios here. Row/forest handling occupies 4–7% in fixture median
shares; catalogue/public-jump setup and walking dominate. A post-run fixed-work
diagnostic makes all recorded row handling free, keeping other work unchanged:
its best-existing/remaining-zero ratio never exceeds 0.407856. Making setup
and row handling both free never exceeds 0.649240. These are conditional
accounting diagnostics, not measurements of an implementation or universal
bounds. Under this assumption, algebra/setup tuning alone cannot supply 10x.
A successor needs a distinct relation-supply/selection cost case against the
compact incumbent; changing elimination alone is insufficient on these inputs.

## Reproduction and custody

`build.sh` builds gate/worker/collector. Set the stable Rust PATH,
CARGO_TARGET_DIR and isolated HEC_ZERO_CORE_BIN. Source/binary hashes are
pinned. Collector command is recorded in RUN.txt and the open-then-closed
bench.record/v1 entry 20261115T1208Z-hec-zero-core-cold. `run/` is raw local
state; the compressed raw archive retains outputs, stderr, PID and terminal
receipts, fixtures and hashes for archival replay.

Native reduction:

```sh
cat research/hec_zero_core_20261115/run/*.out | jq -Rs 'split("\n")' |
  jq -f research/hec_zero_core_20261115/summary.jq
```

Native Rust render.rs consumes jq-generated plot.tsv; Typst produces REPORT.pdf.
The one-page PDF and graph were visually inspected. Benchmarks entry remains
outside the admitted table; no VALIDATION.json, default or standing changed.
Research 29 tests, library 12 tests, synthetic 8064-prefix replay, collector
success/rejection/owned-timeout controls pass locally. Standalone gate is not
automatically executed by existing native CI.

Sources: primary GTTD weighted-graph method and existing coefficient-aware
projection references/KB receipts are recorded in PROTOCOL.md and the prior
hec_dlp_graph_20261115 study. No new mechanism or novelty claim is introduced.

Errors retained: initial gate syntax error predates scientific execution and
is preserved in BUILD-FIRST.stderr. An empty-input reducer smoke command used
jq raw-string input instead of the required array; the actual reduction uses
split-lines and all 108 output cells. No scientific worker rerun or changed
seed/cap/acceptance criterion followed either error.
