# N83 D18 VSIDS solver-policy gate — frozen 2026-11-14 UTC

## Question and frozen input

Test whether VSIDS-only branching, with all 28 available Gaussian matrices
active, finds a checked relation on the primary `a=0` N83 curve. The prior
full-Gauss policy solved the constructed positive but its first public natural
query ended `INDETERMINATE` at one million conflicts. The sole policy change
here is `--branchstr vsids` in CryptoMiniSat 5.14.7; the D18 base, ordered
four-summand XCNFs, random seed, resource limits, and all other flags remain
fixed by `EXPORT_LOCK.json` and the prior natural solver lock. This is an L0
solver-policy diagnostic, not an end-to-end candidate comparison.

The immutable D18 object has 5,418,738 subgroup points and 32,643 signed
Frobenius columns. Its compressed SHA-256 is
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`.
The constructed positive XCNF SHA-256 is
`c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74`.
Its original CNF and XOR clauses and the group relation are the positive gate.
The natural XCNF is the previously frozen public target
`(1834913121178197242856238, 8797964190732846213981377)`, SHA-256
`bafed0f783ad93fcc91dcbd4c0ab45a505fc4ee3f2b0607a561e63995d0c89c3`.

## Execution and admission

Run the positive first. The source-bound worker and exact provider-checked
object must match the lock. Use one solver thread and seed 20261114, at most
one million conflicts or 600 solver seconds, plus a 660-second process-group
watchdog. Require three 15-second host samples with one-minute load no higher
than 24, unchanged swapouts, and at least 2 GiB free scratch. Record every
result, terminal status, complete stdout/stderr, wall and CPU, sampled RSS,
host samples, and hashes outside the repo at
`/Volumes/SSD990-2/tmp/n83-d18-vsids-20261114`.

Only a SAT model satisfying every original positive XCNF clause and XOR row,
followed by independent curve, subgroup, and relation-row replay, releases
the natural solver arm. Freeze its exact solver lock and runner in Git before
that arm. The natural arm retains the same caps and host gate. Any natural
SAT model must pass the original-XCNF verifier and independent group/row
replay before it counts as one relation. Retain `INDETERMINATE`, watchdog,
UNSAT, host rejection, and producer failure as distinct terminal records.

The prior full-Gauss observations are separate bounded controls. A positive
gate or one natural relation cannot select a factor base. Useful-rank yield,
full-width subgroup logs, unseen-target descent, complete cold cost, and
same-point rho remain required for the two N83 curve selections.
