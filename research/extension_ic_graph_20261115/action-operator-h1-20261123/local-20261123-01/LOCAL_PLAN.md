# Authorized local H1 fixed-prefix control — run 01

User instruction on23November2026: **Run locally**. This selects local
execution for the prepared H1 comparison after the Runpod/checkpoint
blockers were reported. It supersedes the prior Runpod-only execution
location for this bounded component run. Original plan, earlier reports
and all consumed trials remain unchanged.

The S3 checkpoint gate was attempted and exits1: no upload token or AWS
profile is available. This explicitly requested local execution uses a
published Git checkpoint, read-only executable copy, exclusive execution
marker and SHA-bound local artifacts. It is not S3-admitted evidence and
does not claim isolation, complete cost accounting or rho parity.

## Frozen computation

Use the already qualified executable without source changes:
`836600c5193e8b019f8d06cd495dcac50417b389423d60cd49dc33db8d586fe3`.
It compiles the current H1 modules against the pinned Root41 library;
source/library/binary manifests in the parent validation packet recheck.

Input is original Run51 unit raw, SHA-256
`6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d`.
Retain exactly the first32 original six-coordinate polynomial systems
over F1009. No new target or relation search. At most32 full F4 calls,
degree24 without early staircase/Hilbert stop; deadline30seconds per call.
Reject incomplete completion flags. On each basis, run one pinned
incumbent and six candidate action-coordinate extractions, preserving
dense fallback and its multiplication counts. Same seed389 throughout.
Check all roots against saved coordinates; check default non-wall counters
and subsequent RNG state against the incumbent.

Development rows0..15 choose the minimum existing partial `lin_muls` sum,
ties preferring incumbent5 then the lowest coordinate. Write selection
before held-out rows16..31. Retain every candidate, all failures and all
development-selection counts. This is a same-fixture exposed-prefix holdout,
not fresh-instance evidence. The metric is an incomplete multiplication
proxy, with no runtime or complete IC cost conclusion.

## Execution and custody

- Root: `/workspace/ic-local-runs/action-operator-h1-20261123-01/`.
- Read-only worker copy: `worker`; output directory `output` must not exist.
- Once-only marker: `execution.claim`, created exclusively before launch.
- One thread, 8GiB virtual-memory limit, outer timeout120seconds with forced
  cleanup after five seconds. Preserve exit status, stdout, stderr and all
  partial files. No retry or replacement prefix on failure.
- Environment: `RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 F4_FP_THREAD_OPS=0`,
  `F4_STAIR=counter F4_FP_NARROW=2 F4_FP_SYMBOLIC_PAR=1 F4_FP_PACKED=1`,
  `F4_FP_GM=1 F4_FP_DEFERRED_PAR=1 F4_FP_FL=1 F4_FP_FL_BLOCKED=1`,
  `F4_FP_LINEAR_INTERREDUCE=1`.
- Command: `worker --fixed-prefix ORIGINAL_UNIT_RAW output`, inside the
  guarded shell wrapper. Frozen source and prior tests are not rerun.
- Generated bases and raw cases remain LOCAL with hashes if oversized;
  compact exact outcomes and a source-linked report land in PR4986.

No new elapsed-time metric is exported. Internal deadline clocks and
custody timestamps remain. Full solver/reference/fresh-target and signed
admission obligations remain separate. Sources are the H1 source and
literature packet; KB/Conductor retrieval limits are unchanged.
