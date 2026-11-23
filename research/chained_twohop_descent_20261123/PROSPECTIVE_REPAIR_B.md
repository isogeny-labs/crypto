# Prospective B runtime configuration after the preserved A failure

A5036b1f5 aborts134 at Lean --version, before any scientific compilation or
native arithmetic. The full negative is public at692c1eb4 with claim93a179.
This is a separately preregistered execution configuration; A is not reclassified
as a pass. The tagged Lean4.35.0 runtime source has a1GiB default thread-stack
reservation and supports LEAN_STACK_SIZE_KB / LEAN_MAIN_USE_THREAD.
[pcc:unverified]

Original PROTOCOL.md says not to use resource-limit overrides. This B addendum
explicitly replaces only the startup allocation configuration: set
LEAN_STACK_SIZE_KB=16384 and LEAN_MAIN_USE_THREAD=0 before launch, and pass-j1
also to the version query. This uses the OS main stack and a smaller16MiB
reservation for runtime threads. It is a tighter stack setting, not a larger
process budget. The2GiB virtual-memory cap,120s phase deadlines, one worker,
space floors, source population, controls and default logical proof limits
remain unchanged. No trust, maxHeartbeats or maxRecDepth option changes.
Whether this fixes the observed startup problem remains untested.
[pcc:unverified]

All Rust and Lean mathematical bytes and original protocol/source-audit text
remain exact to A. Keep the A caller, helper, lock and original workflow bytes.
The new caller/helper/lock are versioned; the CI entry point selects B only.
Publish/read back the full B closure before its first call, retain that first
outcome whole, and do not adapt populations, controls or limits to its result.
Canonical schema, signed verdict, general geometry and cost obligations remain
as before. [pcc:unverified]
