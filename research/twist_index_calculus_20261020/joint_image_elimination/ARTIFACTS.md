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

## Earlier 22-directory joint raw checkpoint

`pack-terminal-checkpoint.sh` packages an immutable selection of 22
terminal or explicitly reconciled interrupted run directories. It includes
all 297 original constructions, their 694 exported systems, source-block
and six-variant regression results, original failed proof/build attempts,
the successful split/factored proofs, and the native pure-zero coefficient
search. The selected input list is literal in the script. The archive
contains `raw-checkpoint-manifest.json` with the actual packing source
commit and timestamp.

At that checkpoint's earlier snapshot, six newer lanes were omitted:
external-audit-v3, external-regression-v2, pure-zero-lean-v1,
pure-zero-verify-v1, uniform-zero-v2 and variant-population-v1. Their raw
trees remain under `runs/`; exclusion is not a statement that they failed
or finished. The complete replacement below includes the reconciled later
outputs and retains this earlier digest as a superseded version.

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

## Complete 43-directory checkpoint: published and verified

The same canonical name now resolves to all 43 literal reconciled run
directories, including failed/interrupted attempts, every new census and
independent public-point replay, external terminal inventories, proof gates,
and the original post-merge software gate:

    store:evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz
    1,153,392,456 bytes
    SHA-256 54b31b3d73602d7758dad263939bdfdcbd8c305df2299fe7beb498029090b729

`pack-complete-checkpoint-level1.sh` records the exact 43-name selection,
source label and normalized GNU tar/xz parameters. One compression worker
uses level 1 under the shared busy reservation. The earlier level-6 packing
attempt was stopped deliberately before completing or uploading; its partial
file, hash and before/after stop receipts are preserved separately. No
scientific run or successful upload was repeated to change compression.

The complete verifier freshly extracts the archive, compares all **9,088**
regular files with native `cmp`, matches the inside manifest to its sidecar,
and reproduces the identical archive SHA from the extracted tree. No
archived script is executed. Exact receipts and the packing wrapper are in
`results/publication/complete-archive-verify-v3/`. The interrupted packing
metadata is in `results/publication/interrupted-packaging-v2/`; its 99 MiB
partial remains on the durable workspace at the path in that record.

`results/publication/complete-store-publish-v3.json` records native store
push with `--replace`, successful native cache/remote verification, S3
whole-object SHA-256 and size, and successful checksum/size verification
of the unchanged older f4ddf5ce object. The index retains that prior digest
and byte count in `superseded`; no remote object was deleted. The local old
archive is retained separately before the canonical local copy is updated.
Credentials entered hidden stdin only and are absent from repository files.

This checkpoint is raw-content preservation for the completed finite joint
round. It does not turn timeouts into zero yield or establish a generic S5
ideal, a complete IC price, a strongest-rho comparison or a breakthrough.
Later integration checks after newer main commits live separately under
`results/publication/` and do not mutate the fixed 43-run snapshot. The
prospective zero-fiber collector has its own sibling study and protocol.

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
figures into the unlisted benchmark diagnostics. Later reviews are retained
in v9, v10, v11-partial and v12 inspection receipts. The v11 receipt covers
five documents/eight pages; v12 separately covers the corrected three-page
mechanisms inventory. Exact hashes and byte-identical benchmark copies are
recorded. All benchmark diagnostics remain unlisted, without a promoted
`VALIDATION.json`/rho/GAE claim.
