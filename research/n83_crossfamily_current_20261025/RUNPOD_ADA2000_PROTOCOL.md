# N83 current-source cross-family Ada 2000 Tier 1 allocation

This protocol runs the existing sealed 16-point, four-candidate cross-family
panel on two independently identified RunPod machines. The candidates,
single-target workload IDs, arm order, child caps, IC/rho correctness checks,
Tier 1 telemetry, verifier and selector are exactly those in [README.md](README.md)
and `run.py`. Each stratum has eight public points; its IC and same-point rho
children alternate in the frozen schedule. The two strata must overlap in
wall time. This hardware profile changes only the allocation envelope.

## Frozen allocation envelope

- `--hardware-profile ada2000` requests one `NVIDIA RTX 2000 Ada Generation`
  per Pod, at least 27 GiB system RAM and four vCPUs, using
  `runpod/base:1.0.2-ubuntu2404` on Community Cloud. The current catalog
  displayed 31 GB RAM, six vCPUs and $0.24/hour on 2026-10-27; the provider
  readback, rather than that display, is authoritative for each allocation.
- Two distinct Pod IDs and two distinct `machineId` values are required.
  One machine hosting both cards fails the gate and both Pods are deleted.
- Container disk is exactly 30 GB per Pod, with zero persistent volume.
  The provider compute rate must be at most $0.25/hour per Pod. Six hours
  at both rate ceilings plus container storage at $0.10/GB/month, using a
  conservative 28-day month, rounds upward to $3.06. The controller's
  projected allocation cap is $3.10. These rates and projected charges
  are frozen before the first provider mutation.
- A provider `terminateAfter` deadline is requested at 360 minutes. An
  independent local deletion process must pass provider readback before
  allocation. Each Pod starts its own `runpodctl` deletion process before
  build or measurement. Both guards retry deletion and retain receipts.
  Provider expiry is not an enforced billing ceiling; if both guards and
  provider access fail, charges can exceed the projection.
- The source checkout must be clean at its committed HEAD. The allocator
  independently regenerates the historical fixture/exclusion inventory,
  verifies the frozen IC source digest and a content-hashed source bundle,
  and rejects a matching pre-existing Pod name. The Pod rechecks extracted
  source, exact workload IDs, subgroup membership, and all fixture scalars.

The 1,200-second IC and 120-second rho child caps, 300-minute producer cap
per stratum, and 15.25 GiB child RSS cap are unchanged. Every failed,
incomplete or noisy pair remains in the raw archive. `aggregate_runpod.py`
requires both distinct machines, all admitted Tier 1 host receipts,
independently verified answers and the frozen selector before it can name a
winner. The primary `a=0` N83 curve remains a separate comparison.

The allocator's read-only invocation reports the exact source head and the
maximum projected cost. Allocation requires separate approval for this
RunPod profile and a working controller API key; neither is supplied by this
protocol. The key must remain in the local RunPod configuration or process
environment and must never be stored in source bundles or receipts.

```sh
python3 -m research.n83_crossfamily_current_20261025.source_bundle \
  --build /absolute/path/outside/repository/n83-ada2000-source-bundle
python3 -m research.n83_crossfamily_current_20261025.allocate_runpod \
  --hardware-profile ada2000 \
  --source-bundle /absolute/path/outside/repository/n83-ada2000-source-bundle \
  --output /absolute/path/outside/repository/n83-ada2000-allocation
# After approval and controller authentication, add --execute to allocate.
python3 -m research.n83_crossfamily_current_20261025.dispatch_runpod \
  --allocation-dir /absolute/path/outside/repository/n83-ada2000-allocation \
  --source-bundle /absolute/path/outside/repository/n83-ada2000-source-bundle \
  --output /absolute/path/outside/repository/n83-ada2000-dispatch --stage-and-watch
```

The hardware type ID is from [RunPod's GPU type catalog](https://docs.runpod.io/references/gpu-types);
the storage rate is from [RunPod's Pod pricing contract](https://docs.runpod.io/pods/pricing).
