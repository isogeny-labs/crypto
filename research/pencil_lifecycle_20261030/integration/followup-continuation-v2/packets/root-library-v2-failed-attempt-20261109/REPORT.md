# Second root library validation: failed attempt

The second native validation invoked compilation, then stopped after the supervisor could not parse a five-digit fractional-second lease expiry with the Python 3.9 fromisoformat call. The captured error is Invalid isoformat string: 2026-11-09T18:10:34.65128+00:00. The worker received signal 15 and reaped its compiler process with exit -9. No control or full-suite test command was invoked.

The compile command occupied 153.821282613 seconds. The worker receipt reports 427.241302049 seconds; completion reports 428.185508705 seconds; the caller reports 428.302402910 seconds. These durations are nested observations, not additive costs. The full 1800-second reservation remains charged. Native limits were 2 CPU, 16 GiB memory, zero swap and 512 tasks.

The completion status is failed and the completion seal status is passed. The supervisor's frozen pre-seal record still says pending_seal with seal_status pending; it was not rewritten into a success. The final completion record binds the terminal output manifest. All nine files in that manifest were checked by size and SHA256; the four durable log prefixes were checked independently and retain their original sealed=false fields.

Worker, supervisor and later root-terminal receipts agree on invocation 2fe218961dcb4bff958ddfd5301234b7. The supervisor recorded owned cleanup complete. The independent root readback records MainPID zero, failed service state, exit status one and absent cgroup. This archives stopped ownership and retained failure evidence, not a passing compile or test result.

The native and caller directories are archived without modifying their original files. raw-hashes.json records every archived file. result.json preserves the actual errors, status distinctions, resources and costs. No payload was rerun for this archival review. The archive is stored by content hash; artifact metadata and replay instructions identify its durable location.
