# Round24 execution

## What was done
T-1411 submitted the frozen panel once after reconciling the exact job, inputs, results and capacity. All 180 executions completed: 162 measured, 18 warmups and 18 cells. Full certificates are byte-identical across arms. All 379 worker-manifest entries verified before native export; the post-export check is retained in result-hash-check.log. The native exporter reran the frozen analyzer and validated row coverage and returned metadata.

## Decisions made
The owner's continuing unlimited budget authorization covers this attempt and subsequent CPU optimization experiments without separate spending approval. Round24 retained 2 vCPUs, 4 GiB, a 300-second deadline, zero retries and the unchanged one-node cap. It completed in 74 seconds including provisioning; the container ran five seconds. Historical qualification and preparation records remain intact; this outcome supersedes their pending operational status.

Early residual rejection increases complete medians by 1.00%, 10.34%, 10.72% at batch64 and 1.00%, 3.35%, 4.01% at batch1, computed as 100*(candidate/parent-1). Fixture0 ranges overlap at both batch sizes; fixture1 batch1 also overlaps. Fixture1 batch64 and fixture2 both batches do not overlap. Baseline A/A ranges overlap in every fixture. These within-panel L0 observations select the suffix-only parent for further work. No earlier timing rows are pooled.

## Open threads
Publish raw evidence before the inspected report and update PR #2980. Remove the regressing early-rejection path from the current implementation while retaining its frozen source and result records. Continue complete-cost optimization from the suffix-only parent under the continuing authorization. The next candidate must retain full certificate checks and a frozen comparison.

## Errors
Initial placement waited for normal Spot provisioning. The UID1000 username warning was nonfatal. No missing executions, retries or deadline changes occurred. A local draft submission-request file contained overescaped shell characters; it was not submitted. The actual submitted argv was saved separately and is preserved here as submission-request.json. Frozen bundle bytes were unchanged. Raw kernel whitespace is preserved to retain manifest validity.
