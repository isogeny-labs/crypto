# Prime-field IC relation collection: affine versus Jacobian

Status: preregistered; no benchmark rows have been collected.

## Question

Does the shipped Jacobian relation collector reduce relation-collection wall
time relative to the shipped affine collector in the prime-field orbit index
calculus implementation?

This is an implementation comparison, not a pure coordinate-only comparison.
The affine implementation has one adding-walk lane and advances it with affine
additions. The Jacobian implementation has 32 independent lanes, advances them
with inversion-free mixed additions, and batch-normalizes once per lane round.
The lane layout and its 32 initial scalar multiplications are part of the
candidate being measured and remain visible in the counters.

## Prior evidence and sources

- `cryptanalysis/experiments/prime-ic-jacobian-relations-20261031/` froze
  the same-revision affine/Jacobian comparison and verified correctness, but
  collected no timing data because the required checkpoint upload failed.
- Bernstein--Lange, [Explicit-Formulas Database, short-Weierstrass Jacobian
  coordinates](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html):
  `dbl-2007-bl` and mixed `madd-2007-bl`. The EFD table lists general doubling
  as 1M+8S and mixed addition as 7M+4S. These counts motivate the
  representation; they do not establish a wall-time speedup.
- The implementation formulas and cross-coordinate tests are in
  `src/cryptanalysis/bsgs_fast.rs` and
  `src/cryptanalysis/prime_orbit_index_calculus.rs`.

## Frozen implementation and metric

The native ecbench method is `ic.prime_orbit_relations_v1`.

Primary metric: the paired ratio

```text
relations.wall_ns(jacobian) / relations.wall_ns(affine)
```

`relations.wall_ns` clocks only relation-collector construction and probe
loops. It includes coordinate-specific lane initialization, normalization and
advancement. It excludes curve construction, factor-base selection, shared
jump-table construction, gain-graph solving, logarithm certification, database
assembly, target descent, and outer verification.

Secondary metrics:

- `solve_wall_ns`, the entire verified ecbench method;
- end-to-end logarithm-precomputation wall time in method detail;
- relation trials, relations, full/combined relations, collection rounds,
  restarts, oracle operations, probe operations and certified columns.

The abstract operation counters receive no coordinate discount. They describe
algorithmic work and expose changes caused by the 32-lane stream; wall time is
the representation-sensitive result.

## Workloads and pairing

`SPEC.json` constructs six certified scaled curves:

- generic `y^2 = x^3 - 3x + b`, 28 and 30 field bits;
- `j = 0`, `y^2 = x^3 + 7`, 28 and 30 field bits;
- `j = 1728`, `y^2 = x^3 + ax`, 28 and 30 field bits.

Each curve has one planted target. Curve search and order certification happen
before measurement; measured children rebuild the resulting explicit curve.
Every arm receives the same workload and algorithm seed in a round.

Arms:

1. `affine`: reference, `coordinates=affine`;
2. `jacobian`: candidate, `coordinates=jacobian`;
3. `affine-aa`: control, byte-identical method configuration to `affine`.

All arms use 32 factor-base orbits, 1.5 requested relations per orbit per
round, and the same operation caps. The measured order alternates with measured
round parity. There are two warm-up rounds followed by eight measured rounds.

The implementation is single-threaded. Iso Lab reserves four CPUs for L2
isolation, but no many-thread result is claimed and Rayon is not used by this
method.

## Correctness and admission gates

Every retained measured execution must satisfy all of the following:

- outer ecbench verification accepts the recovered target logarithm;
- `rejected_relations = 0`;
- `verified_descents = 1`;
- `uncertified_columns = 0` when present;
- method, workload, binary, image and source identities match the frozen run;
- the session passes ecbench audit and replay;
- the Iso Lab receipt is L2 with no eviction, retry or fallback;
- uploaded artifact hashes re-download and verify from S3.

An exhaustion, timeout, failed verification, retry, host downgrade or missing
receipt is retained as a failure and is not silently replaced.

## Decision rule

Report the paired ratio for every workload/round and a workload-stratified
bootstrap 95% confidence interval for the geometric mean ratio.

A collection speedup is supported only if:

- the Jacobian/affine interval is wholly below 1.0;
- the affine-A/A interval contains 1.0;
- the absolute A/A geometric-mean deviation from 1.0 is at most 5%; and
- all correctness and admission gates pass.

Otherwise report no demonstrated speedup. Report the point estimate even when
the decision is negative. Any conclusion is limited to the tested 28--30-bit
scaled curves and the shipped one-lane-versus-32-lane implementations.

## Route and reproducibility

Performance data may be collected only after this protocol, spec and harness
are merged. Dispatch `.github/workflows/isolab-experiment.yml` against the exact
40-character merge revision on `main`, with `SPEC.json`, L2, four CPUs, and no
retries. Before dispatch:

1. initialize and complete `.resume/RESUME.md` with
   `scripts/checkpoint.sh init`;
2. push and verify the checkpoint with `scripts/checkpoint.sh gate`;
3. refuse launch if that gate fails.

The current development environment has no checkpoint upload token or AWS
identity, so dispatch remains blocked until that prerequisite is supplied.
Local smoke runs are correctness checks only and must not appear in the result
table.

## External comparison

No external implementation is scheduled. The measured component is an
internal relation collector with repository-specific factor-base, relation,
counter and verification semantics; no separable external tool accepts the
same state stream and emits matched records. This study makes no "fastest"
claim.

## Required artifacts after the run

- raw session, audit, replay and Iso Lab receipt;
- checkpoint and S3 hash-verification receipts;
- a machine-readable paired analysis;
- SVG and 150 dpi PNG ratio/A-A figures;
- `REPORT.md` and illustrated `REPORT.pdf`;
- archival copies under `benchmarks/20261101-prime-ic-jacobian/` and a row in
  `benchmarks/README.md`.
