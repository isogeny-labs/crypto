# Public-input target pipeline comparison

This continuation implements the registered
[Phase 2](../PHASE2_EXECUTION.md). It retains the seven original curves and
three presentations of the same known-answer scalar. The existing complete
physical sets are post-run replay references; the candidate reads none of them.

## Fixed comparison

For each of the 21 cases, compare stopped zero precomputation plus target
descent with the matched plain collector, for both `phi-l0-active` and
`double-l0-active`. The committed native baseline selection adds only
p97 seed1's plain `union-l2` configuration. The other twenty best-parent
roles reuse the identical registered plain configuration. There are **85
distinct configurations, 255 candidate executions, 85 independent first-run
audits, and 170 A/A comparisons**.

The fixed full-stream unique-yield comparison remains the completed parent
study. These new stopped prefixes have different exposures: fewer relations
or queries in a recovered prefix alone do not imply better relation yield.
Compare zero versus plain within each arm, then phi versus doubling and the
frozen best-parent arm. Report all systems consumed through verification,
including zero-precomputation systems, not only online systems saved.

## Programs and ownership

`panel_native` is a separate Rust package. It prepares and seals the fixed
workload from existing records and independently reduces the new records;
it contains no curve solver. Build it with `build-panel-native.sh` through
the common `tools/isolated_bench.py busy` reservation. Its build directory
retains source snapshots, commands, logs, compiler versions and executable
hashes. Failed builds stay in separate directories.

The panel driver is invoked **directly**, without an outer busy wrapper:

```text
bash phase2/run-panel.sh FRESH_OUTPUT WORKLOAD_JSON SUCCESSFUL_NATIVE_GATE PANEL_BUILD
```

`PANEL_BUILD` is the successful metadata-tool build directory. Each candidate,
audit and parity phase independently obtains the common reservation. The
driver refuses a nested outer reservation before creating its output,
copies its executed script, verifies source and input seals before work,
and records every command, start, end and exit. The frozen script has one
owner and does not restart an interrupted or terminal phase automatically.
Reconcile its native receipts and original deadlines before any continuation.

Candidates use one science worker, an 8 GiB address-space cap, a 1,200-second
outer watchdog and a 1,180-second cooperative deadline. Compilation is a
separate reserved phase without that address-space cap. All per-system
input/result journals, failed systems and partial cases are preserved.

## Measurement status

The shared migrated host permits counts and correctness diagnostics only.
The pipeline records exclusive setup, zero-precomputation, query generation,
decomposition, matrix, recovery, verification and journal intervals. Their
sum is the observed operational interval. Process startup, public-input
decoding and final output serialization are excluded and identified.
Allocation calls, allocated bytes, peak live Rust bytes and process RSS
remain distinct. Scoped field/group counts and F4 row-reduction counts do
not yet cover or price every primitive. Neither elapsed diagnostics nor a
lower F4-system count establishes an admitted complete IC/GAE speedup.

Full calibrated cost, strongest matched IC and rho references, the formal
group/field bridges, and broader scaling remain open. This comparison is
not a `VALIDATION.json` or a leaderboard promotion.
