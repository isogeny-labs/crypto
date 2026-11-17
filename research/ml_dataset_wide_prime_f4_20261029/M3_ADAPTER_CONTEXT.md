# Wide-prime m3 adapter implementation plan

Design only, 2026-10-30. Current parent source inspected at
ab54110dffcdb411c8b9589503b8a2691d5afe4f in
/home/ec2-user/crypto-ml-wide-prime-f4-20261029. Read the two retained designs:
wide-prime-m3-next-design.md and wide-prime-m3-native-callsite-contract.md.
The latter is the correct prefixed filename. No repository edits, builds, tests,
draws, cloud operations or new authorization. This is an incremental adapter
plan; the complete m2..15/families/solvers/policies/intermediate-corpus objective
remains open. Native four-variable work is a separate prerequisite and owner.

## 1. Exact existing hooks and intended changes

| Existing source/helper | Reuse and necessary limitation |
|---|---|
| solvers.rs::prime_f4 | Insert supervised wide m3 dispatch BEFORE the current `m != 2` chain return. Require m==3, prime range2^32..2^61-1 and admitted SolverExecutionContext. Preserve small-prime chain and no-context behavior. Do not send m4+ to four-variable ring. |
| solvers.rs::prime_chain_polynomials | Reuse five original polynomials for m3; variable order x0,x1,x2,u0. Existing coefficient arithmetic is u128 modular, not the independent BigUint evaluator. |
| solvers.rs::prime_chain_input / PrimeChainInput | Reuse modulus, canonical polynomials, domains and original chain artifact. Domains are [Some(xs),Some(xs),Some(xs),None]; u0 stays all_Fp. Its n_vars is4. |
| solvers.rs::verify_prime_f4_chain_input | Reuse mathematical reconstruction first. Current helper compares expected keys but tolerates additional keys. New-profile strict replay should additionally compare the whole artifact to reconstructed canonical artifact, without silently tightening legacy acceptance. |
| solvers.rs::solve_with_context | Preserve existing group/public validation, field tally, config outcome fields and final solver-domain staging. New strategy must be recognized by retention predicate before final solver-domain write. |
| solvers.rs::retains_canonical_artifacts / record_prime_chain_persistence_error / flush_prime_chain_payloads | Extend explicit strategy predicates to m3; retain first persistence error, force solver_completed=false, never discard staged data. Current wide-only special case in record_prime_chain_persistence_error must include m3. |
| solvers.rs::wide_begin_failure and both context-taking event wrappers | Recognize admitted wide m3 before native entry, stage exact chain input and new result on begin failure. Existing m2-only condition must not remain a hole. No native call after stream refusal. |
| prime_f4_wide.rs::Limits::derive/native | Extract/reuse common native budget derivation, but keep separate typed m3 domain-unit fields: ordered signed triples, not m2 x-pairs. Same original memory/deadline/work ceilings. |
| prime_f4_wide.rs::encoded / push / read_bounded / artifact | Reuse bounded serialization, strict reserved role/path/encoding and bounded readers through a shared private support module or narrowly pub(super) child helpers. Do not duplicate the collector. |
| prime_f4_wide.rs::NativeStep / NativeStatistics / Event / stop_name | Reuse diagnostics and boundary semantics unchanged. Native reservations are not executed operations/RSS/instructions. |
| prime_f4_wide.rs::Term / Input / eval / Extraction / ResultRecord / finish | Do NOT widen these silently: m2 is an existing typed protocol. Introduce m3 four-exponent terms, chain input wrapper/reference, signed-triple extraction and result schema. Reuse implementation fragments with explicit profile constants. |
| prime_f4_wide.rs::verify_byte_admission / admitted_file_cap | Parameterize by an explicit closed profile descriptor listing paths and caps; preserve m2 byte-for-byte expectations. The m3 input remains the existing basename, new result/events/extraction basenames are distinct. |
| prime_f4_wide.rs::verify_censored_outcome / verify_unproved_claims | Share exact false/null claim guards and all retained-outcome checks, including bound solver-outcome/json record and current setup-error exception. |
| mod.rs::verify_cell and solvers::verify_prime_f4_wide | Add dedicated m3 verifier invocation independent of input-file presence/strategy. Existing chain replay still reconstructs the unchanged input. New verifier must understand the new strategy and avoid reading m3 result as m2. |
| execution.rs | Reuse unchanged original LaunchContext, admission and historical replay. Do not restart the original worker deadline at m3 entry. |

Use a new child module `src/ml_dataset/prime_f4_wide_m3.rs` under solvers.rs so
Group/Element remain private to the solver implementation. If shared support
requires a new file, reserve that path before editing. Exporter/prefix files remain
T828-owned; this plan grants no ownership there. Native file/API changes remain
with the native owner. Freeze the refactor boundary and source hashes before code.

## 2. Admission and canonical encoding

New strategy/profile: `prime-f4-fp64-affine-chain-m3-group-replay/v1`.
Original mathematical input remains `ml-dataset.prime-f4-chain-input/v1`, with its
existing strategy string and metadata. This is deliberately a distinction between
an input encoding and a new execution strategy, not a claim that the old narrow
visitor ran wide. Keep `native_modulus_limit_exclusive=4294967296` and
`affine_chain_exceptional_infinity_branches_covered=false` in that old encoding.
New execution metadata specifies native profile, actual backend and group replay
coverage. The input's negative_scope remains encoding-relative; an independent
finite signed-group negative certificate in the new result has another scope.

Require the original public/config family,n,m,twist equality, public curve/group
verification, factor-base verification, affine target and homogeneous on-curve
signed points. Build the ordered signed domain with existing `domain`, retaining
its exact order, no duplicates, repetitions allowed between summands. Require
1..32 distinct leaf x coordinates and four variables. Reject conversion failures,
never filter a point out to fit. Canonical input has exactly two S3 equations and
three membership polynomials. Check n_free4, zero tower rules, <=5 polynomials,
<=125 total terms, <=33 terms/polynomial, input total degree<=32 and every exponent
representable before packing. Native must independently enforce these counts.

Fp64 input conversion uses from_u64, output uses to_u64. Pack [e0,e1,e2,e3], not
[x0,x1,x2,0], and check mask/canonical stored degree on replay. Auxiliary u0 has
no finite membership polynomial and no factor-base-domain substitution. Use the
native admitted four-variable profile only after its lookup/insertion algorithms
and resource charges are independently reviewed; the bounded m2 gate alone is
not sufficient.

Common native limits stay work=min(config.work_limit*1024,16777216) with checked
multiplication, accounted bytes=min(original memory_bytes/4,64MiB), degree<=12,
original remaining deadline capped locally at5seconds, and existing rows/columns/
basis/pairs/steps/events limits. No raised ceilings without a frozen amendment.
Use adapter bytes=min(original trace_bytes/32,4MiB), protected metadata, per-file
and aggregate serialized caps before allocation/parsing. Account canonical plus
live duplicate bytes inside the existing outer envelope, and charge zero-budget
or cap refusal before work. Native profile refusal is retained, not counted as
successful four-variable execution.

## 3. Extraction algorithm: complete signed-group prefix

Freeze `extraction_policy=ordered-signed-triples-continue-after-first-witness/v1`.
This is intentionally different from m2's stop-after-first-positive extraction;
keep m2 untouched. Iterate domain indices lexicographically (i,j,k), k fastest,
with checked total D^3 and cursor ((i*D)+j)*D+k. Permit at most
min(config.work_limit,1024) triples. Native elimination and group extraction have
separate counters; native reservation exhaustion alone does not establish a
negative result or prohibit independent group checking within the original case
deadline. A live sink error stops both subsequent work and submissions.

For each triple, reserve its entire record plus two group-add permits BEFORE
performing P_i+P_j and then prefix+P_k. Do not reuse/collapse prefix calculations
in this first profile: two Group::add calls per visited triple make the counted
unit explicit even when the prefix is infinity. Report actual completed additions
separately from reservations. Check original deadline before each permitted call.
If deadline expires between calls, retain a partial triple with first-add state,
next-stage cursor and completed_adds=1; it must not increment completed-triples.
Likewise preserve a serialization/event failure after computed mathematics as
partial evidence without a false committed-prefix claim. A pre-admitted record
avoids doing unretainable work merely because the result is verbose.

If prefix is affine, save [x(P_i),x(P_j),x(P_k),x(prefix)] and independent BigUint
residues for all five ORIGINAL polynomials. Check each coefficient/exponent and
membership domain against reconstructed input. The first S3 equation must vanish
for the computed affine prefix. The second need not vanish for an arbitrary
nonmatching triple. Every matched affine total must satisfy all five equations;
a mismatch is a correctness failure to retain, not a reason to skip the triple.
Conversely all-zero equations do not imply this signed triple reaches target:
retain the actual group match separately.

If prefix is infinity, save a distinct enum branch `exceptional_infinity_prefix`
with prefix=null, total=P_k, original_equation_status=not_applicable and no u0 or
polynomial residue list. Still count and execute the second addition. A matching
total is a valid group-domain PDP witness. It is not an affine-chain root and
must not be used to contradict a native affine UnitDerived report. This branch
must not be discarded from finite-domain negative completeness.

On the first matching group triple, reserve a separate three-add witness replay
(start at group identity), verify all summands lie in the actual signed domain,
and save the existing independently-verified-pdp artifact. Recheck the five
original equations for an affine witness under separately recorded equation
replay counters; exceptional witness has explicit inapplicability instead. If
witness-replay budget/deadline/persistence fails, retain the candidate/mathematics
but do not mark it a completed verified certificate. Later triples do not replace
a successfully verified first certificate and do not consume extra certificate
replays. Continue the admitted ordered triple prefix after a first positive,
including subsequent affine and exceptional matches. The matching-row bit is
independent of whether that row owns the first certificate.

Why continue: with signed order [P,-P] and target P, (P,P,-P) is an affine
witness before (P,-P,P). Stop-at-first-positive would silently avoid the proposed
infinity control. Continuing all8 triples under the frozen small control exposes
and preserves the exceptional marker without changing seeds, domain order or
skipping earlier entries. No later cap erases the first verified certificate;
report its existence independently of extraction/prefix completion.

Full extraction means D^3 completed triples, both additions for each, all defined
original-equation replays completed, mandatory bytes retained and no unresolved
partial row. A negative label requires full extraction and no matching group
triple at all (not merely no saved certificate). A matching candidate with an
uncompleted certificate can never coexist with negative=true. With D<=64 and
1024-triple cap, most large domains are censored; only D<=10 can finish under this
initial triple ceiling. Do not narrow the requested corpus to those domains.

Counters: triples_started/completed, group_prefix_adds, group_total_adds,
witness_replay_adds (0..3), original_equation_evaluations, witness_equation_replays,
affine_prefixes, exceptional_prefixes, matching_triples, first_verified_cursor,
committed records/bytes. Count the latter separately from computed rows. No
hardware instruction or complete arithmetic-instruction claim follows from them.
Budget exhaustion, native deadline and original case deadline have distinct stops.

## 4. Output records and status semantics

Keep `prime-f4-input.json` / complete-prime-field-polynomial-system for exact old
chain input. Proposed new restricted artifacts:

* prime-f4-wide-m3-events.jsonl / restricted-wide-prime-f4-m3-boundary-events/v1:
  existing bounded native event schema/profile identity, strict contiguous lines.
* prime-f4-wide-m3-result.json / restricted-wide-prime-f4-m3-result/v1: original
  LaunchContext, effective limits, native explicit stop, invocation and statistic
  flags, decoded four-coordinate basis/retention flags, bounded native diagnostic
  steps, extraction summary, source scopes and unproved flags false/null.
* prime-f4-wide-m3-group-extraction.json / restricted-wide-prime-f4-m3-group-replay/v1:
  ordered signed-domain identity, policy, cursor/stage, completed rows and optional
  partial row, counters, first certificate cursor, affine/exceptional enum data.
* pdp-witness.json / independently-verified-pdp: existing group certificate,
  preserving the first verified triple; additional restricted result metadata
  links its exact cursor and branch without changing public exporter features.

Use deny_unknown_fields structs and a tagged branch enum so exceptional records
cannot contain affine coordinates/residues. Four-exponent terms have exact array
length4; no arbitrary vector silently truncated. Retain the actual domain order
and bind it to existing signed_factor_base/solver-domain artifact. Do not put
solver outcomes, timing, private scalars or mathematical labels into public features.

Separate booleans: native_statistics_complete, native_basis_derivation_verified
(alwaysfalse), group_prefix_complete, group_domain_complete, pdp_witness_verified,
exhaustive_no_decomposition, live_persistence_complete, first_certificate_retained.
The input's affine encoding coverage flag staysfalse even if group replay covered
infinity branches. Use a separate `group_infinity_prefixes_covered`/scope instead.
Native Complete/UnitDerived is a producer result. UnitDerived plus an independently
checked affine all-zero tuple is contradictory even without optional native stats;
UnitDerived plus an exceptional group witness is not a contradiction.

Mirror current status contract: worker `verified` iff positive PDP certificate
exists and no live persistence failure; otherwise `partial`. solver_completed is
positive existence or independently full negative, gated by required persistence.
A later work cap leaves a verified positive status/certificate intact while
prefix_complete=false. Outer supervisor censorship always removes all promoted
completion claims and keeps the producer outcome as separately bound evidence.
No first/last fall degree, full trace, retired-instruction, independent scalar or
ECDLP-pipeline-success label may become known/true in this increment.

## 5. Central verifier sequence and bypass controls

1. Trigger m3 verification from original supervised config/field applicability,
   new reserved paths/roles, strategy/schema OR any native/extraction/completion
   claim. Never depend on input presence; combined omission+stripped strategy
   must fail. Preserve only narrowly bound pre-solver error and genuine raw-partial
   exceptions, with the same no-promoted-claims guard as complete censorship.
2. Require exact public/base/context descriptors and public family,n,m,twist
   equality. Independently validate public/group/base without private witness.
3. Derive limits from run/context/profile, check every serialized per-file and
   aggregate allowance before bounded reads; include whitespace. Reject duplicate,
   missing, relocated, wrong-role/encoding descriptors and empty event lines.
4. Reconstruct old chain input through verify_prime_f4_chain_input plus exact new
   expected-artifact equality; verify unrestricted u0 metadata and all five
   original polynomials. Retained input profile metadata must not be mistaken
   for native execution capability.
5. Validate native stop/counters/boundary sequence, decoded four-coordinate basis
   structure, diagnostic bounds and false derivation flags. No basis derivation
   claim or negative group label may follow merely from [1].
6. Recompute deterministic signed triple cursor and every saved completed/partial
   stage. Independently recompute group prefix,total, affine u0/residues or
   exceptional inapplicability. Reject swapped triples, alternate domain order,
   duplicate/omitted prefix rows, fabricated infinity and false match bits.
7. Verify exact counters, all-full criterion, first certificate selection, and
   canonical witness summands/cursor/branch against group replay. Later matching
   rows are allowed by the frozen continuation policy; do not reuse m2's rule
   forbidding any row after the first match.
8. Reject UnitDerived contradiction only for independently checked affine roots.
   Ensure full-negative includes infinity prefixes and no matched candidate.
9. Compare result and effective worker outcome, status/persistence/completion and
   forbidden flags. On outer censorship, require bound solver-outcome/json before
   reading retained result.json, repeat all forbidden-claim/status checks there,
   and keep outer false completion claims independently enforced.

Current chain verification may also run in verify_cell's replay block, which is
acceptable reconstruction duplication initially, but must not require the old
narrow chain result/events or treat new group-negative labels as old encoding
nonexistence. Review that interaction explicitly. Independent consumer support
for m3 extraction is a separate gate; generic hashes or a group certificate alone
do not validate native four-variable algebra or prefix-completeness semantics.

## 6. Implementation order and fixed tests

A. Freeze profile/continuation policy/paths; review native four-variable readiness.
B. Extract only dimension-neutral bounded serializers/admission/claim helpers;
   preserve m2 schema, caps and all m2 regressions. No broad rewrite of capture.
C. Add m3 canonical builder conversion and early input staging; connect explicit
   context dispatch, initial-stream failure and final domain/terminal retention.
D. Add typed signed-triple state machine and independent verifier together.
E. Add central dependency/claim/byte checks; inspect exporter compatibility without
   editing T828 files. Add capability restricted to m3 supervised wide profile.
F. Parent owns builds, focused/full tests, immutable native source review and later
   fixed CLI replay; do not start scientific draws before these gates.

Fixed controls, no seed or target search:
- Existing seed42/curve7/generator1/key1 at n47/53/61, both twists, domain P,-P,
  target3P: require actual native invocation and valid group certificate.
- Same domain target4P with asserted point order>7, not assumed prime: all8 group
  triples cannot reach4P. Require group_domain_complete and negative=true.
- Same domain targetP: all8 triples retained under continuation policy; assert
  earlier affine match and later (P,-P,P) exceptional match, both replayed.
- Group budgets2*8 and one-less for full negative, plus explicit independent
  witness replay0..3 limits for positive; no negative after a matched but
  uncertified candidate. Test a cap after first verified positive preserves it.
- Inject original deadline before first add/between adds/before certificate,
  initial/step/terminal persistence refusal, and record cap before computing next
  triple. Retain partial stage and do not overstate committed prefix.
- Rehash auxiliary-domain metadata, coefficient/exponent3, branch enum, prefix
  nullability, residues, total/match, cursor/order, first-certificate reference,
  counters, byte-padding, status and forbidden proof flags; require rejection.
- Native controls from the separate callsite contract must exercise real useful
  four-variable reduction under unchanged limits, including nontrivial ties,
  interval predicate, coordinate3 overflow and exact/one-less admissions.

No tests were executed to select these controls. Failure of a frozen precondition
or useful-native-execution control is retained and investigated, never silently
replaced with a new seed. m4..15 still needs more than four free variables and
remains outstanding alongside scalable extraction, other solvers/policies and
verified scientific cloud corpus generation.
