# Independent V2 runner source review

Verdict: no remaining blocking source issue under the documented trusted local cache/toolchain and cooperating host-builder assumptions. V1 and its independent review remain unchanged.

Runner `../owned-authentic-validation-runner-v2-20261107/run.py`: SHA-256 `bdbebde76d9276173cdb35268606804767916b95c2fe27a3f48af5d87c782678`, 20,381 bytes. Producer manifest: `2ec878d1f9863ef4c1f6eabaa9cd32605f5ef52574ebe802ff53500e111cd5cf`.

Reviewed the complete V1-to-V2 diff and unchanged acceptance constraints. Only the fixed existing `/var/tmp/crypto-ml-search-recovery-target-20261107` cache is admitted. A nonblocking exclusive flock on the fixed host build-lock path is acquired before the one-shot marker and held through compilation, exact controls, native acceptance, cleanup and receipt serialization. Regular-file/no-symlink lock admission is explicit. Contention refuses without consuming the attempt. The additional process scan inspects cache references in argv, executable location and exact CARGO_TARGET_DIR environment; environment contents are not logged. This scan remains advisory against noncooperating users and is not a race-proof lock.

The prior release/crypto digest is authorization-bound, checked before archival copy and again afterward; the archive is fsynced and independently hashed. This executable is custody evidence only: executable admission still requires exactly one matching crypto/lib profile.test artifact from the current Cargo JSON output and matching repository manifest. There is no old-executable fallback. Standard Cargo cache fingerprints remain a trusted build prerequisite, not independent cached-dependency authentication.

Git HEAD and status checks each receive up to 60 seconds, always truncated by the shared remaining allowance. The 1200-second/two-job compile, all eight exact discovered registry controls within 60 shared seconds, one complete snapshot-1 acceptance within 300 seconds, overall 1560 seconds, no retry, no cloud operations, source/manifest/executable joins, and failure/cleanup receipts remain intact. Earlier V1 review covers those unchanged mechanisms.

Review correction: the initial V2 process scan omitted environment-selected target use; exact CARGO_TARGET_DIR inspection was added and the cache provenance limitation made explicit in README.

Validation performed here: read-only source comparison and Python ast.parse, without importing or running the runner. No compilation, registry test, native cell, cloud action or cache mutation was performed. Kernel/filesystem hangs, host loss or external SIGKILL can prevent final cleanup or receipt completion; preserve the one-shot marker and any artifacts for reconciliation. The parent must independently authenticate final source publication, authorization, and prior-cache custody; this review does not grant execution permission.
