# Round21 evidence release

The scientific round,full Lean build/audit and archive validation are
complete. The sealed validation archive is now published and
retrieved into a fresh local cache. The fresh bytes match the sealed archive
and committed SHA256. Remote verification passes. See
[publication receipt](ARTIFACT_PUBLICATION.json). This closes the artifact
publication gap;the mathematical requirements remain partial.

The committed raw archive is5607768 bytes. The validation archive is
14861500 bytes,SHA256
`e9c89a33a4cf96ae075e015e02829d0ad40165634c0317d40d5c2091ed576fa2`.
Its246-member manifest and archive hash are committed. The immutable name is
`store:evidence/koblitz_modular_ic_20261018/round21-validation-runs.tar.xz`.
The store index records its hash,bytes and remote in provenance. Its remote
is `s3://setec-astronomy-artifacts/store`,as specified by the user.

The store currently selects one remote per invocation. Select this artifact's
remote explicitly;the default remote and other26 index entries are unchanged:

```sh
CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
  ecbench store pull \
  store:evidence/koblitz_modular_ic_20261018/round21-validation-runs.tar.xz \
  --out research/koblitz_modular_ic_20261018/rounds/21/validation-runs.tar.xz
cd research/koblitz_modular_ic_20261018/rounds/21
sha256sum -c validation-runs.tar.xz.sha256
tar -xJf validation-runs.tar.xz
sha256sum -c validation-runs.sha256
```

Use normally configured credentials authorized for the artifact bucket.
No credentials were written into the repository or persisted as configuration.
The upload used native ecbench store push without --replace;the receipt
records uploaded=true,replaced=false. A fresh remote pull,native SHA256,
byte comparison and remote store verification all pass. No object was
overwritten or deleted.

Earlier failures remain in STORE_UPLOAD_ATTEMPT01.txt and
STORE_UPLOAD_ATTEMPT02.txt. They targeted the default
`crypto-autoresearcher` bucket. Supplied artifact credentials authenticated
but that bucket rejected access. The user corrected the intended bucket;
this was a destination mismatch,not invalid credentials or a need to expand
permissions. The successful upload used only the corrected artifact remote.

CI replays the new scientific results from the committed raw archive and
retains the full Lean proof gate. Retrieving the validation archive above
provides the sealed prior validation logs;it is not required to rerun the
mathematical generators. ARCHIVE_VALIDATION.txt records completed local
fresh extraction,strict replay and byte-identical repacks. Its222-source
check refers to ledger-seal01.json. ledger-release01.json preserves the
source pins from51d1eb3 before the corrected publication metadata. The final
ledger.json repins the release protocol/CI correction with the same raw
manifest;PUBLICATION.txt records current checks.

The wider requirements remain partial as listed in README.md:four node187
models,full quotient saturation,final polarized embeddings and the integral
level77 factor map remain open. No IC efficiency verdict.
