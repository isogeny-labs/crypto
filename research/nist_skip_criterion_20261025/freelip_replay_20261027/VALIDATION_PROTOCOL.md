# Saved-output format correction,27October2026

Replay001 completed all84 fixed calls after the successful source/S3 gate.
Every actual legacy stdout has RESULT1; no RESULT0. Initial native analysis
admitted only the192/224-bit orders (4published/5skipped per configuration)
and retained all12 larger orders as indeterminate. No process crashed or
timed out, and no failed output was called a composite or weak curve.

Original zfwrite_c at1.0 lip.c:7263 emits OUT_LINE_BREAK and newline when a
large decimal number crosses its configured line width. zwriteln uses that
formatter. Actual239/256/384/521-bit order reports contain backslash-newline
continuations. The strict observer incorrectly expected a single root line.
This is our I/O grammar restriction, not an arithmetic or primality failure.
The original frozen protocol/source, outputs and initial summary stay intact.

Correct only the native observer. Require the same exact configuration line,
INPUT prefix, decimal digits and complete requested root, allowing only
source-defined backslash-newline between nonempty decimal fragments. Require
exact method, one final RESULT0/1 and newline, no trailing data. Preserve all
process/identity/role/count checks and unknown handling. Do not alter or rerun
any legacy executable, input, certificate or transcript. No label-dependent
arithmetic threshold is added. Test three positive format cases and seven
malformed/root-bound integrity cases before validating all84 saved packets.

This correction is made after seeing the saved outputs and is explicitly post
hoc. It is not a new preregistered hypothesis or84 new samples. The original
version3 protocol remains unchanged; its analysis hash names the initial
observer. Fresh validation002 binds the corrected observer and copies every
original input/status/stdout/stderr packet and cohort identity, with per-file
hashes checked before/after. Build and execute under768MiB/60s limits; require
a successful S3 checkpoint gate first. Emit corrected results separately and
retain original initial indeterminate labels in the raw archive/report.
