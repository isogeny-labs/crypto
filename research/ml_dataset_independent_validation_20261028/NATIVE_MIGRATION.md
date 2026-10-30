# Native consumer migration — 2026-10-28 UTC

The repository's `AGENTS.md`, “Implementation language: no Python”, explicitly
covers field arithmetic, certificate verifiers, correctness tests and replay.
The original consumer in PR #2754 and the phase-label extension in PR #2757
used Python. Those historical files and evidence are retained; they are not
an exception to the current rule and are not the supported execution path.

The `native/` crate replaces audit, phase-label validation, static export, S3
restore and optional producer replay. Independent positive-certificate
arithmetic is implemented in Rust without linking the producer's library or
calling Python. The crate uses separate integer/field/group code and native
software tests. It makes no timing or scientific comparison with the historical
implementation.

An unpublished Python arithmetic draft was superseded during this continuation.
Its snapshot is retained under
`s3://setec-astronomy-artifacts/research/ml-dataset/validation/20261028/native-consumer/superseded-python-arithmetic-draft.tar.gz`.
The draft's software-test results do not establish correctness of the native
replacement. Native validation has separate source hashes and receipts.

The mathematical scope is the public positive equation and specified domain.
Group/subgroup cardinality, solver independence, UNSAT, exact instruction cost,
field-size generalization and fall-degree claims remain unverified. A software
fixture is not a native producer observation, and storage verification through
the controller role does not establish permissions for the EKS worker role.

The superseded draft archive was downloaded and rehashed successfully:
`5457479f629afc34b19baf573f69b2a4e5b6ccc53878f8df70dd19c538cf0a15`
(27,154 bytes). The native replacement's separate validation is in
`evidence/native-20261028/validation.json`.
