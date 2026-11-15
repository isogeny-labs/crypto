# r=27 attempt-04 placement investigation

Captured at 2026-10-31T07:51:56Z. The Job remains pending with required anti-affinity against other lab Jobs. The known active Semaev pod carries `isolab.dev/job=true` and is on a `cpu-spot` node, matching the producer's node selector. The cluster-scoped node list is forbidden to the worker identity, so this package does not claim a complete node inventory. It records exact permitted API responses and filtered pod details to guide the next scheduler action.

## Tree

- `r4-status-tool.json`: Status API response for the N83 producer.
- `blocking-job-status-tool.json`: Status API response for the already-running Semaev Job.
- `r4-pod-placement.json`: Filtered Kubernetes pod spec and scheduling status.
- `blocking-pod-placement.json`: Filtered Kubernetes pod labels, node placement and pool selector.
- `node-list-rbac-denial.json`: Exact cluster-scoped node-list authorization response.
- `results-listing.json`: S3 results-prefix inventory.
- `container.log`: Bounded container log payload.
- `DATA_INDEX.json`: per-file SHA-256, byte length, and role.
- `FORMAT.json`: machine-readable package schema and interpretation limits.
- `manifest.sha256`: checksums of the evidence and documentation files, excluding itself and DATA_INDEX.

The observation identifies an occupied compatible node but does not establish that no additional eligible node can be provisioned. No Job was modified or cancelled.
