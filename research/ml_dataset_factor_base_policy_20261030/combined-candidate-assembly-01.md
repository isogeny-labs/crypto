# Combined isolated validation candidate01

Base655b690ef56d6869023a41326a127bf319cf4342 plus donor composition02 and native
fixture04 fixture-only patch. Existing public protocol canonicalizer is already in
the donor overlay; its visibility patch is not applied twice. The existing Cargo
example wrapper includes research/ml_dataset_20261028/startup-integration-cli-
fixture-native/run.rs, which resolves sibling acceptance.rs and all seven embedded
metadata JSONs. No Cargo manifest change or execution approval is introduced.

33 source/metadata files are copied and checked against component inventories.
The complete patch passes git apply --check against the unchanged base source.
Snapshot preparation will independently verify copied base blobs/modes, apply this
patch, hash final source, and compare all33 proposed files before any Cargo.

Run dataset unit tests and then the maintained dataset_startup_cli_fixture example
unit tests sequentially with dirty proposal provenance. Native CLI workflows,
protocol approvals, remote publication and cloud generation are not invoked.
The original full data/measurement/CLI scope remains required; this is a software
integration milestone, not its replacement.
