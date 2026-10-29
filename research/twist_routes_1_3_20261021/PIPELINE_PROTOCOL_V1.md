# E15 native execution protocol, version 1

Freeze this protocol, `E15_SPEC_V1.json` and the adapter source before running.
Use native `ecbench`, new session directories and Rust 1.90.0. This is the first
bounded correctness/operation-count execution of E15, with m=2 F4 oracles. The
original broader factorial, tuning, calibration and scaling requirements remain.

The frozen ten odd prime-order fixtures supply eight independent **public**
targets each. No logarithm is supplied to the candidate or decomposition oracle.
Run all eight trace/cover/first-pair-symmetry combinations. Rebuild the base and
discover the cover translate for each cold one-target invocation. H=8 and at
most eight negation columns: retain the actual undersized base, with no padding.
An empty intersection is an unsupported cell and keeps its error and discovery
counts. Each changed base also gets the existing blocked-MITM2 oracle on exactly
that base. Ordinary IC gets the Cartesian tuning panel B=8,12,16 and blocked
MITM2, root-finding subtraction m=2,m=3. Include the matched strongest applicable
negation rho, the frozen rho reference, and a negation-rho A/A arm.

Trace images use every u,v in [0,H), the exact quadratic-extension square root
and S+Frob(S), with signs folded. Cover images use u in [0,H), x=u²+c. The cold
cover search chooses the first ascending c whose complementary coefficients
have an actual Fp coefficient isomorphism to the quadratic twist. E10/E12 already
checked these selected covers and their class transfer independently. The
combined adapter does not run an unimplemented Jacobian decomposition solver.

The F4 arms use the existing finite-prime backend, grevlex and degree bound 16,
with no cooperative wall deadline. A truncated, unresolved call supplies no
refutation or relation. A completed basis is followed by explicit finite-base
root extraction, evaluation of every candidate against the original equations,
and the counted curve group law for signed lifting. The symmetric chart handles
its diagonal and identity branches explicitly. Native F4 field-multiplication
counts, symbolic input sizes, extraction probes, polynomial term evaluations,
construction probes and reference-arithmetic calls remain visible and unpriced.
The latter counts are high-level call units, **not a complete calibrated field
operation or CPU-instruction count**. Peak-memory/accounting closure is open.

Each pipeline stops at target pin, 8,192 relation trials, or the ecbench child
limit of 30 seconds; preserve exhaustion and all errors. The outer run has a
90-minute hard limit. A process kill is unresolved, never a mathematical null.
The session is L0, one measured round, zero warmup because this is a cold native
count/correctness panel. No wall-clock outcome or scaling fit is admissible.

Run `ecbench plan` first and require all fixture identities registered. Then run
`ecbench run --cpus none --allow-busy --spec E15_SPEC_V1.json --out FRESH_DIR`.
Audit every deterministic record with `ecbench verify --replay-all` into an
external receipt. Any non-identical result remains rejected. Derive the native
cost table and 0/0.1/1 GAE sensitivity bands equally for every arm, including rho.
Retain failed/incomplete pairs in the denominator ledger; rank no incomplete or
uncalibrated full-cost comparison. Same-host replay is not independent-host
validation. No canonical leaderboard or benchmark result is eligible from this
protocol alone, and no NIST-size extrapolation or novelty claim follows.
