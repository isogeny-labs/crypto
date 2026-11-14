# D18 ordered-XCNF full-Gauss capacity probe (frozen 2026-11-14 UTC)

## Question and input

Test whether enabling every available CryptoMiniSat Gaussian matrix and keeping
Gauss active lets the existing D18 ordered-first-pair **constructed positive**
formula pass its external solver gate. The earlier bounded run formed 28 XOR
matrices, selected five under the default `--maxnummatrices 5`, and stopped at
the conflict cap. This is a solver-policy diagnostic on the primary N83 `a=0`
curve, not a factor-base selection row.

Use the exact archived positive XCNF and receipts pinned by `EXPORT_LOCK.json`:
source commit `afebef39283466fca4ddff01063d26df9b59c45e`, D18 object SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
and 50,523,891-byte XCNF SHA-256
`c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74`.
The original native witness replay must say `PASS_ORIGINAL_XCNF_REPLAY`.
Only the external solver executes in this probe; the exporter source is retained
as an immutable Git object and checked through the export receipt.

## Frozen execution

CryptoMiniSat 5.14.7 binary SHA-256
`a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`;
one thread, random seed 20261114, `--maxnummatrices 32`,
`--autodisablegauss 0`, `--gaussusefulcutoff 0`. Stop at the first of one
million conflicts or 600 solver seconds; a separate 660-second watchdog kills
the process group. Capture full stdout, stderr, command, status, CPU and wall
time, sampled peak RSS, and swapouts. Require three 15-second host samples:
one-minute load at most 24, unchanged swapouts, and at least 2 GiB scratch.
This is an L0 shared-host capacity observation.

`run_cms.py` and `EXPORT_LOCK.json` must be committed before execution.
The output root is `/Volumes/SSD990-2/tmp/n83-d18-fullgauss-20261114`; it
links the immutable positive export and replay under `export-positive` and
`replay.json`, and writes a new `solver-positive` directory. The script refuses
an existing solver output directory and verifies all pinned hashes before
launch. Keep all outcomes, including indeterminate, UNSAT, watchdog, and
producer failure.

## Promotion gate

Only a terminal SAT model independently satisfying every original CNF clause
and XOR row, followed by independent elliptic-curve group replay, enables a
separately frozen natural-query arm with the same solver policy. Otherwise
record the capped positive result and leave natural-query yield unknown.
Neither outcome supplies relation rank, subgroup logs, target descent, or a
same-point rho comparison; these remain necessary for both-curve selection.
