# Complete cold timing handoff

## What was done
Revalidated terminal cold/native correctness outputs and exact regenerated
objects; recorded coldbuild13 results before this experiment. Copied its worker
with timing boundaries and added nativeFast/Hybrid/Compact controls. Built Rust
write-once20cell collector and frozen protocol. No timing execution yet.

## Decisions made
Same public source pair, one binary, five alternating sequential blocks;
core and processwall separate. Compare to best native median, not a weaker arm.
All construction/validation included; metadata emission outside core explicitly.
L0/unisolated, producer unvalidated, no broad10x claim. No science in Python.

## Open threads
Source/fingerprints commit, open benchrec and checkpoint; run collector once,
archive/uploadraw, report exact outcome including gate failure/censoring. Publish
both cold correctness and cost reports/benchmark entry/PR4274/current-headCI.

## Errors
None in benchmark build. Prior cold compile adapters fixed before source gate.
TaskT-6257 scopes held; .benchrec bookkeeping separately checked/claimed.

## Registered measurement
Sourceedf84d571f5b/checkpoint185949 preceded one benchrec-wrapped Rust collector,
20/20cells valid, allk183049. Cold median1.780974708s; compact0.000797166s(best),
fast0.001452792s,hybrid0.001188166s. Best/cold0.00044760096615589,10xgatefails.
No rerun. All rawstdout/stderr retained. Source read onlyPUBLIC per freshchild.

## Post-run reduction and decision
Rust analyze.rs (compiled --edition2021 after an initial edition mismatch)
produces no-map residual median240.507208ms and solve/descent/verify52.211917ms.
Both exceed native total substantially. These are descriptive counterfactuals
from per-cell differences, not a new run or preregistered gate. Reject10x for
this implementation/instance; next mechanism must address more than mapping.
Publication claim refreshed after old lease expiry; scopeT-6271 epoch1 confirmed.

## Publication closure
Raw archive uploaded using configured ecc-challenges-new profile after default
credentials returned AccessDenied. Record closed measured (scientific gate fails),
content-addressed SHA fce76687a443082c06b28f2986e5cf2634fe1c1971a4e1c956b05724e3ad7e07.
Both authored reports rendered and inspected; benchmark copies and unadmitted
README entry retained. No measurement rerun. Publication task T-6282 epoch1.

## Custody checks
Current record passed existing benchrec.record_errors with require_closed and
verify_s3 enabled. Whole-tree benchrec check reports1862historical problems
across186records, primarily absent sparse-checkout artifacts; none names the
new record. PR prose lint passes with zero findings. git diff --check passes.
