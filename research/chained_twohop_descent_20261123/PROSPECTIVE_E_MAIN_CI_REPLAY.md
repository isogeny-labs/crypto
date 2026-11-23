# E: one combined main-CI replay with whole artifact custody

Local native C and Lean-only D are completed components; do not repeat them
locally. Preserve overall C134, A/B134 and the failed first metadata exporter.
The existing study workflow still selects C, whose later Lean startup is known
to fail under its frozen allocation settings. E replaces only this new study's
workflow selection and combines C's exact native-first phases with D's smaller
allocator reservation and runtime-library guards. No existing gate is changed.
[pcc:unverified]

The E shell caller refuses execution outside an ordinary main-branch GitHub
push job. It uses unchanged Rust/Lean mathematical bytes, populations,
controls, strict warnings, default logical proof limits, 2GiB virtual process
cap, 120s phase deadlines and one worker. Set LEAN_STACK_SIZE_KB=16384,
LEAN_MAIN_USE_THREAD=0 and MIMALLOC_ARENA_RESERVE=65536 before Lean starts.
Copy the immutable qualifier and check distinct BUILD/PRE/POST manifests.
Retain complete phase stdout/stderr/exits and the Lean startup trace. Pin the
same compiler and four Lean shared-library hashes; installed standard
libraries, OS and trace utility remain ambient, recorded inputs. The space
floors measure free tmpfs/scratch space, not reserved physical RAM.
[pcc:unverified]

Publish the complete E source lock before the first main replay. The job
checks each source against the actual public GITHUB_SHA before calls, then
uploads the entire packet through ordinary Actions artifacts with a requested
90-day retention. This is a prospective independent-host replay and whole-byte
custody route, not a claim that an artifact already exists or that a signed
independent arithmetic verifier accepted it. No S3 write, paid experiment,
workflow-dispatch, schedule, pull-request replay or local E run is authorized
by this addendum. First CI failure must remain intact before any repair.
[pcc:unverified]

Historical A/B/C/D source locks pin their historical workflow bytes. They are
not rewritten after E updates the physical workflow; their original immutable
Git commits and copied packets remain the replay references. E has a distinct
flat lock for its actual closure. A successful later E result cannot change
any earlier caller's exit or earned verification standing. [pcc:unverified]
