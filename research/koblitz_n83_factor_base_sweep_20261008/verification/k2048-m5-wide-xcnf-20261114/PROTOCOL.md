# N83 a=0 K2048 m5 wide-XOR diagnostic — frozen 2026-11-14 UTC

## Question and fixed input

On the exact retained K2048 five-summand XCNFs, test whether admitting
CryptoMiniSat Gaussian matrices up to 25,000 columns changes the bounded
constructed-positive and public-natural outcomes. The original
[`K2048 m5 protocol`](../k2048-m5-xcnf-20261114/PROTOCOL.md) fixes the curve,
339,968 subgroup-usable base points, 2,048 signed-Frobenius columns, witness
construction, public point and original-XCNF replay. This arm changes one
solver flag, `--maxmatrixcols 25000`. It does not regenerate either formula.

| Input | SHA-256 | Bytes |
| --- | --- | ---: |
| Positive XCNF | `f46a900495ec1a3594add973ee641de21a318a8fda6c9bfa67bd19057326d44c` | retained export receipt |
| Public-natural XCNF | `02d07716333e073f7eb40a7646ae3652b86b2129cb4d226f5dcc611d6bda7fff` | 20,511,067 |
| CryptoMiniSat 5.14.7 | `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac` | frozen binary |

The public-natural point is
`(1834913121178197242856238,8797964190732846213981377)` on the frozen
`a=0` curve. The existing native baseline returned `INDETERMINATE` at
1,000,001 conflicts and declined 20,916-column matrices. The prior
[`matrix-admission smoke`](../external-xor-heuristic-panel-20261025/RESULT.md)
admitted 20,916-column matrices on different N83 XCNFs and did not establish
a timing improvement. This arm tests the exact retained K2048 formula.

## Source, runner and ordered gates

Use `examples/n83_k2048_m5_native_gate.rs` from one clean committed source
snapshot and the binary built from that snapshot. First freeze a positive
lock with source commit, runner source/binary SHA-256, solver SHA-256, exact
positive formula/export receipt, an unused output directory and the flags
below. Commit the lock before the process starts. The host gate takes three
15-second samples with one-minute load at most 24, unchanged swapouts and at
least 2 GiB scratch. Preserve a rejected preflight.

Require the positive to return SAT and pass independent Rust replay of every
original CNF and XOR row, archived base point, subgroup and relation row.
Only then freeze and commit the natural lock, binding its exact XCNF, the
positive replay hash and a second unused output directory. Run one public
natural query. Preserve complete stdout/stderr, command, wall, wait4 CPU/RSS,
host samples, hash and terminal status. Replay a SAT model independently
before counting any relation.

For both calls use one thread, seed `20261114`, `--maxconfl 1000000`,
`--maxtime 600`, a 660-second process-group watchdog, `--maxnummatrices 32`,
`--autodisablegauss 0`, `--gaussusefulcutoff 0`, `--maxmatrixcols 25000`,
and VSIDS branching. The binary accepts only this exact wide flag vector
under lock arm `wide_25000`. The native baseline arm remains separately
bound to its earlier source and binary.

`INDETERMINATE`, watchdog, UNSAT without checked certificate, memory abort,
host rejection and producer failure retain their terminal statuses. This
single public query is a solver diagnostic; a common-query panel with verified
relations and novel rank, full-width logs, unseen-target descent, charged
online phases and same-point rho remain the factor-base selector gates.

## Sources checked

- The original K2048 m5 protocol/result and the 2026-10-25
  matrix-admission smoke give the exact local comparison and observed
  matrix-width issue.
- CryptoMiniSat 5.14.7's installed `--help` states that
  `--maxmatrixcols` caps Gaussian matrix columns and defaults to 1,000.
- Shared KB revision `a340f0566afc4abb83dd81cd3eef5b01` had indexed
  `source:007a1d17ad595712f30908a1c41cf26b` for SAT/Semaev context and
  `source:ad4d05b2a1e4c2678babfc4b27b3bb90` for the N83 arity catalogue.
  Neither source predicts this exact K2048 matrix-admission outcome.
