# Public-header and known-answer qualification, before its implementation

Use a separate std-only Rust control driver and copied immutable executable.
The driver is not a solver and feeds no recovered scalar to target-only modes.
Its public inputs are the original12headers plus the explicit8-seed list
101,202,303,404,505,606,707,808 on cases0..5. For each of48headers call C,
parent and candidate target-only modes and require exact canonical TSV equality.
Check old12 against the archived public projection, curve/generator consistency,
and8distinct target coordinates per domain. Export the36new headers separately.

A separate input contains12archived known-answer scalars, bound by the original
full archive hashes. These are consumed only by the C validation controls, not
target generation or either solver. On each old target require a positive
kP=Q check, rejection of k+1 modulo r, and rejection of the consistent pair
(-Q,r-k) by the public-target binding check. Thus36known-answer validation
calls supplement144target-only calls. This qualifies the new raw scalar-replay
implementation before any new recovery.

Two explicit invalid fixtures (case6/seed303, case0/seed999) must be refused by
each of the three copied target constructors with the intended fixture cause,
giving6more calls. The complete prospective census is186child calls. Record
begins/ends, raw stdout/stderr, attempted/passed/unattempted counts, and fail
fast. Additional native parser unit controls precede this census. Every output
is exclusively created; no retry or overwrite of the same output path.

Use60s foreground child timeouts,600s phase, one worker, unchanged memory/disk
guards, and an explicitly checked allowed CPU. Freeze all driver/source/input/
binary/runtime identities before calls. Output is public metadata/correctness
evidence, not a solver, timing, novelty or statistical result. No main cohort
execution follows until export/protocol qualification is separately complete.
