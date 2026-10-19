# RunPod execution of the sealed 8,000-point N83 screen

The [six-arm protocol](PROTOCOL.md), public points, answer sidecar, source
SHA, exact factor bases, memory guard, mirrored order and 99% paired selector
remain fixed. This adapter runs the secondary diagnostic on one RunPod CPU
Pod after AWS rejected its Spot allocation. It does not turn the 8,000-point
average into a one-target IC/rho speedup row.

Allocate one new Secure Cloud `cpu3g-8-32` Pod with Ubuntu 22.04, at least four
visible CPUs and 20 GiB exposed memory. Require the provider's observed
hourly price at or below $0.40, and terminate after retrieving raw evidence
or at 165 minutes, whichever comes first. The maximum Pod compute charge is
$1.10. Keep the RunPod API key in the local controller and watchdog process;
do not copy it to the Pod, source tree or artifacts. Preserve every failed
allocation and partial run. Require the actual Pod ID, machine ID, cost and
source commit in an allocation receipt before computation.

The Pod clones the exact pushed commit with cone-mode sparse checkout,
including tracked root Cargo files and the `docs` files compiled into the
library, then verifies every
hash in `runpod_dispatch.json`, records OS/CPU/build receipts, compiles and
tests the pinned IC producer, and runs six fresh processes with the same
1,500-second per-arm cap. The global runner cap is 7,200 seconds, leaving
time for verification and archive transfer before the 165-minute watchdog.
`runpod_run.py` binds the Pod and machine to host/source receipts;
`runpod_verify.py` independently replays all bases, relations, scalars and
resource gates. Archive the raw results and build receipts, split into 48 MiB
parts, and record every part SHA-256 and the full archive SHA-256.
Use `runpod_retrieve.py --user <authenticated-pod-host-id> --output
<local-evidence-directory>` after the final part manifest is written. The
retriever uses the RunPod SSH gateway, verifies every part and the assembled
archive, and extracts only regular files under `results/` or `evidence/`.

After retrieval, `runpod_aggregate.py` validates all transfer hashes and
archive members, confirms the provider allocation against embedded host and
source records, reruns the independent verifier, and applies the unchanged
selector. The result is admitted only with complete evidence and all six
resource-eligible arms. Otherwise preserve the exact failure/cap and keep
the selected candidate ID null. This remains a Tier 1 cloud-instance
secondary comparison, with observed topology, affinity, cgroup throttling,
steal, frequency and A/B repeatability receipts.
