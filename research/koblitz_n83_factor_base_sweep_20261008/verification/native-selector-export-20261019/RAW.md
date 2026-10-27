# Successor raw evidence archive

`runs.tar.xz` is a small committed successor archive: 1,796,576 bytes,
SHA256 `561c25b06cae953ac55f8b7ade4a3affcfe0b3158ae2e698238ce9de25b8be3b`.
It contains 139 payload files plus `runs/SHA256SUMS`. Fresh extraction
checks every payload hash; deterministic repacking is byte-identical.
The separately approved 49,012,484-byte predecessor remains published in
`setec-astronomy-artifacts`; this successor does not alter that object.

The archive preserves all three attempts, their partial source snapshots,
exact measured workers and runners (including the failed original and
corrected transport checker), inputs, cards, relays, assessments, complete
validation logs and coherent phase-forgery clone. The clone retains its
copied original audit as historical input; the new audit invocation rejects
it, as recorded in forgery.stderr/exit. Six measured/verification ELF files
are explicitly archived under runs/binaries/, rather than staged loose.
The full library test executable remains local; its source, command, logs
and applicability are recorded separately in VALIDATION.md.

Build provenance includes toolchain, Cargo manifests, frozen source tree
IDs and a source delta from published commit27d17bd1 to cbe8e44da. The
published predecessor contains the 27d17bd1 source snapshot; both exact Git
commits also remain accessible through PR2036. No chat transcript or
credentials are included. COLLECT.sh preserves the exact collection recipe.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
(cd runs && sha256sum -c SHA256SUMS)
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/n83-repacked.tar.xz
cmp runs.tar.xz /tmp/n83-repacked.tar.xz
```

The recorded runner configs use frozen absolute paths. Replaying their
native custody checks requires restoring the archived binaries, inputs and
source directories at those paths (or the same preserved host); do not
rewrite configs or hashes to make a check pass. Compact records/ are exact
copies for review, with original byte hashes retained. The archive is the
source of truth for the complete run tree. No raw tree was deleted.
