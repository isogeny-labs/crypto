# Exact v13 frontier and validation scope

All 12 prescribed executions succeeded: eight 52-bit controls and both fixed
61/62-bit targets. Each passed fresh CPU certificate replay and scalar+1
rejection; prescribed 52-bit cross-arm certificates are byte-identical.
Largest complete prescribed rung: 62 field bits, not an absolute hardware maximum.

Cold 62-bit solves: 2152.177507022s and 2274.229212139s. Audit totals:
2152.226222806s and 2274.281052908s. All clocks are L0/wall_valid=false.
V13 solver_seconds includes the per-target GP generation invocation; audit
wall_seconds additionally includes separate replay and negative-control checks.
The 62-bit TABLE admission estimates are 60,880,922,152 and 68,978,885,440 bytes,
not measured peak VRAM. Both actually cached every right row: 2,276,312,640
and 2,759,268,600 respectively. Both 61-bit targets also used full right caches.

Canonical schema, flow and file hashes pass. The canonical protocol checker
retains five rectangular-coverage findings: 24 assumed versus 12 actual rows,
b/u/x each cover 1 rather than 3 sizes, and arm instance sets differ. The frozen
plan explicitly uses four arms only at 52 bits and v only at 61/62 bits. Native
analysis and strict coverage audit pass; the strict checker accepts a valid
first-rung limit and rejects nine deliberate coverage/status/input mutations.
This is not a full canonical-checker pass or independent scientific validation.
No row was omitted or excluded.

The recorded host, Rust, Clippy, service, header and device checks passed.
All seven query variants passed memcheck, racecheck and synccheck before data.
Worker/controller exited 0, archive SHA256 matched, and deletion of
7tugqyoxyvnikm was verified; fresh provider reconciliation confirmed absence.
Archive SHA256: f00ea3e1ef9d32ebbf2063cb43b99939a42a19f31085c0f1ccb1d32d8e083338.
Source: e9e87ffb83807d7505c29fa04358d104e72d4b20.
The generic MITM GPU oracle is not the paper's conditional tensor backend.
No certified speedup, matched-rho comparison or external-fastest claim is made.
