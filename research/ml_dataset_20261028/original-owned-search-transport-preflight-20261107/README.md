# Original-owned search transport: source integration preflight

The original frozen transport proposal passed every bound-source hash/size and
donor comparison, but its registration patch failed ordinary `git apply --check`.
The failure is retained. A separately corrected registration patch now applies
to the exact bound owner source; the worker patch also applies, and its resulting
`solvers.rs` is byte-identical to the supplied `solvers.proposed.rs`.

This is a source integration preflight, **not a compile, type check, solver run,
resource measurement, mathematical validation or SAT rebuild authorization**.
Both retained receipts explicitly say `compiled:false`. The staged tree contains
only 13 relevant files and is not represented as a complete buildable checkout.
The later lift proposal is outside this packet's scope.

## Frozen inputs and result

Original proposal:
`/home/ec2-user/ml-dataset-resume-20261029/original-owned-search-transport-proposal-20261107`.

- Original `MANIFEST.json`: `edde080415bea86cdec339188b744e4c8553a02c0b7a172e6a93a9b6ed43f387`.
- Original `SOURCE_BINDINGS.json`: `4adf02341a1fff60cb8fb5d8cc1ab59228f26b2784124c6f983a4c78948b1293`.
- All 12 listed bindings were copied into private scratch and checked against
  their exact expected byte length and SHA-256; no frozen source was edited.
- Search, BQI bridge and structural-checker donors are byte-identical. The
  scheduler equals its donor after exactly one declared filename replacement:
  `#[path="search.rs"]` to `#[path="dynamic_boolean_search.rs"]`.
- The original registration fails at `registration_patch_apply`; see
  [original failure receipt](positive/RECEIPT.json) and
  [git diagnostic](positive/checks/registration.check.log).
- [Worker-only check](worker-only/RECEIPT.json) passed against fresh copies of
  the bound solver and parent files, independently of registration correction.
- [Corrected full receipt](corrected-positive/RECEIPT.json) records 23 verified
  input snapshots and all 13 staged output hashes, with `success:true`.

The source mapping and arithmetic semantics are reviewed separately by the
parent's source/lift agents. No passing standalone component test is relabeled
as assembled-adapter validation here.

## Registration correction

The original patch uses `@@ -961,1 +961,4 @@` with context on only one side.
The bound owner file's `boolean_rows` registration is at line 972 and has later
registrations. Ordinary application rejects that hunk. The correction inserts
the same `boolean_search` declaration after the same unique source line and
regenerates a normal unified diff with three lines of context from the exact
bound source. It does not relax matching or use `--unidiff-zero`, `--3way`,
`--reject`, whitespace fixing or unsafe paths.

- [Corrected registration patch](corrected-proposal/registration.patch):
  `2cea364a763ee3657b795cd77728e4ac2230d86a7bdb9bc21d9d492e6c662d59`.
- [Corrected proposal manifest](corrected-proposal/MANIFEST.json):
  `7266985559c5d8070aaa55053bb672659933cfb77b595a1137c6e3d700644dbb`.
- The manifest changes only the registration patch's hash/size. All other
  proposal files, including source bindings and proposed solver bytes, remain
  byte-identical to the original.

Reuse this correction strategy against each proposal's own exact bound source;
do not transplant a registration patch from another source generation.

## Bounded verifier and negative controls

`preflight.sh PROPOSAL TRUSTED_MANIFEST_SHA NEW_OUTPUT` verifies an explicit anchor,
all ten proposal members and all twelve source bindings before patching. It
accepts only the known member/basename set, regular non-symlink canonical files,
at most 2 MiB per file and 8 MiB total input snapshots. Copies use bounded,
nonblocking, no-follow reads and are hashed before use. Patches are restricted
to the three named destinations and ordinary textual hunks. All writes occur
under an exclusively created new output directory. Existing outputs are refused,
and successful or failed receipt creation never overwrites an earlier receipt.
The script changes no repository index or shared checkout.

The meaningful controls in `test_preflight.sh` are:

1. A valid relocated fixture passes all gates.
2. Stale bound solver bytes fail source binding validation.
3. An altered patch without a new anchor fails proposal member validation.
4. A rehashed patch targeting an unbound destination fails path validation.
5. A rehashed supplied destination differing from the actual patched solver
   fails byte equality.
6. An extra scheduler body change, despite rehashing, fails donor equivalence.
7. Rehashed stale registration context fails normal patch application.
8. A source symlink fails source admission.
9. A source length mismatch fails source binding validation.
10. An existing output is refused without changing its retained receipt.

All fixture mutations are under `tests/`; no source listed by the original
bindings is altered. Anchors are recomputed for deliberately changed synthetic
fixtures only when testing gates after manifest validation. The final summary
and `tests/results.jsonl` record terminal counts and expected rejection stages.
Each invocation is bounded at 60 seconds; the complete control suite is bounded
at 180 seconds and performs no compilation or native scientific execution.

The implementation follows [official git-apply documentation](https://git-scm.com/docs/git-apply):
ordinary application modifies files without requiring a repository, `--check`
checks applicability, and the default application is atomic for each patch.
Content hash admission and destination restrictions are additional local gates.
The anchored proposal is trusted metadata; this script is not a general hostile
repository parser or an independently authenticated release verifier.

The remaining gate is a separately authorized exact-source compile and genuine
original-owner/capture/worker tests, followed by the missing durable publication,
original-source replay and signed-lift work described by the proposal. No retry,
cloud permission, scientific input or budget changed in this preflight.
