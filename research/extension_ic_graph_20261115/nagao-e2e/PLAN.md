# Cold scalar recovery: exact Nagao elimination

User request, 17 November2026: “Measure e2e please.” Retain the actual complete
scalar-recovery pipeline and matched rho; a component count is not acceptance.

## Frozen scope before measurement

Use the previously authorized local CPU backend (amendment17), USD0 new paid
compute. Local isolation preflight3 passes the maintained `isolated_bench.py`
defaults on CPU0. No EKS/Runpod allocation or changed admission threshold.
The original USD5 balance is unknown, so paid work is not launched.

Same fixed public validation instance as stage41: p1009, fixture seed389;
worker seed401, full factor base,1024B collection batches, full p² B cap,
65536source cap,8192descent residual cap,32accepted descent cap, degree24,
staircase64. Generator holds the planted answer separately; worker receives
only public curve/G/Q/L and Spec.d=0. This is a known-answer validation target,
not unknown-log T1 or a fresh-seed distribution claim.

Arms, each a fresh native process:

1. Reference IC: existing quadratic-remainder graph pipeline with six-variable
   Nagao finder.
2. Candidate IC: identical pipeline with the checked five-variable finder.
3. Rho: plain, batch16, r256, DPbits16, seed401. This is the lowest observed
   registered rho configuration on this exact instance in stage41, against
   plain/negation × batch1/16. No global parameter-optimum claim.

One retained warmup per arm; two A/A control calls per arm; then five measured
rounds of three arms, reversing IC order on odd rounds and rotating rho's
position.24calls total. Worker timeout300seconds, whole runner7200seconds,
zero automatic retries; all failures, caps, refused isolation and missing rows
remain visible. An isolation refusal starts no crypto worker and is not an
empty solve. No source, fixture or threshold changes after first measurement.

Primary metric: maintained isolation wrapper's complete worker-process wall,
including process startup, public-input validation, cold transfer/model/base,
collection, failed relations and descent attempts, graph/rank/kernel/affine
solve, scalar replay and report serialization. Public challenge generation,
source compilation, the wrapper's preflight, post-run independent qualification
and statistical analysis are separate harness work. No reusable attack state
is supplied. Per-phase operation counters remain incomplete diagnostics.

Require all expected scalars and public elliptic replays, identical IC kernel,
accepted descent equations and collection/source signatures, deterministic
within-arm repeats and zero foreign hooks. Preserve the existing source/norm,
graph witness, rank/kernel and affine checker. For wall ratios require every
compared run uncontended and A/A max/min ≤1.10 per arm. Report all individual
walls, medians/minima, paired candidate/reference ratios and a deterministic
paired-round bootstrap interval (10000draws). Do not report a wall ranking if
admission fails. No exponent or general rho-parity conclusion from one fixture.

## Source and literature

Reuse the actual stage41 full pipeline at
fcda1e22cb0c5424ca3244f783e69be8b99792f4 in a separate historical build
worktree. An explicit native library overlay exposes the existing Nagao solver
and extension-pipeline call through a fallible solver callback. Existing
entrypoints retain the old solver; only the candidate passes unit elimination.
This avoids claiming that current main's incomplete global dependency tree
builds. The unit finder is the exact merged PR4493 source, with no new algebra.

Prior work read: stage41 report, amendments17/36, actual collector/descent/rho
worker and full qualifier; Joux–Vitse *Cover and Decomposition Index Calculus
on Elliptic Curves Made Practical* §§3.1–3.3, author PDF SHA256
7ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38,
already read in the component continuation. KB unavailable: no KB MCP and
repository wrapper has no executable; intended queries “Nagao unit elimination
complete scalar recovery” and “matched strong rho weak Fp6 full pipeline”.
No novelty assertion. Source/protocol/public input/binary manifests are frozen
and published before the sole finite measurement invocation.

Large untruncated raw reports remain in a hash-bound local run directory if
the previously documented artifact-store access failure persists. Compact
analysis, manifests, code and reports are published; no S3 upload is invented.

