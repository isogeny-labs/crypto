# Amendment 5: boot-volume output for the pinned Linux exporter build

Freeze this amendment and `run-build-attempt-5.sh` in a commit before
attempt 5. It changes only the **build-output placement** after
[attempt 4](attempt-4/RESULT.md) crossed the SSD990-2 disk floor. The
source snapshot remains `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`,
the Linux ARM64 image remains
`sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`,
and Cargo remains `--release --locked --offline -j 1` with no network,
12 GiB memory, zero swap, four CPUs, and a 1,800 s child cap.

## Admission and custody

1. After the complete attempt-4 raw receipts, disk samples, visual
   report, and PDF are committed, remove only its 1,395,996 KiB
   rebuildable `target/` compilation cache. Preserve every
   attempt-4 receipt and the original source/registry inputs.
2. Add the writable host mount
   `/private/tmp/n83-v5-colima-output-20261116` to this task's
   custom `n83-arm64` Colima profile, restart the idle profile, and
   check that the guest exposes the exact path as writable virtiofs.
   Preserve a before/after config diff and mount check in attempt-5.
   The output path on the boot APFS volume is write-once
   `/private/tmp/n83-v5-colima-output-20261116/attempt-5`.
3. Reuse the attempt-4 dereferenced Cargo registry **read-only**.
   Verify it against its source with the same symlink-aware
   `rsync -aLrcn --delete` comparison and the pinned
   `aws-credential-types` crate SHA-256 before Docker.
4. Admit only when boot output has at least 10 GiB available,
   SSD990-2 at least 2 GiB, and the VM root at least 10 GiB.
   The watcher stops the container if either host volume drops
   below 2 GiB, preserving ten-second samples, child/outer receipts,
   Docker inspection, cleanup status, and binary hash if present.

This is a build admission, not an IC measurement. A successful exporter
binary enables a separately frozen factor-base export and paired
same-target controls; it does not select a minimum-runtime factor base.
If the mount, hashes, source identity, or capacity checks fail, record
the exact stop without launching a replacement. Do not reuse
attempt-4's incomplete target output.
