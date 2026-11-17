# Goal
Extend incidence and labelled-pool generation with faster native candidates and compare complete cost on Runpod, within the existing USD1 combined lane ceiling.

# Lead / hypothesis
Tail splitting enables explicit FLINT Brent-Kung on equal-degree inputs. Batched fixed-window public label multiplication and Montgomery pair lookup may lower complete cost; matched ablations and A/A remain required.

# Spec
Exact published source SOURCE_REVISION. accelerated/PROTOCOL.json, SOURCES_v2.sha256, job-v2.sh, bootstrap-v2.sh and launch-v2.sh are new siblings; closed experiment sources and proof manifests remain unchanged. No validation-secret file enters any timed solver.

# State
No accelerated paid run has been allocated. Native development checks and their status are in accelerated/development/. The earlier closed experiment's admission cannot admit this source. This workflow only checkpoints source and prepares narrow upload credentials.

# Next steps
Require successful normalized S3 receipt with exact source, protocol and closure hashes. Check current Runpod API access, available credit and actual CPU price. Launch one bounded CPU pod, supervise from outside, mirror rows, collect/upload/readback archive and delete only that pod on completion or original deadline.

# Runs in flight
None from this workflow. Actual pod, creation time, rate, source, supervisor and deadline must be recorded by the requesting executor. Never modify another workload.

# Stop and success conditions
One new allocation, no retry, 8vCPU/16GiB, maximum1800seconds creation-to-deletion, worker timeout1500seconds, maximumUSD0.25 compute inside the existingUSD1 combined cap including earlier expenses and storage. Failed S3 admission or API access stops allocation. Success requires exact outputs and complete charged costs; no signed standing, isolation tier, priority or fastest-tool claim is implied.
