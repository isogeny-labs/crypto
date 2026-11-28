# Source and submission audit

**Scope:** source/access audit and additive-integration review on 22 November
2026 Pacific (23 November UTC in the receipts); no solver, test, benchmark,
workflow dispatch or paid launch was executed
by this audit. Findings are producer-local and do not constitute independent
PCC verification. Historical findings retain their original revision boundaries.

## Revision boundaries

| Revision | Role in this audit |
|---|---|
| [`269cd7b552bd3abcc88c88a6eb5771b59cceb0bf`](https://github.com/isogeny-labs/crypto/commit/269cd7b552bd3abcc88c88a6eb5771b59cceb0bf) | Published public-relations checkpoint on `lane/prime31-rr5-public-relations-20261122`; source and development01–08 retained here. |
| [`e51060051c6963b8852f86844f1ec53dc61f895f`](https://github.com/isogeny-labs/crypto/commit/e51060051c6963b8852f86844f1ec53dc61f895f) | Development09 execution base; already contains the separate same-named RR5 study and later production F4 changes. |
| [`816f9c3a942efed569049618c056d12a0c56d693`](https://github.com/isogeny-labs/crypto/commit/816f9c3a942efed569049618c056d12a0c56d693) | Refreshed publication base; declared production/source-selector inputs unchanged from development09, with no rerun implied. |
| [`f0ca664eca0081a30adfcc79dcf62a32de40d2af`](https://github.com/isogeny-labs/crypto/commit/f0ca664eca0081a30adfcc79dcf62a32de40d2af) | Earlier main snapshot used by the original source/access audit and historical checkpoint qualification. |

At the start of the historical audit, the local branch was
`5514f88782e977945c940e424eb1a94aef06ba48`, based on
`59e81016f77766c6feafd0c4c9dc575ff745735b`. Those are historical states,
not this integration branch's head. The 160-file checkpoint was copied into
`research/prime31_rr5_public_relations_20261123` with per-file Git-blob equality
checked before integration edits. Existing main's
`research/prime31_four_summand_20261122` remains separate. **Fresh development09
now qualifies the relocated declared source closure and its Grevlex integration.**
Its actual new receipts supply that evidence; historical passing tests are not
substituted for qualification against changed production sources. The full-root
gate and requested-size recovery/comparison remain open.

### Publication-base refresh

Before publication, main advanced from `e510600…` to `816f9c3…`. The retained
[expected source inventory](reports/publication-expected-source.sha256) and
[publication source inventory](reports/publication-current-source.sha256) each
contain 41 whole-production/source-selector input hashes and compare
byte-for-byte equal. Root `Cargo.toml`, `Cargo.lock` and `tools/pcc/pcc.py` also
have no diff between those two commits. The integration can retain those
declared source bindings while preserving the newer main coordination and
ledger history. Development09 was executed on `e510600…`; this source-custody
check is not a test rerun on `816f9c3…`, a full-root qualification or a broader
hermetic-build claim.

The GitHub changed-files responses identify [#4942](https://github.com/isogeny-labs/crypto/pull/4942)
as adding the concurrent RR5 example, report and workflow. The checked
[#4949](https://github.com/isogeny-labs/crypto/pull/4949) changed-files response
contains only `docs/ACTIVE.md` (one addition and one deletion). Its qualification
title is not itself evidence of additional source or executed tests.

## Current-main merge findings and preservation boundary

The published checkpoint and main share 44 paths within the old study name:
25 have identical Git blobs and 19 differ. The differences comprise three
study documents, seven historical build logs/manifests, and nine native
source/configuration files. The checkpoint has 116 additional paths; main has
298 additional paths, including its own qualification and PCC evidence. These
are distinct source/receipt histories, not files to resolve by overwriting one
side or by combining same-named development directories.

| Source checked at main `e510600…` | Concrete finding |
|---|---|
| `native/relation_adapter.rs`, blob `ae4fca2ca6e95304b8473a821bd75419c7d1d267` | Identical in the checkpoint and main; authentic-RHS replay, projective duplicate handling, exact rank and target recovery are shared existing source. |
| `native/independent_tests.rs`, blob `53f4976a5f7b4e275485b9b0c55a922ea2facf22` | Main already has a test-only public-label F11 extractor-to-adapter loop. It recovers at augmented rank two, then finishes its fixed schedule at rank three. This is distinct from the checkpoint's reusable SplitMix query loop. |
| `evidence/b8-positive-02/stdout.txt`, blob `3206a407a11053716f15841cd9ea62d60ee91512` | Main's Grevlex quotient-product control is `Complete` with one signed row. |
| `evidence/b8-empty-02/stdout.txt`, blob `4c334148841f7b0f5e5a68554d07f456d2cb44a6` | Main's corresponding generator control is `Empty`, with no signed rows. |

All four paths above are under main's existing
[`research/prime31_four_summand_20261122`](https://github.com/isogeny-labs/crypto/tree/e51060051c6963b8852f86844f1ec53dc61f895f/research/prime31_four_summand_20261122).
Its report is blob `3920ec99bcab2c38283c10d6e1d254c19a6a716e`.
The two B8 raw receipts specify seven variables, eight equations, 99 terms,
degree cap ten and thirty seconds; their observed solving degree is four,
with maxima of 1,085 rows and 1,121 columns. Both record zero full-field
branches and domain evaluations. The historical checkpoint's cofactor call
used Lex, degree cap five and five seconds. Different configurations and
limits prevent a causal ordering claim or matched speed comparison.

The new namespace preserves the reusable `solver.rs`, original-B677 input
construction and compact-selector setup, two sampled identity records, native
evidence exporter, and their historical receipts as an additive continuation.
It does not claim to originate the shared adapter, cofactor construction or
already completed B8 controls. Development09 separately adds and qualifies an
explicit Grevlex cofactor form in the reusable extractor/public-query consumer,
while retaining the historical Lex forms and their incomplete receipts.

Shared coordination files must preserve both histories. Main's ACTIVE entry
names its existing qualification continuation; it must not be replaced by this
lane's row. The November 22 journal tails differ, and main has appended
November 23 entries. Add a clearly identified continuation referencing the
new namespace and checkpoint instead of replacing either journal with the
older branch's complete file.

At `e510600…`, main's PCC ledger preserves the checkpoint's 1,552-entry prefix and appends ten
entries, sequence 1552–1561: nine claims and one reconciliation note. The
observed final entry hash is
`754ca3695c8763054ce9b50a6e16942bdaafcb0da6058dfcd918ff2532019698`.
This is a source-prefix comparison, not a new cryptographic chain-verifier
execution. No new registration was made by this audit. Retain
main's entire current ledger and claim files; any later registration must
validate the then-current complete prefix before appending. `tools/pcc/pcc.py`
has no source difference between the compared revisions.

### Native bindings and relocated metadata

The checkpoint's selector inventories candidate files using literal
`research/prime31_four_summand_20261122/native/...` paths. A directory copy
without a metadata update would compile the relocated modules while
inventorying different files from main's existing study. Development09's
[actual source manifest](evidence/development-09/source-manifest.json) now names
the candidate source/configuration files in the new namespace. Historical
build manifests and their recorded paths remain unchanged; they describe the
original execution.

The checkpoint's complete F4 parent was copied from the repository and
inventoried at build time; that whole-file copy was not guarded by a fixed
expected hash. Main
`e510600…` changes `src/cryptanalysis/f4_fp.rs` to include opt-in, default-off
visitor timing and associated tests. Its Git blob is
`0d48b85ed969cdd3f5a477cf3da18cb208819896`, SHA-256
`367a323b5bec3df0afe4859cb995c0116a62b336fbc04c1917af33b84f75728e`.
The checkpoint used blob `98feb6400c8a2f8f982dfd22554db0c4adefddaf`,
SHA-256 `5fe27d110586e251f725af34a891355aa6dadb67ddc9af19b6f25597481c5aba`.
Byte-exact copying from current source therefore does not reproduce the
historical executable. Development09's actual manifest records the new
`367a323b…` hash and its separate immutable executable receipts qualify that
declared closure. No old receipt or expected hash was rewritten to imply that
the historical executable contained the later F4 source.

The copied `reports/development-export/observation.json` is historical and
retains its original namespace pins. Registering that copied observation as
though those pins named the relocated evidence would be incorrect. The new
[integration observation](reports/integration-export/observation.json) instead
pins 160 raw files in the new namespace and preserves 41 parsed stdout marker
objects plus two identity certificates. Its SHA-256 is
`688e4c7dd5ca6be980ecf1c5c29dcfa355a4e994724f0e5a7bb951a1f736a507`.
The old observation and raw manifests remain unchanged. Historical source
snapshot paths are retained as metadata, while the new raw-file pins name the
relocated files actually read. The observation is unsigned, records unverified
standing and issues no independent-execution or complete-cost claim.

### Development09 result binding

Both the library and metadata builds exit zero. Four separately recorded test
invocations then exit zero:

| Invocation and raw source | Recorded result |
|---|---|
| [Regression](evidence/development-09/regression.stdout) | 22 passed; three focused controls filtered out; none ignored. |
| [B8 positive](evidence/development-09/b8-positive.stdout) | One case passed; Grevlex `Complete`, one exact signed row. |
| [B8 generator empty](evidence/development-09/b8-empty.stdout) | One case passed; Grevlex `Empty`, zero signed rows. |
| [F11 public recovery](evidence/development-09/public-recovery.stdout) | One case passed; scalar 3, rank three, nine attempts, final replay true, deadline exceeded false. |

Each focused invocation filters the other 24 cases; these records are not one
25-case suite invocation. Both B8 traversals complete without timeout, trace
stop, forbidden branching, finite-domain evaluation, refused fallback or
rejected assignment. Their maxima are 1,085 rows / 1,121 columns and solving
degree four. The new public-query integration still accepts only public inputs.
All operation ledgers remain partial; no benchmark or cost ratio is inferred.
The original B677 controls remain input-construction checks and the compact
selector circuit remains unimplemented. The next algorithmic gate is actual
requested-size public relation extraction and verified scalar recovery.

## Earlier RR5 example: exact equivalence at `f0ca664…`

Checked at main `f0ca664…`:

- [`examples/prime31_rr5_quartic.rs`](https://github.com/isogeny-labs/crypto/blob/f0ca664eca0081a30adfcc79dcf62a32de40d2af/examples/prime31_rr5_quartic.rs),
  Git blob `5288938c5f17e6a3568951913347b9ac6a8b7330`.
- [`research/prime31_rr5_quartic_20261122/REPORT.md`](https://github.com/isogeny-labs/crypto/blob/f0ca664eca0081a30adfcc79dcf62a32de40d2af/research/prime31_rr5_quartic_20261122/REPORT.md),
  Git blob `4af43fd085c0a23cb744faa7a61247d092a8083d`.

That implementation writes the public target as `Q`, takes `R=-Q=(u,v)`, and
uses `F=(X-u)(alpha+beta*X)+(Y-v)(X+t)`. Our notation takes `Q=(r,s)`,
`V=X+b`, `L=cX+d`. The exact change of names is

```text
u = r,  v = -s,  t = b,  beta = c,  alpha = d.
```

Its norm is `D=[vV-(X-u)L]^2-fV^2=-(X-u)M`. Substitution gives
`D=((X-r)L+sV)^2-fV^2=-(X-r)U`, so its monic quartic `M` is our `U`.
Both impose the same four coefficient equations `h mod U=0`, where
`h=product(X-x_i)` is the public distinct-abscissa membership polynomial.
Long division versus Horner reduction is an implementation choice. This lane
must be described as **additive integration and qualification of the existing
RR5 monic-quartic construction**, without claiming a new representation or the
first four-summand extractor.

This comparison concerns the earlier example revision only. It does not
describe the capabilities of the separate study now present on main; its
shared adapter, F11 integration and completed B8 controls are credited above.

| Aspect | Earlier example at `f0ca664…` | Published checkpoint `269cd7b…` |
| --- | --- | --- |
| Arithmetic/scope | Original `PrimeCurve`, fixed PRIME31, B=4..8 | Original native `Arithmetic`, checked curve input, bounded B4/B8 and original-base B677 cap control |
| Algebraic traversal | One native `f4` call, then custom triangular univariate substitution | Existing traced recursive F4 visitor; refuse non-univariate fallback before specialization |
| Limits | Degree 12; F4/triangular deadline; signed-lift quartic factoring receives no deadline | Shared absolute cooperative deadline, construction/output limits and retained incomplete outcomes |
| Signed lift | Original curve replay; `main` discards failed `signed_lift` results with `filter_map` while printing `algebraic_complete=true` | Failed assignment diagnostics affect the reported completion status |
| Row consumer | `sum signs*log(P_i)-log(Q)=shift`; rank only; no additional consumer-boundary group replay | General authentic `aG+bQ`, independent group replay, projective deduplication, exact mod-n rank and target-functional recovery |
| Completion | No complete public-target query/recovery loop | One-target public coefficient loop, retained attempts/receipts and final `dG=Q` replay |

Both discovery paths use public polynomial equations and rational-root
factoring, without factor-base tuple enumeration. Neither source inspection
nor either development path establishes a fully charged cold PRIME31 advantage.

## Original dependency closure and inherited manifest failures

The original
[shared-S3 selector](../s3_shared_elimination_20261122/integration_qualifier/build.rs)
was compared against its older source snapshot and the initial local snapshot.
All nine old whole-file pins matched the old snapshot. Of twenty selected byte
ranges, fifteen were identical and five changed. Six whole-file pins still
matched; the three mismatches were:

| Source | Original pinned SHA-256 | Initial local SHA-256 | Selected-range finding |
| --- | --- | --- | --- |
| `groebner_f4.rs` | `576eceef9f2abd24ec8fadb2c14f74e7bcca9db57999c7978ed6485503c18c4e` | `36b0bc3232df109b08feb246b0ff97d132397470a6897c84ac63500d5005974f` | Ordering range identical: 1,586 bytes, SHA-256 `6e778a51bac23df95521897998c2094fb07e046364ac5a13a8317952c0ce56f5`; formatting changed outside it. |
| `ic_boundary.rs` | `8bf3a729bb553cbc2f09d9eca64f3aa2025de4c06d2f16c8f2a199f932444d4c` | `23c8f2d1300a20ce1f17c98f22100f8c9c6f9b75ace45ecd1915eb8fc8a876f5` | Three of eight ranges changed substantively: maps/group hooks, prime arithmetic and factor-base code. Initial source also contained malformed method closures/default bodies. |
| `prime_fast.rs` | `ace8a36eabd60f189d5b6a77e249e103091cc4353a944f52fe8027dfc175dc2a` | `03397cbcd5146df5341eea15febe34912bfed346d25759343a33a7b123712731` | Two of three ranges changed: reduced Montgomery conversion, scratch reuse/arithmetic construction, joint multiplication and batched affine conversion. |

The new [F4 closure selector](native/f4-source/build.rs) is disjoint from the
merged shared-S3 qualifier. It retains the full unchanged F4 parent/children
and actual hashing/counting hooks, selects exact required production ranges,
checks their reviewed whole-file pins and byte equality, and records source
hashes. It omits the unrelated `ic_boundary`/`prime_fast` dependency surface;
it does not replace their algorithms or disable the old qualifier's checks.
The [native arithmetic manifest](native/original-native/Cargo.toml) points to
the complete original native `lib.rs`. Original identity issuer/checker and
SHA-256 source are bound separately; their `hex = "0.4"` dependency belongs
in closure metadata. Missing-dependency build failures remain separate evidence.

The initial root `Cargo.toml`, SHA-256
`3f559087a5ef7c81357fc3e6a079c365df0ead7cbdc5e6f6710f4a921ec4bfb0`,
had multiple `name`/`path` keys in one `[[bin]]` table: missing table headers
before `curve-transfer-sweep` and `odd-transfer-catalog`. This was inherited,
not introduced by the adapter.

**Historical main correction:** at `f0ca664…`, root Cargo has SHA-256
`097c73742907c4d710cb0daff8cd0eaca763e260420675d2cc29a6c8f0cbf692`.
It restores those headers and a missing `[[example]]` header. Main also repairs
the inspected `CountedGroup` syntax and adds `TwTable` work;
`ic_boundary.rs` now hashes to
`ee70c40d611bea2a86e782e8513d6e18be49ca575e1159a42512afb18dc346ce`.
The previous syntax defects must not be reported as current-main blockers.
No full-root build or test success is inferred from these textual repairs.

The retained historical `root-library-complete-paths-gate` no-run build
exited124 at its60-second external cap while compiling dependencies. A separate
`root-library-validation-gate` invoked the required full library test command,
`CARGO_BUILD_JOBS=2 timeout 300 cargo test --release --locked --offline --lib -- --test-threads=1`,
and likewise exited124 during dependency compilation. Root Cargo/lock hashes
match before and after that call. No library tests executed and no fresh source
compile defect was demonstrated. These receipts leave the full-library gate open.

The separate current-integration `root-library-integration-gate` targets main
`e510600…`. Its [exit receipt](evidence/root-library-integration-gate/exit)
records 124 at the 60-second process cap; the retained
[stderr](evidence/root-library-integration-gate/stderr) ends during dependency
compilation. This is a distinct integration outcome, not a replacement for
the historical `f0ca664…` 60-second no-run and 300-second validation attempts.
No passing full-library test result was produced. Development09's passing
component qualification is separate: its manifest explicitly excludes the
full root library and its test suite.

At the historical checkpoint, the inspected `f4_fp.rs`, its three children,
`groebner_f4.rs`, `prime_fast.rs` and `identity_certificate.rs` had no
local-source diff against `f0ca664…`. In particular, that `f4_fp.rs` hashes to
`5fe27d110586e251f725af34a891355aa6dadb67ddc9af19b6f25597481c5aba`,
and the identity checker to
`d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270`.
The focused closure explicitly excludes full-root and legacy-module unit-test
qualification; its manifest is build provenance, not a hermetic-build or test
certificate.

## Actual remote comparison route and present blockers

The following files were read at the historical `f0ca664…` main revision.
IDs below are Git blob IDs, not SHA-256 digests. These source/access observations
do not establish a fresh authenticated submission route for the new namespace.

| Source path | Checked Git blob |
| --- | --- |
| `taskq/deploy/eks-karpenter/HARNESS.md` | `171e7b8034daaf59ee744dfbbb11c7340d4e2d1b` |
| `taskq/deploy/eks-karpenter/ACCESS.md` | `6ca99d07c80c1d8d2de94060351d63f2f10abc6f` |
| `.github/workflows/isolab-experiment.yml` | `cfd10f7c5c42d2a3ffa1c5ca31a4092cd176e7af` |
| `taskq/deploy/eks-karpenter/build-bundle` | `c4385bb1fd9b804a8c0c5b523175ec2eb792a05e` |
| `.github/workflows/isolab-source-checkpoint.yml` | `4755c984e3e245be67ce438940e298af2fe619d4` |
| `taskq/deploy/eks-karpenter/checkpoint-source` | `a0175aede26fa745bef74c96e5ff743a28a3cb62` |

The [supported workflow](https://github.com/isogeny-labs/crypto/blob/f0ca664eca0081a30adfcc79dcf62a32de40d2af/.github/workflows/isolab-experiment.yml)
dispatches on `main`, builds an exact source revision, publishes an immutable
ECR image and submits through the CodeBuild/private-EKS launcher, retaining
source/input/image/executable and S3 receipt checks. A feature-branch dispatch
does not enter its AWS experiment job. The standard build bundle compiles root
`ecbench` and `isolab-fresh-owner` using Rust 1.99.1 and musl; it does not build
or execute this standalone RR5 crate. The existing custom norm-branch mode is
a specific payload, not a generic RR5 entry point.

The checkpoint workflow requires committed `ecbench.spec/v1` and
`experiment.protocol/v1` JSON, with frozen/filed-before-data fields. Its
`checkpoint-source` script archives a fixed source allowlist that excludes
both this lane and `research/prime31_ic_20261115/native`. Unchanged invocation
would therefore omit our source closure. A supported ecbench integration or
reviewed custom image/command route, matching complete source checkpoint and
frozen comparison inputs remain necessary.

Runtime access checks found no loaded IsoLab/Kubernetes tools and no `aws`,
`kubectl`, `isolab` or `isolab-eks-mcp` executable on PATH. `gh` is installed,
but `gh auth status --hostname github.com` returned exit 1: no logged-in host.
GitHub connector reads succeeded. Its exposed workflow capabilities include
reads/reruns but no fresh dispatch operation. No authenticated lab submission,
live balance, capacity, image admission or checkpoint readback was established.
Read access and checked workflow source do not establish those permissions.

The full accounting and matched cold single-target obligations remain in
[COMPARISON_GATE.md](COMPARISON_GATE.md). This audit supplies source custody and
route findings; it supplies no comparison result.

## Final discovery call-graph review

Independent static review followed the actual candidate call graph. The legacy
F4 `solve`/`roots_univariate` path does scan0..p, but this candidate does not call
that path. It invokes `solve_visit_traced`, passing no finite domains. That
visitor's univariate branches call the unchanged GLV
`poly_roots_fp_counted_until`; direct quartic factoring uses the same backend.
The backend's p=2 scan is unreachable because the extractor rejects p<5.

The native-result guard refuses an absent univariate before fallback selection;
the branch guard refuses any non-univariate source before assignment or
specialization. The visitor restores every missing incoming generator to its
basis before reporting the native result, specializes that preserved set, and
checks every original equation before calling the candidate's solution visitor.
The B8 inconclusive outcomes therefore do not arise from silently dropping
original equations. Candidate sign recovery and original group replay then
check every emitted relation.

The exhaustive tiny signed-tuple oracles are separate test code. Some run before
the corresponding candidate call to compute its expected answer; none of their
outputs enter the candidate. This call-graph audit is static source evidence,
not an independently executed extractor or a complexity bound.
