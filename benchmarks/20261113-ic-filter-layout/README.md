# IC partition-local presence filter: A/B (2026-11-13)

These are copies of [`research/ic_tool_program/explorations/filter-layout-20261113/ab/`](../../research/ic_tool_program/explorations/filter-layout-20261113/ab/PROTOCOL.md).

- **Result:** the build phase is faster with the top-bits filter layout, 1.104x at k0n53 and 1.142x at k0n61; both are established against a same-session A/A. The whole set-up is not established.
- **No row in `benchmarks/README.md`:** a component A/B has no replay certificate or matched-rho reference, so it does not pass the validation gate.
- **Record:** `benchmarks/records/20261113T1855Z-ic-filter-layout-ab.json`.
