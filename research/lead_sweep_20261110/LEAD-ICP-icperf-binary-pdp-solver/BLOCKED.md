# BLOCKED: LEAD-ICP-icperf-binary-pdp-solver

Missing (PROTOCOL.md section 5; phase1_index.json "blocked_on"):
- Tools: BRiAl (C++ Groebner engine) and WDSat (CDCL+XOR) builds. No installs or builds are allowed in this sweep.
- Data: Weil-descended Semaev-S3 PDP ANF generator for a grid of n across the ICPERF boundary (~300 to 409).
- Host: a large-memory host; the ANF exceeds 2^31 monomials near the writability limit. The sweep limit is stdlib-only, 200 MB.
- No run.py exists and run_cmd is empty.

Smallest unblocking action: submit a frozen task card through isolab-eks (EKS Spot) that builds BRiAl and WDSat in an image, generates the descended ANF for a small n grid, and runs the protocol's null, positive (planted key) and A/A (two hosts) controls with experiment.protocol/v1 frozen first. Needs a resume checkpoint and a benchmark record per repo rules.
