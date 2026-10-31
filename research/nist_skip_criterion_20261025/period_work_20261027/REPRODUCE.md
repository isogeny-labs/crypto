# Reproduce the recorded-work analysis

Hydrate this study's managed files with tools/artifacts/artifacts.py. Four
earlier factorizations and ICV1/DiSSECT records are unchanged. Source identities
are inputs/historical.tsv; the analysis inventory/admission labels are the
two other inputs/*.tsv files. All indices and counts here are decimal.

Check runs.tar.xz.sha256 and extract into a fresh directory. Compare the
internal MANIFEST.json byte for byte to evidence/final-manifest.json, then
check every member's bytes/SHA and reject missing/unlisted files. The input
copy map binds all 65 consumed original packet/catalog files to their
archive copies under runs/inputs-used/. The unchanged original admission
table stays tracked. No original ECPP executable is needed for this replay.

The primary source snapshot/binary/output is runs/analysis001; the separate
token scanner and three deliberately altered-table controls are validation002.
Compile profiler.cpp and verify.cpp with C++17/O2/Wall/Wextra/Werror. Either
restore the original paths from evidence/input-copy-map.json and use the gated
launchers, or point the native executables directly at the checked
runs/inputs-used/ as their SAVED_RUN argument. Use a fresh output, the declared
60s/768MiB limits and a successful checkpoint gate for any new research pass.
Canonical results are byte copies of the closed outputs, not a fresh sample.

Native export.cpp only serializes the recorded observations, hashes and
literal proof data. AJV8 metadata admission checks experiment.export/v1 and
every referenced file; it is not the canonical experiment-protocol checker.
The saved-result plan, visible labels and post hoc validation are disclosed.
Unrecorded timing/CPU/rusage/source-dirty fields remain null.

The 27-member preparation archive contains original sources, development
controls, the exact primary native binary and frozen source/input metadata.
Its full readback precedes execution. The compiler release archives/caches
are omitted; hashes, URLs, source and runnable native binaries are preserved.
Proof reproduction and all earlier failed proof checks are documented under
proof/ and retained in the final raw archive. The last proof source is the
only active Lean target; failed source versions are raw evidence.

Repack the extracted final root with the original deterministic settings:

```sh
tar --sort=name --mtime='2026-10-27 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - MANIFEST.json custody runs | xz -6 -T1 > runs.tar.xz
```

Python is used only by reports/render.py for existing recorded values and
document layout. The new data extraction, bounds, verification and export are
native. Generic source-byte/pointer checks and repository operational scripts
remain separate from research arithmetic.
