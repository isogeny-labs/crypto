# Exact linear-root shortcut, version 1

Freeze shared solver source, 200-cell plan and binaries before profiling.
Use the same ten original public curves, B=8, arities 2 and 3, target label 0.
Twenty configurations and ten target pairs, already observed training cases.
No final E28 label is opened.

Default solver policy remains unchanged. With F4_FP_LINEAR_ROOTS=1, after each
F4 basis first look for a pure nonconstant univariate linear equation a*x+b.
Combine coefficients, reject mixed/nonlinear or zero-leading terms, solve
x=-b/a analytically and recurse. If none exists, retain the old exhaustive
root scan/variable choice. Every basis equation is in the input ideal even
under truncation, so the forced value does not discard a valid solution.
Other constraints still run through recursion and the exact input filter.
No environment mutation occurs in concurrent unit tests: private policy
entry points test both settings and all small-field linear coefficients.

For each configuration, profile direct A/A, direct, eight helpers and pair
invariants with policy off, then the same four with policy on, and group-MITM
A/A plus group-MITM. Every task explicitly pins the boolean policy and the
parent sets the matching child environment; the worker asserts agreement and
records it. Use original sign search in both policies, preserving the same
minimum-mask rows. Do not combine this with the separate propagation change.

Each worker has 15,000 ms cooperative shared budget, 18-second hard timeout
plus one-second kill grace, 4 GiB address space, 32 MiB/file and one thread.
Batch cap 3,800 seconds/1 GiB; outer transport 3,900 seconds. Charge actual
startup, base/equation construction, F4, substitutions/roots, signs, infinity
exceptions, deduplication, source verification and serialization in Ir.
No wall time, kernel, parent, relation algebra, full DLP or matched rho.

Require exact full signed [x,y,sign_mask] list equality to the frozen E26
census, complete projections and three <=5 percent A/A gates. Reopen all
hashes and raw Ir independently in Rust; replay every retained sum and a
wrong-sign control in PARI with actual exit/clean-error/hash gates. Interpret
each unique candidate/control comparison once, as in the corrected control
audit. Preserve every failed, capped or missing cell and all regressions.

This tests a generic implementation optimization, not new twist geometry or
a novel index-calculus algorithm. Compare with ordinary group MITM as well
as the polynomial baseline. The external full-root reference and complete IC
comparison remain open. No validation-gated benchmark/leaderboard row follows.
