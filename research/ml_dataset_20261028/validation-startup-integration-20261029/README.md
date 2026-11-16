# Startup and prime-chain integration software validation

Runtime source: `7a1bd350b26eaeccf51bf9078915649a7c4335fc`.
Frozen CLI fixture: `127ab13b48e1ec70b54fd189a2fe8c3c87430a26`.
Release binary SHA-256: `733ed77434ddc80e9d99ea085cd7a523ee24be4997126c76aad092f786afbda7`.

The `.artifacts.json` manifest locates exact objects in the approved S3 store.
`remote-readback.json` records fresh-download sizes and SHA-256 checks for the
complete manifest. Earlier readbacks preserve the incremental check-ins.
Logs and exit receipts bind the runtime revision, command and source readback.

Validated: 142 dataset tests, 26 F4 unit tests, one native checkpoint integration
test, 5,170 full release library tests (zero failures, 123 ignored), strict
maintained Clippy, release build, eight actual startup CLI checks and a
separately frozen ten-command generation/replay fixture. The fixture's two
local cells cover both twists at n=7/m=3. One has a verified PDP lift; the other
has encoding-relative completion without a lift. Neither is independent
ECDLP recovery. Both are excluded from the requested research corpus.

The archive `startup-integration-cli-fixture-7a1bd350.tar.xz` contains the entire
fixture output tree under `runs/`: frozen protocol/runner, command records,
original intermediate artifacts, negative-control copy and final receipt.
Its SHA-256 is `fa002e50ffa0af70b2650666345515918c5ca1284e4d2e762bb4ac56897cdb9c`.
An independent read-only audit checked all 158 inventory files and 159 archive
members; the audit record is in the parent research directory. Hydrate the
manifest objects with the repository artifact tool, then extract into a fresh
scratch directory with `tar -xJf startup-integration-cli-fixture-7a1bd350.tar.xz`.
The packer uses sorted names, the fixed 2026-10-29 UTC date, uid/gid zero and
0644 file modes; its source is archived here.

Failed build/materialization attempts are retained alongside successful
retries. The abbreviated-build-identity attempt was intentionally terminated
and is not a test result. Missing tracked inputs were restored from the frozen
Git blobs; zlib-devel repaired the host linker prerequisite. These repairs did
not change runtime source or suppress lint warnings.

No cloud worker was launched by this validation. Zero research-corpus rows,
zero independently recovered ECDLP cells and zero verified cloud dataset cells
are established by this bundle. Startup activation, worker IAM, grant delivery,
finite instance lifetime, cloud restore/replay, complete instruction counts,
exact fall degrees, a trained model and the full corpus remain separate work.
