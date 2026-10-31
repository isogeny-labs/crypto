# Eight exact root-set comparisons

Job `20261023-023821-twist-s4-f4-roots-c91c59`. Native execution and evidence only; no admitted elapsed-time comparison.

## Exact execution

Protocol commits: `509df89c5e`, `3ff04c056`, `40fbae5a7`, `e6fc49a7d`, as applicable to each stage. The actual executed source snapshot is retained in this job's raw tree.

Host: c7i.large Spot. Started 2026-10-23T02:39:29Z. Terminal phase `done`, exit `0`. All launched instances terminated. Exact command: [command.sh](command.sh), SHA-256 `75ed57283dd2f32c7bc4abbfba4e8dc99c9271fb48483336f6d5b0889f872ae1`. Host, environment and rusage receipts are in the extracted raw job tree.

## Recorded observations

All eight F4 field-root sets match complete Fp^3 enumeration and signed curve checks. Both missed relations are recovered. A coefficient mutation fails the frozen input hash before solving. 48 tests passed.

![Recorded native observations or execution diagram](graph.svg)

## Scope and limits

Structural and collector processes have an 8 GiB address-space cap and 1,200 s watchdog where invoked by the recorded command. Fixed native source and exact raw inputs are retained. No seed or failed stage is dropped. Proof-wrapper setup and extraction do not use the theorem process's cap. Counts shown are recorded native values, with no estimated intervals or timing ratios. F4 row-reduction counters are not complete solver or index-calculus costs. The final prefix A/A checks deterministic output, not timing isolation. Full cost, five-round isolated timing, strongest matched rho calibration and msolve/Magma comparison remain separate uncompleted gates.

[Round report](../../REPORT.md). [PDF](report.pdf).
