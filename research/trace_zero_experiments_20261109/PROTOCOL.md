# Matched finite-domain trace-zero experiment

The executable, exact input manifest and canonical protocol are prepared.
Execution is pending access or a specific user exception; no solver panel has
run. The mandatory checkpoint gate currently fails on missing upload access.

Compare three real production encodings: explicit full-point witnesses,
certified finite domains, and certified domains with necessary trace bits on
finite partial sums. The fourth arm repeats certified unchanged for A/A.
The new partial-trace option retains every identity pattern and never requires
an intermediate to lie in the original factor-base subspace.

## Exact workload and limits

- Fields: E0 at n7,9,83,131 (ECC2K-130 uses131).
- Bases: all full-kernel affine points with x in the fixed trace-zero projection
  of span(z,...,z^d), d4/5. Both signs retained. Exact coordinates are in workload.json.
- Targets per cell: planted three-term relation; cancellation control; two
  uniform scalars in the stated fixture-generated subgroup, sampled across the
  full order width with pinned StdRng. Seeds are fixed in the native source.
- Independent known answers: complete group pair table, never passed to the
  algebraic candidate. Add one n5 independently refuted signed-base control.
- 33 instances x4 arms x2 repeats =264 primary rows. Deterministically rotate
  arm order per instance/repeat. No favorable-row deletion or adaptive seeds.
- Per query:2000 cumulative SAT conflicts,750ms safety fuse,500000 variables,
  all two m3 identity patterns,1024 model limit. Global limit600 seconds,
  one CPU,2GiB address space, zero retries and zero cloud spend for a local exception.
- 16 separate engineering preflight calls, one per arm/field; pilot targets
  are3*base[0]. Record elapsed values only to bound execution, never as an
  isolated timing or algorithm-performance result. Complete before final protocol freeze.
  Every pilot row must pass independent validity checks; at n7/9 every arm
  must also return Found with a valid original-group replay. Otherwise stop
  before primary rows. The600-second outer limit includes preflight, freeze
  preparation and primary execution; give the primary process only the remaining time.

## What the result can establish

Report exact valid-completion/outcome counts and conflict distributions for
this fixed panel. Every Found answer must re-sum in the original group;
every Refuted answer must match complete independent truth. UNKNOWN is retained.
The A/A pair detects deadline/caching/environment sensitivity. No population
p-value, natural ECC2K-130 recovery, end-to-end speed or scaling conclusion.
The analysis code is already compiled and hash-bound before affected data.

All data, failed attempts and guards are retained. The runner flushes every
row; after interruption the prefix is preserved and unattempted rows remain
in the flow denominator. Do not retry a row to change its outcome.

## Execution authorization

Repository AGENTS.md requires EKS by default and a successful S3 checkpoint
before experiments. Fresh environment inspection shows no AWS identity or
checkpoint token; checkpoint gate fails. Reading the active workflow succeeds,
but does not grant upload or cloud budget access. The workflow builds ecbench,
not this standalone new instrument, so submitting its fixture would not run
this workload.

The reviewable alternative is this exact local run, within the limits above,
with durable Git input custody and flushed local rows, explicitly waiving the
EKS/S3 rule for this run. It needs the user's explicit exception. No gate is
silently changed and no experiment is relabeled as a unit test.

## Reproduction

Build the pinned native crate; validate protocol.json with the canonical
../crypto-autoresearcher/tools/experiment_protocol.py. With actual execution
authorization, run a separate runtime preflight, record its engineering bound,
freeze the final protocol and commit it before primary rows. Verify the
instrument hash, apply the CPU/address-space limits, and invoke:

~~~sh
native/target/release/trace-zero-experiments run workload.json rows.jsonl
native/target/release/trace-zero-experiments analyze rows.jsonl > summary.json
~~~

Use the canonical from-rows/check-export adapter for experiment.export/v1 and
publish a source-linked report/PDF including every guard and A/A discrepancy.
No new runner changes or workload selection after looking at primary rows.
