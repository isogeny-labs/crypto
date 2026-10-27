# Recovered polynomial proof gate v2

Gate `lean-gate-v2` was launched at 2026-10-24 01:21 UTC against the
original combined `OrbitCoordinates.lean` module. Its standalone-file path
corrected the earlier v6 directory-path error. No final gate receipt
survived app recovery.

![Verification boundary](../../figures/joint-mechanism.svg)

Reconciliation found no surviving compiler or launcher. The temporary build
directory survived and its log records failure to create a Lean worker
thread. The log is retained as `runs/lean-gate-v2/recovered-build.log`.
The original stdout/stderr and reconciliation note remain intact.

No theorem or axiom audit passed in this attempt. A fresh bounded
one-worker run follows; it is not counted as a second successful proof.
No comparable performance values exist, so the visual is a process diagram.
