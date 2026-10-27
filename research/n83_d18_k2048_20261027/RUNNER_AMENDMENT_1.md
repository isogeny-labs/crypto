# Runner mount refinement after the K2048 base export

The export and replay receipts in `evidence/` bind the executed runner to
source commit `a1e6b778dcee8f4ec72f359df7ce3728b302b78a`. That runner
mounted `/Volumes/SSD990` read-only so `git show` could verify the committed
exporter bytes. It had no network and wrote only to the separate output mount.

For later reruns, `run_guarded.py` narrows the read-only checkout mount to
`/Volumes/SSD990/crypto` and adds the checkout's read-only Git alternate
object store at
`/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019`. A container
preflight confirmed `git rev-parse HEAD` and `git show
HEAD:examples/n83_d18_k2048_export.rs` with those two mounts. The factor-base
selection, exporter bytes, container compute/memory limits, and stored object
were unchanged. The original run receipts remain bound to the earlier commit.
