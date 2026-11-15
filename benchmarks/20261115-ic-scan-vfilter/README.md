# IC aimed scan, block-filled filter tests: A/B (2026-11-15)

These are copies of [`research/ic_tool_program/explorations/scan-vfilter-20261115/ab/`](../../research/ic_tool_program/explorations/scan-vfilter-20261115/ab/PROTOCOL.md).

- **Result:**
  - collect per-key/block: 1.086x at k0n53 and 1.097x at k0n61, both established against a same-session A/A;
  - set-up: 1.086x at k0n61, established; 1.049x at k0n53, not established.
- **No row in `benchmarks/README.md`:** a component A/B has no replay certificate or matched-rho reference, so it does not pass the validation gate.
- **Record:** `benchmarks/records/20261114T2243Z-ic-scan-vfilter-ab.json`.
