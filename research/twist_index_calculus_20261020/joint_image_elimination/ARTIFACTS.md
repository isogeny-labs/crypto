# Raw evidence and publication state

This is the joint S5 continuation. Its raw evidence is separate from the
already completed 189-case/doubling archive, whose canonical registration
and remote verification now pass:

    store:evidence/twist-index-calculus/20261023-image-base-runs.tar.xz
    28,241,048 bytes
    SHA-256 1e0a2139c9a32571a9476e4b9579f73da91e93e4a8cc7c2143d2bf4536c36038

That registration is published in PR #2368 at
`7c9f8c663bddaf4003bfbcaa2ce96aa578150e71`. Its actual failed and
successful receipts remain in the parent study's `results/publication/`.
No credential appears in repository files.

## Joint raw checkpoint

`pack-terminal-checkpoint.sh` packages an immutable selection of 22
terminal or explicitly reconciled interrupted run directories. It includes
all 297 original constructions, their 694 exported systems, source-block
and six-variant regression results, original failed proof/build attempts,
the successful split/factored proofs, and the native pure-zero coefficient
search. The selected input list is literal in the script. The archive
contains `raw-checkpoint-manifest.json` with the actual packing source
commit and timestamp.

Six newer managed lanes are deliberately omitted from this checkpoint:
external-audit-v3, external-regression-v2, pure-zero-lean-v1,
pure-zero-verify-v1, uniform-zero-v2 and variant-population-v1. Their raw
trees remain under `runs/`; exclusion is not a statement that they failed
or finished. A later canonical archive must include their reconciled
outputs and retain the earlier digest as a superseded version.

The checkpoint output is `runs.tar.xz`, with byte count, SHA-256 and exact
manifest sidecars. Packing and fresh extraction verification now pass:
542,528,576 bytes, SHA-256
`f4ddf5ce185d0866ff02e7a4e4085065e59756cf86798aaff85f7a8398eaa39c`.
Native `cmp` matches all 4,035 archived regular files to the selected
originals; the manifest inside the archive matches its sidecar. The
actual receipt is `results/publication/terminal-archive-verify-v1/verification.json`.
Canonical store registration and remote verification now pass:

    store:evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz

`results/publication/terminal-store-publish-v2.json` retains the actual
native receipt, successful cache/remote verification, S3 whole-object
SHA-256 checksum and exact byte count. The first wrapper attempt failed
on a local Python-version incompatibility before credential input,
authentication or upload; that failure is retained separately. The
portable wrapper then uploads and verifies the exact checkpoint. No
successful upload was repeated. Credentials were supplied through hidden
stdin to the short-lived process and are absent from repository files.
The archive is not committed to Git. A managed service exit alone or loose
report copies do not establish a successful publication.
The deterministic GNU tar/xz command and one-thread compression are in
the packaging script; packaging uses the busy reservation and never reads
active run trees as a completed snapshot.

## Source and numerical replay

Each run retains its executed script, source commit and/or actual source
hashes, public input SHA-256, frozen runner hash, commands, output, errors,
limits and terminal receipts. Failed and interrupted attempts are retained
under their original names. Recovery and provenance addenda are in
[MIGRATION_RECONCILIATION_20261024.md](MIGRATION_RECONCILIATION_20261024.md).
Historical absolute paths in receipts are not silently rewritten.

Loose copies in `results/` provide the native construction validation,
complete source-block audits, complete pure-variant audits, proof logs and
the exact pure-zero coefficient certificate. Every reported number names
the relevant native result. Candidate inputs have 13 public keys and
exclude reference hits, factor logs and known scalar answers. Oracle-only
verifiers and population diagnostics are separately labelled; they are
not called by polynomial candidates.

Fifteen unchanged run PDFs have an actual visual inspection receipt in
`results/pdf-inspection-20261024-v3.json`. Refreshed study and later proof/
certificate PDFs require their own review before publication. Source
SVG whitespace was normalized without changing PNGs or inspected PDFs;
raw command receipts keep their original bytes. Visual review is not a
mathematical, isolation or complete-cost validation gate.

Three later unchanged PDFs have an additional actual page-by-page review
in `results/pdf-inspection-20261024-v5-partial.json`. It covers only the
three listed documents. The refreshed study, mechanism, pure-zero,
square-obstruction and full variant-population PDFs now also have actual
page-by-page review in `results/pdf-inspection-20261024-v8.json`. Its
five exact PDF hashes were rechecked before copying the reports and
figures into the unlisted benchmark diagnostics. Later managed lanes are
outside the checkpoint's literal selection even if they have since finished.
