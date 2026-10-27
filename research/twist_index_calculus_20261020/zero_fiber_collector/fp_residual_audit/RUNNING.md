# Managed finite-residual audit owner

The native source and fixed 42-arm protocol were committed as
`be9af2af161025a4cd1b7ccfda9339e0a15f07a7` before production.
The eight synthetic tests, separate release build, formatting and source
seals passed. Frozen binary SHA-256:
`d0f56b52bddfe3ef9a0731c4fdaeb65daa9e9d311e71a60e851406840005137f`.

One managed owner was accepted at **2026-10-25T06:31:41Z**:

- Unit: `twist-zero-fp-residual-audit-v1.service`
- Main PID at acceptance: `3809049`
- Restart policy: no
- Raw output: `../runs/fp-residual-audit-v1/`
- Immutable gate: `../runs/fp-residual-audit-native-gate-v1/`
- Launch receipts: `../results/fp-residual-audit-v1/launch/`

The driver owns per-process common reservations for the first and A/A
execution. Each process receives one 1,200-second/8 GiB phase. Reservation
wait is recorded separately. Owner acceptance does not establish actual
science admission, completion, successful replay or any numerical outcome.
Read `metadata/terminal-exit`, both phase exits and parity before reporting
success. Do not launch another owner or reuse an existing output directory.

The three pre-existing recovery/certificate/degree owners remain unchanged.
The original 189+21 run is complete and is not repeated. Diem remains
expired at 50/52 records under its original deadline, and the old freeze
waiter remains quiesced. No generator, old science job, role, lease or
Conductor executor was reset or duplicated for this audit.

This is a counterfactual, native audit of existing complete records. No
filter has been deployed. Relation/rank preservation, historical partial
F4 counts, full cost, strongest IC/rho and formal group/embedding coverage
remain separate obligations; the full original request stays open.
