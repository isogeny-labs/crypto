# K13 duration admission probe

The original [protocol](PROTOCOL.md) and [v1 spec](SPEC-K13-ADMISSION.json)
are preserved. [Amendment 1](AMENDMENT-1.md) corrected the native EKS shape;
its [v2 spec](SPEC-K13-ADMISSION-v2.json) is historical. [Amendment 2](AMENDMENT-2.md)
fixes the coverage gap: [v3](SPEC-K13-ADMISSION-v3.json) is the intended
288-process, six-arm x86 probe of **every non-Gröbner arm** in the frozen panel,
including its actual rho reference. It checks ecbench's 1 ms solve-duration
floor and L2 grade on all eight fixed K13 targets. It is unrun. Its result
will govern whether submitting the frozen ten-arm panel on the same source
and host is justified. It is not a solver comparison.

The [complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
tracks the unresolved isolated timing and pricing requirements.

The [read-only audit](REPORT.md), [visual PDF](report.pdf), and
[machine readback](DURATION_READBACK.json) document the archived L0 duration
signal that motivated the original probe. Their 48-process design description
and the [v2 amendment PDF](amendment.pdf) are historical; the
[v3 amendment PDF](amendment-v3.pdf) shows the intended coverage.
Its PCC record is unverified; no x86 measurement has been made.
[pcc:unverified]

[`validate_v2.py`](validate_v2.py) reproduces the
[historical v2 static check](VALIDATION.json); [`validate_v3.py`](validate_v3.py)
reproduces the [v3 static check](VALIDATION-v3.json). The
[source-bound plan audit](../rootfind_k13_source_plan_audit_20261116/REPORT.md)
checked the 288 admission cells against the frozen panel. A separate
[source-compiled native launcher check](../rootfind_k13_native_launcher_check_20261116/REPORT.md)
accepted the exact v3 spec without submitting a Job. The deployed launcher,
verified image, current budget and checkpoint gates, L2 execution, and
independent replay remain outstanding. [pcc:unverified]
