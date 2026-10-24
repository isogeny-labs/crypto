# Source import and validation receipts

This directory records the validation accompanying the import into
`aburan28/crypto` under `integrations/envoy-fpga-crypto`.

The original 36 source files are preserved from `envoy-fpga-crypto.zip`,
SHA-256 `f2267c211fa647ec9ce995b79aea909c4f1ddab11498cd043e4759c642592174`.
`manifest.json` records their individual hashes, commands, tool versions,
validation outcomes, and the source repository's base commit. The receipt
files in this directory are additional import metadata.

Raw logs are stored in the deterministic `runs.tar.xz` archive. Verify and
extract them in this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Repacking the extracted logs with the following command reproduces the archive:

```sh
tar --sort=name --mtime='2026-10-17 00:00:00Z' --owner=0 --group=0 \
    --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > runs.repacked.tar.xz
cmp runs.tar.xz runs.repacked.tar.xz
```

The extension tests exercise CPU crypto through the behavioral FPGA mailbox
model. Native Envoy verification compiles the production source objects
against the pinned headers. These checks do not establish FPGA acceleration,
a deployable AFI, a fully linked/live Envoy deployment, or F2 interoperability.
No throughput or cryptanalytic performance result is claimed.

The root Rust library check is a repository regression gate; the standalone
C++ extension does not call the educational Rust cryptographic primitives.
That check could not compile the unchanged base revision
`c6d64436a33359320ad9d5b42c4acebee2968b9a`: the `Instance` enum in
`src/cryptanalysis/ecbench/workload.rs` defines `Fp4` twice (lines 377 and 385).
This is recorded as a failed repository check, not a passing test run.
The existing Rust files are unchanged by this import.
The initially omitted `isogeny_algos` sparse-checkout dependency was materialized
before retrying that check. The failed checkout attempt is retained separately.
Conductor was unavailable in this session (`conductor: command not found`);
the repository's `docs/CONDUCTOR.md` documents the non-blocking fallback.

See [validation scope](../../validation.md) and the
[post-quantum integration guide](../../post-quantum.md) for reproduction commands
and implementation boundaries.
