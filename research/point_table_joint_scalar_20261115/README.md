# Joint public-coefficient relation targets

`joint::JointTable`, `prepared::solve_joint` and `solve-hashed-joint` add an
optional shared doubling chain for the existing public `uG+vQ` relation target.
The existing prepared point search, selected witnesses, coefficients, matrix,
elimination and certificate replay remain. The three-point table is created
once, lazily after the original subgroup validation, and reused across relation
attempts/base trials for the same curve/G/Q. [pcc:unverified]

This is HAC Algorithm14.88 in additive notation for two fixed bases. All group
operations use the checked native affine addition, with no scalar reduction or
unchecked formula. The public table constructor validates both subgroup points;
the collector uses only its certified generator and already validated target.
The decomposition oracle sees the resulting point and never the coefficients.
The original `solve-hashed` target-generation path remains a direct control.

## Development evidence

The initial locked release all-target suite passes 169 cases. The first formatting
check then found module declaration ordering only; the failed logs/source are
retained. Final current-source targeted controls and strict all-target clippy
pass after reordering. The initial/final source snapshots and both source
identities remain distinct. [pcc:unverified]

Common controls cover every scalar pair0..18 with equal/opposite/identity
point pairs, mixed/unreduced scalars through1024 bits on F17, invalid and
noncanonical/fixed-cofactor subgroup points, step/width/vector errors, full
seeded certificate/RNG/search-event equality at seeds1,42,83, identity solve,
and actual p5 CLI receipt equality/replay plus exclusive output preservation.
No scientific/benchmark run or general independent audit is recorded.
[pcc:unverified]

Crypto's unpriced addition-event controls, on the same fixed F17 instances,
record the following complete-collector changes, with joint setup and final
verification included:

| Seed | Separate targets | Joint targets | Fewer addition events |
|---|---:|---:|---:|
|1|1131|1015|116 (10.26%)|
|42|905|855|50 (5.52%)|
|83|985|923|62 (6.29%)|

A separate dense131-bit scalar control on F17 records523→260 query addition
events; its setup is metered separately. Events include trivial identity
branches. This is not a131-bit curve run, a runtime comparison, or calibrated
work. No per-query result is promoted to a pipeline/runtime claim.
[pcc:unverified]

Current-source formatting and strict all-target clippy pass in both mirrors.
Rust1.78 all-target checking passes. [pcc:unverified]

[source.sha256](source.sha256) pins every native Rust source/test, fixture,
manifest/lock and API document, plus the actual parent Rust workspace inputs
when applicable. [qualification.json](qualification.json) records commands,
exit codes, source transition, archive bytes/digest and scope. Unchanged
source is pinned by dependency parent `02cc5b6a9183a64acccc9ee58a319e58fc2cf54a`.
[development.tar.xz](development.tar.xz) retains raw stdout/stderr, first
formatting failure, initial/full manifest and touched initial/final sources.
Extract into a fresh directory, then run `sha256sum -c SHA256SUMS` from its
`development/` directory. These are producer artifacts, not signed verdicts.
[pcc:unverified]

## Runtime qualification remains open

Fresh preflight finds no local AWS credentials, checkpoint upload token or bound
cloud identity. The actual checkpoint gate reached transport selection and
failed before upload/readback; its complete attempted note is retained and the
original tracked note restored unchanged. The documented GitHub checkpoint
workflow lookup returns404; the experiment workflow exists, but it does not
supply the missing checkpoint/native-image admission. No workflow dispatch,
image build, cloud job or scientific batch was launched. [pcc:unverified]

The [named runtime dependency](../../problems/point-table-joint-runtime-qualification.md)
keeps the matched runtime comparison open. It does not make a mathematical
impossibility claim. The prior first-pilot USD10 override and original
32-context/2368-attempt workload in separate PR2984 remain unchanged. No study
branch, frozen evidence, scalar result or general-audit standing was changed.

## Sources checked

- [HAC chapter14, Algorithm14.88 and Note14.90, pp617–618](https://cacr.uwaterloo.ca/hac/about/chap14.pdf): established simultaneous multiplication, precomputation and zero-column costs.
- Native `curve.rs`, `reduction.rs`, `prepared.rs` and existing pair controls: validation, lifecycle, randomness, selector ordering and replay contracts.
- Sources checked (KB): KB unavailable; no MCP tools loaded and repository wrapper status/three mechanism, negative and batch-inversion queries fail for missing executable. Lead atlas fails for missing orchestration module; narrow native paths and the primary HAC text are the fallback. No novelty/absence claim.

## Canonical registration remains separate

The existing [canonical-ledger storage dependency](../../problems/prepared-point-tables-ledger-artifact-policy.md)
also affects this source. Exact proposal `b99780d512ee7b407c72a7c14edf0377e2d9b27e`,
retained on `codex/point-table-joint-registration-20261115`, contains the normal
producer statement PCC-20261115-21bbde and its full append-only ledger prefix.
Its artifact check selects the696074-byte canonical ledger for migration;
[original output](registration-policy.stdout.log) is retained. No waiver,
canonical ledger rewrite or registration/verification claim is made on the
code PR. Its canonical ledger remains byte-identical to the parent.
[pcc:unverified]

## Subsequent parent refresh

The stacked parent received automated merge683647fbb2c9d21bf8a87161d7e9d4b0f3ffe23f
from mainb05c8a03af1034fdc6c92c1bb061a9258ad0b555. Its native source and old
feature evidence are byte-identical to the preceding parent; its canonical PCC
contents equal that main commit. Full journal-resolution review remains pending.
A normal local merge into the successor found only the dated journal conflict
and was aborted without choosing sides. [pcc:unverified]

The [attempt receipt](crypto.parent-refresh-attempt.json) pins both commits,
original source-journal blob and conflicted result digest. The complete raw
conflict is losslessly represented by the original Git journal plus the
[compressed unified patch](crypto.parent-refresh-conflict.journal.patch.xz);
its reconstruction was compared byte-for-byte before upload. To reconstruct,
extract the pinned source journal with `git show`, decompress the patch with
`xz -dc` and apply it with `patch --output RESULT BASE PATCH`; check RESULT's
SHA256 against the receipt. The full uncompressed conflict is retained locally
as well. No journal entry or frozen source/body was picked over another.
[pcc:unverified]
