# V5 Linux exporter build: dereference the registry source

Written before attempt 4. [Attempt 3](attempt-3/RESULT.md) exited 103
before Docker because its dry-run verifier skipped the host registry's
symlinked `src` tree. The source link points to an approximately
895 MiB extracted-crate tree on SSD990, outside the Colima mounts.
The attempt-3 input and report are preserved.

Permit exactly one attempt 4 with [its dedicated launcher](run-build-attempt-4.sh).
It uses `rsync -aL` to copy the configured host Cargo registry,
dereferencing its source link into a **new** input directory. It
compares the copied bytes using `rsync -aLrcn --delete`, checks the
locked `aws-credential-types` archive SHA-256, and rejects a remaining
source symlink. The copied registry is mounted read-only at
`/usr/local/cargo/registry`. The code source, clean source snapshot,
`Cargo.lock`, pinned image, `--locked --offline`, no-network policy,
12 GiB/zero-swap/four-CPU cgroup, 1,800-second cap, and host disk
watchdog are unchanged. Use a new output directory and container name.

This corrects the verifier and ensures the extracted sources are
available inside Colima. A failed byte comparison, missing dependency,
compiler error, timeout, OOM, or disk stop remains a separate stopped
result. A successful build is still only worker preparation, not a
factor-base or total-runtime selection.
