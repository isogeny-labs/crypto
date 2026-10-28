# Exact embedding degrees: two proposed filters fail

All21 exact embedding degrees passed factor-reduction/PARI cross-checks and
a separate native modular-power minimality checker. Neither of the two fixed
criteria separates the ten published from eleven skipped curves.

Start with [the report](REPORT.md), [all exact values](results/degrees.tsv),
[rule outcomes](results/summary.json) and [separate verification](results/verification.json).
Integers in the table/certificates use explicit hexadecimal; counts and bit
lengths are decimal. The original four-factorization follow-up stays unchanged.

The published and skipped P-256 curves both have index `(n-1)/k=0x3`.
Thus any deterministic filter using only that normalized degree and the
common field cannot distinguish them. Published P-224 and skipped1485/5736
also share index0x3. This rejects those specific proposed filters; the
historical reason for skipping curves remains unknown.

[PROTOCOL.md](PROTOCOL.md) froze maximal-degree and RFC7748-degree rules
before execution. The labels were already visible; there is no new p-value,
hidden holdout or searched cutoff. No DLP experiment or cost is reported.

The native producer and separate checker use C++17/PARI2.17.3. `run.sh`
copies sources/inputs/binaries, runs strict builds and fixed controls and
records every status under a fresh output directory after the S3 gate.
All six phases exited0. The actual executed shell is an immutable copy.

Hydrate [the managed archive](evidence/.artifacts.json), then unpack
`evidence/exact-001.tar.xz` here. The [manifest](evidence/manifest.json)
binds74 files; a [full remote GET](evidence/delivery.json), internal manifest
and every listed member hash verified. The archive includes all21 original
minus products, historical table, source snapshots, native binaries,
degree certificates, logs and statuses. Only that closed verified raw
directory is ignored. To reproduce, set a fresh `NIST_ORDER_OUT` under
this study's `runs/` and follow the checkpoint requirements; never reuse
historical process identifiers or overwrite the preserved attempt.

`render.py` only plots recorded indices and renders the report. Its PDF
embeds the raw archive, protocol, exact table and verification records.
The original selected/skipped weakness and historical-selection obligations
remain in [the problem record](../../../problems/nist-candidate-selection-weakness-hypotheses.md).
