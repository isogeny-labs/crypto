# Preserved phase2 failure

The one source-validation invocation did not produce diagnostics or an olean before its work cutoff. The cleanup wait also exceeded2seconds, and the finally block raised before writing its native receipt. The exact tool-return traceback is transcribed as compiler-phase2-tool-traceback.txt. Later process observations found no matching Lean command; they do not retroactively certify the parent successfully reaped the child.

The full posthash subsequently completed:16,736 bound files and3,111,892,318bytes exactly match the combined premanifest, including file identity. The current source remains uncompiled.

No per-process CPU, page-fault or I/O history was captured for this compiler invocation. Empty diagnostics plus long startup and delayed kill wait are compatible with cold-cache I/O, but do not establish that cause. The post-termination host snapshot is a later observation only. No claim that theorem elaboration itself is slow is supported.

An unexecuted successor compiler-phase3.unreleased.py makes finally-block kill and wait failures explicit failed-receipt fields, including unresolved owned PID, rather than allowing them to bypass receipt generation. It references a new phase3 recipe that is intentionally not present yet. It has not been tested, frozen, released or run. Any further validation requires an amended finite plan; no retry has occurred.
