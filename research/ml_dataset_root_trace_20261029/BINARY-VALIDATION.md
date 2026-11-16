# Binary root integration validation

Runtime source `dd1092a0c0cdd07f032cee6f4cf692e753035a33` passed release compilation, 21 binary-root tests, 23 prime-root regression tests, maintained strict Clippy, and the full release-library suite: 5,284 passed, zero failed, 123 ignored, 1,538.99 seconds. These results do not validate later main/cofactor combinations.

The preregistered native CLI software fixture passed all 34 commands and all eight binary/Koblitz, n11/n12, twist-position certificate/export gates. Its downloaded executable verified all eight downloaded datasets; verification stdout matched the original byte for byte. No PDP success requirement or resampling was imposed. These are software fixtures, with zero scientific corpus rows and zero verified cloud-generated cells.

Full-suite evidence and immutable test executable:
`s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/binary-root-full/a976d3e20fe1175610ab73ca2291a60ce439aa66042eec17ebb0ba69403d3670/full.tar.gz` (42,922,187 bytes).

CLI and fixture archive:
`s3://setec-astronomy-artifacts/ml-dataset/20261029/software/binary-root-cli/2cb8a226d2080dcc58cdc47a4bdb543bf0f1e88b9e062273b153ef1110f4d365/fixture.tar.gz` (15,156,498 bytes).

Restored native replay attestation:
`s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/binary-restored-replay/3a1d1a888256803c31bcae88707588925708410814fc07c7c3c5286e3cddadfc/attestation.tar.gz`.

All archives passed conditional PUT, fresh HEAD and ETag-bound GET, archive-byte equality, lengths and contained-file hashes. The native replay was an additional check after that readback. Arithmetic distinct-root certification remains separate from execution completeness, total operation accounting, PDP existence, scalar recovery and fall-degree bounds.
