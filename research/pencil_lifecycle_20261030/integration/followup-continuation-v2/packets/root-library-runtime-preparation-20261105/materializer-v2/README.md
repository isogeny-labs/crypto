# Materializer v2: candidate custody and bounded child cleanup

Preserves committed v1 unchanged. Independent source review found M1: candidate hash and copy used separate reads; M2: subprocess cleanup used an unbounded wait and skipped descendants after leader exit.

M1: copy_candidate reads once, hashes those exact bytes, writes them, and requires the destination digest to equal the approved candidate digest. The materializer separately checks that the manifest row has that same pinned digest. All initial hash-bound input files are rechecked before writing the final input manifest. A mutation before the single read is rejected; mutation after it cannot alter the copied bytes, and final input readback catches a changed bound source.

M2: every owned subprocess cleanup sends SIGKILL to its session group even after leader completion, waits with a two-second timeout, records timeout/failure explicitly, then probes group absence. Any incomplete cleanup prevents a passed receipt. ACTIVE is cleared before cleanup to avoid repeating ownership cleanup after an exception. Outer systemd InvocationID/cgroup-empty confirmation remains mandatory; a worker receipt never substitutes for it. Uninterruptible kernel waits are retained as incomplete cleanup, with the existing 900-second unit ceiling as final containment.

Six mock-only controls cover candidate single-read behavior, wrong candidate, corrupt destination, exited-leader descendant cleanup, bounded wait timeout, and surviving group rejection. Controls are provided but not executed in this source-only preparation. Python3.9 AST parsing accepted the worker, helper and test source. No materializer/compiler/native test or cohort ran.

Root still fills the exact configuration, publishes bindings, obtains review and launches only after the active norm payload is terminal. All source revision, candidate hash, resource bounds and full library validation scope remain unchanged. The new helper is an additional required hash-bound input.
