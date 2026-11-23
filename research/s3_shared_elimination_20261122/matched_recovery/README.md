# Matched Original/Fresh/Shared public recovery

The source-bound example suite passed 17 tests. The seven-arm Prime31 CLI
recovered both signs of the fixed public target and retained all 28 scheduled
outcomes, including two deliberately incomplete controls. The separate F17
test recovered every nonidentity target with every arm. These are bounded
producer correctness observations, not complete-cost or performance evidence.
[pcc:unverified]

## Inputs and results

Coordinate arithmetic uses p=2147483647; relation Gaussian elimination uses
r=2147446831. The curve, generator, structured base and target are unchanged
from the earlier Prime31 receipt. Original runs once; Fresh and Shared each
run with Integer, FMA and Split16. F4 and IncrementalGauss remain the actual
integer implementations.

| Panel or check | Retained observation | Scope |
| --- | --- | --- |
| Q, seven arms | k=7, three queries, three independent relation rows, rank four | Same query identities and stopping lengths; final group equality |
| -Q, seven arms | k=2147446824, five queries, four relation rows, rank four | A dependent row at a=2 and direct-base skip at a=3 remain visible |
| Query-cap panel | One query_budget_exhausted and six recovered outcomes | All seven planned arms execute |
| Expired-deadline panel | One f4_incomplete and six recovered outcomes | Zero F4 runs in the interrupted arm; no exhaustion conclusion |
| Prepared interruption test | One Fresh and one Shared interrupted record | Their setup and specialization work are retained |
| Exhaustive F17 test | 18 targets times seven arms: 126 successful recoveries | Complete nonidentity target coverage on that diagnostic curve only |

The full example suite has 17 passed, zero failed, zero ignored and zero
filtered tests: twelve retained cases plus five matched cases. The CLI
scheduled, attempted and retained 28 arms: 26 recovered scalars and the two
expected incomplete outcomes. No arm is excluded from that flow.

## Evidence and reproduction

- [PLAN.md](PLAN.md) records the scope and cancellation limits before calls.
- [REVIEW.md](REVIEW.md) records source findings and the corrected reviewed hash.
- [development-receipt.json](development-receipt.json) binds commands, outcomes,
  scope and all 30 captured evidence artifacts.
- [artifact-inventory.postcall.json](artifact-inventory.postcall.json) lists
  byte sizes and SHA-256 values of those captured files.
- [Raw CLI JSON](evidence/attempt1/cli-matched.postcall.json) retains full
  Prime31 arms, exact accepted assignments, signed witnesses, replay, rank and
  phase records. This is compact JSON without compressed or omitted outcomes.
- [Raw test output](evidence/attempt1/integration-tests.postcall.log) retains
  the native suite, all F17 summaries and full prepared-interruption records.
- [F17 output extraction](evidence/attempt1/f17-outcomes.postcall.json) is a
  metadata extraction of those printed outcomes, not another solver run.
- [Bindings audit](evidence/attempt1/bindings-audit.postcall.json) records the
  51 source/input checks and unchanged immutable executable identities.

The example digest is
`dd965d7116f387ed9cd61cf79fedcee8230af768f3352a503d3da563d69d0868`.
The receipt digest is
`13014e0abf1b1e5333e6d7902bcf819ec1ff158e82c904f0161bc8fbcf27ec81`.
Build manifests and write-once pre-call records precede the separate copied
mode-555 executable calls. Post-call checks compare the same binaries and
sources. Both builds use one compiler job and the actual-source qualifier's
development opt-level 0. Neither guard durations nor test-runner elapsed
diagnostics are performance samples.

The exact build and immutable execution commands are in the prebuild and
precall JSON files. From the repository root, the relevant native entry
points are the example suite with --test-threads=1 --nocapture and
shared_s3_matrix --check-prime31-recovery-arms. F4 diagnostic counters are
process-global, so these arms and tests must remain sequential.

## What remains open

The new enclosing entry performs each arm's family and factor-base setup, but
their arithmetic and allocation are still unpriced. Other phase counters also
have explicit exclusions and overlaps. cost_counter_coverage_complete remains
false. Original equations, finite-prefix signed checks and the unrestricted
auxiliary domain do not supply the missing full-chart production dispatcher.

The [execution-gates record](../../../problems/shared-s3-prime31-execution-gates.md)
retains the full-library compiler/access gap, unreadable canonical protocol,
production adapter/accounting work and unestablished supported-runner
admission. No CI or paid job was launched, no timing sample was collected and
no signed scientific verdict was produced. Earlier evidence remains unchanged.
