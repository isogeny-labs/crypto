# Resource receipts for later watchdog caps

Review of the frozen `full-m4-v1` worker found that its watchdog cap
contains identity, status and elapsed time, while CPU/RSS readings exist
only in the durable progress samples. Its final cgroup memory peak is
absent. The full-panel auditor therefore reports the latest durable
readings as lower bounds and retains the unobserved interval between
the last sample and cap. It does not reconstruct a missing final peak.

The current source adds the following to **future** watchdog receipts:

- Process peak RSS and accumulated process CPU from `getrusage`.
- Cgroup `memory.peak`, read immediately before the cap timestamp.
- Explicit nulls and error fields if a resource reading is unavailable.
- An accounting note that the timestamp precedes final serialization
  and the separate progress stream carries solver counters.

The resource-read failure path still writes `UNKNOWN_budget`; it does
not discard the censor or invent a zero measurement. A native regression
test covers missing, malformed and valid cgroup files and verifies the
retained receipt after serialization. No rank, model or completed-search
claim is added to a cap.

This change was made after the full-size card began. That card continues
with source `d9ef42301cc3d22f437e1f89304e673836afb6a3` and worker SHA-256
`d4324884fdd52ee05a82e2f053708d320d8255591ec61640b700a8a6684a0486`.
Its source snapshot, binary, receipts and interpretation are unchanged.
Any subsequent measured run needs a new source/binary pin and frozen
command card. This implementation has no measured speed or memory result.
