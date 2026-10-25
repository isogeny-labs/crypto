# Round21 evidence release

The scientific round,full Lean build/audit and archive validation are
complete. Publication of the validation archive is **blocked by artifact-store
access**;this is an access failure,not a mathematical or verification result.

The committed raw archive is5607768 bytes. The sealed validation archive is
14861500 bytes,SHA256
`e9c89a33a4cf96ae075e015e02829d0ad40165634c0317d40d5c2091ed576fa2`.
Its246-member manifest and archive hash are committed. Its bytes remain at
`research/koblitz_modular_ic_20261018/rounds/21/validation-runs.tar.xz` in
the retained migrated workspace,ignored by Git. No remote upload succeeded.

The configured store is `s3://crypto-autoresearcher/store`. The migrated
worker returns AccessDenied for ListObjectsV2 and403 for HeadObject. The
actual failed native push is preserved in STORE_UPLOAD_ATTEMPT01.txt.
The requested immutable name is
`store:evidence/koblitz_modular_ic_20261018/round21-validation-runs.tar.xz`;
it is **not indexed or resolvable yet**. Restore the intended store access,
repeat the recorded `ecbench store push` without `--replace`,verify a fresh
pull against the committed hash,then commit its store/index.json entry.
No permission expansion,remote deletion or duplicate research job occurred.

CI replays all new scientific results from the committed raw archive and
retains the full Lean proof gate. It does not assert that the validation
archive was downloaded. ARCHIVE_VALIDATION.txt records completed local
fresh extraction,strict replay and byte-identical repacks. Its222-source
check refers to ledger-seal01.json,the source snapshot before the release
protocol/CI amendment. The final ledger.json repins that amendment with
identical raw manifest;PUBLICATION.txt records final source checks.

The wider mathematical requirements remain partial as listed in README.md:
four node187 models,full quotient saturation,final polarized embeddings
and the integral level77 factor map remain open. No IC efficiency verdict.
