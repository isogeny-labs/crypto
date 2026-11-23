# V5 Linux exporter build: offline registry staging

Written before attempt 3. [Attempt 2](attempt-2/RESULT.md) reached Cargo
inside the pinned Linux image and stopped with exit 101 because that
image's offline registry lacks `aws-credential-types`. It did not
produce a binary. The raw log and receipts remain unchanged.

The host's configured Cargo registry at
`/Volumes/SSD990-2/indexsmash-cache-relocation-20261115/cargo-registry`
is 196 MiB and contains the locked `aws-credential-types` 1.3.1 crate.
Its archive SHA-256
`576e8da35751dc85409ec1cd15999f692c277a1fbe9336780fe6f15e49dd637d`
equals the checksum in the frozen `Cargo.lock`, whose full SHA-256 is
`cf805b650c4e42ce27a9621749abbde2d337266a6856e39aa0b15cf012f444e6`.

Permit exactly one attempt 3 with [its dedicated launcher](run-build-attempt-3.sh).
It reuses the source snapshot checked in attempt 2, rechecks its
critical files and attestation against the clean branch, copies the
host Cargo registry into a fresh mounted input directory, and verifies
the copy by checksum comparison. The container receives that registry
read-only at `/usr/local/cargo/registry`. Keep `--locked --offline`,
no network, source commit `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`,
the same pinned image, 12 GiB/zero-swap/four-CPU cgroup, 1,800-second
outer cap and 2 GiB host disk watchdog. Use a new output directory.

This changes dependency-cache availability only. Any missing additional
crate, cache-write error, compiler failure, timeout, OOM, or disk stop
is retained distinctly, with no automatic retry. A terminal build pass
creates a Linux exporter binary and still does not execute a relation
or determine total cold IC runtime.
