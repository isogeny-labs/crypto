# Goal
Run the published native trace/pullback and relation-recovery comparison on Runpod at a total attempt ceiling of USD1.

# Lead / hypothesis
Source-coordinate Sylow splitting avoids canonical divisor reconstruction. The complete incidence norm, public pool generation, failures, datum construction and witness costs must still be measured. Prior art and assumptions are in reports/PULLBACK.md.

# Spec
Source commit SOURCE_REVISION. Runpod PROTOCOL.json, SOURCES.sha256, job.sh and benchmark.cpp are the frozen job inputs. Original natural public fixtures and native kernel files remain unchanged. No validation-secret file is a solver input.

# State
Native development checks preceded this checkpoint. No Runpod pod for this request has been launched by this checkpoint workflow. Runpod credentials remain in the requesting executor's private temporary client configuration, never this repository or checkpoint. This workflow spends no Runpod credit and launches no experiment.

# Next steps
Read the normalized checkpoint receipt, require gate_exit zero and exact source/protocol/closure hashes, refresh live Runpod balance, create one bounded CPU pod using the published job, start external cleanup supervision, and collect all outputs. Preserve failed runs and original deadlines.

# Runs in flight
No pod submitted by this workflow. The requesting executor must record the actual pod ID, creation response, source/image/binary hashes, public-pool seeds, external supervisor session and deadline when launch succeeds. Existing unrelated pods must not be changed.

# Stop and success conditions
One pod, no automatic retries, USD1 ceiling, at most8 vCPUs/16GiB, at most1800seconds from creation, with a1500second task timeout. Stop sooner on completion and terminate the pod after collecting results. A missing/failed S3 gate stops launch. Success means source-exact completed or explicitly censored rows with public scalar checks, complete charged cost and cleanup, not an admitted speed or rho-parity claim.
