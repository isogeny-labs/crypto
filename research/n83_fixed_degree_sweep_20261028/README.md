# Fixed-base n83 regularity sweep

Requested grid: Koblitz, general binary and prime curves, n83, m2..9, fixed ell27.
Each curve uses the same manifest from
[the saved point dataset](../n83_fixed_factor_base_20261028/data/index.json)
throughout the sweep. Every planted point is looked up in a hash-verified actual
point shard. The [protocol](PROTOCOL.md) describes the encoding and metric.

This study runs the existing S3-chain candidate system over the fixed x-domain.
It does not introduce an expanded polynomial for the full liftable-point lookup
set. Candidate solutions require rational point lifting and sign consistency.

There are48 inputs (one planted and one natural target per cell), each with a
top-form and an affine computation:96 attempts. Top-form regularity is measured
only after a complete homogeneous basis and zero-dimensional Hilbert quotient.
Maximum final affine basis degree is recorded separately, not as solving degree.
Timeouts and memory exits leave the degree unresolved.

The source is an isolated copy of the prior runner with fixed ell27 and stored
factor-base identity/membership checks. The old study and evidence are unchanged.
See protocol.json, request.json, run-local.sh and run-container.sh for the exact
source, mounted native binary, Singular runtime image, limits and launch command.
The source uses SplitMix64 seed20261027 with the retained per-family/m rule.

Backend: existing Spot workspace, four single-threaded Singular4.3.1 workers,
40s plus5s kill grace and3GiB virtual memory per solver. Container ceiling:
4CPUs,16GiB,1800s. EKS shared-bucket authorization failed in the preceding task;
that unresolved failure is the reason for the recorded existing-host fallback.
All timestamps are diagnostic. No speed or end-to-end IC claim is made.

Five native tests and eight same-driver positive/negative solver controls pass.
Results, export and report are added after the run reaches its terminal state.

## Completed run

All 96 attempts finished:4 completed affine bases, 63 time-limit exits and 29
memory-limit exits. No top-form regularity computation completed. The four
completed affine runs are m2 Koblitz and binary, planted/natural:2/0 and2/0.
The natural cases returned the unit ideal. These are final-basis degrees, not
regularity or maximum intermediate solving degree.

[Report](reports/run-20261028T0325Z/report.md),
[PDF](reports/run-20261028T0325Z/report.pdf),
[observations](evidence/observations.json), [export](experiment.export.json).
The full root release suite passed 4966 tests, with 122 ignored and zero failures.

The raw tree is one deterministic `runs.tar.xz` in the shared artifact store.
Hydrate and verify from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_fixed_degree_sweep_20261028
cd research/n83_fixed_degree_sweep_20261028
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
(cd runs && sha256sum -c manifest.sha256)
```

The archive contains the inputs, membership receipts, all 96 solver attempts,
controls and terminal receipt. [storage.json](evidence/storage.json) identifies
both the archive and the raw S3 prefix. Full-curve fixed-base semantics and
remaining regularity gaps are retained in the
[problem record](../../problems/n83-balanced-ic-regularity.md).

To reproduce the archive bytes from the extracted tree, use GNU tar with sorted
names, date2026-10-28, owner/group0, normalized permissions and xz preset0:

```sh
tar --sort=name --mtime='2026-10-28 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -0 -T1 > runs.repacked.tar.xz
```
