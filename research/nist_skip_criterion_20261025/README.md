# NIST skipped-curve criterion search

The [report](REPORT.md) and locally delivered PDF give an independently checked
two-test empirical separator for all 21 recorded eligible curves, its
root-convention dependence, and the calibration that prevents treating the
fit as an established historical explanation. The broader weakness and
selection hypotheses remain unresolved.

Start with [the truth table](results/criterion-truth-table.tsv),
[calibration](results/calibration-union.json), and the [RNG-model verdict](results/rng-models.json).
The raw evidence is packed in `evidence/runs.tar.xz` with
per-member hashes. The PDF, archive and large PNG are delivered in the local
workspace and stored in S3. Every uploaded object was downloaded and SHA-256
checked without changing the frozen payload bytes.
See `ARTIFACT_DELIVERY.json` for exact names, sizes, hashes and delivery status.
No active curve registry, solver or benchmark is changed.

| Requested result | Status |
| --- | --- |
| Original four-curve ICV1 and DiSSECT assessment | Preserved in the [earlier study](../curve_candidates_dissect_20261021/README.md) |
| Exact empirical criterion for the recorded choices | Independently verified on all 21 curves |
| Historical explanation or applicable vulnerability | Unresolved; see the [selection problem](../../problems/nist-candidate-selection-weakness-hypotheses.md) |
| Current PDF and full evidence bundle | Built, inspected and delivered locally |
| Artifact-store delivery | Upload and full-object read-back verified; see the [delivery record](ARTIFACT_DELIVERY.json) |
| Four remaining factorizations | All four independently proved complete; 84/84 full records and the all-21 comparison replay verified; see the [follow-up](factor_followup_20261026/README.md) |
| Exact SEA coverage replay | All21 values match; uniform larger/smaller preferences have counterexamples; see the [replay](sea_replay_20261026/README.md) |
| Exact embedding-degree criteria | All21 degrees certified; maximal-degree and modern RFC filters fail; see the [new test](exact_embedding_20261026/README.md) |

## Reproduce the native checks

Install native PARI headers/library and OpenSSL development files. Compile
the C++ sources with C++17; the recorded build uses `-O2 -Wall -Wextra -Werror`.
All native execution is independent of Python. A scratch feature run is:

```sh
PARI_PREFIX=/path/to/pari SKIP_SEARCH_OUT=/new/scratch/directory sh native/run_features.sh
```

`native/root_convention.cpp` writes historical and smaller-root input variants.
`native/search.cpp` reads the native feature tables and enumerates all 4,480
permitted label assignments. Its `--self-test` exercises planted rules and
negative controls. `native/verify_rules.cpp` independently evaluates the
explicit candidate formulae from the exact parameter table.

`native/rng_models.cpp` performs the defined 52,520-model visitation/point-gate
test and has a separate `--self-test`. `native/factor_target.cpp` and
`native/verify_factor_hint.cpp` produce and check proof-enabled factor records;
`native/extend_features.cpp` only enables complete-factor features available
for the entire selected cohort. `native/primality_gates.cpp` checks candidate
incomplete-test gates on the proven prime orders. No unknown factor is
represented as a prime or a fabricated numeric observation.

The evidence archive contains the exact outputs for each phase, including
timeouts, rejected partial hints and a superseded interpretation bucket.
Every source used by a run is frozen beside those records. Reproducing the
integer results is distinct from reproducing a machine's timeout outcome.

## Integrity and rendering

```sh
sha256sum -c SHA256SUMS
sha256sum -c evidence/runs.tar.xz.sha256
tar -xJf evidence/runs.tar.xz -C evidence
python3 reports/render.py
```

The first command checks tracked sources and records. The archive commands
also need the hydrated archive. Run the repository artifact tool's `pull`
command for this directory when starting from a fresh checkout.
`ARTIFACT_DELIVERY.json` binds the delivered binary files separately and links
the upload, remote verification and full-object read-back records.

The last command only plots recorded counts and renders `REPORT.md`, using
matplotlib, ReportLab, PyMuPDF and DejaVu fonts. The PDF embeds the raw archive,
historical input table and direct truth table. Rebuilding a document can change
timestamps and byte hashes; the recorded files are bound by `SHA256SUMS`.

The archive contains `runs/manifest.json`, whose SHA-256 entries name every
other member. Pack the extracted tree deterministically from `evidence/` with:

```sh
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 \
    --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > runs.tar.xz
```

This is a frozen study. Exact models and witnesses are held in its parameter
tables and raw archive; it does not introduce new active catalogue aliases.
The report retains its original workflow status. The dated delivery update
records the later artifact-store upload; the follow-up records new research.
