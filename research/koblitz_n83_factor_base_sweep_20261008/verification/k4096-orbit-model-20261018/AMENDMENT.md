# Corrected K4096 model prelaunch input

The first frozen capacity launch ran for 16.480 seconds and exited 1 with
`PRODUCER_FAILURE_worker_exit`. It wrote no `worker.json`, and its stderr
reported an operating-system missing-file error. The reused worker's
`chain_capacity_cli` calls `public_target(root, ...)`, which reads
`root/probe-corpus.json`; the fresh panel had the exact public corpus only in
its adjacent fixtures directory. The manifest, compressed object, replay and
upload receipts were present and hash-checked. The first source, config and
binary stayed frozen; Docker cleaned up. This is an input-preparation failure,
not a SAT-model cap or search result. Preserve the entire first receipt.

For one explicitly corrected second attempt, copy the frozen fixture
`probe-corpus.json` byte-for-byte into the panel root. Run `preflight.py`
before Docker to require the exact S3 bytes, source snapshots, worker hash,
panel receipts and **both** corpus locations to agree. Do not alter any
other input. Reuse the original model limits: five summands, orbit-phase
domain, identity mask zero, 150,000 variables, 2,000,000 domain clauses,
one CPU, 4 GiB, zero swap, no network and 900 seconds. A passing model may
still trigger only the originally frozen one-trial 5,000-conflict search.
Stop after any second failure or cap; no further automatic retry.

Charge the first launched model stage its full 900-second cap, the corrected
launched stage another 900 seconds, a conditional search 1,200 seconds and
at most 300 seconds of preparation/audit reserve. Thus the amended maximum
conservative new-stage envelope is 3,300 seconds, separate from K4096 base
construction. Retain observed walls independently. No model-construction
failure or capped search ranks complete index-calculus runtime.
