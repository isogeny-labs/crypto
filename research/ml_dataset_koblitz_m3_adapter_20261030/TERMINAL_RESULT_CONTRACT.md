# Original-bound pipeline result assembly

This is an integration design, not a completed verifier or a generation receipt.
Keep the frozen canonical result path and role. Use a strict versioned envelope
with original attempt binding, public/factor-base/input digests, engine, degree,
original resource limits, terminal phase, first adapter stop, optional native
construction failure, optional matrix report, and reported resource counters.
Never infer original limits or public input from this envelope.

Keep these counters distinct: cumulative native reservations; checking-work
reservations; checking work actually reported; adapter byte reservations; trace
bytes charged; actual group additions; native attempted and acknowledged events;
canonical capture acknowledgements; and global capture count including witness
commitments and other event kinds. Persist the final native snapshot even when
the callback or deadline fails. The result must not invent refunds or erase a
reported overshoot merely to fit the original budget. An overshoot must exclude
completed claims and retain the first stop and reported totals as censored data.

Represent a pending boundary separately from canonical acknowledged JSONL, with
its anticipated sequences, complete snapshot and optional raw partial encoding.
A pending capture sequence may be 4097 when admission refused the next event.
Do not send pending bytes through the canonical parser or count them as durable
acknowledgements. Capture sequence must agree with authenticated outer records;
a matrix Finish capture alone does not prove the native operation returned.

Verify in dependency order: original inputs and exact manifest identity; strict
original equations; canonical boundary stream and full capture custody; optional
matrix metadata and finite-assignment row consistency; signed arithmetic replay
and exact witness publication; terminal result phase/counters; supervisor outcome.
All observations remain provisional until every required downstream custody check
succeeds. A missing canonical result cannot become a completed cell from a valid
prefix. A separately verified witness can survive a censored outer outcome.

The assembly must retain errors at each stage: original admission, construction,
input persistence, matrix, signed enumeration, witness publication, terminal
result serialization, parent artifact flush, and supervisor finish. Complete
negative requires all registered phases plus exhaustive signed arithmetic and
no target tuple. A retained positive witness is separate from full execution.
Keep exact fall degrees and hardware instructions unknown without their required
certificates or instrumentation. No full matrix derivation claim follows from
finite-assignment checking.

Before runtime registration, test actual original-bound F4 and F5 pipelines,
trace/work/memory/deadline boundaries, interrupted publication, rehashed result
mutations, and supervisor censoring. Then run shared capture tests, current-head
lint/full checks, CLI fixtures, and S3 restore/replay. Preserve the full requested
family, twist, size, factor-base and solver coverage as outstanding scope.
