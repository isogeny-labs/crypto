# K13 archived-clock boundary audit

Status: post-hoc, read-only diagnostic. The exploratory clock totals were read
before this plan was written; this is not a preregistered performance test.
It creates no new ecbench execution, timing sample, speedup claim, or change to
the frozen L2 admission and ten-arm protocols. [pcc:unverified]

## Input and selection

Read only the 453,126-byte archived JSONL object with SHA-256
`dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44`
from session `ECBS1hc322a24b6610`, source
`8f4cc824c87a236aafd28aa390c744e355a08e45`. Require its 48 records:
two Koblitz E0 n13 targets from seed `202611160511`, eight named arms, one
warmup and two measured rounds per target and arm. Select all 32 measured
records; retain the 16 warmups in the identity/count check. Reject a record
without the producer's affirmative answer flag, complete work/clock fields, exact
factor-base identity within the matched cells, or a nonpositive clock.
The independent scalar audit is
[`../rootfind_k13_l0_independent_audit_20261116/REPORT.md`](../rootfind_k13_l0_independent_audit_20261116/REPORT.md).

## Calculations and controls

For each arm, sum `time.solve_wall_ns`, `time.process_wall_ns`, and
`work.bitops` over the four measured rows. Define outside-solve time as
`process_wall_ns - solve_wall_ns` on each row, then sum. Record the minimum
and maximum individual solve clock. Use the A/A batch arm to check that
reported model work agrees cell by cell while clocks remain distinct. Check
each `(workload_sha256, round)` cell has all eight arms and the same target
and factor-base points digest. Report all eight arms, including rho and both
matrix engines; show the matrix pair on a separate scale in the graph.

The result is arithmetic over producer records. An L0 process clock is not
isolated or independently attested. A `work.complete` flag is the producer's
coverage assertion; this audit does not test that assertion. The next
discriminating checks are a source-bound instruction audit for priced work and
the frozen x86 L2 admission/full panel for wall time. [pcc:unverified]
