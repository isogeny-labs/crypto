# Existing library fixture triage, 2026-10-28

The completed selected test invocation runs all20 cases that were observed
failing before the earlier full-library attempt was interrupted: **19 pass,
1 fails,0 ignored,5037 filtered out**, Cargo exit101. The remaining failure is
the unchanged extension-registry assertion:1324 actual versus2287 required.
This is a completed20-case diagnostic subset, **not** a completed or passing
5057-test library suite. Further full-suite failures/interactions are not ruled
out. The prior interrupted run and its raw evidence remain unchanged.

## What changed locally

Only sparse-checkout materialization of already-tracked immutable inputs.
The first14 archival/metadata cases reproduce missing-file failures. Five
existing device cases, explicitly set to their documented CPU-emulation mode,
reproduce the missing `gpu/certicom` source prerequisite. Materializing the
recorded fixtures and source resolves those19 observed failures without changing
any source, test, assertion, saved record, registry row or workflow.

`input-inventory.json` binds134 tracked files/36321132 bytes by Git blob and
SHA256. This inventory covers the declared additional fixture/source paths;
it does not claim to enumerate every build/runtime input. All bytes match the
pinned input head `dae0eaf37aabda6c42455d132122be23bf63cec6`.
The nine unit-source files are byte-identical to the reported PR base
`0c9fee8d6c19577e0bbb48c3cec01069168cedf1`, as recorded in
`source-identity.json`. The diagnosis is local checkout omissions, not an
assertion that unrelated product defects were repaired or base CI was green.

The device cases use the existing C++ worker in **CPU emulation** through the
existing Rust unit tests. No CUDA/Metal hardware, EKS job, paid resource,
benchmark campaign or new challenge result is involved. Their timings in raw
logs are diagnostic only. No standalone study producer/auditor/corruption CLI,
new research workload or Lean elaboration is launched.

## Preserved attempts and exact scope

| Attempt | Selected cases | Outcome |
| --- | ---: | --- |
|Archival/metadata reproduction|14|0 pass/14 missing-input failures|
|Explicit CPU-emulation prerequisite reproduction|5|0 pass/5 missing-source failures|
|First materialized-input retry|19|18 pass/1 additional bound-session input missing|
|Short-filter plus exact-match probe|0|Not accepted as verification; erroneous selection retained|
|Final full-name selection|20|19 pass/1 strict registry failure|

The final selection is checked against the exact original20 names and the
native harness's declared20-case result. The zero-case probe is never counted
as a pass. The600-second caps are not hit. Both the missing bound-session retry
and the selection mistake are retained, together with exact commands/settings,
exit statuses, fixture inventory and final result. The orchestration is ordinary
existing-unit validation, not a replacement research harness or correctness
oracle. JSON/file/hash bookkeeping performs no curve arithmetic.

`runs.tar.xz`, `RUNS.sha256` and `restore-runs.sh` preserve all raw records;
the receipt checks restored hashes and deterministic repacking. Canonical result
and input/source ledgers remain directly reviewable. No tracked fixture payload
is copied into this change, and no storage upload/credential reuse is performed.

Sources consulted before validation: existing unit bodies in
`src/cryptanalysis/{ecbench,ic_framework,certicom_dlp}`, the documented emulator
selection in `device_tests.rs`, immutable tracked fixture trees, current
main/head AGENTS, and the earlier interrupted library receipt. Tests and
correctness predicates are neither disabled nor weakened.

## Remaining holds

The active registry SHA256 remains
`e7e8cd2ce2570e6e6931674ff653fbaf0cd513b447738364dbb5440a0974a420`.
All1921 missing study models still need reconciliation and combined native
identity/catalog/source-binding validation. Preserve the strict2287 assertion;
do not lower it to1324 or select a whole registry side. The earlier
[field review](../registry-field-review-20261028/README.md) remains applicable
only while its pinned inputs match.

Checkpoint-only authorization and secure access remain pending for dependent
scientific/catalog runs. Conductor exits127, with no reservation or substitute
coordinator. No checkpoint gate, manual upload, new identity, theorem or timing
claim occurs. Historical208 public/269 audited declarations/17 identities and
all dated scientific evidence remain unchanged. Uniform96/896/210 execution
and recurrence elaboration remain pending. Keep PR2381 open and auto-merge off;
no local subset result or review-job success satisfies absent required checks.
