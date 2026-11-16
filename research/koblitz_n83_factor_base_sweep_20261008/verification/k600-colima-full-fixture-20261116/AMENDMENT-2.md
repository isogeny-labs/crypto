# Full-fixture validation: mount-path amendment

Written before attempt 2. The original `PROTOCOL.md` and `launch.sh` remain frozen as attempt 1.

Attempt 1 at source `c57dd5a6d2ff7e862295c216aca319c82fe3c8ee` stopped at 2026-11-16T02:47:03Z before a container or Rust test started. Docker exit 125 reported that `/Volumes/SSD990-2/tmp/n83-full-validation-20261116` did not exist inside the Colima VM. Docker inspection also returned no object. This is an admission-path failure, not a Rust test result or a cold IC result. Its raw log and receipts are retained under `attempt-1-admission/`.

The Colima profile mounts `/Volumes/SSD990-2/tmp/n83-colima-inputs` read-only. The same full Git worktree was moved, without changing tracked source or the exact restored fixture, to `/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116`. The VM's `ls` sees that directory; host `git status --porcelain -uno` is empty and the archived fixture still hashes to `8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738`.

Attempt 2 uses `launch-attempt-2.sh`. Its only behavioral changes from the frozen attempt-1 launcher are the new source path, distinct attempt-2 output/receipt/container names, and the corresponding in-container path. Image, cgroup, timeout, test command, source cleanliness, tracked-fixture presence, archive hash and byte-count gates remain as in `PROTOCOL.md`. The launch records its own committed source SHA. A later PR merge resolution still requires validation at that new SHA.
