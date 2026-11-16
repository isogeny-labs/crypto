# Local scale amendment, filed before performance calls

2026-11-15. The user answered **“Yes”** to the explicit question: “May I run
isolated local scaling with Git-only checkpoints?” This authorizes the local
backend and replaces S3 checkpoint custody with committed/pushed Git custody
for this study. It does not authorize cloud spending or change the workload.

The original `protocol.json` is retained as the prospective EKS design.
`protocol-local.json` will bind this amendment, native analysis, actual calibration
and exact executable before the main scale data. No performance call preceded
this amendment. The original development evidence remains byte-equal.

- Preserve all 20 two-term configurations and the 12 three-term configurations
  in PROTOCOL.md. Main run: 960 two-term query rows plus 576 three-term rows and
  576 secondary two-term rows inside the three-term panel, including warmups.
  Labels prevent pooling repeated configurations across panels.
- Calibration is a separate `preflight` panel: word31, m=32, two warmups plus
  one retained target call, four arms. It estimates launcher/fixed cost only;
  exclude it from scale summaries and do not use it for a comparative claim.
- Use unmodified `tools/isolated_bench.py run --cpus 4` with default settle,
  other-CPU and PSI thresholds. Evict other movable threads, pin the native
  qualifier, record any unmovable threads and contention, and run no builds,
  tests or heavy analyses while the scale panel executes.
- This 5-vCPU KVM VM exposes no `/proc/pressure/{cpu,memory}`. Missing PSI is
  recorded as null, not zero. **Local clock admission is below L2.** Counts
  remain primary; clock intervals are diagnostic only. Do not claim an L2
  speedup or relax any threshold to admit a busy host. This limitation is
  intrinsic to the selected local backend, not an EKS substitute certificate.
- Main attempt: original 3600-second deadline, no retries, 8-GiB address-space
  cap, at most one native worker. Preserve every timeout, failure and missing
  row. Restore CPU affinities after the driver returns. No source changes or
  builds during the timed interval.
- Copy and hash the qualifier, replay, analyzer and shell driver before calls;
  execute the copied qualifier. Use distinct build, pre-call and post-call
  manifests and refuse replacement of a pre-call manifest. Publish reproducible
  source, lockfile, compressed executable copies and data in Git as checkpoints.
- Independent-process replay covers both two- and three-term complete sets via
  wide scalar arithmetic and scalar pair-table enumeration. This is still a
  producer check; independent signed PCC standing is not supplied by it.
- Frozen native analysis reports counts, descriptive quartiles, matched warm and
  cold ratios, A/A ratios, failures and flow separately by panel, field, m and
  target population. Identity-target results remain separately labeled.
- Benchmark records retain raw hashes. Any default S3/independent-verdict gate
  remains explicitly unmet/waived; no canonical frontier or admitted leaderboard
  row is promoted from this local diagnostic study.
