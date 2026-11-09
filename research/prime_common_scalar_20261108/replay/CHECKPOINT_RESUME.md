# Resume: native rational Hermite recovery

## Goal
Native correctness replay of the actual finite rational recovery primitive.
Follow with coordinate precision, efficient input jets and uniform recursive
bit-cost proof for the research objective.

## Lead / hypothesis
Constructed finite quotient buffers and rational Hermite sum preserve target
residue at original source child precision. See the Hermite recovery report,
sourcebd8cbfe23 and prospective replay/PROTOCOL.md. No asymptotic conclusion
from a bounded correctness census.

## Spec
research/prime_common_scalar_20261108/replay/protocol.json, frozen before data;
180positive cases plus3defective controls each, seed0, two physical invocations.
Native sourcecommit77aed44a5; instrument.json pins binary hashes.

## State
Lean source audit269 declarations, fresh FiniteRecovery exit0, unsigned.
Native checker and independent output recount compile. No census executed.
No AWS CLI or checkpoint upload token is available in this runtime.
PR3755 contains mathematical report; auto-merge enabled.

## Next steps
From /workspace/crypto, use replay/run.sh with absolute checker/analyzer/native
contract binary paths in instrument.json and /workspace/audit/common-scalar-contract-target/release/common-scalar-contract.
Launcher first validates protocol, hashes and checkpoint gate. Do not bypass a
failed S3 gate without explicit authorization covering that gate. Once admitted,
retain raw outputs and write schema-valid export/report; no third invocation.

## Runs in flight
None. Compilation completed. No experiment data or census process exists.

## Stop and success conditions
1CPU,2GiB,600s maximum each of2 invocations,32MiB output each; no retries or
cloud spend. First failure stops and preserves partial output. Acceptance is
all183 unique records each, correct controls, exact A/A bytes and native recount.

## Open questions
S3 checkpoint transport admission; coordinate lattice/fast jets/uniform costs.
