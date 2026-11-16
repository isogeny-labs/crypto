# K13 duration admission probe

The original [protocol](PROTOCOL.md) and [v1 spec](SPEC-K13-ADMISSION.json)
are preserved. [Amendment 1](AMENDMENT-1.md) corrects the native EKS admission
requirements before data; [v2](SPEC-K13-ADMISSION-v2.json) is the intended
144-process, three-arm, five-measured-round x86 probe. It checks whether the
batch rootfinding arm can meet ecbench's 1 ms solve-duration floor and L2
grade on the full panel's fixed K13 targets. It is unrun. Its result will
govern whether submitting the frozen ten-arm panel on the same source and
host is justified. It is not a solver comparison.

The [complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
tracks the unresolved isolated timing and pricing requirements.

The [read-only audit](REPORT.md), [visual PDF](report.pdf), and
[machine readback](DURATION_READBACK.json) document the archived L0 duration
signal that motivated the original probe. Their 48-process design description
is historical; the [amendment PDF](amendment.pdf) shows the corrected plan.
Its PCC record is unverified; no x86 measurement has been made.
[pcc:unverified]

[`validate_v2.py`](validate_v2.py) reproduces the
[static EKS predicate and identity check](VALIDATION.json). The native
launcher and a source-bound `ecbench plan` remain execution prerequisites.
