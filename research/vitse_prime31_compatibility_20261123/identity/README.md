# S3 coordinate-negation identity record

The production `identity.certificate/v1` implementation issued certificate
`IDC1h095fa9aa1df01499` for the identity in the parent mathematical review,
section 3. The standalone issuer and checker both exited zero. The issuer
checked 32 points with seed 20261123; the separate checker replayed those
points and checked 32 additional points with seed 20261124. Both expression
trees have syntactic degree 4 with ordered variables `X,Y,t,a,b`.

The wrong RHS-sign control was rejected by both programs at their first
derived point. A one-digit value-digest mutation produced `DigestMismatch`.
A declared degree of 3 was rejected by the production structural validator.
The certificate, executable hashes and complete source/dependency closure
were unchanged across the native calls.

- Certificate: [identity.certificate.json](identity.certificate.json).
- Frozen method: [IDENTITY_PROTOCOL.md](IDENTITY_PROTOCOL.md).
- Source/build binding: [source-build.json](../evidence/identity-01/source-build.json).
- Dependency archive and source pins: [dependency-source-manifest.json](../evidence/identity-01/dependency-source-manifest.json).
- Raw issuer output: [stdout.txt](../evidence/identity-01/issuer/stdout.txt).
- Raw checker output: [stdout.txt](../evidence/identity-01/checker/stdout.txt).
- Qualification summary: [summary.json](../evidence/identity-01/summary.json).

Certificate SHA-256:
`dab1ee01be679af66b3109b8213fd921807dd294d18ff8b9080dc7bface009bf`.

This is a same-team deterministic finite-evaluation record over `F_(2^61-1)`.
It is not a formal proof over the integers or the Prime31 target field. No
independent verification, solver symmetry, scalar recovery, or performance
claim is made. The optional formal-proof field is null.

The crate is self-contained in its Rust source and pinned Cargo lock. Its two
binaries accept one certificate path: the issuer requires a new output path,
and the checker reads that path. `qualify_identity.py` is the exact metadata
runner used in the original workspace; its source/cache/toolchain locations
are recorded in the evidence. A later provenance-qualified replay needs a new
attempt and output directory and must not overwrite this evidence.

One metadata preparation failure is retained in
`../evidence/identity-01/preparation-01.failure.json`: the borrowed cache lacked
optional `.cargo-checksum.json` files. No build or identity evaluation had
started. The remedy preserved mandatory locked archive checksums and
byte-for-byte archive/source equality, and made only that optional metadata
check conditional on its presence. Cached dependencies were copied to a new
task CARGO_HOME; the parent cache was not modified.

`cargo-home/` and `target/` are local preparation/build directories excluded
from publication. All native outputs and per-call pre/post manifests are in
the evidence directory.
