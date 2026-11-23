# Batch scheduling correction before the first child

**Frozen 2026-11-18 PST before target generation or solve execution.**
The [V1 protocol](PROTOCOL_V1.md) specified both one child per arm and round
with all 64 targets inside that child, and a six-arm rotation at each target
position. Those two scheduling instructions cannot both hold: one process
cannot pause after a target to interleave five other independent processes
while retaining a simple one-time setup and child ledger.

Keep the 18 whole-batch children, 64 target order, inputs, seeds, cap, arms,
and all acceptance and accounting gates unchanged. Rotate child launch order
by `round mod 6` through the fixed arm sequence
`direct-dual, direct-inverse, destination-direct, source-bsgs, rho-negation,
rho-negation-aa`. Within each child, visit positions 0–63. This balances the
first three launch positions across rounds, while rho A/A remains a direct
same-seed control. The current Darwin host has no physical-core isolation,
so the schedule does not make its wall times admissible. No child or input
was run under the inconsistent V1 schedule.
