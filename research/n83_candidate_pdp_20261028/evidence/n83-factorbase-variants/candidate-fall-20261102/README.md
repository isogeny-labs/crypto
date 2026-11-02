# Candidate first-fall panel, attempt 01

This directory runs the existing frozen `N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2` procedure on the 144 already-generated PDP systems for `spike35`, `spike36`, and `spike37`. Each candidate uses the same fixed 2^27 x-coordinate domain across m=2..9. The cohort contains all three curve families and both natural and planted targets.

The preflight verifies every `inputs.json` and `system.sing` against the attempt-08 dataset index and the source SHA recorded in each PDP row. It also pins the V2 protocol freeze digest and the release `fall` binary digest. Each first-fall invocation has a 60 second timeout and 8 GiB address-space cap. Every completed certificate is replayed in a fresh process. Failures and timeouts remain as attempt records.

This run records standard first-fall certificates only. It does not complete the V2 panel's separate degree-two closure work, and no LFD, affine degree of regularity, solving degree, or factor-base ranking is inferred. The exact scientific definitions and assumptions are documented in [the frozen V2 protocol](../../../protocol-pdp-factorbase-v2.json), [the original fall design](../../../../n83_ic_degree_sweep_20261027/fall/DESIGN.md), and the primary sources listed there: [Kosters–Yeo](https://arxiv.org/html/1503.08001) and [Caminata–Gorla](https://arxiv.org/html/2112.05579).

## File structure

See [FILE_LAYOUT.json](FILE_LAYOUT.json) for the machine-readable layout. `run_first_fall.py` performs the pinned, source-checked production run. `analyze_run.py` independently recomputes counts and certificate hashes after production. Under `attempt-01/`, the preflight and source index are committed before data collection. Each `cases/<candidate>/<system-id>/attempt.json` preserves the exact command, output, input identity, outcome, and fresh replay. The aggregate summary and independent validation are derived only from those immutable case records.

The immutable source `system.sing` files remain in the attempt-08 content-addressed dataset at the hashes recorded in `attempt-01/source-index.json`; they are not copied into the result bundle. Certificate objects are uploaded individually to `s3://setec-astronomy-artifacts/runs/n83-ffd-lfd-factorbase-panel-20261102/v2/<candidate>/first-fall-attempt-01/certificates/`. The completed Git bundle, S3 custody receipt, and byte-readback evidence are added after execution.
