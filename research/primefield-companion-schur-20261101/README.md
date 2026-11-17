# Prime-field companion operators and Schur certificate transport

This publication collects the companion-operator and Schur work from the
standalone prime-field research workspace. It preserves original source,
frozen protocols, results and failures. The user requested this GitHub PR
while the full fused-operator validation was running.

## Included work and evidence

| Component | Evidence status at publication |
|---|---|
| Direct companion generators | Complete panel:162 cases,66 normalized witnesses,1038 rejected mutations, exact A/A, export check passed |
| General companion/block operators | Complete panel:738 contexts,32328 products,3072 principal identities,192 reference-inverse identities,67920 rejected mutations, exact A/A, export check passed |
| Schur construction | Original3600-second cap reached, exit124;42 completed contexts:37 checked constructions,5 leading obstructions,456 directional checks,1903 rejected mutations; all raw bytes retained |
| Deferred-reduction fused operator | Full1698-context/34248-product protocol frozen; invocation live in original workspace at publication; included inspected launch snapshot has18 contexts/2250 products, not a full-panel result |
| Primal/dual lift and fused Schur adapter | Derived source and syntax checks; numerical integration trials0 |
| Full Schur/lift integration checker | Implemented and syntax checked; retains642 original contexts; protocol/locks not frozen and no execution |

Field operations, native integer operations, bit-length volumes and byte
volumes remain separate. Counts do not establish elapsed-time or peak-memory
improvement. Leading solves use explicit reference oracles; feature-annihilation
is checked separately from Gram nullness. Source elliptic-curve scope remains
prime fields, excluding index calculus, GHS and generic elliptic collision
search. The research goal remains active.

## Read first

- [Fused construction and carry bound](companion-fused/DERIVATION.md)
- [Fused design report](companion-fused/reports/design/report.pdf)
- [Frozen full fused validation contract](companion-fused-validation/CONTRACT.md)
- [Inspected live-run snapshot](companion-fused-validation/reports/launch/report.pdf)
- [Completed block panel report](companion-blocks-validation/reports/final/report.pdf)
- [Capped Schur report](companion-schur-validation/reports/capped/report.pdf)
- [Schur/lift integration design](companion-schur-fused/INTEGRATION.md)
- [Integration checker preparation](companion-schur-fused-validation/PREPARATION.md)

Reports and receipts retain their original timestamps and historical execution
status. A document saying “unexecuted” describes its dated source preparation;
its later validation is recorded in the corresponding validation directory.
Local source commit IDs in frozen records belong to the standalone research
repository; `publication-manifest.json` binds the exact imported bytes and
source head. No frozen source was rewritten for this repository layout.

## Verify and restore the evidence

First hydrate the repository-managed artifact files from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/primefield-companion-schur-20261101
```

The artifact tool checks each download against its adjacent `.artifacts.json`.
It requires the repository's configured artifact-store access. Then run with
Python3.9+ from this research directory:

```sh
python3 verify_bundle.py
python3 verify_bundle.py --hydrate
```

The first command checks every original file and every compressed archive,
including streaming decompression hashes, without writing the raw archives.
The second also restores the exact original paths. Large files are losslessly
stored as ordered gzip chunks, each at most32MiB, in the repository artifact
store. Git carries the native `.artifacts.json` manifests, not those data bytes.
`publication-manifest.json` records original sizes/hashes and ordered chunk
sizes/hashes. Existing mismatched files cause failure; they are never replaced.
Restoring creates about1.7GB of raw archives. The full capped Schur output and
the complete block/direct-generator archives are preserved, not downsampled.

The script also checks all source/input hash locks for the four core validation
directories. No kernel, trial or launch script is executed by bundle verification.

After restoring, protocol/export inspection uses the included original tool:

```sh
python3 -m venv .venv
.venv/bin/pip install 'jsonschema>=4,<5' 'numpy==2.0.2' typst PyMuPDF
.venv/bin/python tools/experiment_protocol.py validate companion-fused-validation/protocol.json
.venv/bin/python tools/experiment_protocol.py check-export companion-blocks-validation/runs/export.json --protocol companion-blocks-validation/protocol.json --verify-files
```

Do not run historical `check.py`, `launch.py`, postprocessors or report renderers
in place: existing outputs are immutable, live process IDs refer to the original
host, and some operational scripts refer to that workspace's sibling virtual
environments. A fresh numerical invocation requires its own frozen contract,
output directory and budget. The integration checker deliberately requires
successful terminal receipts from the full upstream kernel panel, which are
not present in this publication snapshot.

## Publication boundaries

The running fused panel's mutable raw output, logs and state are not copied.
Its frozen source, protocol, seed audit, launch receipts and inspected snapshot
are included. The original producer continues on its existing handle and
original deadline; this PR launches no jobs and changes no external job.
Older unrelated research portfolios and interrupted cyclic-recovery artifacts
remain in their original workspace.

`publication-validation.json` records packaging checks and the repository test
attempt separately from the scientific evidence above. This PR publishes the
implementation and evidence at their stated scopes; it does not promote the
unfinished integration panel to a completed result.
