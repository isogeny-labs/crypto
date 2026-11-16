# Reversible host cache relocation before K600 arm retry

The first boot-space cleanup command using `rm -rf` was rejected by automatic approval review: it disallowed that deletion style and requested a safer approach. No deletion occurred. I instead moved only three inactive **generated Cargo incremental** directories across volumes and placed symlinks at their original paths. The files were Rust `.o` objects and `.lock` files under `debug/incremental/`, not source or frozen receipts. A process/open-file check found no Cargo or Rust process and no open file in the incremental cache at inspection. The original names remain usable through the symlinks. The move is reversible; the destination is temporary task-owned cache storage.

| Source suffix `cryptanalysis_suite-…` | Destination root | `du -sk` after move | Observed boot free before → after (KiB) | Observed SSD990-2 free before → after (KiB) |
| --- | --- | ---: | ---: | ---: |
| `26dlisbrohh4c` | `/Volumes/SSD990-2/tmp/n83-boot-cache-quarantine-20261116/` | 1,670,332 | 4,336,152 → 5,352,412 | 24,290,520 → 22,619,868 |
| `1bbc7bofpklms` | same | 714,332 | 5,352,412 → 5,760,140 | 22,619,868 → 21,902,852 |
| `0lrbe1hcs97sk` | same | 373,432 | 5,760,140 → 6,060,720 | 21,902,852 → 21,527,276 |

`df` is a host-wide sample and its differences are not attributed entirely to these moves; other processes may change free space concurrently. This cleanup did not lower the frozen 5 GiB boot or 20 GiB SSD stop floors. It did not execute a K600 worker, stage the S3 input, or alter H2. Attempt 4 later stopped at the unchanged boot floor, so this relocation alone did not solve the resource gate. The symlinks and destination contents remain on disk for a later host operator to restore if needed; moving them back before more compile work would reduce boot-space margin.
