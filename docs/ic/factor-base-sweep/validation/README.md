# Raw validation bundle

`runs.tar.xz` seals the completed native records, earlier failures/partials,
recovery notes, exact commands, input hashes and retained functional run.
Archive SHA256:
`2563a83feee992afcdff5ea408542700d834c8f1c94bfa8431efba9d9bf02931`.
All **567 regular-file member hashes** in the detached `SHA256SUMS` passed
after extraction. Deterministic repacking of the extracted tree is byte-for-byte
identical to the published archive. Native verification of the extracted run
also passes all 16 sealed attempts and 29 point witnesses.

From the repository root:

```sh
sha256sum -c docs/ic/factor-base-sweep/validation/runs.tar.xz.sha256
mkdir extracted-records
tar -xJf docs/ic/factor-base-sweep/validation/runs.tar.xz -C extracted-records
(cd extracted-records && sha256sum -c ../docs/ic/factor-base-sweep/validation/SHA256SUMS)
factor-base-sweep verify --run extracted-records/functional-both-run
```

Repack using the same metadata normalization:

```sh
tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner \
  -cJf repacked.tar.xz -C extracted-records .
cmp docs/ic/factor-base-sweep/validation/runs.tar.xz repacked.tar.xz
```

Extraction/hash/native-replay/repack receipts are beside the archive rather
than added into its already sealed input tree. The initial member-check
command ran from the wrong working directory and therefore failed to find its
relative member paths; `member-check-initial.log` retains that metadata error.
The corrected check ran inside the extracted root and passes every member.
This does not alter the scientific or software test outcomes in the bundle.
The generated SVG's line-end whitespace was normalized without changing its
recorded values or geometry. The PDF has a local binary Git attribute to
preserve renderer bytes; it was rebuilt and all four pages inspected again.
The initial text-style whitespace diagnostics are retained compressed.

See [the validation report](../VALIDATION.md) for the actual failed upstream
gates. Hash/repack success never promotes a failed or partial native run.
