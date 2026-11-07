# Bounded root-input materialization: failed attempt

The single released materializer attempt failed before copying any files. Git ls-tree exited 128 while mapping a repository packfile; the materializer exited 1 after 0.363226752 seconds. No compiler or test ran, and no retry occurred.

The native error states that the packfile could not be mapped and mentions vm.max_map_count and/or RLIMIT_DATA with Cannot allocate memory. The source sets RLIMIT_AS to 4 GiB before invoking Git; the unit independently enforces 4 GiB physical memory. These are different limits. This attempt alone does not isolate every possible cause of the mapping failure.

The native receipt records cpu.max=200000 100000, memory.max=4294967296, memory.swap.max=0 and pids.max=512. The independent outer read of those cgroup files arrived after the failed unit's cgroup disappeared; that outer observation is retained as unavailable. systemd recorded 71464000 CPU nanoseconds. MemoryPeak was not returned and remains unknown.

InvocationID 9e44ca2f1d5a48c69949ea4c06a22303 was captured 0.335 seconds after launch. Terminal readback showed the same owner, ExecMainStatus 1, MainPID 0 and absent captured cgroup. An exact-owner stop request returned 0; subsequent readback still confirmed no running process/cgroup. Cleanup was confirmed. The outer observation interval was 68.796810697 seconds, including agent response and custody/cleanup steps; it is not native processing time.

The campaign is charged 900 worker-seconds conservatively, leaving 42300 before the separately reserved 1800-second library validation. No new complete input manifest exists; the existing validation cannot consume a successful preparation result from this attempt.

The native directory preserves launch/source/review bindings, stdout/stderr readback, native failure receipt, ownership, cleanup and terminal hashes. The diagram shows the recorded sequence; no performance comparison or inferred cost ratio is provided.

Read-only diagnosis: the named packfile is 4,680,295,288 bytes, larger than the source-set RLIMIT_AS of4,294,967,296 bytes. No core.packedGitWindowSize/core.packedGitLimit override was returned by Git configuration readback. Linux documents RLIMIT_AS as a virtual-address limit affecting mmap; this supports the mapping-pressure interpretation but does not measure the exact failed mapping. Source: https://man7.org/linux/man-pages/man2/getrlimit.2.html . Git configuration reference: https://git-scm.com/docs/git-config . No diagnostic retry was run.
