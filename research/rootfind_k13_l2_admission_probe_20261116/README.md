# K13 duration admission probe

The [protocol](PROTOCOL.md) and [spec](SPEC-K13-ADMISSION.json) define a
48-process x86 check of whether the batch rootfinding arm can meet ecbench's
1 ms solve-duration floor and L2 grade on the full panel's fixed K13 targets.
It is unrun. Its result will govern whether submitting the frozen ten-arm
panel on the same source and host is justified. It is not a solver comparison.

The [complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
tracks the unresolved isolated timing and pricing requirements.

The [read-only audit](REPORT.md), [visual PDF](report.pdf), and
[machine readback](DURATION_READBACK.json) document the archived L0 duration
signal that motivated this probe. Its PCC record is unverified; no x86
measurement has been made. [pcc:unverified]
