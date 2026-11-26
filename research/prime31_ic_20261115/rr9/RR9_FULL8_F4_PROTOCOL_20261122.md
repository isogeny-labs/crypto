# Prime31 B8 full-eight RR9 F4 extraction screen — frozen protocol

**Frozen 2026-11-22 before implementation or measurement. [pcc:unverified].** This is a bounded mechanism and cost screen on the ordinary cofactor-one curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`. It does not substitute for natural B31 relation yield, rank, or a complete fresh single-target comparison.

## 2026-11-25 pre-run evidence-contract addendum

This pre-data erratum/addendum was written after source review but before any
registered full-eight data were produced under this evidence contract.  The
2026-11-22 text remains the frozen historical protocol; this addendum does not
rewrite an observation or change the three public residuals, their order, any
original solver cap, or the decision rule below.  It freezes additional
evidence-handling limits before data and makes the already required
independent replay and custody evidence machine-checkable.  If this addendum
and the original text differ, the stricter evidence requirement applies.  An
earlier development invocation or an invocation made while these fields are
absent is a smoke test only and cannot satisfy this protocol.

The registered producer output schema is
`prime31.rr9.full8-f4-screen.v2`; its canonical input object uses
`prime31.rr9.full8-input-system.v1`.  The independent verifier output schema
is `prime31.rr9.full8-independent-replay.v1`, and the outer receipt schema is
`prime31.rr9.full8-f4-suite-receipt.v1`.  These are distinct contracts: the
wrapper does not infer scientific success from producer fields, and a
verifier does not accept the old producer v1 schema.  Before launch, the exact
source and executable artifacts are frozen.  At minimum the receipt and its
bound artifacts bind:

- the protocol, producer, independent-verifier, and live launcher source
  bytes and their SHA-256 values; the informational source commit and
  dirty-state receipt; the Rust/toolchain lock; every admission-bearing
  scientific child executable's SHA-256; the bootstrap interpreter receipt
  defined below; and the image digest and relevant environment.  A registered
  attempt stages and rechecks all non-bootstrap admission-bearing bytes in its
  fresh custody directory; a path, commit, or dirty-state description is not
  a substitute for immutable byte custody;
- the curve parameters, ordered B8 abscissas, ordered residual list, and, for
  each case, the compact canonical JSON `input_system` plus
  `input_system_sha256`.  That object contains its schema, `p`, `case`, public
  residual, `base_x`, `h8_coefficients`, and all eight ordered sparse
  `input_quadrics`, whose terms each contain seven exponents and one canonical
  field coefficient;
- the fixed case order `[0, 1, 2]` (positive, first negative, second
  negative), a unique launch-attempt identifier, monotonic start/stop times,
  UTC start/stop times, elapsed nanoseconds, exact argv, exit status, and an
  outer-wrapper attestation that the complete three-case suite stayed within
  the frozen 210-second producer-suite outer cap, with the start and endpoint
  defined below; and
- the raw producer stdout, stderr, and `/usr/bin/time -v` bytes, paths, byte
  counts, and SHA-256 values; the raw verifier input/output bytes and SHA-256
  values; and per-case user/system CPU time, wall time, and peak RSS.  The F4
  counters named below remain separate from process-resource observations.

The wrapper runs each case once in the frozen order in one launch attempt.  A
timeout, OOM, solver exit, oversized matrix, incomplete basis, missing
certificate, or failed replay is a scientific outcome and is not replaced by
a retry.  Any separately authorized rerun receives a new launch identifier
and receipt and does not overwrite or stand in for this attempt.  The wrapper
must create new outputs, refuse an already populated output directory, and
seal all bytes before interpreting the three outcomes.  Its receipt reports
observed schemas and cases but has no producer-derived scientific `pass`
field.

Every case row contains an exact `process_started` boolean and
`process_pid`.  A normal attempted row has `process_started=true` and its
integer child PID.  If `Popen` itself fails, the current row instead has
`process_started=false`, `process_pid=null`,
`launch_failed_before_process_start=true`, `launch_failure` exactly
`{type,message}`, `launch_attempted_utc`, and
`launch_attempted_monotonic_ns`; it preserves the argv and bounded raw
captures, while process/evidence start and finish timestamps are null and
`endpoint_fsync=null` with
`endpoint_fsync_errors=["launch_failed_before_process_start"]`.  Only later
cases that were never launch-attempted because of that exception use
`not_started_launcher_exception=true` placeholders.

An exception after successful `Popen` must not be rewritten as an unstarted
placeholder.  Its row has `process_started=true`, the actual `process_pid`,
`started_but_endpoint_failed=true`, and `endpoint_failure` exactly
`{type,message}`.  It preserves argv, UTC and monotonic start, known process
and evidence finish times, exit/timeout state, group-cleanup and stream-drain
state, bounded raw bindings, and every partial endpoint fact obtained before
the exception.  `endpoint_fsync` is null when no helper map was obtained or
the preserved partial normalized map otherwise, and
`endpoint_fsync_errors` states the failure.  Such a started case counts in
`all_cases_attempted`, but its incomplete evidence endpoint makes
`complete_suite_finished_by_deadline=false` and the suite non-admissible.
Unlaunched rows do not count as attempts.  No launch or endpoint exception is
a mathematical negative.

### Custody, parsing, resource, and clock admission

The custody manifest is sealed before launch and names the staged protocol,
producer source and executable, verifier source and executable, native
`Cargo.toml` and `Cargo.lock`, `/usr/bin/time`, and container image.  Every
staged file binding includes path, byte count, and SHA-256.  Scientific child
inputs use staged bindings, not mutable working-tree aliases.  Regular-file
mode bits alone are not executable custody because the same UID can change
them.  A pre/post mismatch, missing binding, symlink/path substitution,
unrecorded scientific child executable, or missing image digest is a custody
failure and cannot be admitted.

After regular-file staging and hash comparison, the wrapper copies the exact
producer and GNU-time executable bytes into separate Linux `memfd` objects,
sets executable mode, and applies all of `F_SEAL_WRITE`, `F_SEAL_GROW`,
`F_SEAL_SHRINK`, and `F_SEAL_SEAL`.  It checks the final seal mask with
`F_GET_SEALS` and receipt-binds, for each memfd, its source binding, device,
inode, byte size, SHA-256, mode, and seal mask.  The wrapper holds both file
descriptors open through the complete producer suite, executes GNU time from
its sealed-FD path, and supplies the producer only through the wrapper-owned
sealed-FD path; the mutable staged executable paths are never executed.
Missing Linux memfd/sealing support, a failed/incomplete seal, a byte/hash
mismatch, premature descriptor closure, or seal-mask drift fails preflight or
launcher custody.  The same-UID staged-path threat is therefore not deferred
to a post-run rehash.

There is one narrow bootstrap exception: the already-running Python
interpreter identified by `sys.executable` and the already-loaded live
launcher source cannot be created by their own custody stage.  The manifest
labels each exactly `bootstrap_trusted_not_staged`, never claims it was staged,
and binds its resolved path, device, inode, byte size, and SHA-256.  At
preflight, post-cap custody recheck, and precommit-seal construction, each bootstrap file is
opened once with `O_NOFOLLOW | O_NONBLOCK` and its same file descriptor is
`fstat`ed and required to be a regular file before it is read/hashed and
`fstat`ed again; device/inode/size/SHA must remain identical across all three
receipts.  Any mismatch makes the local producer suite non-admissible.  The
container-image and external EKS pod/job receipts must corroborate those exact
bootstrap bindings before registered/comparison admission.

Preflight `git` calls and their commit/tracked/dirty output are informational
provenance only and never participate in hashes, scientific decisions, or
admission.  The `git` runtime is therefore not claimed as staged or trusted;
missing/failed `git` metadata is recorded as unavailable rather than changing
an outcome.  Making it admission-bearing would require a prospectively frozen
bound runtime and schema.  Git provenance does not override any staged or
bootstrap byte receipt.

This registered accounting lane is single-threaded.  Every producer and
verifier child receives only the frozen minimal environment map
`LC_ALL=C`, `LANG=C`, `TZ=UTC`, `RAYON_NUM_THREADS=1`, and
`OMP_NUM_THREADS=1`; the complete map is receipt-bound.  The wrapper constructs
it from scratch and does not inherit `PATH`, `LD_PRELOAD`, `LD_LIBRARY_PATH`,
or any other parent variable.  A multi-core experiment is a separate
prospectively frozen lane that must bind thread count, CPU affinity, host
topology, and its own same-host controls; it cannot be pooled with this
receipt.

The frozen 210-second boundary is precisely the **producer-suite** boundary,
not the later comparison-admission boundary.  Its monotonic clock starts
immediately after a complete preflight: fresh-output-directory creation,
binding validation, repository/environment capture, immutable input staging,
custody-manifest write and `fsync`, and containing-directory `fsync`.  The
clock begins before suite metadata or first-case preparation.  Preflight is
separately wall/CPU/RSS-accounted and is mandatory setup in the later complete
comparison; its exclusion from the producer cap does not make it free.  The
producer clock stops at the recorded `suite_finished_monotonic_ns`, only after
all three ordered producer commands have exited or been killed; every case's
process group has been drained; each complete raw stdout, stderr, and
`time -v` artifact has completed the bounded endpoint-durability operation
below; fresh snapshots have been sized and SHA-256 hashed; and the producer
interface and required resource metrics have been parsed and validated.  Those
operations therefore consume the same 210-second budget as the producer
calls.  Crossing the deadline during this post-process work sets the outer
attestation false even if the last producer process exited earlier.

The receipt records this complete wrapper-level interval in
`producer_suite_resource_accounting`, containing exactly
`outer_cap_included`, `not_additive_with_case_time_v`,
`started_monotonic_ns`, `finished_monotonic_ns`, `start`, `finish`, `delta`,
`includes_rusage_self`, and `includes_rusage_children`.  The two inclusion
flags and `outer_cap_included` are true, and
`not_additive_with_case_time_v=true`.  `start` is the same `RUSAGE_SELF` plus
`RUSAGE_CHILDREN` snapshot used as the preflight resource finish, taken
immediately after preflight and before `suite_start_ns`; the accounting
`started_monotonic_ns` equals the producer-suite start.  After all three case
endpoints, helper `fsync` operations, bounded snapshots, hashing, parsing, and
metric checks, the wrapper calls `resource_snapshot()` for `finish` and only
then records `suite_finished_monotonic_ns`; the accounting
`finished_monotonic_ns` equals that suite finish.  The finish-snapshot call
itself is therefore inside the 210-second boundary, not post-cap accounting.

The nested `self` and `children` objects in `start`, `finish`, and the checked
`delta` contain `user_seconds`, `system_seconds`, and
`maximum_resident_set_kib`.  The `self` CPU delta covers wrapper-side draining,
bounded reading, hashing, parsing, and endpoint bookkeeping; the `children`
CPU delta covers reaped GNU-time/producer processes and endpoint-`fsync`
helpers.  The delta's CPU values are the exact nonnegative finish-minus-start
values.  Its maximum-RSS values are the disclosed cumulative high-water
values at finish, not arithmetic subtractions or a sum across cases.  Per-case
GNU-time observations remain individual scientific-command metrics and are
not added to this wrapper interval, since that would double-count child work.
Missing, malformed, nonfinite, negative, or arithmetically inconsistent
wrapper-resource fields make the suite non-admissible without becoming
scientific evidence.

Before `Popen`, the wrapper initializes the case progress, stream, endpoint,
descriptor, and cleanup state needed to recover from every later fault.  A
successful `Popen` returns directly into an already active protected cleanup
boundary.  The first operation there records the child PID/process in the
minimal progress object and invokes the started-state registration; no pipe
initialization, drain-thread start, wait, or other fallible case setup may
precede that registration.  A registration/setup/body fault caught inside the
protected case region runs group/descriptor/drain cleanup and raises the typed
`StartedCaseFailure` carrying the same PID and accumulated progress.  A later
fault in cleanup or endpoint construction may propagate directly to the outer
handler, but the already registered current-progress object remains its
authoritative started-case state.  Both paths produce the same post-start
failure row defined above; neither may misclassify that child as a `Popen`
failure or unlaunched placeholder.

Each timed case runs in a dedicated process session/group, and after the
leader exits or is killed the wrapper terminates and drains that group before
closing, parsing, or hashing raw output.  Producer-suite launch additionally
requires a recognized containment declaration at preflight.  “Recognized”
means the exact literal `dedicated-job-cgroup` selected from the staged
wrapper's fixed allowlist, not a free-form caller string.  The receipt binds
that declaration, the raw `/proc/self/cgroup` bytes, PID-namespace inode, and
wrapper PID, session ID, and process-group ID, and requires stable local
values, successful group drain, and no local artifact drift.  A missing or
different declaration or missing/malformed local evidence fails preflight.
The declaration is explicitly an assertion, not by itself proof that a child
which calls `setsid` cannot escape.

At the outer deadline the wrapper sends `SIGKILL` to the producer process
group and observes the leader only with `Popen.poll()`/nonblocking
`waitpid(..., WNOHANG)`.  The row's `deadline_kill_observation` preserves
`kill_sent`, `leader_terminal_observed`, `leader_return_code`,
`observation_method="subprocess_poll_waitpid_wnohang"`,
`blocking_wait_used=false`, and `observed_monotonic_ns`.  Group reaping is
likewise WNOHANG-only; every cleanup poll sleep and drain-thread join is
clipped to the positive remaining outer budget, becoming a zero-time
observation after expiry.  No new blocking wait, join, or `fsync` begins once
the deadline is reached.  An endpoint helper is not launched once the budget
is exhausted; a previously launched helper is sent `SIGKILL` and thereafter
observed only with WNOHANG, never awaited past the cap.  An
unobserved/unreaped leader or helper, unverifiable group absence, or incomplete
drain makes `evidence_endpoint_complete=false` and the suite non-admissible.
The wrapper does not wait past the cap to alter that outcome: final
reap/termination belongs to the externally corroborated job boundary.

After group cleanup, each raw artifact is durably flushed by a dedicated
helper operating on the wrapper's already-open, validated regular-file
descriptor.  The helper receives only the remaining monotonic time before the
same 210-second deadline; no durability grace period is added.  For every
case, `endpoint_fsync` is a map keyed `stdout`, `stderr`, and `time_v`.  Each
entry contains exactly `attempted`, `helper_pid`, `cap_bytes`,
`started_monotonic_ns`, `finished_monotonic_ns`, `deadline_monotonic_ns`,
`timed_out`, `kill_sent`, `reaped`, `wait_status`, `exit_code`,
`termination_reason`, `success`, `error`, `pre_fsync_binding`, and
`post_fsync_snapshot`.  Every attempted entry has this normalized key set:
`error` is null or exactly an error type/message object, and either binding or
snapshot is null when it could not be obtained.  A case placeholder for a
producer that could not start instead has `endpoint_fsync=null` and an
explicit reason in `endpoint_fsync_errors`.  The capture/drain thread performs
no `fsync` itself, and every case preserves `endpoint_fsync_errors`.
`pre_fsync_binding` contains device, inode, bytes, mode, `mtime_ns`, and
`ctime_ns`; `post_fsync_snapshot` contains path, presence, bytes, SHA-256,
`hash_complete`, observed prefix bytes/SHA-256, `observed_bytes_kind`, device,
inode, and mode.

A timeout, nonzero result, or path in which the helper is only sent `SIGKILL`
and observed/reaped with nonblocking `waitpid(..., WNOHANG)` is not successful
durability and makes the producer endpoint incomplete.  All three entries
must have `success=true`, no endpoint-fsync error, and a stable bounded
post-fsync snapshot before that case reaches its evidence endpoint.  The row's
`evidence_endpoint_complete` is true only when the process group is absent,
all three drains are complete, and all three endpoint helpers and snapshots
satisfy those conditions.  Its top-level `stdout`, `stderr`, and `time_v`
artifact bindings equal the corresponding `post_fsync_snapshot` values.

After a successful helper result, the wrapper takes a fresh bounded snapshot
through that same still-open descriptor: `fstat`, read/hash no more than the
stream cap plus one, then `fstat` again.  Its device, inode, size, SHA-256, and
initial `pre_fsync_binding` and final `post_fsync_snapshot` are serialized;
the intervening and final `fstat` values are checked for stability rather than
claimed as separately serialized objects.  A descriptor change, partial
capture, overflow, mutation, missing snapshot, or helper ambiguity makes
`complete_suite_finished_by_deadline=false` and
`producer_suite_admissible=false`; none is scientific evidence.

The wrapper captures the containment tuple again immediately post-cap and at
precommit seal: raw `/proc/self/cgroup` bytes and SHA-256, PID-namespace link and
inode, and wrapper PID/session/process-group IDs.  Each must equal the
preflight receipt exactly.  A missing recheck or any mismatch fails local
launcher integrity and `producer_suite_admissible`, without rewriting the
temporal deadline fact.

Post-run registered/comparison admission therefore waits for a SHA-bound
external EKS pod/job receipt corroborating that the declared dedicated cgroup
and PID namespace prevent every descendant from surviving the job or mutating
artifacts after drain.  After external job termination, that receipt also
binds a fresh bounded rehash of every raw artifact, receipt, precommit seal or
failure marker, required completion artifact, and custody artifact.  The
raw/receipt/precommit/custody hashes must match their locally sealed bindings.
Because no local artifact can non-recursively contain the completion file's
own hash, the external receipt instead supplies that terminal hash, strictly
validates the completion's backward bindings and flags, and accounts for its
write, file `fsync`, and containing-directory `fsync`.
Until both termination and matching post-job rehash are linked through
`required_followup_links.external_eks_job_containment`, the full8-only receipt
keeps `comparison_admissible=false`.  Missing external corroboration is a missing
follow-up, not a retroactive launcher-integrity or deadline failure; it does
not rewrite the already sealed local `producer_suite_admissible` field.  By
contrast, an unproved local group drain, surviving in-domain descendant, or
local drift is a launcher/custody failure.

`complete_suite_finished_by_deadline` is only that immutable temporal fact:
all three ordered producer attempts and the endpoint work above finished by
the deadline.  `producer_suite_admissible` is a separate conjunction covering
the clock fact, expected exits, strict interfaces/metrics, hashes, and
launcher integrity.  A nonzero child, malformed metric/evidence, or custody
error can therefore leave the temporal fact true while making admission false;
it must not rewrite the historical timing observation.  The local
producer-suite attempt fails and its launcher exits nonzero unless both local
fields are true.  Even when both are true, comparison admission remains false
until the verifier, controls, and external containment follow-ups are bound
and accepted.

Accounting is continuous across the producer/post-cap boundary rather than a
set of independently sampled best-effort intervals.  The single open boundary
is initially seeded at the producer-suite start with the preflight-finish
resource snapshot.  Once the producer interval closes, its finish
time/resource snapshot is exactly the
`postcap_custody_recheck` start.  When that interval closes, its
finish time/resource snapshot becomes the sealing-attempt start, before final
suite-receipt construction or any seal-path work; the completion artifact's
`started_monotonic_ns` and `resource_start` must equal those post-cap finish
values byte-for-byte.  At every transition the wrapper retains one open
accounting boundary.  It may advance that boundary only when the preceding
interval has closed, and it must pass the active original boundary through
any error fallback or outer exception handler rather than taking a fresh
start snapshot.  Thus there is no unmeasured CPU/wall/RSS gap between the
producer endpoint, post-cap checks, receipt construction, a partial success-
seal attempt, and failure persistence.  If an exception prevents an interval
from closing, the failure chain uses that interval's still-open start rather
than pretending the next boundary was reached.

Suite-receipt serialization and the two-stage sealing work occur after that
exact producer-suite endpoint.  Any wrapper-private temporary used by a
replacement or failure path is durably handled and separately accounted
there; the successful new-file path is not claimed to use one.  Independent
verification and fresh D9/signed-control executions are likewise separately
wall/CPU/RSS-accounted and remain mandatory for admission.  These costs are
not silently charged to or excluded from a differently described 210-second
quantity.  Receipt sealing, verifier, or control failure is non-pass even
when the producer-suite outer attestation is true.  Changing this endpoint
requires a new protocol/schema; it may not be changed after observing a run.

The success path first writes `suite-seal.json`, schema
`prime31.rr9.full8-f4-suite-seal.v1`, as a **precommit**, then `fsync`s that
file and its containing directory.  This precommit must not call `fsync`
again on attacker-writable raw or custody inodes.  It takes only fresh bounded
single-open same-FD snapshots, compares them with the producer-endpoint raw
snapshots and preflight custody bindings, and binds the already successful
per-artifact endpoint/preflight durability receipts.  Each snapshot may
reopen its path under the strict `O_NOFOLLOW | O_NONBLOCK` rule below, but its
own `fstat`/read/hash/`fstat` sequence uses that one descriptor.  Any drift or
missing raw/custody artifact fails sealing and admission while preserving the
earlier temporal fact.  The precommit enumerates the exact artifacts, helper
receipts, identities, byte counts, and SHA-256 values it accepted.  Its fields
include `raw_artifact_endpoint_fsync_receipts` with exactly nine successful
receipts, `raw_artifacts_refsynced_postcap=false`, and
`raw_artifacts_final_rehash` with exactly nine bounded bindings.  It also
binds the preflight `custody_manifest_fsynced_preflight` receipt,
`custody_manifest_refsynced_postcap=false`, and
`custody_manifest_final_rehash`.  Neither a blanket durability boolean nor a
post-cap re-`fsync` can substitute.  It names
`required_completion_artifact="suite-seal-completion.json"` but neither hashes
that future artifact nor claims that the precommit or its directory has
finished syncing.  In particular, the precommit omits
`finished_monotonic_ns`, `elapsed_ns`, `resource_finish`, `resource_delta`,
`receipt_fsynced`, and `containing_directory_fsynced` rather than making a
self-referential finish or durability claim.

Only after the precommit file and directory `fsync` have returned does the
wrapper take the sealing resource-finish snapshot and finish time.  It then
writes and file-`fsync`s `suite-seal-completion.json` and `fsync`s the
containing directory.  That completion uses schema
`prime31.rr9.full8-f4-suite-seal-completion.v1` and contains exactly `schema`,
`launch_attempt_id`, `producer_cap_excluded`, `seal`, `receipt`,
`started_monotonic_ns`, `finished_monotonic_ns`, `elapsed_ns`,
`resource_start`, `resource_finish`, `resource_delta`,
`seal_and_directory_fsync_completed`,
`completion_receipt_emission_accounted_locally`, and
`external_whole_job_accounting_required`.  `seal` and `receipt` are full
bounded artifact bindings, including path, device, inode, bytes, and SHA-256.
The start/resource fields reuse the continuous sealing-attempt boundary above;
the finish is sampled only after receipt construction, rehashing, precommit
emission, and the precommit file/directory durability operation;
`producer_cap_excluded=true`, the finish is no earlier than the start,
`elapsed_ns` is their exact difference, and `resource_delta` is recomputed
from the two snapshots.  `seal_and_directory_fsync_completed=true`.  The
one-way completion explicitly sets
`completion_receipt_emission_accounted_locally=false` and
`external_whole_job_accounting_required=true`: its own terminal write and
durability cannot be included in the interval it serializes.  Completion
points to the precommit; the precommit never hashes completion.  All bounded
receipt/precommit rechecks and other potentially failing local validation
occur before completion publication.  A successful completion's same-FD
write, file `fsync`, and containing-directory `fsync` are the final fallible
local actions; the wrapper neither reopens nor mutates an artifact afterward.

The failure path is the same one-way construction.  Its precommit
`suite-seal-failure.json` names
`required_completion_artifact="suite-seal-failure-completion.json"` and does
not claim its own finish or directory durability.  Its
`producer_cap_excluded` value, and the failure completion's matching value,
are false exactly when the active boundary stage is the unfinished
`producer_suite`; they are true for a `postcap_custody_recheck` or
`receipt_sealing_attempt` boundary.  After that failure marker and directory
are synced, the wrapper snapshots finish/resources and emits
`suite-seal-failure-completion.json` with schema
`prime31.rr9.full8-f4-suite-seal-failure-completion.v1`.  It contains exactly
`schema`, `launch_attempt_id`, `producer_cap_excluded`, `failure_marker`,
`receipt`, `started_monotonic_ns`, `finished_monotonic_ns`, `elapsed_ns`,
`resource_start`, `resource_finish`, `resource_delta`,
`failure_marker_and_directory_fsync_completed`,
`completion_receipt_emission_accounted_locally`, and
`external_whole_job_accounting_required`.  The two artifact fields are full
bounded bindings.  This completion sets
`failure_marker_and_directory_fsync_completed=true`,
`completion_receipt_emission_accounted_locally=false`, and
`external_whole_job_accounting_required=true`; its monotonic ordering,
elapsed difference, and snapshot delta obey the same checks as the success
completion.  Its start/resource fields are the active original open boundary,
never a new snapshot taken after the error.  A failure after partial success-
seal work therefore covers final receipt construction, all partial rehash or
seal work, failure-receipt replacement, failure-marker durability, and every
fallback step through the finish snapshot.  An outer exception retry must
reuse that same earliest still-open boundary.  It likewise points only
backward and excludes its own emission from local accounting.  Admission
requires the appropriate completion file, matching launch identifier, hashes,
timings, resource delta, and flags; missing, malformed, cross-linked, or
tampered precommit/completion evidence fails closed.  Registered/comparison
admission additionally requires the external EKS post-job rehash and
whole-job accounting above to confirm the terminal completion emission after
descendant termination.

Exactly one terminal chain is admissible: receipt + success precommit +
success completion, or receipt + failure marker + failure completion.  A
partial completion publication may enter the failure path, but the external
receipt must bind the launcher exit and reject any surviving dual-chain,
post-completion mutation, or additional local artifact emission.  Successful
failure-completion publication is governed by the same pre-publication-check
and final-fallible-action rule as successful completion publication.

Every JSON artifact used for admission is exactly one UTF-8 JSON object.
Parsers reject duplicate keys at any nesting depth, `NaN`, positive or
negative infinity, non-JSON numeric spellings, invalid UTF-8, and trailing
non-whitespace bytes.  Schema, integer-range, canonical coefficient, and
declared-count mismatches fail closed; last-key-wins parsing is forbidden.
Every value in a schema-designated integer position must be encoded as a JSON
integer token.  A boolean, decimal, or exponent-form number is rejected even
when it denotes a mathematically integral value such as `1.0` or `1e0`.
For every case the resource receipt requires parseable, finite, nonnegative
user CPU, system CPU, elapsed wall time, and peak RSS, plus a parseable
`time -v` exit status consistent with the recorded process result.  Missing,
malformed, nonfinite, inconsistent, or locale-altered metrics are preserved
raw but make the case and suite non-admissible.

Raw capture is also bounded while each child is running.  The inclusive
per-case limits are:

- producer stdout: 134,217,728 bytes, including the terminating newline;
- producer stderr: 16,777,216 bytes; and
- GNU `time -v` output: 1,048,576 bytes.

Stdout and stderr use concurrently drained bounded pipes so neither a large
write nor a full pipe can deadlock the other stream.  The `time -v` stream is
bounded either by its own drain or by an independently applicable
`RLIMIT_FSIZE`; a single shared file-size limit is not used to approximate
three different caps.  The first byte above a cap triggers termination of
the complete per-case containment domain.  The wrapper retains at most the
bounded prefix and records, for each stream, the cap, exact observed byte
count when known or an explicit `cap+1` lower bound, overflow/termination
reason, whether a prefix rather than a complete artifact was retained, and
that prefix's byte count and SHA-256.  A retained overflow prefix is never
parsed as producer JSON or resource evidence.  More generally, a missing raw
artifact or any capture marked overflowed, partial, truncated, or
hash-incomplete is never parsed and makes the local suite non-admissible.

A raw-stream cap trip sets `producer_suite_admissible=false`, makes the
launcher exit nonzero, and is a launcher/resource-invalid censored attempt.
It may leave the pure temporal deadline fact true, but it is never a
mathematical negative or evidence against this lead.

Every endpoint, verifier-input, and precommit-seal snapshot opens each
attacker-replaceable path once with `O_NOFOLLOW | O_NONBLOCK`, checks by
`fstat` that it is the expected regular file and inode before reading,
performs a bounded read of at most cap plus one, and uses that same byte
snapshot for size, SHA-256, strict parsing, and metric extraction.  This
nonblocking open ensures a substituted FIFO/device is rejected rather than
blocking before `fstat`.  A second `fstat` must show no mutation during the
read.  Size, hash, and parse must not come from separate path reopens; any
symlink, non-regular type, inode, size, or mutation mismatch fails
custody/admission.

The following inclusive evidence-handling caps are frozen before data:

- each complete raw producer JSON artifact presented to the verifier is at
  most 128 MiB (`134217728` bytes), including pretty-print whitespace and its
  terminating newline; compact-size estimates do not satisfy this cap, and
  the verifier enforces the raw byte count before JSON decoding;
- `basis_evidence.canonical_polynomials` contains at most 100,000 polynomials
  and at most 1,000,000 serialized terms in total, with its declared counts
  equal to the observed counts;
- the exactly eight unit-identity multipliers contain at most 1,000,000
  serialized terms in total;
- during independent expansion of `sum_i c_i q_i`, the canonical sparse
  accumulator contains at most 2,000,000 live nonzero terms after every
  modular combine;
- while reconstructing original-generator cofactors, the internal exporter
  carries at most 3,000,000 aggregate live provenance terms across all eight
  generator components; and
- the exporter's separate deterministic recomposition-work counter performs
  at most 2,000,000 monomial-product attempts.

The last two limits are internal exporter safety caps.  They are distinct
from both the 1,000,000 terms allowed in the final serialized certificate and
the 2,000,000 live terms allowed in the canonical recomposition accumulator.
Reaching either internal cap is a resource refusal, not a statement about
whether a unit identity exists.

The recomposition order is frozen so that the transient peak is replayable:
visit original generator/multiplier pairs in index order `0,...,7`; within
each pair visit multiplier monomials in strict ascending lexicographic order
of the seven-exponent vector, then quadric monomials in that same ascending
order; reduce and insert/remove the resulting coefficient modulo `p` after
each product.  The live-term peak is sampled after every such combine.  The
producer and verifier must report the same exact peak, not merely two values
below the cap.

The producer and verifier enforce the same limits before allocating or
expanding unbounded attacker-controlled structures.  Evidence is never
truncated, sampled, or silently dropped to fit.  Exceeding or refusing any
limit, failing to serialize within 128 MiB, or lacking enough resources to
complete a bounded parse/recomposition is a precisely reported
resource-censored non-pass.  It is neither a certified negative nor evidence
against the mathematical lead, and it cannot set an admissibility flag.

`complete_basis` retains only its mathematical solver meaning: no timeout or
oversize, no genuine pair above the frozen bound, and no staircase stop.  An
evidence cap must not rewrite a mathematically completed basis as incomplete.
On a cap refusal the producer instead sets both `status` and `outcome` to
`resource_censored`, sets `admissible_positive=false` and
`admissible_negative=false`, and emits `resource_refusal` with schema
`prime31.rr9.full8-resource-refusal.v1`.  Its exact fields are `reason`,
`basis_polynomials_observed`, `basis_terms_observed`,
`certificate_terms_observed`, `certificate_product_accumulator_peak`,
`internal_provenance_terms_observed_lower_bound`,
`internal_provenance_terms_observation_kind`,
`internal_recomposition_product_work_observed_lower_bound`,
`internal_recomposition_product_work_observation_kind`,
`attempted_final_pretty_json_bytes`, and `suppressed_evidence`.  The allowed
reasons are:

- `basis_polynomial_limit_exceeded`;
- `basis_term_limit_exceeded`;
- `certificate_term_limit_exceeded`;
- `certificate_product_accumulator_limit_exceeded`;
- `internal_provenance_term_limit_exceeded`;
- `internal_recomposition_product_work_limit_exceeded`; and
- `final_json_limit_exceeded`.

If multiple limits are observable, `reason` is the first applicable item in
that listed priority order while every observed count/lower bound is still
preserved.

The counts are the actual observations at refusal, not a cap-clamped value.
In particular, an accumulator refusal preserves the first violating peak
(`2000001`, i.e. cap plus one) and a certificate-term refusal preserves the
decoded term count at its refusal point.  An internal exporter refusal
preserves the exact attempted count when available; otherwise its explicitly
named lower-bound field is `cap+1` (`3000001` provenance terms or `2000001`
product attempts), never zero or a falsely exact count.

Each internal numeric field is paired with its respective
`internal_provenance_terms_observation_kind` or
`internal_recomposition_product_work_observation_kind`.  The only values are
`exact`, `lower_bound`, and `not_observed`: `exact` preserves the engine's
actual attempted count, including any value above the cap; `lower_bound` is
permitted only when arithmetic overflow or unavailable cancellation-aware
accounting prevents an exact count and then the numeric value is exactly cap
plus one; `not_observed` requires numeric zero, and numeric zero must use
`not_observed`.  The kind and numeric value
must agree exactly between `evidence_limits` and `resource_refusal` whenever
the refusal object is present.
Producer and verifier mutation tests must independently reject every invalid
kind, kind/value mismatch, or disagreement between the two objects before a
registered run.

For `final_json_limit_exceeded`,
`attempted_final_pretty_json_bytes` is the exact byte count that the final
pretty serde JSON plus terminating newline would have occupied; it is null
when no such full-size serialization attempt exists.  `suppressed_evidence`
is exactly `true` when the bounded refusal artifact deliberately omits the
bulk basis/certificate fields.  A normal artifact has
`resource_refusal=null`.  Its `prime31.rr9.full8-evidence-limits.v1` object
separately preserves all cap constants, both internal-exporter bounds, and
observed counts.  It includes `max_internal_provenance_terms`,
`max_internal_recomposition_product_work`, both named observed-lower-bound
fields, both observation-kind fields, their corresponding `*_within_limit`
booleans,
`final_json_size_checked_before_write`, `final_json_within_limit`, and
`attempted_final_pretty_json_bytes`; the attempted-byte value must equal the
one in `resource_refusal` whenever that object is present.

The independent verifier returns its ordinary output schema with
`disposition="resource_censored"` and `pass=false` only after strictly
checking the case/public-input bindings, cap constants, solver-completion
fields, refusal reason, observed counts/peak/attempted bytes, and exactly
permitted suppression.  It does not demand the deliberately suppressed basis
or certificate in that path.  A missing refusal object, inconsistent reason
or count, unexpected suppression, bad binding, or malformed refusal remains
`invalid_producer_artifact`, never a valid resource-censored observation.

### Seven-variable exponent-encoding correction

Source review before any registered run found that the initially contemplated
`n_free=7` representation is impossible in the selected
`f4_fp_tower` engine: its packed monomial format supports at most four free
variables.  No full8 result has run under either representation.  The
candidate instead represents the seven ordinary coefficient variables by
seven independent four-level radix-2 Kummer exponent-carry blocks, for 28
tower variables and `n_free=0`.

For ordinary variable `x_j`, block `j` has variables
`y_(j,0),...,y_(j,3)`, weights `1,2,4,8`, and rules

```text
y_(j,0)^2 = y_(j,1)
y_(j,1)^2 = y_(j,2)
y_(j,2)^2 = y_(j,3)
y_(j,3)^2 = 0.
```

A squarefree block mask `(b_0,b_1,b_2,b_3)` decodes to the ordinary exponent
`e_j=b_0+2b_1+4b_2+8b_3`.  Binary expansion is unique for
`0 <= e_j < 16`, so this map is injective; the first three square rules are
exactly binary carries.  Consequently, before the top rule is reached,
normal-form addition, multiplication, coefficient recovery, and evaluation
decode coefficient-for-coefficient to the same polynomials in the seven
ordinary variables.  This is an internal exponent encoding, not a new
Kummer-point constraint or a relaxation of any original quadric.

The existing maximum F4 weighted degree remains 9.  Thus every monomial in
an admitted row has each decoded exponent at most 9, strictly below 16, and
the zero top rule must be unreachable.  Nevertheless, the engine's default
quotient-ring scheduler would retain artificial tower-pair instances induced
by the seven `y_(j,3)^2=0` representation-boundary rules as
`pairs_above_bound`.  Their first possible weighted degree is 16, so they are
not ordinary-polynomial critical pairs of weighted degree at most 9.  There
are exactly seven open-top rules/bits, but the scheduler may create more than
seven pair instances as intermediate basis leading monomials contain those
bits; the receipt must not assume the instance count is seven.

The registered path may therefore use an explicitly selected and validated
**open-zero-top mode**.  Its allowed mask contains exactly the seven top bits
and is enabled only after the engine checks for every block that the top rule
is square-zero, its top weight is 8, `2*8=16` is strictly above the frozen
degree cap 9, and no input term violates the degree/exponent embedding.  Every
omitted pair instance must name a bit in that mask and have weighted degree
strictly above 9.  The mode must not omit an ordinary F4 pair or any tower
pair of weighted degree at most 9.  Raising the cap to 16, processing a
top-zero relation, or treating the omitted boundary instances as genuine
unresolved work is forbidden: each would change either the frozen experiment
or the intended ordinary-ring embedding.  The receipt records the allowed
seven-bit mask and the actual omitted-pair-instance count;
`pairs_above_bound=0` continues to mean that no genuine pair remains under
this validated embedding.

This mode must be opt-in and leave the engine's default scheduling and
accounting unchanged.  Before a registered run, tests must show that (1) the
default mode retains its prior top-pair behavior and existing engine tests
pass, (2) open-zero-top refuses a nonzero or misweighted top, a top whose
double weight is within the cap, a layout other than the seven registered
blocks, or an unsafe encoded input, (3) default and open modes produce the
same admitted rows and basis through the frozen degree cap 9 apart from the
separately identified boundary-pair accounting, and (4) focused differential
tests of internal encoded multiplication through degree 9 agree
coefficient-for-coefficient with a seven-variable ordinary polynomial
reference.

The producer reports this as `radix_embedding` with schema
`prime31.rr9.full8-radix-embedding.v1`.  It binds the ordinary/tower/free
variable counts, levels and block layout, level and full tower weights, carry
rules, artificial top exponent, frozen degree cap, artifact round-trip and
degree-bound checks, decoded-basis bound, engine open-top metadata, and its
admissibility result.  Its open-top fields are
`requested_open_top_pair_omission_mask`,
`open_top_pair_omission_mask`, `open_top_rules_selected`,
`open_top_pairs_omitted`, `open_top_reductions`, and
`open_top_receipt_valid`.  Admission requires requested and applied masks to
equal the frozen seven-top-bit mask, seven selected rules, a reported
nonnegative actual pair-instance count, zero reductions, and a true receipt
validity flag.

The producer must machine-check the exact 28-variable block/rule/weight
layout; round-trip every input-system, serialized-basis, and
serialized-certificate monomial through the seven-exponent decoder;
recompute their weighted degree as ordinary total degree; and assert that
every decoded exponent is below 16, the allowed mask contains exactly the
seven registered top bits, engine metadata attests that omissions were
restricted to that mask and the cap inequality, the actual omitted-instance
count is reported, `open_top_reductions=0`, and no genuine pair remains.  The
engine's checked `2*top_weight > degree_cap` invariant and the bound focused
differential tests remain required no-overflow evidence; the runtime counter
does not replace them.  A missing invariant or artifact check, a degree above
9 in an admitted artifact, a decode mismatch, an omission outside the allowed
mask/cap, or any observed top-rule use makes the case inadmissible and is
preserved as a failed/censored artifact.  This encoding correction changes
none of the frozen time, degree, nonzero, dense, or memory caps, the fixed
case order, the replay requirements, or the decision rules.

For every case the independent verifier echoes this admission evidence in
`radix_open_top`, using schema
`prime31.rr9.full8-radix-embedding.v1` and the fields
`frozen_max_weighted_degree`, `requested_open_top_pair_omission_mask`,
`open_top_pair_omission_mask`, `open_top_rules_selected`,
`open_top_pairs_omitted`, `open_top_reductions`, and
`open_top_receipt_valid`.  A verifier pass requires exact agreement with the
producer artifact, cap 9, the requested/applied frozen mask, seven selected
rules, a nonnegative omitted-instance count, zero reductions, and a valid
receipt.

### Positive evidence contract

For case 0 the producer emits its canonical seven-coefficient vector as
`recovered_coefficients`.  Its `linear_extraction` object uses schema
`prime31.rr9.full8-linear-extraction.v1` and preserves `raw_rows`,
`rref_rows`, `pivot_columns`, `coefficient_rank`, `augmented_rank`,
`unique_solution`, `recovered_coefficients`, and any `blocker`.  Its
`basis_evidence` object uses schema
`prime31.rr9.full8-basis-evidence.v1` and preserves the complete canonical
ordered polynomial list, with every term encoded by seven exponents and a
canonical field coefficient, plus the relevant counts.  A byte-for-byte
external basis artifact is acceptable only when it is sealed and SHA-bound
by the producer output and suite receipt.  Monomials, coefficients, variable
order, and field encoding must be explicit; a summary boolean or a basis hash
without the underlying bytes is insufficient.

The independent verifier must not import or call the producer implementation.
Starting only from the public curve, residual, ordered B8 abscissas, producer
artifacts, and frozen encodings, it must:

1. reconstruct `H_8` and all eight original quadrics;
2. recompute the linear-system rank and recovered seven coefficients from the
   raw rows;
3. evaluate every original quadric and every serialized basis polynomial at
   that vector; and
4. reconstruct the eight signed curve points and check their sum equals the
   public residual.

The positive verifier also requires `complete_basis=true`,
`inconsistent=false`, `timed_out=false`, `oversize=false`,
`pairs_above_bound=0`, and `staircase_at_stop=null`.  Its single `pass` value
is the conjunction of those conditions, the four checks above, and all
hash/schema bindings.  The verifier output binds `producer_artifact_sha256`,
`verifier_source_sha256`, `case`, `public_residual`, `base_x`, and
`input_system_sha256`; it uses disposition `positive_verified` only for this
admitted positive.  The suite wrapper must reject a producer/verifier
field-name or encoding mismatch rather than translating it heuristically.
Producer `outcome`, `admissible_positive`, or similar convenience booleans
are observations, not independent admission.

### Negative evidence contract

Cases 1 and 2 count as certified only when the producer supplies, for the
**original eight public quadrics** `q_0,...,q_7`, explicit coefficient
polynomials `c_0,...,c_7` over `F_p` such that

```text
c_0 q_0 + ... + c_7 q_7 = 1.
```

The producer field is `negative_unit_identity`, and a non-null certificate
uses schema `prime31.rr9.full8-unit-identity.v1`.  It binds its case, public
residual, and `input_system_sha256` and contains exactly eight multiplier
polynomials in original-quadric order.  The certificate encoding fixes
variable order, monomial order, canonical field representatives, and every
term of every multiplier as a coefficient with seven exponents.  Every
multiplier has ordinary total degree at most 7, and every product of a
multiplier and its quadric has ordinary total degree at most the frozen cap
9.  The independent verifier parses these bounds, reconstructs the `q_i`
independently, performs the polynomial products and sum, and requires the
canonical result to be exactly the constant one.

A negative verifier pass additionally requires a complete, uncensored F4
result: `complete_basis=true`, `inconsistent=true`, `timed_out=false`,
`oversize=false`, `pairs_above_bound=0`, and `staircase_at_stop=null`.  A unit
basis, an inconsistent flag, a solver-trace hash, or a certificate for the
older D9 ideal is not a substitute.  If the current F4 implementation cannot
export the identity, `negative_unit_identity` is null and the verifier uses
disposition `certificate_unsupported` with `pass=false`; the producer may
also record the precise blocker, but it cannot set
`admissible_negative=true`.  Dispositions `negative_verified`,
`verification_failed`, and `invalid_producer_artifact` retain their literal,
fail-closed meanings; only `negative_verified` may accompany a negative
`pass=true`.

### Same-host controls and admission

The comparison-admission receipt, either directly or through SHA-bound
immutable child receipts, also binds fresh same-host executions of the
existing D9 positive extractor, both independently replayed D9 negative
controls, and the 128-chart signed-system direct control.  Each control
records its own source/binary hashes, public input, raw stdout/stderr,
CPU/wall/RSS values, and replay result.  Historical timings from another host
may be cited as context but are not pooled into this comparison.  A full8-only
launcher receipt records its control and verifier links as unpopulated and
sets `comparison_admissible=false`; it is precursor evidence, not a complete
comparison receipt.  Missing or failed controls prevent suite-level
comparative admission; they do not turn a full-eight case into a mathematical
negative.

For avoidance of doubt, **full8 is a squarefree, distinct-abscissa B8
precursor only**: it imposes `M=H_8`.  It is not a multiplicity-aware B4
result, a multiplicity-aware B8 result, or B31 evidence.  Even a three-case
pass does not establish the predicate `M | H_B^8`, complete blinded B4/B8
fibres, weighted W8 recovery, natural relation yield, a six-size scaling law,
or a B31 discrete-log solve.  Those regimes retain their own custody,
correctness, accounting, and replay gates and remain open unless separately
disproved under their own frozen tests.

Accordingly, the original instruction to “stop this B8 F4 branch” after a
capped or no-coefficient positive stops or revises only this frozen full8 F4
implementation attempt.  A timeout, resource refusal, incomplete artifact,
or absent independently checked coefficient vector does not disprove the
mathematical squarefree-B8 lead or any multiplicity-aware B4/B8/B31 regime;
those may be pursued under a prospectively frozen revised implementation and
protocol that preserves, rather than replaces, this attempt's receipt.

Complete-cost promotion remains blocked by the separate
[RR9 call-owned F4 and whole-collector accounting problem](../../../problems/rr9-f4-whole-collector-accounting.md).
The bounded full8 receipt cannot substitute for either ledger named there.

## Prior-work gate and hypothesis

The seven-of-eight B8 ideal is already known to split into 128 rational signed charts on these controls (`RR9_SEVEN_QUOTIENT_SPLIT_RESULT_20261117.md`). Evaluating its eighth equation by enumerating those charts is an exact generic-equivalent control. The new candidate runs the existing Rust sparse F4 engine directly on **all eight original coefficient quadrics**, formed from public residual `R` and the fixed B8 polynomial `H_B`, without entering signed leaves, a support mask, a coefficient witness, or a target scalar. The test asks whether a complete F4 basis reveals a unique positive chart or a unit negative at a cost below the 11,440-column D9 Krylov extractor. A positive result would be a lower-cost sign-independent *implementation path* at B8, not evidence of a non-generic ECDLP advantage.

## Frozen inputs and caps

- B8 abscissas: `(0,1,2,3,5,6,8,9)`.
- Public positive residual: `(1911579686,548820096)`.
- Public negatives: `(625037715,2100627251)` and `(1923055775,307731458)`.
- Polynomial: `M=(X²+rX+r²+a)V²-(X-r)A²-2sAV`, with `A=a0+a1X+a2X²+a3X³` and `V=X³+v2X²+v1X+v0`; input quadrics are all eight coefficients of `M-H_B` below degree eight.
- Run order: positive, first negative, second negative. One deterministic F4 call per case; no seed search. F4 maximum degree 9, cooperative wall budget 60 seconds per case, at most 3,000,000 symbolic nonzeros and 1,000,000 dense entries. Outer process cap 210 seconds. Do not automatically enlarge a cap after observing a result.
- Record source SHA, full wall and CPU time, per-case F4 wall, maximum degree, basis size, staircase, peak nonzeros/dense entries, multiply-adds, timeout/oversize/censored status, and peak RSS if available. Charge public equation construction, F4, extraction and checks. Preserve all three case outputs, including failed/censored cases.

## Soundness and decision rule

The positive counts only if F4 terminates with no pairs left, the completed basis contains seven independent affine linear equations yielding seven coefficients, and an independent implementation checks all eight original equations and the full signed point sum against public R. A basis with unresolved pairs or an unverified candidate is censored.

A negative counts as certified only with a replayable unit identity from the original eight quadrics. A completed F4 unit basis without an independently replayable identity is a solver observation, not a certificate. Existing D9 negative unit receipts may be cited as separate controls; they do not certify this new F4 trace by themselves.

Compare complete same-host wall and CPU with the existing D9 positive and 128 signed-system direct control, without pooling hosts or calling a component speed a complete solve. If the positive F4 call exceeds its cap or yields no independently checked coefficients, record the precise limiting phase and stop this B8 F4 branch. If it passes but is slower than the signed control, retain it as a correctness route and screen B31 all-multiplicity cost before implementation. A B31 proposal must handle `rad(M)|H_31` and exceptional/repeated-root strata, produce natural relation rank, and satisfy a frozen full single-target budget against matched optimized rho.
