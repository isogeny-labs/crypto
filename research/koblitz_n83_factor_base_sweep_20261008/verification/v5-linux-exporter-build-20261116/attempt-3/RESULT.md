# V5 Linux exporter build: registry-copy verifier stopped

The [attempt-3 amendment](../AMENDMENT-3.md) and [launcher](../run-build-attempt-3.sh)
were committed at `e200b4d5b1c7` before execution. The launcher exited
**103** with [stderr](stderr.txt) `registry copy differs`, before
creating its Docker output path or launching a container. The
[post-stop inventory](containers.txt) was empty. The [command](command.txt),
[exit](exit.txt), [disk snapshot](post-stop-df.txt), and exact
[verifier output](verifier.stdout) are retained.

The host registry's `src` is a symlink to an extracted crate tree on
SSD990. The initial `rsync -a` preserved it in the new registry input.
The subsequent `rsync -rcn --delete` dry run omitted symlink-aware archive
mode and printed `skipping symlink` / `skipping non-regular file` for `src`.
Its nonempty output tripped the launcher's fail-closed comparison.
The [link metadata](symlinks.txt) records both paths. This is a
**launcher verification defect**; it is not evidence that a locked
crate archive differs. The preserved symlink would also point outside
Colima's configured mounts, so a corrected copy must dereference it
into a fresh mounted input rather than merely suppressing the warning.

The [diagram](admission.svg) and inspected [PDF](REPORT.pdf) show the
stop. There was no compiler invocation, binary, solver call, or IC timing.
A versioned next attempt should copy the 895 MiB extracted source tree
into a new registry input, verify its bytes with symlink-aware rules,
and retain the frozen source, image, offline/no-network policy and caps.
