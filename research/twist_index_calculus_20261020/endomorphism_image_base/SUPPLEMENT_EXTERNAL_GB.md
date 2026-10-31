# External Groebner-basis control

Frozen before exporting or timing these external workloads. This supplements
the complete cold native comparison; its interval is the GB kernel alone.
It does not substitute GB timing for complete polynomial decomposition.

Use official msolve v0.11.0, released 2026-10-09, with its source commit,
container digest, compiler/configuration and binary hash recorded. Compare
grevlex reduced bases over the same Fp with the native D=12 F4 path. Use
one thread on the same host and reserved core, warmup, A/A and five
interleaved rounds. Measure each engine's internal GB interval, excluding
input parsing, startup and output serialization. Retain errors and reject
truncated, timed-out or unequal outputs.

Select inputs without consulting hit sets: all 21 frozen instances,
original / phi-l0-active / double-l0-active, public residual index 0,
the first, middle and last physical peel in the frozen sorted order,
both peel signs. This specifies 378 positions. Retain infinity positions
explicitly as having no finite GB input. Export exact coefficients in
both native JSON and msolve syntax and bind them by SHA-256. Compare the
complete normalized reduced bases, not just dimensions or a count of roots.
Check native S-pairs and reduction of input generators as an additional
completion check. A D=12 truncation is retained as incomplete rather than
compared as if it were a complete basis.

Magma and FGb availability is checked separately; no fastest-implementation
claim is admitted from an open-reference comparison alone. A GB-kernel
ratio describes these fixed systems and does not change the cold native
costs already recorded, prove a relation-density advantage, or establish
an asymptotic improvement.
