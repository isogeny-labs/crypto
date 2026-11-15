# IC folded build, one packed stream a partition: A/B (2026-11-14)

These are copies of [`research/ic_tool_program/explorations/fold-pack-20261114/ab/`](../../research/ic_tool_program/explorations/fold-pack-20261114/ab/PROTOCOL.md).

- **Result:**
  - build two/packed: 1.244x at icv1-f2m53-tm56619371-dac20a85 and 1.252x at icv1-f2m61-t158598901-ab42b6c5, both established against a same-session A/A;
  - set-up: 1.068x at icv1-f2m53-tm56619371-dac20a85, established; 1.038x at icv1-f2m61-t158598901-ab42b6c5, not established.
- **No row in `benchmarks/README.md`:** a component A/B has no replay certificate or matched-rho reference, so it does not pass the validation gate.
- **Record:** `benchmarks/records/20261114T0422Z-ic-fold-pack-ab.json`.
