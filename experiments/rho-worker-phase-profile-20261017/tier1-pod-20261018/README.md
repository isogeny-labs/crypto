# Tier 1 Pod worker panel, 2026-10-18

The first 32-point, four-repeat hosted panel completed all 768 arms and 384 same-point pairs on RunPod Pod `94uxi8f876o5gz` (`cpu5m`, 8 vCPUs). Every arm and pair passed exact output and host-counter checks. The independent auditor left the aggregate speedup **unknown** because three A/A ratios crossed the frozen `[0.90, 1.11]` gate. The second attempt uses a separate frozen allocation receipt with Pod service processes pinned to housekeeping CPUs; preserve both attempts.

| Attempt | Arms | Valid pairs | Invalid arms/pairs | A/A failures | Memory failures | Decision |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 768/768 | 384/384 | 0/0 | 3 | 0 | `unknown_gate_failed` |

The three A/A exceptions were `q15-aa_batch` repeat 2 (1.15886), `q29-aa_serial` repeat 2 (0.88958), and `q11-aa_batch` repeat 3 (1.12253). Their CPU receipts remain in the raw archive. Diagnostic median charged times were 2,642.37 ms serial and 954.86 ms batch; the audit deliberately withholds an admitted ratio and interval.

- [attempt1.tar.gz](attempt1.tar.gz) is the full raw archive: all 1,551 entries, including each stdout/stderr file, ordered ledgers, preflight, plan, summary, source archive, built executable, toolchain/build log, and audit. SHA-256: `db41889205d031b031626616d3a3d8f41d3c7d0d56deba0e869195c00b686bc6`.
- [attempt1-audit.json](attempt1-audit.json) is the independent audit; [attempt1-build-receipt.json](attempt1-build-receipt.json), [attempt1-plan.json](attempt1-plan.json), and [attempt1-allocation.json](attempt1-allocation.json) expose the frozen provenance without extraction.
- Source bundle SHA-256: `ccc4c83128510e145bed6e8c04b26eb9a63f617bb2d3665826856ccd91902904`; Linux release executable SHA-256: `1bbbef507589180facc928ef6689bc474adadce5b3389a45d5116828479ef2a3`; plan SHA-256: `e14fd2916a1f066f661bcc0a1b983ad3d0436a200fc0b9603cb41d3cbc3e2def`.

The Pod's visible cgroup CPU set was `2-5,26-29`; measurement used CPU 5 and the runner used CPU 2. Physical-host exclusivity remains unproven, so this is Tier 1. The archive was created on the Pod after the panel and rehashed after transfer. To replay the independent audit, extract the archive on the original Pod path so the manifest's absolute artifact paths resolve, then run `analyze_tier1_worker_batch.py --result-dir /workspace/isolated-bench/results/5eb19735ce7948bfb81f4d140cf078b8 --output /workspace/rho-worker-audit-replay.json`.
