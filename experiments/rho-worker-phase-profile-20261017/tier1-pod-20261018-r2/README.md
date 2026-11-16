# Tier 1 Pod worker panel, second attempt

The second frozen 32-point, four-repeat panel completed all 768 arms and 384
same-point pairs on RunPod Pod `94uxi8f876o5gz` (`cpu5m`, 8 vCPUs). Every arm
and pair passed exact-output and host-counter checks. The independent auditor
returned `unknown_gate_failed`: the `q25-aa_batch` control at repeat 2 had a
ratio of `0.8944817743`, below the frozen `0.90` A/A floor. The aggregate
paired ratio and its interval are therefore `null`. No run was discarded.

The second allocation pinned permitted Pod service processes to housekeeping
CPUs `2,26`, measured on CPU `5`, and stopped the Pod nginx worker that could
not be pinned. This improved the observed A/A pass count from 253/256 in the
first attempt to 255/256 here, but did not pass the complete gate. The Pod did
not expose physical-host exclusivity; the measurement tier is Tier 1.

| Attempt | Arms | Valid pairs | Invalid arms/pairs | A/A failures | Memory failures | Decision |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| 2 | 768/768 | 384/384 | 0/0 | 1 | 0 | `unknown_gate_failed` |

Diagnostic median charged times were `2604.30 ms` serial and `947.31 ms`
batch. They are retained as phase diagnostics, not an admitted speedup. For
the failed A/A pair, the two arm records show equal verified work, zero major
faults and cgroup throttling, and CPU-frequency samples spanning roughly
`2.75–4.79 GHz`. Those samples do not identify a unique cause for the timing
variation.

- [attempt2.tar.gz](attempt2.tar.gz) contains the full 1,552-entry raw archive:
  ordered ledgers, 768 stdout/stderr pairs, preflight and summary, source
  bundle, built executable, build log, allocation and process-isolation
  receipts, and independent audit. SHA-256:
  `d168f3cfdc01a81aa2ac3913a99c808c99c1e0f38e12aed0158cde6001b8b57f`.
- [attempt2-audit.json](attempt2-audit.json),
  [attempt2-build-receipt.json](attempt2-build-receipt.json),
  [attempt2-plan.json](attempt2-plan.json),
  [attempt2-allocation.json](attempt2-allocation.json), and
  [attempt2-isolation.json](attempt2-isolation.json) expose the frozen
  provenance without extraction.
- Source bundle SHA-256:
  `ccc4c83128510e145bed6e8c04b26eb9a63f617bb2d3665826856ccd91902904`;
  Linux release executable SHA-256:
  `1bbbef507589180facc928ef6689bc474adadce5b3389a45d5116828479ef2a3`;
  second plan SHA-256:
  `3974c6c470d6fb86e49b3d8fb73ac886c9630f9c59faef65d62b3eee30676d48`.

To replay the independent audit, extract the archive at `/workspace` so the
manifest's absolute artifact paths resolve, then run
`analyze_tier1_worker_batch.py --result-dir /workspace/isolated-bench/results/c75ffdf0cf1d4e2b8bbb8a4763310a64 --output /workspace/rho-worker-audit-r2-replay.json`.
