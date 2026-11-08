# Fixed-label residual split: completed finite census

CM-RESIDUAL-SPLIT-V1 completed its single42,480-row native census. Independent GMP replay validates every raw schedule and20minimum sets. All300 selected-plan point/count checks pass. Before data, native qualification passed3150 point/count cases,450 explicit-map/endpoint cases and five wrong-remainder controls. Five post-scan corruptions reject at their intended checks: wrong selected count, wrong raw count with updated checksum, missing row, duplicate row and changed protocol. No scientific scan was repeated.

| Curve | Optimized direct D+A | Binary split control | Relation residual split |
| --- | ---: | ---: | ---: |
| P-192 | 228 | 229 | 232 |
| P-224 | 260 | 260 | 267 |
| P-256 | 300 | 300 | 304 |
| P-384 | 449 | 450 | 451 |
| P-521 | 606 | 606 | 612 |

These are the preregistered weight1 minima, not runtime results. The relation arm is higher than both controls on every curve. Weights2..4 retain the same conclusion. Each method receives the declared complete optimization, and every D/A/N/table vector remains in plans.json. The scalar stays the original mu on all curves; no generator, subgroup or target was substituted.

The selected relation vectors (D,A,N,T) are P-192 (192,40,19,6), P-224 (222,45,19,10), P-256 (252,52,24,10), P-384 (379,72,26,18) and P-521 (521,91,33,18). The earlier published translated zero-remainder recipe selected235,273,309,458,613 D+A, respectively. That is historical engineering context from a different frozen design, not this run's decisive comparison. The new P-192 schedule uses four fewer doubles, one more addition, two more negations and three fewer table entries than its earlier recipe; setup/field/runtime effects are not inferred from this vector.

## Source, protocol and raw custody

Source/checker publication df8001b87da1 preceded frozen-protocol publication206ba5c3e9bd. The canonical body digest is31b28e2913c78fc1c78c819f8190af17b777d4f18b810dec9120cc22071fe335. The frozen record binds the producer/replay binaries and sources, seven input files, widths, radices, offsets, tie rule and finite resources. Hosted publication and open-PR readback precede data in runs/publication.json. PR #3674 existed before implementation and execution.

The complete raw tree, corruption inputs and all process exits are in runs.tar.xz,104,052 bytes, SHA256 f45bf33a443157615b1a7a68992fe14db5ce3fc1e10a65e85a37e7bc671e6b70. The local runs directory is ignored. Verify `sha256sum -c runs.tar.xz.sha256`, extract with `tar -xJf runs.tar.xz`, then verify `sha256sum -c runs-manifest.sha256` from this directory. Fresh extraction/replay is byte-identical and deterministic repacking reproduces the archive. Do not invoke the producer --scan into an existing output or repeat the census.

From the repository root:

~~~sh
make -B -j1 -C research/cm-marked-scalar residual-binary residual-replay-binary
/tmp/cm-marked-scalar-build/residual-replay research/cm-marked-scalar/fixtures-v2 research/cm-marked-scalar/protocol/residual-v1/protocol.json research/cm-marked-scalar/residual-v1/runs/scan/scan.csv research/cm-marked-scalar/residual-v1/runs/scan/plans.json
/tmp/cm-marked-scalar-build/residual --qualify-plans research/cm-marked-scalar/fixtures-v2 research/cm-marked-scalar/residual-v1/plans.json
~~~

Replay binds the original binary hash; rebuilding in another compiler/library environment requires reproducing that instrument or a separately recorded software-validation instrument, not editing the frozen record. Original build flags, sources and hashes are in ../qualification/residual-build-v3/.

The schema-valid experiment.export/v1 contains60 exact derived minimum records: five curves x three method arms x four weights, from one census. They are not60 independent process runs or timing trials. The full42,480-row CSV is archived; all controls and sensitivity weights remain available. An initial export-schema lookup used a nonexistent root path, stopped before producing an export, and was corrected to the already committed schema snapshot. No trial or analysis selection changed.

## Cost and next step

Every invocation was capped at120 seconds, one process/thread and2GiB address space, with one scientific scan and zero unchanged retries. No cryptographic clock, new cloud job or controller was launched. No total-dollar cost or native-time claim is inferred; local existing-host billing was not measured. The full cold comparison still needs plan preparation, allocation, normalization, conversions, verification and destruction, and an admitted funded host.

The next implementation opportunity is retaining useful intermediate multiples from the radix prefix, with the same reuse offered to binary controls. The present evaluator discards those points. Its full-vector count gap is now four calls on P-192 and two on P-384 against direct multiplication. That next implementation requires its own published source/freeze before comparative data; this run and its source remain immutable.

## Artifact-store materialization

The repository bot externalized the archive in4942fc80cad1. Its .artifacts.json binds the original SHA256 and104,052 bytes. The local archive is now ignored, and a byte-identical copy is retained. From this directory, materialize before verification/extraction:

~~~sh
aws s3 cp --no-progress s3://setec-astronomy-artifacts/store/sha256/f4/f45bf33a443157615b1a7a68992fe14db5ce3fc1e10a65e85a37e7bc671e6b70 runs.tar.xz
sha256sum -c runs.tar.xz.sha256
~~~

The bot's index was retained without force-adding the archive back to Git. Local bytes match the bot manifest and the previously verified extraction/replay/repack. The agent did not perform an additional remote GET after migration; artifact-location.json distinguishes those checks. No scientific scan was repeated.
