# Retained preflight: combined accounting and timing is invalid

Session: `ECBS1h86a2a793b270`

Directory: `sessions/common-cost-local-lt-l2-20261031T064500Z`

Producer binary SHA-256:
`ad1ca4bf73b8a37864ed4f76c31ada3cee62549642db4feb792b8311305c4096`

This dirty-tree L0 preflight is retained because it falsified the first
measurement design. It must not be used for a speed comparison.

The run completed all 768 planned records and verified every result. The
all-measured replay reproduced 640/640 records byte-for-byte, with audit
receipt SHA-256
`d82f1b3c2e934278f6989fd15aea2b2e95b9c32b01115a0dc7b6af929202d75a`.
Every measured record was L0, and hardware counters were unavailable with
`perf_event_open: No such file or directory`.

The preflight activated the thread-local field ledger inside the existing
destination timing methods. Because each field operation updated a string-keyed
map, the ledger dominated wall time. Across `n=9,13,19,23`, paired median
destination/source wall ratios were approximately:

| `n` | destination BSGS / source | destination rho / source |
|---:|---:|---:|
| 9 | 35.2 | 79.5 |
| 13 | 105.4 | 234.1 |
| 19 | 453.5 | 1,021.7 |
| 23 | 1,099.6 | 2,967.3 |

These ratios measure instrumentation overhead, not the transfer. The field
counts themselves are useful accounting diagnostics, but the wall samples are
inadmissible. The preflight also used five measured alternating rounds, which
the runner correctly warned were order-unbalanced.

The accepted design therefore uses distinct method identities:

- `jacobian_image.rho3` and `jacobian_image.bsgs` for timing, with no field
  tally;
- `jacobian_image.rho3_field_tally` and
  `jacobian_image.bsgs_field_tally` for deterministic native accounting, with
  timing explicitly invalid; and
- six measured rounds in the paired-host timing spec.

The session's embedded `spec.json`, `plan.json`, binary hash, dirty flag, and
complete records are preserved so this negative control cannot be mistaken for
or overwrite the final protocol.
