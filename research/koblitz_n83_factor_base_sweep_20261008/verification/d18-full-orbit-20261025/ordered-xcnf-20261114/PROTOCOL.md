# Ordered D18 XCNF replay gate

**Frozen 2026-11-14 before export.** This gate tests whether the exact
ordered-first-pair D18 constructed control assignment satisfies a fresh
export of the original CNF and XOR rows. It does not time a solver or select
a factor base. The source change that adds the export-only command is commit
`72c98297b0ae990abbdf34f568451dbeac3f8dce`; the executable checkout
commit and binary SHA-256 must be sealed in a write-once `FREEZE.json` before
export. No algorithm source may change between this protocol and execution.

| Input | Frozen identity |
| --- | --- |
| Curve | primary N83 `a=0`, subgroup order `2417851639230796216685689` |
| D18 base | 5,418,738 points, 32,643 folded columns, point-set BLAKE3 `31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d` |
| Compressed object | `objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz`, SHA-256 `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b` |
| Public corpus | `pilot-01/probe-corpus.json`, SHA-256 `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd` |
| Positive target | `([6594052263204820946437856, 9593508870534830450764009])` |
| Native certificate | [attempt 7 positive-2](../native-cadence-replay-20261026/attempt-7/evidence/positive-2/result.json), 329,808 bits, packed low bit first, SHA-256 `e41a2df1bd8cbf940f04797318f071045bb547e7270991f92f2155979920ee3f` |

Use a clean checkout and the exact `primary-full-d18-m4-l0-ordered-xcnf`
`positive BASE_DIR FIXTURE_DIR NEW_OUTPUT_JSON` command. The worker's internal
wall cap is 300 seconds and XCNF cap is 400 MB; use a 310-second outer
process-group watchdog. The formula must have 329,808 variables, 2,807,999
CNF clauses, and 3,901 XOR rows, with `ordered_first_pair=true`, shared root
bits, no choice pins, phase-zero-first, and cadence 32. The expected extended
DIMACS header count is 2,811,900. A nonzero exit, cap, source mismatch, input
mismatch, or shape mismatch remains a retained failure.

Independently unpack the archived model, check its SHA-256 and completeness,
and replay **every original exported CNF clause and XOR row**. Check the
exported XCNF SHA-256 against the worker receipt and preserve the exact bytes,
receipt, phase log, exit status, binary hash, and source commit. A replay
failure is a failed gate; do not start a natural solver arm. If replay passes,
the next step is a separately frozen external solver arm on this exact XCNF,
with positive control before natural. A natural SAT model still needs field,
orbit, curve-group, subgroup, and relation-row replay. This gate cannot by
itself rank end-to-end IC factor bases on either N83 curve.
