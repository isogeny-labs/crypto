# IC folded build, one packed stream a partition: A/B (2026-11-14)

These are copies of [`research/ic_tool_program/explorations/fold-pack-20261114/ab/`](../../research/ic_tool_program/explorations/fold-pack-20261114/ab/PROTOCOL.md).

- **Result:**
  - build two/packed: 1.244x at k0n53 and 1.252x at k0n61, both established against a same-session A/A;
  - set-up: 1.068x at k0n53, established; 1.038x at k0n61, not established.
- **No row in `benchmarks/README.md`:** a component A/B has no replay certificate or matched-rho reference, so it does not pass the validation gate.
- **Record:** `benchmarks/records/20261114T0422Z-ic-fold-pack-ab.json`.
