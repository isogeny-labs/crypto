# Fp4 metadata mirror erratum

The four Fp4 candidate records were accidentally appended to a byte-for-byte mirror of cryptanalysis metadata. They were not registered ICV1 models or representations. This correction restores `docs/curves/ic/curves.yaml` to cryptanalysis `main` at `3ce4c29ec497ff3149eb841c07e9dd9f6631de77`. The local mirror lock already hashes those authoritative files and is unchanged.

[`candidate-entries.snapshot.yaml`](candidate-entries.snapshot.yaml) retains the exact appended bytes, including the original computed identities, unsupported status literals and historical registration wording. It is research provenance, not a current catalog or an input to the canonical metadata validator. All four exact `field`/`curve` records, EC1 aliases and full UIDs are unchanged and their canonical hashes check. A computed identity string does not establish registry membership; none of the four is registered in the current model catalog, traits are not newly certified, and no representation link is asserted.

[`provenance.json`](provenance.json) pins the upstream source, original file and snapshot hashes, reconstruction rule, exact preserved identities and unavailable Conductor check. Old plans, run manifests, source snapshots, registration receipts and reports remain bound to their original commits. No arithmetic trial or new mathematical conclusion was produced.

The upstream mirror lock is itself stale for its YAML. It is not a peer-compared file. Copying it would introduce a new failure; retaining crypto's unchanged, correct local lock keeps all three file digests valid while all five peer files match.

Native trait-input freshness and browser generation follow this correction in the integrating workstream. A dependency hash must not be refreshed without the native parsed-input equivalence and final freshness checks.

Validation passed: unchanged semantic controls ran nine tests (two expected cryptanalysis-only skips); local and pinned-peer validators both report zero errors. All five peer files match the pinned source, all three unchanged local lock digests match, all four computed model slugs are absent from the current model registry, and all four preserved EC1/UID preimages check. Exact commands, exit codes and retained logs are in [`validation.json`](validation.json).
