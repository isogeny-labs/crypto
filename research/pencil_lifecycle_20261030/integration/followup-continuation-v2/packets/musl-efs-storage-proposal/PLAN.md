# Storage amendment, prospective only

The local root volume has3.3GiB available. The incomplete exact-commit tree inventory already exceeds3.68GB of tracked blobs, so a full source checkout cannot fit there. /home/ec2-user is the existing EFS mount.

Use a new detached full source worktree at /home/ec2-user/pencil-musl-source-b985-20261031 with exact commit b985ddb30b46abbd9a050e5753c7e301f40b07f6, and a new exclusive target at /home/ec2-user/pencil-musl-target-b985-20261031. Preserve the227MiB old /tmp/pencil-musl-target cache unchanged; copying its existing files to the fresh target is optional and must finish before build admission, without modifying fingerprints/timestamps. Registry and qualified compiler locations remain pinned.

The only wrapper changes from reviewedv2 are source-root parent and target-cache path. Detached HEAD, source clean checks, external-path/config review, ownership lock, deadline, toolchain/features and ELF/link-map checks stay unchanged. Outputs/receipt directory remains /var/tmp; final binary receipt copy requires a fresh space check. Root will select a bounded build cap no more than3600seconds only after preparation and review. No worktree/cache creation or build has occurred in this proposal.
