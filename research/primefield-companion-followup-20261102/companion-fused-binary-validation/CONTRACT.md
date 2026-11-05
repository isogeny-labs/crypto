# Binary-trace validation contract — predata, no numerical execution

2026-11-02. Exact finite component validation with a changed fingerprint
instrument. The original companion-fused-validation invocation is terminal
at exit124, with849candidate and28AArows, and will not be restarted.

Retain all849contexts perarm, candidate/list andAA/tuple:1698rows and34248
products. All369archived contexts and16164products perarm are retained,
including everyfull/inverse/principal stage. Retain all480carrycontexts and
960carryproducts perarm. Original plan.json bytes and expanded seeds stay
identical. No exclusions, selectedpilot, shortenedcohort or replacement of
missingAArows by candidate rows.

Everyproduct retains output/reference equality, complete independent count
and native event reconstruction, fourmutationcontrols and threeequalwrap
controls. Carryproducts retain every coefficient-list intermediate equality,
maxima, two uncertifiedradix rejections and the separate actualcarrycertificate.
Full totals remain136992mutations,102744equalwrap and3840radixguard rejections.
All849AApayloadpairs must match. The numerical check body and stage schedule
are unchanged according to source-preservation.json AST checks.

The changed record is trace_fingerprint: schema companion-trace-fingerprint/v1,
64lowercasehex SHA256 and exact encodedbytes. The producer computes it from
canonical binary chunks of the full trace after all independent checks.
The analyzer rejects legacy trace_sha256 and validates schema/hash/bytebounds
for every product. Bytebounds follow the exact trace container structure and
packed integer capacities from the carryradix. This is metadata validation,
not hash-preimage reconstruction or a substitute for numerical replay.

The original rawrecords and JSONfingerprints remain immutable. Newfingerprints
are never presented as equal to oldJSONhashes. The wholepayload AAfingerprint
continues to use sortedJSON, with the same versioned tracefingerprint record
on both newarms. No kernel arithmetic code is changed.

A fresh invocation requires a complete frozen protocol.json, full source/input
locks, seed audit, committed source, explicit finite resource/deadline limits,
and a fresh runs directory. The draft resource proposal is one sequential
process, one CPU,8GiB addressspace,4GiB newoutput and3600seconds, zero retries
or restarts. No newcloudcapacity or paidexternaljob is requested. The bounded subprocess helper and data-writer controls are now implemented and
synthetically tested; the experiment entry point and execution admission are
not yet complete. No numerical invocation is authorized by these test results. Existing
host/storage costs must remain accounted under their current authorization;
unknown funding or admission does not permit externalcapacity creation.
The old3600second limit is not reset by this contract.

Only complete1698row output with exit0 may run the full analyzer/export. Failure,
OOM or cap retains allrawbytes and originalnative exitreceipts. No fallback to
partialacceptance. Counts remain descriptive in separatefield/native/bit/byte
units; elapsedtime andpeakmemory improvement are not claims. The changed
serialization's effect on runtime is unknown. The goal of this panel is full
component acceptance needed before the Schur/lift and recursiveinverse panels.

Completed software checks: three deterministicmetadata tests validate exact
wire-size envelope endpoints and rejection of badversion/hash/size/parameters.
They construct codec fixtures only, never sourcekernels. Codec tests live in
../companion-trace-codec. Protocol draft and seed checks are metadata work.

## Implemented resource controls, 2026-11-02

limits_exec.py sets one allowed CPU in process affinity, an inherited per-process
RLIMIT_AS, CPU-time and per-file size limits, and disables core dumps before
exec. Address-space limits are not an aggregate process-tree RAM guarantee.
bounded_process.py owns the subprocess group, enforces a wall deadline, and
writes at most the configured combined stdout/stderr log bytes. It kills only
its owned process group on deadline or log overflow and records native exit
plus the supervisor reason. No retry is built in.

The artifact budget is 4GiB minus32MiB. budget_io.py accounts existing data
files, reserves UTF-8 bytes before every routed write, opens outputs exclusively
and retains charges on write failure. Both producer and analyzer route all
artifact writes through it. Reserve16MiB for logs (producer8MiB, analyzer4MiB,
export4MiB) and16MiB for control receipts. The aggregate policy requires a single
writer and an experiment entry point that enforces those log/receipt allocations;
it is not a filesystem quota against arbitrary code. Existing outputs are never
overwritten. Audit histories and source inputs remain subject to the separate
address-space limit.

Six synthetic tests passed: child-observed affinity/resource limits, oversized
allocation failure, wall-deadline termination, log-limit termination, native
file-size refusal, and aggregate multi-file byte accounting/no-overwrite. They
use temporary directories and synthetic subprocesses, never research kernels.
check_preparation.py normalizes only declared guard/output-routing changes and
checks that stage enumeration, numeric verify body and main numeric schedule
retain identical ASTs. No original frozen source or receipt was modified.

Before experiment execution, publish the exact source, protocol, inputs or input
manifest and launch command to GitHub as required by /home/ec2-user/AGENTS.md,
verify that revision, complete source/input locks, freeze the protocol, and bind
the actual single-owner entry point to the declared allocations and deadline.
The old run remains terminal. No synthetic check is numerical acceptance.

## Single-owner entry point

launch.py admits only a frozen canonical protocol, all1698 seedcells, verified
local input/source locks and committed clean source. admission.py reads an
immutable40hex GitHub commit, compares its exactsource/inputmanifests and
frozenprotocol, and verifies everypublishedsource. Python scripts must be
direct GitHub files; noncode data may use nativeartifact manifests with matching
SHA256 andbytecount. The publication.json receipt supplies explicit remote paths;
missing or mismatching publication refuses execution. It is produced only after
the exact publication commit exists, and is not fabricated during preparation.

The owner itself is pinned to the same singleCPU and perprocess8GiBaddressspace
limit. Use python -B; children suppress bytecode output through the environment.
One3600seconddeadline begins before admission and is shared by producer,analyzer
andexportcheck. Logs are bounded at8/4/4MiB respectively; at most eight control
receipts plus terminal fit below the16MiBreserve, each capped at64KiB.
A pre-existing launch.json orruns directory refuses a second invocation.
Eachstage starts only after its predecessor exits0. There is no resume/retry
branch. Stage-start/exit andterminalJSON retain native exits and supervisor
reasons. --execute is required for numericalexecution; without it only admission
is checked. A failed admission starts no kernel and consumes no numericalattempt.

Four additional synthetic tests check exactpublicationacceptance, mismatched
source/protocol/manifests, immutablecommit/scope/directscriptrequirements,
and exclusive bounded ownerreceipts. Combined software suite:13tests pass.
execution-command.json is the exact planned host command. No test result is
an executionreceipt. Numerical integration acceptance remains pending.

The protocol validator requires a numeric planning estimate. We use twice the
complete historical JSON-instrument candidate row span:2*3054.706495=6109.41299
seconds, excluding pre-row setup, as an explicitly labeled reference. This is
not a measurement or completion forecast for the binary instrument. The
new3600second shared cap is unchanged; no pilot was run to fill this field.
