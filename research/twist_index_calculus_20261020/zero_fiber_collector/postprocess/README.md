# Independent accounting of the existing panel

This tool reads the frozen panel's records. It adds no relation search,
solver execution, target or pipeline variant. It is separate from both
currently frozen native binaries and cannot affect their source checks.

## Prospective analysis boundary

The primary input is an explicit SHA-256 of a native panel summary, with
all 85 workload rows retained. `complete` mode requires all 255 candidate
exits and the panel's terminal exit 0. `snapshot` mode requires a partial
summary and only analyzes configurations already verified in that exact
historical snapshot. Later completions do not silently enter it. Every
other configuration remains present with null metrics.

For each admitted configuration, bind the first record to its independent
audit, public/reference hashes, three successful registered executions and
two parity certificates. Check deterministic equality directly. Recompute
the reported counts from the records, compare them with the separately
compiled panel reducer, and require the audit's independent counts/ranks
and point-verification checks to agree.

Report zero and target signed physical sets separately, projective row
sets and their union/difference, factor and final matrix rank, queries,
finite F4 systems, row-reduction multiplication counts, scoped arithmetic
counters, allocation observations and process RSS. Retain six fixed paired
comparisons per case, including plain phi versus plain doubling. Exact
reference/candidate count ratios retain both integers and their reduced
fraction. A zero denominator is null, including 0/0.

The three registered executions retain their own raw diagnostic clocks and
allocation data. Check that the eight exclusive phases sum to the cold
interval and that the reusable/online cut telescopes. Infer the reusable
and target portions of the journal phase from that cut; assigning all
journal work to one side would misstate the online interval. Do not form a
wall-time speedup from these shared-host observations.

The first application is the already committed historical 29-exit snapshot
at `../results/phase2-live-progress-v1/summary.json`. No new live snapshot is
needed for this implementation gate. Three tests check journal allocation,
missing/non-telescoping intervals, duplicate rows, missing counts and
undefined ratios. An actual complete-mode attempt against that partial
summary must fail before creating its requested output. Preserve this
negative control and the successful explicit snapshot analysis.

## Cost coverage

All complete end-to-end prices and admitted speedup fields stay null. The
recorded counters are not a common additive cost unit: group calls contain
lower-level arithmetic, and F4 row-reduction multiplications cover only
part of the solver. Unmetered F4 work, additions/inversions, hashing,
allocation, journal I/O, and calibrated group/scalar primitive prices
remain unresolved. This tool audits accounting coverage; it does not fill
missing prices with zero or discharge the strongest IC/rho comparison.

The workload still comprises seven public synthetic curves in three
presentations with the same known scalar. These are separate one-target
executions, not independent secret samples. Partial results cannot answer
the full-panel question.

## Build and apply

Invoke the thin script through the common busy reservation from the
repository root. It records source snapshots, commands, stdout/stderr,
exit codes and hashes. It compiles with one worker, tests, checks the
partial-as-complete rejection, then runs the explicit snapshot analysis.
It never writes to the live panel's directory.

```sh
python3 tools/isolated_bench.py busy -- bash \
  research/twist_index_calculus_20261020/zero_fiber_collector/postprocess/build-and-check.sh \
  FRESH_RUN_DIR SEPARATE_CARGO_TARGET EXISTING_PANEL_RUN \
  HISTORICAL_SUMMARY_JSON EXPECTED_SUMMARY_SHA256
```

After the panel terminates and is reconciled, use the recorded accounting
binary with its final native summary and `complete FRESH_OUT`. Preserve the
original partial analysis. Reports, inspected PDF, source diagram and
publication status live beside the raw run, not inside sealed evidence.
