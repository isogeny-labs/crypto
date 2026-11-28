# D: Lean-only continuation with a smaller allocator reservation

The native C component is completed and public atfcc90790; never rerun it to
repair Lean. C overall remains134. Its complete trace shows a successful1GiB
anonymous reservation and an immediately following1073745920-byte MAP_STACK
request rejected ENOMEM under the2GiB cap. Tagged mimalloc2.2.7 options.c and
arena.c document arena_reserve in KiB, default1GiB; the installed Lean library
also exposes that option. This identifies a source-supported allocator/stack
address-space interaction to test, not yet a successful repair.
[pcc:unverified]

D invokes only one Lean version/startup trace and, if it succeeds, the first
compilation of the unchanged Lean source. Set MIMALLOC_ARENA_RESERVE=65536
before process initialization, requesting64MiB arenas. Keep B's smaller runtime
stacks,2GiB virtual process cap,120s phase deadlines,one worker and default
logical proof limits. This is a separately frozen smaller allocation setting;
no mathematical hypothesis, target, proof trust, heartbeat, recursion, time or
process-memory limit changes. All A/B/C failures remain recorded as failures.
[pcc:unverified]

Pin the four actual Lean shared runtime-library hashes in addition to the
compiler binary, copied source and tool manifests. Installed stdlib/OS remain
ambient inputs, so do not claim a hermetic or signed execution proof. There is
no Rust compile or native enumeration in this caller. A D failure is retained
whole before any further revision; a D success gives a separately scoped formal
component and does not retroactively turn caller C into an overall pass.
[pcc:unverified]

Primary implementation references:
https://github.com/microsoft/mimalloc/blob/v2.2.7/src/options.c
https://github.com/microsoft/mimalloc/blob/v2.2.7/src/arena.c
https://github.com/leanprover/lean4/blob/v4.35.0/src/runtime/thread.cpp
