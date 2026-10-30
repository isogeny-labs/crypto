# Exact v11 frontier and validation scope

All12 prescribed executions succeeded: eight52-bit controls and both fixed
61/62-bit targets. Each passed fresh CPU certificate replay and scalar+1
rejection; prescribed52-bit cross-arm certificates are byte-identical.
Largest complete prescribed rung:62field bits, not an absolute hardware maximum.

Cold62-bit solves:2139.880951918s and2187.942594727s. Audit totals:
2139.908483146s and2187.971582474s. All clocks are L0/wall_valid=false.
The62-bit admission estimates are48,798,250,775 and49,539,778,479bytes,
including queried CUB scratch; these are not measured peakVRAM.

Canonical schema, flow and file hashes pass. The canonical protocol checker
retains five rectangular-coverage findings:24assumed versus12actual rows,
b/u/x each cover1ratherthan3sizes, and arm instance sets differ. The frozen
plan explicitly uses four arms onlyat52bits and vonlyat61/62bits. Native
analysis and strict coverage audit pass; eight coverage/status/input mutations
are rejected. This is not a full canonical-checker pass or independent
scientific validation. No row was omitted or excluded.

All19Rust tests and the recorded device, Clippy, header and sanitizer checks
passed. Worker/controller exited0, archiveSHA256matched, and deletion of
jees49ew9dkfd7was verified. Fresh provider reconciliation also confirmed absence.
Raw archiveSHA256:e80cf3fb2c64fd2510e1eac7f15d0477b3a3df9408dfd88d73df0b6e3a888372.
Source:d44b59149f468357e86186f956ecb7e323f38915.
