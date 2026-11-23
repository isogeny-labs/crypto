# Reproduce the fixed trace-lift controls

Verify SOURCES.sha256 against the two archives in FROZEN.md. Use Rust1.100.0
and the commands in tools/torus-trace-lift/README.md. Use a NEW output
directory: the executable refuses an existing destination, writes compiled
source/binary hashes before running and preserves all native output files.

Trace ordering and each fixture's offsets are in native-v1/controls.json:
j ascending, real x ascending, imaginary b ascending. Bit0 is the original
norm/power equation, bit1 the lifted-coordinate answer. The complete149776
bytes compress to353 bytes; raw and gzip SHA256 digests are recorded.
The24 original p101/q3 accepted members additionally replay through the
unchanged membership circuit. Actual18 chart cases comprise nine original
positive labels and nine inverse-label conjugate controls, not18 members
of the original positive-label base. Original norm and power replay is
separate from the adapter's scalar evaluation.

Native certificates use fixed32-point streams and extra32-point checks over
the maintained certificate module's declared Mersenne field, with plus-one
mutation refusal. They do not formally connect AD, Chebyshev, Fp2 or groups.

Lean uses the sibling lean-trace-v1 project and the pinned existing-law
instance map. Run only its focused proof gate, not the full root library.
All failed source/log attempts remain in development/ and the labelled
failed first gate. There was no timing, paid job, sampling or solver campaign.
