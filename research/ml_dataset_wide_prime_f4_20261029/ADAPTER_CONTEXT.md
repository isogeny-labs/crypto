# Wide-prime m=2 dataset adapter integration

Design-only, 2026-10-29. Read current wide-prime worktree, its native IMPLEMENTATION.md and the parent implementation contract. No source edits, builds, tests, draws or cloud operations. Native API remains under separate review. This increment adds bounded wide-prime m=2 execution; higher m, other solvers, policies, cloud publication and the full requested corpus remain outstanding.

## Current source facts that constrain integration

`solvers::solve(config, case)` dispatches prime `f4` to `prime_f4(config, group, domain, target)`. Neither signature receives run memory, trace allowance or remaining worker time. `CaseConfig` supplies work and degree caps only. Both live wrappers install a thread-local EventSink then call this same solve function. CaptureLimits controls stream/payload sizes, not memory or the original process deadline.

Worker admission already reads and verifies run/attempt/capture binding before measured phases. The supervisor starts its Instant immediately before spawning and kills the process after `seconds_per_case`; this Instant is NOT transmitted to the worker. Run.json contains seconds_per_case/memory_mib/trace_mib, but seconds_per_case alone is not a remaining duration. Starting a fresh full worker duration at solver entry would extend the alleged inner deadline and must not be presented as the original deadline.

The existing m=2 input builder constructs exact u64 S3 coefficients and membership polynomials before checking the small-prime F4 guard. It currently calls `save`, which submits the live payload before appending it to SolverData. It therefore does NOT provide the canonical retention guarantee of the corrected chain path. Both wrappers, the final solver-domain append, and `set_witness` have similar generic fallible-live behavior. Fixing only the native observer does not fix retention.

Current central replay reconstructs chain inputs only for m>=3. A legacy m=2 prime-f4-input.json is hash checked but not reconstructed by that branch. New wide schema requires an explicit central verifier; do not rely on chain replay or public PDP witness checks alone.

## Actual native API and representation

Use `f4_tower_generic_bounded(&[GPoly<u64>], &GTowerRing<Fp64>, &TowerF4Options, &BoundedF4Limits, &mut observer)`. Limits fields: work, bytes, rows, columns, basis, pairs, steps, events. Report: native stats/basis, typed stop, work_reserved, bytes_peak_reserved, acknowledged_events, native_backend_invoked, native_statistics_complete.

Defaults are ceilings, not per-case inputs. Derive work=min(16777216, checked work_limit*1024) and bytes=min(64MiB, checked memory_mib*MiB/4); reject zero/overflow/missing context. Copy remaining ceilings from the admitted versioned profile. Degree=min(config.degree_bound,12). Disable staircase/full-rank exits and old callbacks. Native validates prime5..2^61-1 to admit differential tests; dataset routing remains 2^32<=p<=2^61-1, m=2. Small-prime dispatch remains unchanged.

Fp64 coefficients are Montgomery words: input uses PrimeField::from_u64; exported coefficients use PrimeField::to_u64. Never serialize raw coefs as public coefficients. Pack exactly two free variables using mono(0,[e0,e1,0,0]), combine duplicate terms modulo p, drop zeros and sort strictly descending native Mono. The documented generic order must match actual packed order; avoid borrowing an order name without checking it. Export independent canonical exponent/coefficient arrays regardless of native sort.

Native event contains boundary, sequence, work_reserved and bytes_peak_reserved only. It exposes begin/initial_echelon/completed-step/terminal boundaries, not matrices, rows or a complete derivation. Do not claim per-row retention. `bytes_peak_reserved` is cumulative NONREFUNDED conservative requested-storage reservations, not measured peak allocation or RSS. `work_reserved` is admission tariff, not completed operations/instructions. Native deadline is min(opts.deadline, native entry+5s), cooperative checks, with the existing outer process kill as hard envelope. Use native_statistics_complete and explicit stop, never infer completion from partial legacy stats.

## Minimal explicit context plumbing

Introduce a small SolverExecutionContext separate from CaseConfig (avoids changing frozen cell identity/exporter schemas): admitted memory bytes, trace bytes, original worker deadline as Instant, and original AttemptBinding. Construct it in worker before curve construction, from already verified run/config/attempt files plus a supervisor deadline handoff. Thread a reference explicitly through new solve_with_context and context-taking live wrappers into prime_f4; keep existing public solve/wrapper signatures as compatibility shims with None. None preserves canonical equations and yields missing-execution-context for wide native work; it must not silently invent runtime ceilings. Unit tests use an explicit fixture context, not hidden globals. No context in public features.

For the original deadline, add a narrowly scoped worker launch record written by the supervisor BEFORE spawn, with version, boot-id, CLOCK_MONOTONIC launch/deadline nanoseconds, seconds_per_case and attempt binding. Compute deadline once from that launch sample and the run duration; use that same deadline in supervisor stop checks. Pass/read the same record in worker; reject mismatched boot, overflow, duration/binding mismatch or elapsed deadline. Map remaining monotonic duration to local Instant once at admission. Do not serialize Rust Instant or substitute wall-clock timestamps. The record must have reserved path/role/encoding and manifest/capture binding coverage. This requires parent-owned mod.rs work, but not exporter edits or IAM. The record provides runtime provenance, not independent proof a malicious producer obeyed wall time. Historical standalone workers without the record continue old paths and cannot claim the new supervised wide profile.

Alternative interim implementation is permissible ONLY if explicitly named unsupervised fixture context with a five-second local deadline; it cannot claim the contract's original-case deadline or count as production admission. Prefer the real launch handoff now to avoid a second incompatible context.

At native entry use min(original remaining deadline, now+5s); extraction and serialization also check the original case deadline. Reserve canonical output headroom: total adapter encoded bytes <=min(4MiB, trace_bytes/4), with protected terminal metadata allowance inside that cap. Live payload and event caps may be tighter (capture payload total is trace/8); lower adapter permits to the actual admitted capture limits when bound. Account duplicate canonical/live bytes and outer trace envelope rather than assuming one artifact copy. If no room for mandatory input/result, fail profile admission before native work while retaining input through canonical writer when feasible.

## Module and call boundaries

Add `src/ml_dataset/prime_f4_wide.rs` for profile structs, bounded JSON writer, native conversion, equation reconstruction and independent replay. Keep Group/Element private if possible by placing orchestration/group extraction in solvers.rs and passing canonical public input plus exact domain points to module APIs. If moving helpers, expose only pub(super), not public library interfaces. No exporter T828 edits.

Refactor the existing m2 builder into a shared canonical public-input helper returning modulus, S3, membership, sorted domain x and target x. Verify public curve coefficients, curve/target/domain points and all x<p; do not filter conversion failures into a smaller domain. Route small p to old implementation; wide admitted p to new profile; unsupported dimensions/fields retain canonical input and explicit unsupported profile.

Stage canonical ArtifactPayload before any live call. Extend the existing chain retention predicate/helper into a named retention-capable strategy predicate covering chain and new wide strategy, without falsely treating every solver as safe. The helper must cover: first input flush; solver-domain append; native event observer; staged basis/extraction/result; witness staging; wrapper solver_return; bound emitter finish. After first live failure make no later live submissions. Preserve first error, acknowledged prefix and canonical_artifact_write_required; finish cleanup can be attempted for capture bookkeeping but must not erase the first error or certify completion. Do not propagate a late generic save error that discards SolverData. Canonical worker writer remains a separate durability attempt, and filesystem failure remains failure.

`set_witness` currently both stages through save and marks solver_completed=true. New wide path needs a stage-and-replay witness helper that can retain a verified mathematical witness while keeping solver_completed=false if persistence failed. Separate pdp_witness_verified from capture_complete/native_complete/extraction_complete. A verified PDP witness is not scalar recovery.

## Reserved artifacts and semantic replay

Use explicit versioned structs with deny_unknown_fields, decimal canonical coefficient strings and strict integer bounds. Suggested artifacts:

- prime-f4-input.json, complete-prime-field-polynomial-system: new schema/strategy, exact original public curve/target/domain/config identity, canonical three equations, profile id. Preserve same established basename without accepting legacy schema as new.
- prime-f4-wide-admission.json, restricted-wide-prime-f4-admission/v1: typed attempt binding, exact run limit inputs, effective native/extraction/serialization limits and deadline provenance reference.
- prime-f4-wide-events.jsonl, restricted-wide-prime-f4-boundary-events/v1: acknowledged sequence only, exact native boundary summaries, no complete trace claim.
- prime-f4-wide-result.json, restricted-wide-prime-f4-result/v1: explicit stop, completion booleans, counters with units, last returned basis canonically decoded, basis_derivation_verified=false, reservation semantics and trace gaps.
- prime-f4-root-extraction.json, restricted-wide-prime-f4-original-domain-replay/v1: deterministic checked pair prefix, original equation values, examined group choices, completion/cap flags, optional verified witness.

Require exact path/role/encoding bijection and unique mandatory records. Trigger new verifier from config applicability OR strategy/schema OR any reserved artifact/role OR native completion/invocation claims; an attacker must not bypass it by stripping the strategy. Do not require a completed native result for a supervisor-killed partial directory, but never accept success labels from such a partial prefix.

Central replay reconstructs original equations using independent BigUint arithmetic from public.json and factor-base.json, checks exact domain/order/config, derives static admission limits from run.json and profile, verifies attempt binding and launch-record identity. Validate basis shape, canonical coefficients, exponent limits and native bounds, but mark derivation unverified. Rehashed arbitrary well-formed basis cannot be disproved without a derivation certificate; do not falsely claim otherwise. Reject basis/candidate contradictions whenever they concern independently reconstructed roots, and reject a derivation_verified=true label.

Check explicit stop/native_statistics_complete/algebra_completed consistency, counters <=limits, acknowledged event count/contiguous sequence/monotone reservation counters, outcome summary equal to result summary, all required false epistemic flags. Complete/UnitDerived permit a producer-complete statement only; neither independently certifies negative PDP. Do not allow relabeling work_reserved as instruction count or exact fall degree. Use existing semantic witness verifier for pdp-witness.json after original-domain checks.

## Independent finite-domain extraction

Enumerate deterministic sorted ordered x pairs (<=1024), evaluate ORIGINAL canonical S3/membership polynomials with BigUint modular arithmetic, independent of F4 basis filtering. Budget min(config.work_limit,1024) pair evaluations separately from native reservations. For roots enumerate the signed actual factor-base point choices and replay sums with Group::add, charging every addition to a separate config.work_limit budget. Check deadline and sticky sink state before every next pair/lift. Record the exact visited prefix; never resample target/domain after an expensive/censored result.

Do not reuse legacy lift's exhausted heuristic as an exact completeness proof: it returns exhausted when used>=limit, even if the last allowed addition completed the last choice. Implement explicit cursor/completed flag for m2 enumeration, with boundary tests at exactly sufficient and one-less permits. Existing set_witness adds two replay additions; reserve/charge these separately or include them in the group budget before setting verified. No invisible extra replay counted as zero.

Positive group replay may finish early and certify existence; extraction_complete can remain false. Negative label requires all original x pairs and all corresponding point choices completed with no witness, persisted and independently replayed. Native UnitDerived alone does not suffice; a native inconsistency contradicted by an original root is a scientific/software inconsistency, not an excuse to skip the independent check. Persistence failure or budget/deadline stop gives partial/unknown completion while preserving independently checkable positive evidence.

## Required verification before claiming delivery

Fixed small/wide positive/negative equation controls; Montgomery wide coefficient conversion; both retained twist fixtures; unsupported profile input retention; no-context refusal; missing/zero/overflowed run limits; deadline elapsed before solver and deadline consumed during setup; exact/one-less extraction and lift caps; start/step/terminal sink refusal; payload cap before first native call; canonical witness retention after terminal failure; malformed basis encoding; rehashed equation/domain/admission/event/outcome tamper; role/path/encoding omission/relocation; public feature invariance. Preserve old small-prime/chain/root tests. Add source-bound CLI worker admission and S3 restored native replay later under the parent's fixed protocol; no scientific draws implied by this document.

Replay timing clarification: expired deadline/boot mismatch reject CURRENT worker admission, not historical artifact verification. Offline or S3-restored replay must validate the saved launch binding, duration arithmetic and structure without requiring the old deadline to be in the future or the current host to share the old boot id. It cannot retroactively prove runtime deadline compliance. Existing inspected worker-process.json records only PID/supervisor/process group/output and is written after spawn; extending that particular record cannot supply a before-spawn immutable deadline without changing its sequencing. Prefer the separate prelaunch record over racing a post-spawn file. Existing campaign/budget deadlines remain independently authoritative and must not be extended by this per-case handoff.

## Implementation freeze

Frozen before edits in isolated context worktree, 2026-10-29. Parent delegates
solvers.rs/mod.rs under T803/T805 and new execution.rs/prime_f4_wide.rs under T851.
Native source is independently owned and is not edited here. References checked:
existing DESIGN.md and IMPLEMENTATION.md, fp_backend.rs representation contract,
current worker/capture/central replay flow, and Linux clock_gettime(2)
https://man7.org/linux/man-pages/man2/clock_gettime.2.html . CLOCK_MONOTONIC is
same-host system-wide and excludes suspend; historical replay does not compare
saved times to the current clock. No clock precision or performance claim.
Shared session log is absent from this sparse worktree and reserved elsewhere;
this owned handoff records decisions and outcomes instead. No tests run yet.

## Implemented handoff (not built or tested)

Context commits: 5b331ba7c48 plus independent-review correction 8d88ce4d5da.
The correction rejects simultaneous checked-duration overflow/reversed deadline,
and censors exits first observed at/after the original deadline. Linux generation
writes the restricted prelaunch record and required-context run marker. Other
platforms preserve legacy generation, without claiming the supervised profile.

Adapter resides as a child module of solvers.rs in prime_f4_wide.rs, preserving
Group/Element privacy. Actual supervised wide m2 input enters Fp64 native F4;
legacy no-context calls retain their previous unsupported wide behavior. The
normalizer, independent original-domain replay, bounded native boundaries, decoded
basis, scalar native counters and available native step summaries are retained.
Native statistics remain producer reports and their timings remain diagnostic.
Bounded native failure does not prevent independent budgeted domain checking,
except persistence failure or the original deadline. No native unit is treated
as an independent negative PDP proof.

Trace allocation is conservatively min(4MiB, original trace bytes/32), requiring
more than256KiB, with separate eighths for boundaries, basis and extraction and
protected64KiB metadata. Native basis output may be explicitly omitted if it
cannot fit; basis_retained reports that fact. Canonical equations remain staged.
This is a narrower software resource profile, not a reduction of requested corpus
scope. Larger profiles remain future work. Checked native work/storage permits
retain their documented cumulative reservation semantics.

Central replay checks exact original equations, profile/attempt/run binding,
reserved roles/paths/encoding, acknowledged event prefix, canonical basis shape,
outcome counters, original-domain/group prefix and witness identity. It does not
prove arbitrary native basis derivations. Censored worker files without a final
registry remain unregistered partial bytes, not promoted records; registered
complete producer records are checked separately from outer timeout censorship.

Authored tests cover setup-consumed and historical deadlines, simultaneous
arithmetic overflow, exact/one-less group replay budget, expired native admission,
initial event/payload refusal retaining equations, both fixed61-bit twist fixtures,
central rehashed equation/limit/counter/outcome/deadline/unknown-field/path/encoding
tamper, and interrupted partial bytes. The fixed controls reuse existing seed42,
curve_seed7/generator1/key1 public doubling recipes; no seeds were searched.
Only scoped rustfmt and git diff --check were run. Parent-owned independent source
review, compilation, focused/full checks and source-bound CLI/S3 replay are still
required. No cloud jobs, IAM changes, exporter edits, builds or tests were run by
this implementation lane. All broader solver/arity/policy/corpus goals remain open.

## Independent adapter review corrections

The actual context artifact is execution-context.json with role
restricted-original-worker-execution-context/v1 and schema
crypto.ml-dataset.execution-context/v1. It contains the original AttemptBinding,
boot ID, monotonic start/deadline, seconds_per_case, memory_bytes and trace_bytes.
The wide result embeds this LaunchContext and effective Limits; there is no
separate prime-f4-wide-admission.json in the implementation. Native observer
records, basis and extraction each use separate fractions of the trace_bytes/32
adapter allowance; the verifier checks their serialized per-file and aggregate
bounds before parsing, including whitespace, and rejects blank event records.
The fallback without admitted limits permits only RESERVE-sized input/result.

Review identified and corrected combined artifact omission/stripped strategy,
unbound public/base descriptors and public arity, unverified result labels,
optional-statistics unit contradiction and serialized padding bypasses. Added
controls retain genuine pre-solver censorship and unregistered partial bytes.
The scalar field's legacy2^32 capability remains explicit, with a separate
supervised-wide m2 profile through2^61-1; higher arities are not advertised as wide.

Additional fixed controls: n47/n53 both twists using the same seed42,
curve_seed7/generator1/key1 recipe, and a complete no-lift domain control at61bits.
For that negative control the retained size2 signed base is asserted to be P,-P;
its possible two-summand sums are 0,+2P,-2P, while the fixed target3P differs in
the declared subgroup of order>5. No target/seed search or filtering is used.
All controls remain authored but unexecuted pending parent validation.
