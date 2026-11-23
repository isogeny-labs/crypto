# V2 amendment: distinct geometric public input

V1 b20b27da82a9 failed its pre-map input inequality because D1=2*(0,44).
Its original protocol, source and raw failure are preserved. This is a new
registered source/run, not a replay of the unchanged frozen cohort.

Change only B definition: B=2*R where R is factor_base(&refc)[1], the second
canonical rational place in x order (with canonical y). Print R, assert B
nonzero and B!=D1. No alternate place or retry on failure. Inputs D1,B,D1+B,
2B,-B,1729D1 and zero identity otherwise unchanged. Same mapper, fixed lambda10,
8192per-input/600second aggregate caps, validity/ell checks and group identities.
Use binarypublicmap13_v2, stdoutRUN_V2.txt/stderrRUN_V2.stderr. Build, source
commit and fresh S3 checkpoint before one execution. V1 outcome remains failed.
This correction creates no timing, speed or independent-implementation claim.
