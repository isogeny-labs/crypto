#set page(paper: "us-letter", margin: (x: 0.7in, y: 0.65in))
#set text(font: "Helvetica", size: 9pt)
#set par(leading: 0.55em)

= N83 V5 ordered K600 zero-trial preflight

_Run `run-20261116-v5-ordered-preflight`, 16 November 2026 UTC._

The frozen cell was V5 ordinal `26960859236`, the primary K600 signed-Frobenius base with ordered-first-pair symmetry, six summands and zero relation trials. Source `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`; Linux ARM64 exporter SHA-256 `87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`. The exact 3,025,798-byte K600 S3 object was fetched and hashed independently before each worker invocation.

#image("FLOW.svg", width: 100%)

*Observed outcome.* Both workers returned `PREFLIGHT_ONLY` with strategy `ordered-orbit-chain-s3`, zero trials, zero relations and zero SAT calls. Attempt 1 ran 15:40:18–15:40:22 UTC, but its trap overwrote the Docker inspection after cleanup, leaving that admission check incomplete. A write-once amendment corrected the receipt path before attempt 2. Attempt 2 ran 15:43:07–15:43:11 UTC and preserved Docker exit 0, `OOMKilled=false`, 4 GiB memory cap, zero swap and 107,536,384-byte cgroup peak. These one-second-resolution host intervals are operational receipts, not comparative algorithm timings.

*Scope.* This confirms exact input and resource admission for the V5 ordered route. The cold single-target index-calculus runtime and selected factor base remain unset. Positive-control and natural-query relation trials, rank, target work, verification and matched rho remain separate future experiments.

*Evidence.* Frozen protocol and runners: `research/koblitz_n83_factor_base_sweep_20261008/verification/v5-ordered-preflight-20261116/`. Raw summaries and container receipts: `attempt-1/` and `attempt-2/` below that directory. The full source-linked Markdown report is `research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261116-v5-ordered-preflight/REPORT.md`.
