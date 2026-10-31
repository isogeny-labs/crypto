# Explicit x-coordinate factor bases and GPU relation finding

Current status, 2026-10-30: the explicit-x plug-in and standalone CUDA service
have been delivered in PRs. The [v11 panel](../../research/explicit_relations_e2e_20261030/reports/run-20261030T1550Z/report.typ)
completed all 12 prescribed executions, including both fixed 62-bit targets,
on one RunPod RTX PRO 6000 with fresh certificate replay. This is a bounded
result for the recorded inputs and source, not a hardware maximum or a
certified speedup. [GPU build and usage](../../gpu/explicit-relations/README.md)
distinguish that validated configuration from the separate 63-bit arithmetic
and negative-filter extensions, whose GPU validation remains pending.

The ecbench x-list plug-in is CPU-compatible; the GPU pipeline uses the
standalone service and paper driver. No root crypto/plugin build was performed
on this disk-constrained workspace. RunPod execution superseded the initial
source-only handoff below. Any future EKS image must be built from the merged
`crypto/main` commit and verified separately.

## Exact x lists in ecbench

```text
explicit-x:xs=X0;X1;X2;...
```

Use a nonempty semicolon-separated list of decimal or `0x` hexadecimal field
words. Quote the entire spec in a shell. The same string works in
`ecbench fb --factor-base` and the `factor_base` parameter of `ic.pipeline` or
`ic.pipeline_counted`. For example, zero always lifts on the supported binary
curves, so this dumps the singleton above zero (a construction example, not a
useful solving base):

```sh
ecbench fb --curve '{"kind":"koblitz","a":1,"n":7}' \
  --factor-base 'explicit-x:xs=0'
```

For solving, supply a suitable list of liftable x coordinates on the actual
workload curve; the plugin does not replace an unusable base with another one.
The contract is:

- Exactly the supplied abscissae, in supplied order as matrix columns.
- All rational points above each x, with coefficients `+1` and `-1 mod r`.
  A self-negative point appears once. The positive representative has the
  smaller canonical y coordinate. Zero is allowed.
- Strict rejection of duplicate numeric values (including `1;0x1`), empty
  tokens, integer overflow, coordinates outside the field, missing lifts,
  missing `xs`, and unrecognized parameters.
- The field representation is canonical residue for prime curves and the
  existing polynomial representation for narrow binary curves. Existing
  ecbench width limits remain: prime `p < 2^62`, narrow binary `n <= 62`.
- Raw curve points and the existing cofactor convention. No subgroup
  projection, Frobenius closure, inferred linear span or `subspace_basis`.

Lifting is counted as `sqrt_solves` or `as_solves`; index construction uses
`FactorBase::from_column_map` and its existing counters. Parsing is recorded as
`explicit_x_parse_uncharged`. Point construction needs no group additions.
Wall time remains diagnostic. The new source file is included in the IC
implementation manifest used for candidate identity.

The exact parameters and sorted point digest enter the FB1 identity. Spellings
and list order are part of the recipe; freeze and reuse the same string. Tests
check exact inventory and column order, dump/run metadata equality, verified
answers and child-process replay on both prime and binary fixtures.

## Signed per-slot GPU bases

The paper's bases contain signed points, with an independent base for each
summand. An x list with both signs is a different construction. The standalone
[CUDA relation service](../../gpu/explicit-relations/README.md) supports both:

- `xs`: GPU lifting of explicit prime-field x lists, retaining both signs.
- `hash`: GPU inverse-hash enumeration of the paper's signed base.
- `bases` and `find`: exact ordered relations over 2–8 distinct slot lists.
- `paper-driver`: the existing complete bounded reduction, GPU base
  construction and relation queries, CPU linear algebra, independent CPU
  certificate verification, and an exported certificate with a replay command.

The GPU search uses meet-in-the-middle sum tables. It does **not** implement
the compact Boolean tensor network or the external fast contraction theorem.
No subexponential runtime, practical speedup, or deployed-curve result is
claimed. CUDA work is not registered as a measured ecbench method or priced
using CPU GAE. The x-list plugin is CPU-compatible; GPU execution uses the
separate service and paper driver.

## Existing binary linear basis interface

```text
binary-subspace:dimension=5,basis=0x1;0x2;0x4;0x8;0x20
```

This preexisting interface enumerates the nonzero binary span, and is not an
explicit x list. It requires exactly `dimension` nonzero independent vectors,
each within the field. The earlier local change remains in this branch:
validate dimensions in `1..=30` before narrowing to `u32`, rejecting values
such as `4294967299` that previously wrapped to 3. Its regression tests are
retained along with the new x-list tests.

## Original source-only validation and builder handoff (2026-10-29)

This dated handoff records the initial local state. Subsequent RunPod evidence
is linked above and in the timestamped entries below; it does not retroactively
turn the initial static checks into executable validation.

Static checks: Rust formatting/parser checks, C++ `-fsyntax-only` with warnings
as errors, diff whitespace checks and scoped curve-name lint. These do not
establish Rust type correctness, CUDA compilation or runtime correctness.
The full curve-name scan is blocked by unmaterialized tracked files in this
sparse checkout; the scoped result does not replace it.

On a builder with sufficient disk, run the repository checks and new tests:

```sh
cargo test --release --lib
cargo test --release --test ecbench
cargo clippy --release --lib --bin ecbench --test ecbench -- -D warnings
python3 scripts/check_curve_names.py --diff origin/main
make -C gpu/explicit-relations test
make -C gpu/explicit-relations test-cuda ARCH=sm_80
```

Select `ARCH` for the actual NVIDIA GPU. Run Compute Sanitizer on the CUDA
self-test, including memcheck, racecheck and synccheck. Test sources include
all-target comparisons with the original CPU oracle, independent affine
arithmetic, both square-root congruence classes, identity and inverse sums,
repeated sums, partial blocks, chunk boundaries, empty bases, exhausted limits,
and full certificate replay. None of these executable tests ran here.

After review and merge, build the runtime image from `crypto/main`; retain the
commit, image digest, compiler/CUDA versions and binary/source hashes. Check
that the image actually contains both the x-list plugin and CUDA service.
The available isolab job plugin is CPU-only; this change launches no jobs.
A performance claim requires a separately frozen experiment protocol and
matched complete costs, including base construction, transfers, table setup,
failed queries, CPU elimination and independent verification.

## Source review and handoff: 2026-10-29

### What was done

Reconciled current `main` at `d5c8ba5fe1bd9b809acc1fc1b780b30eaa9931b8`,
including its new `src/cryptanalysis/hashed_ecdl.rs`. Reused that reduction,
`gpu/rootfind/fp64.cuh`, the framework's factor-base indexes, and ecbench's
identity/replay machinery. Read the user-supplied v0.2 paper, particularly
sections 3.2–3.5, and the repository's `docs/hashed-ecdl.md`.

Primary source: [Akshima et al., v0.2 source repository](https://github.com/bochum-crypto/subexponential-ecdl/tree/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f).
The external [Boolean tensor contraction theorem source](https://github.com/openai/math/blob/main/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/build/quantitative-reduction.tex)
was inspected for scope; its recursive algorithm is not implemented here.
No novelty or theorem claim follows from these source changes.

### Decisions made

Keep exact unsigned x-list bases distinct from signed slot-specific hash
bases. Retain the CPU default oracle. Add an optional base-construction hook
with sorted-point and membership validation; certificate replay independently
rebuilds all bases. Preserve limits as errors, never negative results.
Stop at a local source change ready for PR preparation, without push, PR,
image build or EKS execution.

### Open threads

All compilation/runtime checks above; hardware performance validation; the
compact tensor encoder and fast tensor contraction backend; wide fields;
GPU execution integrated with measured ecbench accounting. The complete
paper driver inherits `p <= 65521`, cofactor one and at most 512 columns.
The standalone GPU service has a wider prime-field domain as documented.

### Errors

The first C++ syntax check found three misleading-indentation warnings;
the code was corrected and reformatted, then the check passed. An attempted
`sed` read used the GPU directory as cwd and was repeated from the repo root.
The shared daily session log is reserved by another Conductor task, so this
scoped handoff carries the entry. T-854 covers the main change, T-856 the
paper hook and T-857 the candidate identity binding.

## Publication and cloud validation follow-up — 2026-10-29

### What was done

Opened [PR #2889](https://github.com/isogeny-labs/crypto/pull/2889) and enabled
auto-merge per repository policy. Merged main and added a standalone Cargo
manifest for its new tensor re-export. Submitted an RTX PRO 6000 validation
job through the managed ephemeral runner; AWS rejected it before launch.
Submitted the owner's requested GPU quota increases. The exact requests and
software validation outcomes are in
[the validation record](../../gpu/explicit-relations/VALIDATION.md).

### Decisions made

Keep the existing CPU tensor implementation on main separate from the GPU
meet-in-the-middle path. Use native cloud builds to respect the local disk
constraint. Retain the requirement that any EKS deployment use an image from
crypto main. Device execution requires actual RTX hardware and successful
tests; a cloud build or a quota request does not establish that result.

### Open threads

AWS GPU quota approval and RTX device checks; root Cargo/ecbench checks and
PR gates; performance validation under a frozen comparison protocol.

### Errors

The RTX Spot request failed with `MaxSpotInstanceCountExceeded`. Both GPU
quotas were zero. No GPU instance was created. A CPU build submission with
`--ref HEAD` initially stopped because this checkout tracks only main's
remote ref; fetching the already-pushed feature ref resolved that admission
check without changing the source.

## RunPod device validation follow-up — 2026-10-29 Pacific

### What was done

Ran the pinned PR #2889 source on RunPod pod `09ollojw9j8vbo`, one full RTX PRO
6000 Blackwell Server Edition. Host and device service tests, all 10 standalone
Rust tests, bounded reduction/certificate replay and Clippy passed. Compute
Sanitizer memcheck reported zero errors. Collected the result archive and raw
provider logs, uploaded them to the task's S3 validation prefix, and verified
pod deletion. Exact source, image, toolchain and artifact identities are in
[the validation record](../../gpu/explicit-relations/VALIDATION.md).
PR #2889 merged while this validation was running.

### Decisions made

Follow the owner's RunPod selection instead of waiting for AWS quota. Reconcile
existing pods before launch and leave the unrelated ECC2K-130 workload alone.
Use a controller deadline and collect artifacts before deleting this task's
pod. Record correctness results without promoting them to a speedup claim.
The implementation sources are unchanged from the tested revision.

### Open threads

Root crypto/ecbench integration checks are outside this standalone run.
Comparative performance, wider fields, the GPU tensor backend, and measured
ecbench GPU accounting remain separate unresolved work. Any EKS deployment
still requires an image built from crypto main. This validation pod is gone;
there is no background GPU workload from this task.

### Errors

No compilation, device correctness or memcheck error occurred on RunPod.
The first detached local controller invocation exited before creating a pod;
a managed exec session launched the one recorded pod after reconciliation.
The shared daily log remains reserved by T-827, so this scoped handoff carries
the entry under Conductor T-861. AWS quota blocks are historical and no longer
block this completed device validation.

## Normalization optimization — 2026-10-29 Pacific

### What was done

Implemented a CUDA product-tree block inverse, retaining old-scan and CUB
reference builds. Ran the frozen four-arm panel on RunPod pod 1lq0cd2l0en5ju:
120 complete rows, identical cross-arm outputs, independent CPU witness
checks, and zero memcheck/racecheck/synccheck errors. The pod was deleted.
Committed the frozen protocol, native instrument/analyzer and raw records;
[the report](../../research/explicit_relations_perf_20261030/RESULTS.md)
records exact sources, hashes, diagnostics and limitations.

### Decisions made

Use the product tree by default: it preserves outputs while reducing the
normalization inversion-preparation count from 4608 to 765 multiplies/block.
Keep timings as L0 diagnostics. The observed kernel change did not establish
an end-to-end speedup; query/cold costs and A/A ranges remain in the report.
No leaderboard or scientific claim is promoted from this software panel.

### Open threads

Chunk synchronization and repeated right sums are next optimization targets;
respect explicit memory caps and deterministic witness order. Root/ecbench
integration and the separate GPU tensor backend remain outside this run.

### Errors

The initial export adapter needed its native passed label mapped to the
schema's verified label. A metadata-only corrected export withdraws an
unobserved unmasking timestamp; raw rows are unchanged. A concurrent Git
staging attempt hit the index lock and was retried after the first commit
finished; the frozen protocol was verified committed before the GPU launch.
Conductor T-863 replaced the expired/cancelled T-862 reservation.

## Query transfer optimization — 2026-10-30 02:24 UTC

### What was done

Implemented radix-packed deterministic query winners, overflow fallback and
device-side reset. This removes the match buffer for encodable batches and
the extra witness readback without changing explicit chunk/work limits.
Added repeated-query/prepare, empty-plan, chunk-tail and integer-boundary
regression checks to `make test-cuda`. RunPod pod `bbrl08j1o8gizz` executed
pinned source `68be1081e696c232881b2da8d864d93b297e5092`: all 120 fixed rows,
host/CUDA/reduction tests, 10 Rust tests, Clippy and all three sanitizers passed.
The pod was deleted, archived evidence was downloaded back with matching
SHA-256, and the temporary local credential was removed. The checked export,
full ranges and inspected PDF are in the new
[query study](../../research/explicit_relations_query_20261030/RESULTS.md).

### Decisions made

Keep the new path as default with the previous array path for unencodable
radix products. The observed query medians fell by 14.24%, 18.43% and 5.81%
on the three fixed sizes; these remain descriptive L0 observations. Query
API payloads fell from 80 to 32 bytes on each small case and 800 to 392 on
the largest, excluding preparation and physical bus framing. Preserve A/A
variation and preparation/cold outliers; no leaderboard claim is promoted.

### Open threads

Repeated right-side tuple construction is the next optimization target;
charge caching to preparation and explicit memory limits under a new frozen
panel. No further GPU run is scheduled. Root integration, general paper
reduction and the GPU tensor backend remain outside this change. PR #2896
contains both optimization stages; check current-head CI separately.

### Errors

No device/compiler/sanitizer failure occurred. The canonical CSV adapter
requires mapping native `passed` to schema `verified` in export metadata;
the completed export has zero checker findings and raw data is unchanged.
GitHub's artifact migration may move large validation logs to its manifest;
retain that commit and verify the manifest against the archived log. The
shared daily log remains reserved, so this scoped handoff is under T-869.

## Full recovery scaling — 2026-10-30 04:54 UTC

### What was done

Removed the standalone reduction's exhaustive tiny-field restriction with
an exact prime-order/Hasse witness, Tonelli–Shanks roots, checked wide
products, deferred dependency reconstruction and sparse certificate replay.
The default tiny constructor and 512-column cap remain. Explicit driver
arguments admit larger work/column bounds. Protocol-v3 ran all44 prescribed
cells on one RTX PRO6000:40verified recoveries through52bits and4retained
56-bit timeouts. Protocol-v4 added bounded high-x-bit table indexing and
completed both56-bit targets and identical repeats in212–232s including
audit; all4attempted58-bit cells hit explicit work limits. Both pods are
deleted, with content-hashed archives and raw records committed under
`research/explicit_relations_e2e_20261030/results-v3` and `results-v4`.

### Decisions made

The52-bit stage records put almost all cost in relation search, so the next
change reuses two right-side partial sums, each bounded by chunk_rows, with
all setup and storage charged. Preserve the uncached prefix path as a build
reference and the exact witness/visited ordering. Frozen protocol-v5 compares
at52bits and probes56/58/60 with both public targets, explicitly raising
max_rows to2^30 and max_work to2^31. The prior2^29 row cap forced a highly
unbalanced58-bit split; its failure was not a nonexistence result.

### Open threads

V5sourcef6338418572c72e1bf97ad24903cd5efd17cf7da is running on RunPod
9fil2lhphdymz0, controller session69390, local launch bundle
`/home/ec2-user/ecbench-e2e-20261030-v5`. Reconcile that exact job before any
launch. One GPU, worker4500s/controller4800s; collect the archive and verify
deletion. The report draft coversv1–v4; appendv5, render and inspect every PDF
page before publication. PR2896 remains the delivery PR; exact-head CI and
runtime validation are separate. No GPU tensor backend, matched rho result,
fastest-tool status or certified speedup is claimed.

### Errors

V1joined two wrapper commands and failed before compilation. V2passed
preflight but failed compiling the native chart analyzer; neither produced
ladder data. Both failures and immutable protocols remain. CI found a
manual-divisibility lint in the wide constructor; it was fixed. Canonical
export schema/flow/hash checks pass, but its protocol-aware coverage check
assumes rectangular panels and retains3findings forv3 and5forv4. The frozen
native analyzers verify the explicitly nonrectangular contracts and stopping
rules; this is not a full canonical checker pass. A concurrent staging
attempt hit index.lock; it was retried after the active commit completed,
without removing a lock or rewriting history. Artifact-bot migrations are
preserved and checked against archived log hashes. T884owns this continuation;
T882was released after its implementation phase. The shared daily log remains
outside this task's scope, so this is the committed project handoff.

## 2026-10-30 05:58 UTC — compact-row continuation

### What was done

V5 completed ten executions: nine independently replayed recoveries and one
58-bit timeout. The largest two-target rung was 56 bits; seed 101 recovered
at 58 bits in 595.862060 seconds cold, while seed 202 reached 600 seconds.
Raw records, archive hash and verified deletion are committed under
`research/explicit_relations_e2e_20261030/results-v5/`.

Exact x/parity compression now provides an opt-in 16-byte table row instead
of 32 bytes, retaining original tuple IDs and deterministic witnesses.
`make cuda-compact ARCH=sm_120` selects it. Full/cached/compact service builds,
13 Rust tests, reduction replay, exhaustive compact/full lookup checks and
query sanitizers passed on RTX PRO 6000. The corrected maximum row-cap
header also passed in the active retry before timing began.

Separate post-run audits validate the earlier 120-row normalization/query
panels and v3/v4/v5 frontier coverage. Their regression controls reject
missing and duplicate executions, wrong inputs/flags and truncated successful
prefixes. Frozen analyzers and measurements are unchanged. Unadmitted
benchmark visual copies and a clearly labeled catalog link are committed;
the admitted benchmark table remains byte-identical.

### Decisions made

The cache showed no matched 52-bit timing benefit, so it remains opt-in.
Compact storage is exact on a fixed validated odd-prime curve (SEC 1 v2,
section 2.3.3), not a probabilistic key. New protocol-v7 keeps the v6 fixtures,
arms and explicit limits, while correcting the service-header cap. Only one
task GPU runs at a time. All clocks remain L0 diagnostics; no external-tool,
matched-rho or conditional tensor-contraction result is claimed.

### Open threads

V7 pod `ex8tm7votw4h4g` is active, controller session `16482`, local bundle
`/home/ec2-user/ecbench-e2e-20261030-v7`, pinned source
`4014622e18caef9a6b6c89d79653e7633f5aaaff`. Reconcile it before any launch.
The six 52-bit cells and both 56-bit targets passed; 58/60-bit outcomes are
pending. Collect the complete archive, verify deletion, run the strict
frontier audit and publish raw records before the final inspected report.
The draft report covers v1–v6; its final chapter and benchmark copies await
v7. T894 owns this continuation; T895 owns the visual catalog. The shared
progress exporter remains reserved by T890 and has not been edited.

### Errors

V6 passed preflight but failed its first solver header: the protocol requested
2^31 rows while the service still enforced 2^30. No CSV data row was recorded.
The failed archive and deletion receipt remain in `failed-launch-v6/`.
Source 4014622 adds consistent cap admission, boundary tests and actual header
preflight under a new frozen protocol. The initial push was rejected after
an artifact-bot update; that update was merged without rewriting history,
and moved log/PNG hashes were verified. Canonical rectangular-coverage
findings remain disclosed; the separate audit verifies the actual frozen
nonrectangular plans. The separate CPU tensor implementation and its open
independent audit are documented in `problems/hashed-ecdl-tensor-backend.md`;
this CUDA service does not port that backend.

## 2026-10-30 06:24 UTC: larger prefix index prepared

### What was done

Reconciled the active v7 pod without launching another GPU. Both 58-bit
public targets passed recovery and fresh replay; 60-bit outcomes remain
pending. Merged remote CPU work already delivered through PR2933 without
rewriting evidence. The v7 source stays4014622; the merged CPU changes are
not retroactively GPU-validated by that run. Added optional ER_WIDE_INDEX,
a 2^24-bucket ceiling with actual storage charged to admission, and exact
CPU/device offset and lookup boundary tests. Host syntax and Rust instrument
metadata checks passed; CUDA validation remains pending.

### Decisions made

Relation lookup dominates the observed large-field stage clocks. The next
matched52-bit panel changes only the index ceiling on identical16-byte rows
and the same driver, then probes56/58/60bits. Frozen v8 hash
fd8bf698d02f02bea73c7ed46c4b55e039ee62927e4f18fa74165b691afc793c;
worker9000/controller9300seconds,60-bit solve1800seconds. No new GPU before
v7 collection and verified deletion. Field61/62 widening was assessed but
not implemented; no task or run for that arithmetic change exists.

### Open threads

Collect v7; preserve every outcome and archive; validate optional wide index
and the merged CPU driver on the next single RTX PRO6000. Publish raw data
before the final inspected report and benchmark copies. The progress exporter
is now reserved by T901, so its shared file remains untouched.

### Errors

The first draft v8 schema check rejected its absent freeze block. Added the
required draft block, validated and froze it before any affected data. This
was a metadata preparation error, not a GPU launch.

## 2026-10-30 06:48 UTC: compact-table frontier collected

### What was done

V7 completed12 prescribed executions: ten verified recoveries through both
58-bit targets and two60-bit timeouts at1200seconds. Exact58-bit cold times
were489.753544246 and530.119796136seconds. Both60-bit tables admitted:
1,202,274,400/1,025,758,188 left rows with conservative allocations
57,709,730,784/49,236,951,816bytes. These were time limits, not memory refusal.
Archive SHA25601cd78dbd5cb64410d0c02ba12473964683299f42702a7c5cb46a8f4b7c37891
matched. Worker/controller exited0 and podex8tm7votw4h4g deletion was verified.
Public receipts were uploaded; the key remains only in v8's restricted binding.

### Decisions made

Retain58bits as the largest complete rung in v7. All six52-bit matched
certificates agree; full32-byte cold times37.660575827/40.318152374seconds
and compact16-byte candidate29.427842022/31.972280910seconds are L0
observations. Keep every timeout. Continue only with the separately frozen
v8 larger-index comparison, pinnedbbf0f2f51e73, after raw publication.

### Open threads

V8 request is prepared, no pod created yet. T910owns the wider-index phase.
Final inspected report and benchmark copies will include both panels.
The shared driver source is reserved by T906; wider-field arithmetic remains
unmodified. The progress exporter is reserved by T901.

### Errors

Canonical schema/flow/file hashes pass. The protocol-aware checker reports
four rectangular-coverage findings; CHECKER-NOTE.md preserves them. Frozen
native analysis and separate strict audit pass the actual12-cell contract
and reject corrupt coverage. No failed execution is counted as a recovery.


## 2026-10-30T14:56Z — completed 60-bit panel

### What was done

Collected v8 on source bbf0f2f51e73afd7e41174094feb2c56a72d3d9d: all 12
prescribed executions recovered, including both 60-bit targets. Fresh CPU
replay, wrong-scalar rejection and matched-arm byte equality passed. Archived
results SHA-256 f8924a3418fd268f0f8492f260b8e7fda66616017e1b60fc84488342a98e7d10;
controller/worker exited zero and royvvmmehidtwt deletion was verified.
Public receipts were uploaded and task credential/request files removed.
Raw results committed before final report generation. The seven-page v1-v8
report was visually inspected; complete graphs and browsing copies accompany it.

### Decisions made

Retain all prior failures and timeouts. Wide indexing remains explicit opt-in.
The 60-bit cold solves were 770.270416129 and 845.179937170 seconds; L0 clocks
remain descriptive. Current source includes separately merged CPU work, so
v7/v8 timing differences do not isolate one change. Extend the existing T-906
field-width task only after this completed panel is safely published.

### Open threads

Fields 61/62 need reviewed hash/driver arithmetic and a separately frozen
single-device admission/recovery probe. No hardware maximum, matched rho,
fastest external solver, GPU tensor backend or independent scientific
validation is established. Current-head CI remains a separate check.
The new progress-metadata reader emits 20 exact metrics in 3.563 seconds;
the entire pre-existing exporter reached a 55-second administrative timeout
before the new function on this storage, so its full runtime requirement is
not claimed passed.

### Errors

System Python lacked the schema validator API; the established tools venv
passed schema/flow/hashes. Four canonical rectangular-coverage findings remain
recorded, while the exact native analyzer and eight-mutation strict audit pass.
An initial sparse git add excluded tools/progress_metrics.py; it is staged
explicitly with --sparse in the subsequent metadata commit. A ready T-906
reservation was initially mistaken for another live worker; inspection and
successful claim resolved it without overriding ownership.

## 2026-10-30T15:06Z — bounded wider probe and exporter runtime repair

### What was done

Committed/pushed e1a38ce4bc586050c3faa67a3ab79f312eb549b5 with p<2^62,
r<2^63 and checked hash s<2^63. Frozen v9 hash
bf1e4a2dddbfdadb2cb8de32a0c659bbcdd528cc2851c63864c66d009f4e5142
was committed before launching pod nd5qev6envru7z at 15:02:46UTC. One full
RTX PRO6000 in US-NC-1 at catalog2.49USD/hour, pinned CUDA image; worker18000s,
controller18300s, single-object upload expires after21600s. Existing unrelated
pods were inspected and left untouched. All19 Rust tests and initial CUDA,
Clippy/memcheck controls passed; first52-bit execution succeeded. Larger
outcomes remain pending. No second taskGPU is authorized concurrently.

### Decisions made

Preserve fast Montgomery hashing below2^62; use exact canonical add/double
only for wider hash moduli. Retain25%free-memory reserve and conservative
sort admission. The new row cap2^32 allocates no memory by itself. Frozen
52-bit controls precede two fixed61/62-bit targets; stop after the first
incomplete rung, recording every refusal/timeout.

### Open threads

Collect, hash, replay-audit, export and report v9; verify deletion before
any further GPU launch. Its source is distinct from the completed v8 record.
Current-head CI and independent scientific review remain separate.

### Errors

The full progress exporter previously exceeded55s due to repeated Git
processes. Persistent cat-file reads now preserve exact git-show bytes,
missing-source behavior and cleanup. Default command with --since2026-10-01
passed in24.17s; HEAD export also passed within55s with675records. The prior
runtime limitation is resolved for these observed checks, not hidden.

## 2026-10-30T15:49Z — 61-bit result and queried-sort follow-up

### What was done

V9 completed with eight executions: seven verified recoveries and one explicit
memory refusal. The 61-bit seed202 recovery took1537.918072841s including fresh
verification (L0 diagnostic). Seed101 refused admission at85969611408bytes;
62-bit cells were unattempted by the frozen stopping rule. Raw artifacts and
strict eight-mutation audit are committed in6e850f7dfc14. Pod nd5qev6envru7z
was deleted and its absence was independently rechecked before continuation.
PR2896 merged; remaining changes will be delivered in a follow-up PR.

### Decisions made

V10 remains frozen but unexecuted. V11 retains separate four-arm52-bit controls
for exact12-byte rows and queried CUB DeviceMergeSort scratch, then probes the
same two61/62-bit targets once. NVIDIA CCCL3.2 dispatch source confirms null
scratch sizing includes keys, partitions and virtual shared memory. The same
25%free-memory reserve remains. No default backend promotion or certified
speedup claim is made. Protocol hash:
0f916e48bea43b0b7dabb85bc8cb50fe0d31c2a268aa89c016e1df6b46a0a09e.

### Open threads

Commit/push v11 before launch; checkpoint; validate CUDA and all sanitizer
controls on one RTX PRO6000, then collect every outcome and verify deletion.
V9's largest complete prescribed rung is52bits, with one61-bit recovery;
the separate v8 panel completed both60-bit targets. Absolute hardware maximum
and the paper's conditional tensor backend remain unresolved.

### Errors

No v11 GPU execution yet. Local host syntax, shell syntax and both native
metadata compiles passed; local root/plugin builds remain prohibited by the
user's disk constraint. The standard dated log is absent in this sparse
branch; this existing scoped project handoff preserves the required headings.

## 2026-10-30T15:54Z — v9 report and v11 execution

### What was done

Published a separate two-page v9 observation report, preserving the earlier
seven-page v1-v8 report. Both new pages were visually inspected; SVG/PNG/PDF
catalog copies are hash-bound and explicitly unadmitted. The post-run strict
checker now recognizes v11's four-arm52-bit and v-only61/62-bit contract;
existing v9 positive and eight mutation controls still pass.

### Decisions made

V11 source d44b59149f468357e86186f956ecb7e323f38915 was pushed and checkpointed
before pod jees49ew9dkfd7 launched15:50:53UTC at2.49USD/hour inUS-NE-1. It is
one RTX PRO6000 with97887MiB reported by nvidia-smi, CUDA13.2.51, Rust1.88.0.
Controller exec90595 collects and deletes with its18300s bound. New PR2971
is open with auto-merge enabled; PR2896 is merged. GPU preflight is in progress.

### Open threads

Collect/audit/export every v11 outcome, verify deletion, remove private request
and credential, publish raw data before its report, and recheck exact-head CI.
The protocol is unchanged after launch. No v11 recovery claim yet.

### Errors

The first report render interpreted an unescaped less-than sign as a Typst
label; corrected to code markup and both final pages rendered and inspected.
CUDA emits ignored defaulted-constructor annotation warnings; no failure has
been observed so far. PR attachment tool calls remain pending in the app.

## 2026-10-30T16:06Z — optional GPU-resident query prototype

### What was done

T940 holds the GPU source and this handoff. Added an optional persistent
right-search grid, exact two-stage minimum witness resolution, a24-byte result
readback and actual search-visit accounting. The ordinary chunked path stays
available. Tests force a two-block grid, multi-stride late matches, complete
misses, partial tiles, resets and empty reprepare. Host syntax passed; the first late-hit fixture constructor failed because
its row cap also bounded input-base size. A corrected explicit unbalanced
split is checked in the following repair; CUDA validation remains pending. Updated README
arithmetic guards to match the already validated v9 source.

### Decisions made

NVIDIA grid-stride guidance and CUDA device-scope atomic memory semantics
were read before implementation; links and correctness argument are in
PERSISTENT-QUERY.md. No cross-block wait or occupancy requirement is introduced.
The active v11 remains pinned to d44b59149f46 and is unaffected by these edits.
A separate frozen matched panel must precede any timing or backend promotion.

### Open threads

V11's eight52-bit control solves verified. Both61-bit outcomes remain pending.
Prepare a separately bounded persistent-query validation after collection and
deletion; never launch a concurrent taskGPU. Preserve every failed attempt.

### Errors

Full HEAD progress export twice hit55s on the shared workspace; a scoped
committed GPU export completed in7.76s with all24metrics including v9 counts.
A default-ref diagnostic invocation completed successfully within55s. Moving
the script outside sparse checkout first lacked its repository selector;
PM_REPO_PATH fixed that administrative invocation. No metrics source change
was made, and no failed invocation is treated as a pass.

## 2026-10-30T16:51Z — exact full-right reuse and separate controls

### What was done

V11 live logs show both61-bit targets verified:1123.455s and1405.366s total,
with full exact values pending final archive collection. Its first62-bit case
is running. Added an optional16-byte exact affine right-sum cache, constructed
once per bases after sort scratch is freed. The selected phase peak preserves
the existing25%free-memory rule; insufficient cache capacity selects an
explicit uncached fallback. Host round-trip, admission-boundary/overflow and
syntax checks pass. CUDA validation remains pending.

### Decisions made

Read NVIDIA memory optimization guidance before implementation; sources and
lifetime argument are in FULL-RIGHT-CACHE.md. Preserve v5's partial-cache loss.
V12 was never executed. V13 retains b/u persistent-query andu/v full-cache
controls, withv/x repeat at52bits, then probes the larger frontier once. It
requires the intended cache mode for every successful52-bit control. Frozen
hash9ab5b955afbc2ba7f9878793a8012b0b740b29fa5107c40b5d8adb23fb012413.
Native instruments typecheck and schema/freeze checks pass; no new data yet.

### Open threads

Collect/publish/delete v11 before any new GPU. V13 is the next panel; v12 must
not launch. Preserve every failure/fallback and charge full construction.
Run all new device differentials and sanitizers before data, then collect,
strictly audit, export, report and deliver exact current-head CI. GPU tensor
backend and independent scientific audit remain unresolved.

### Errors

No new CUDA build has been performed locally. The initial persistent host
fixture cap error was corrected before this combined protocol. The earlier
V12 plan and source remain unchanged; supersession is not a failed run.

## 2026-10-30T17:00UTC — v13 launch wiring correction

### What was done
Removed a duplicate full-cache compile before the launcher enters the CUDA
source directory. The intended compile remains after validation/build setup.
Shell syntax and administrative pin checks pass; no v13 GPU was launched.

### Decisions made
Frozen protocol, analyzer, arms, inputs and bounds remain unchanged. This
corrects launch wiring only, before affected data. PR2983 records CUDA pending.

### Open threads
Collect and delete active v11 before v13 CUDA validation and execution.

### Errors
Static syntax alone missed a valid shell command in the wrong working
directory. Launch review caught it before deployment. T957 is cancelled and
cannot reopen; scoped correction is owned by T961.

## 2026-10-30T17:28UTC — separate 63-bit arithmetic preparation

### What was done
Prepared p<2^63 and full-u64 hash support under T966; retainedr<2^63.
Overflow-safe hash operations, widened Hasse radicand, modular affine formulas
and a direct terminal prefix offset remove identified width hazards. Host
fixtures pass; new native Rust/device boundary tests await GPU execution.

### Decisions made
HAC14.32/14.33 reviewed before changes; see FIELD63.md. Shared Montgomery
operations are unchanged; their documented bound widens with explicit argument
and differential fixtures. V11/v13 source pins and frozen designs are untouched.
No new frontier protocol is frozen until the current cache panel is collected.

### Open threads
V11 final62-bit target remains running; collect/delete, then execute pinnedv13.
Only afterward validate this width extension and freeze any larger probe.

### Errors
A guarded source-edit script stopped on a changed text match after earlier
replacements; inspected the partial edit and corrected the interval comparison.
A candidate hash-fixture integer was composite; the host fixture selected and
checked6917529027641081737 before it was recorded in the pending Rust test.
No GPU result is claimed for this extension.

## 2026-10-30T17:38UTC — current-main arithmetic reconciliation

### What was done
Merged main3c13bc691a87 into the pushed width-extension branch. Resolved three
shared Rust conflicts by retaining main's cached field context, small inverse
table, incremental inverse-hash enumeration and locally reused primality facts.
Composed the new full-width bounds and overflow-safe addmod with those paths;
updated the two private arithmetic fixtures for the added Curve fields.

### Decisions made
Preserve both source histories; no rebase or discarded optimization. V11/v13
remain pinned. Full Rust/GPU tests are pending on the next appropriate worker.

### Open threads
Await v11 final62 result, then collect/delete and execute pinnedv13. PR2985
requires checks on the reconciled head and subsequent GPU validation.

### Errors
Main advanced in the same arithmetic functions and changed Curve internals;
the automatic merge correctly stopped. Conflict resolution was scoped to
T967 after a clear ownership check; syntax/format checks follow.

The initial whole-merge whitespace check also scanned historical SVG/raw
artifacts imported from main and reported existing trailing spaces. Those
immutable artifacts are preserved; the corrected check is scoped to this
change's source and handoff files.

## 2026-10-30T17:54UTC — v11 complete through62bits

### What was done
Collected12of12verifiedrecoveries, including both61and62-bit targets.
Archive hash matched; controller/worker0 and podjees49ew9dkfd7deletion verified.
Uploaded public receipts, retained credential onlyforauthorizednextv13, then
removedv11privatefiles. Packaged raw hashes, certificate audit and export.

### Decisions made
Native exactplan audit plus8mutationcontrols passes. Schema/flow/filehashes
pass. Preserve5canonicalrectangularcoveragefindingsinCHECKER-NOTE.md.
ClocksremainL0; 62islargestcompleteprescribedrung, nothardwaremaximum.
Push raw before report. Nextv13pins e9e87ffb8380; widthPR2985is separate.

### Open threads
Render/inspectv11report and catalog copies, verifycommittedmetrics, runv13
GPUcorrectness/sanitizers and boundedpanel, then evaluate63-bitfollow-up.
Fresh RunPod catalog2.49/hour; noold/newtaskpodpresent in readinesslist.

### Errors
No v11 execution failed. Canonicalchecker assumes rectangulararms×sizes and
retains5findingsagainstthefrozenstagedplan; no full canonicalpassclaimed.

## 2026-10-30T17:59UTC — v11 report and v13 launch

### What was done
Rendered the separate two-page v11 report after raw commit 099dff2edd81 and
visually inspected both pages. Added versioned PDF/SVG/PNG catalog copies and
hashes. Preserved artifact-bot migration of the raw validation log.
Launched frozen v13 on pod 7tugqyoxyvnikm at 17:57:20 UTC; controller 25269
owns collection and deletion. One RTX PRO 6000, EUR-IS-2, USD 2.49/hour.

### Decisions made
V13 remains pinned to e9e87ffb8380, separate from the newer 63-bit source.
Its correctness and sanitizer preflight must finish before measurements.
Reports preserve L0 timing limits and five canonical coverage findings.

### Open threads
Collect, audit, publish and delete v13; then validate the separate width
extension and freeze any larger probe. Recheck PR2985 at its final head.

### Errors
Poppler reported a tagged-PDF metadata warning; both pages rendered and were
visually inspected successfully. No v13 result is claimed during preflight.

## 2026-10-30T19:06UTC — separate 63-bit probe prepared

### What was done
Reconciled v13 pod7tugqyoxyvnikm and its existing controller process; no duplicate
launch. Eight52-bit controls and both61-bit targets have completed;62-bit data
is still running on pinnede9e87ffb8380. PR2985 merged at5c0dd7ee432f. Its head
later showed four failed CI checks; merged status is not a green-CI claim.
Upstream2c8f16e2eca1 carries the owner's binary_semaev_chain_sat delimiter fix.
Prepared separate native launcher/ladder/analyzer and protocol-v14 for52/63bits.

### Decisions made
Read FIELD63.md, existing validation/fixture code, and HAC14.32/14.33/14.36
https://cacr.uwaterloo.ca/hac/about/chap14.pdf before the new plan. Use the fixed
63-bit modulus rule precprime(2^63-2^34) to leave full-u64 hash and order room.
Preserve all four52-bit controls, then two63-bit full-cache/fallback targets;
7200s per63-bit solve,18000s worker,18300s controller. Same25%free-memory
reserve and row/work caps. Launch only after v13 archive/deletion. Complete
Rust/host/CUDA arithmetic and all query sanitizers before new data. No root
crypto build locally, no cross-version speed claim, no global maximum claim.

### Open threads
V13 archive/certificates/cache-admission details, raw publication and report.
V14 GPU/Rust validation and actual recovery are unexecuted; protocol preparation
is not evidence of63-bit success. Preserve scalar-corruption rejection and
complete-rung stopping. Update exact-current-head CI on the follow-up PR.

### Errors
Initial protocol validation caught the required draft freeze block omitted by
metadata copying; restored before freeze/data. A stray nonexistent helper path
returned127 after successful native-instrument compilation; no run occurred.

## 2026-10-30T19:12UTC — optional negative GPU prefilter

### What was done
Added opt-in blocked Bloom prefilter before exact relation lookup. Every left
point key is inserted using atomic OR after sort completion; only definite
misses bypass the existing exact comparison. Added bounded word sizing,
phase-aware storage accounting, TABLE filter_bytes, host fixtures and CUDA
fixtures including a deliberately saturated filter. Existing deterministic
witness, infinity, empty-plan, late-hit and reprepare tests remain enabled.

### Decisions made
Read the primary GPU Bloom paper, NVIDIA cuCollections interface/policies and
Vigna SplitMix source before implementation; links and argument are in
NEGATIVE-FILTER.md. Keep frozenv13/v14 source pins unchanged. This candidate
requires a separately frozen panel and GPU checks; no measured gain claimed.
Filter storage is charged before right-cache admission; any fallback must be
reported. Preserve exact certificate replay and corrupt-scalar rejection.

### Open threads
CUDA build and sanitizer validation, full-pipeline matched controls, and any
larger probe. V13 first62-bit target still runs; v14 prepared but not launched.
No automatic default change or claim of fastest external implementation.

### Errors
None in the standalone filter fixture or host syntax check. GPU checks have
not run; pending validation is not a pass.

## 2026-10-30T19:30UTC — first-rung stopping audit

### What was done
Fixed the post-run checker's regression controls to accept a complete first
rung containing a recorded limit. Previously its own truncation test assumed
that first rung was successful, causing the audit to panic on a valid stop.
Added a check rejecting any later execution after that stopping boundary and
made the verification-label corruption invalid for successful and failed rows.

### Decisions made
This is an audit correction; no frozen protocol, measurement, solver or active
GPU source changes. Native checker compiles with warnings denied and passes
retained v11 and v9 records plus an explicitly synthetic, temporary first-rung
limit fixture. Do not publish the synthetic fixture as experiment data.

### Open threads
V13 has11verified rows; final62-bit target runs on pod7tugqyoxyvnikm. V15 pins
23dc8a61ca92 and remains unexecuted. PR2991 merged after13success/5skip checks.
After collection, publish raw evidence before the fresh report and inspect all
pages. The updated audit includes one positive stop and nine negative controls
when the first rung succeeds; a valid first-rung stop is itself accepted.

### Errors
The old regression-control assumption was the error; exact plan validation
already recognized first-rung stops. No recorded data was changed.

## 2026-10-30T19:45UTC — current usage documentation

### What was done
Replaced the stale opening statement that PR creation and all device validation
were pending with the published v11 result and exact source boundary. Dated the
original source-only handoff, updated the implemented field/hash domain and
2^32 service cap, and added the dense-CUB service build target used by v11.

### Decisions made
Keep current source support separate from measured scale: both62-bit targets
are verified, while63-bit and negative-filter GPU validation are pending.
Link the reports, implementation arguments and frozen v15 plan. Preserve the
GPU service versus CPU ecbench plug-in distinction and the root-build constraint.
These are documentation corrections, not new performance evidence.

### Open threads
V13 final62-bit target remains active. Collect and publish its raw/report,
then execute the separately frozen v15 probe after verified GPU deletion.
Refresh this overview again with the actual new validation outcomes.

### Errors
The original introductory source-only status and row-cap/domain descriptions
had become stale as later PRs landed. Historical entries remain preserved.

## 2026-10-30 20:12 UTC — v13 complete, v15 launched

### What was done
Collected v13 with all 12 prescribed recoveries, including both 62-bit targets.
Raw results and source/instrument hashes are in `results-v13`; every certificate
passed fresh CPU replay and scalar+1 rejection. Archive SHA256 is
`f00ea3e1ef9d32ebbf2063cb43b99939a42a19f31085c0f1ccb1d32d8e083338`.
Pod `7tugqyoxyvnikm` was deleted, with a fresh complete provider list confirming
absence. Schema/file hashes/native strict coverage pass; five canonical
nonrectangular-coverage findings are retained in CHECKER-NOTE.md.

### Decisions made
Preserve diagnostic L0 timing scope and every raw row. Both 62-bit cells actually
cached all right rows; admission estimates are not peak VRAM. V15 launched only
after v13 collection/deletion, pinned to `23dc8a61ca928e47237f8fb5a3a8d87228649628`
and frozen protocol `8e079c9e143ddfcb00dcf66a5d5002d22e9ce7722291e14b4f28292c8eee65ad`.

### Open threads
Publish v13 inspected report after raw commit/push. V15 pod `ndiwvrvehbf4hy`
started 20:10:04 UTC on one RTX PRO6000 at 2.49/hour; full preflight precedes
matched 52-bit filter controls and two 63-bit targets. No v15 result yet.
The full paper GPU tensor backend and matched-rho/external comparisons remain open.

### Errors
No v13 process failure. Canonical checker assumes rectangular arms by sizes,
so its five findings remain explicit; no full checker-pass claim is made.

## 2026-10-30 20:14 UTC — v13 report and v15 infrastructure failure

### What was done
Published v13 raw commit `84d5159e3635` before rendering the two-page report at
`research/explicit_relations_e2e_20261030/reports/run-20261030T1757Z`.
Both PDF pages were visually inspected; versioned catalog copies retain hashes.
V15 stopped with bootstrap exit128 before compilation: partial-clone lazy
checkout could not fetch Git blob `52a224ca9141900ae3779a85a56f3e810251d6d3`
(`.artifacts.json`) because the public GitHub fetch requested authentication.

### Decisions made
Preserve the failed archive and delete the pod; do not label this a recovery
panel or alter v15's one-attempt contract. Prepare a new frozen infrastructure
attempt using the same measured source and unchanged instruments, with an
offline-rehearsed source object pack instead of runtime GitHub lazy fetches.

### Open threads
Publish v15 failure custody and v16 freeze before another GPU launch. The
63-bit arithmetic/filter remain unvalidated on GPU; no new recovery data exists.

### Errors
V15 pod `ndiwvrvehbf4hy` deleted with verified absence; controller1, bootstrap128.
Failure archive SHA256 `ce7b1a34a7059c30331e4a9f9e3b34473c101be74d51c603cd2fd7db0ebe6554`.
Typst CLI was absent; the existing Python Typst binding rendered the report.
Poppler emitted a tagged-metadata warning but rendered both pages successfully.

## 2026-10-30 20:18 UTC — source delivery repaired

### What was done
V15's zero-trial failure is preserved in `failed-launch-v15`, with its inspected
one-page report at `reports/run-20261030T2010Z`. V16 is frozen at hash
`fe424abdd265c0ded0c191edf1c36810d86a87163502a1497c2da43ffece378f`.
Its source archive retains exact original commit `23dc8a61ca928e47237f8fb5a3a8d87228649628`;
all 1030 materialized files match original Git blob IDs. S3 readback is byte-identical.

### Decisions made
Use the verified sparse checkout archive and unchanged run-filter instruments.
Keep v15's one-attempt contract closed. The owner clarified reporting style:
lead with completed component work; state concrete dependencies once where
needed, while retaining exact checks, measured costs and failures.

### Open threads
Launch v16 after checkpoint, run all preflight checks, then six matched 52-bit
controls and two 63-bit targets. Update this handoff with actual outcomes.

### Errors
An administrative catalog-copy script initially used the wrong JSON key;
corrected to `files`, completed all v13 copies, and verified every catalog hash.
The first v16 freeze call omitted the required draft freeze object; it was
rejected before data, then corrected and successfully frozen.

## 2026-10-30 20:21 UTC — cache documentation reconciled

### What was done
Updated PERSISTENT-QUERY.md and FULL-RIGHT-CACHE.md from pending-validation
status to the completed v13 checks, exact cache rows and report links.
V16 source extraction passed and CUDA compilation is underway on pod
`omb2jpo4ntoq5g`; controller PID1044437 owns collection and deletion.

### Decisions made
Keep the frozen source and active run unchanged; these edits document completed
component validation. Timings and memory admission estimates retain their labels.

### Open threads
V16 device preflight and recovery outcomes; current-head PR2996 checks.

### Errors
Artifact bot moved the large v13 validation log to its hash-addressed store;
its commit was fast-forwarded into this branch without changing measured data.

## 2026-10-30 20:27 UTC — twelve-slot implementation prepared

### What was done
Extended the native reduction/oracle/certificate bounds and GPU service plan
arrays to12slots, keeping ordinary defaults and row/work/column caps unchanged.
Added9/10/12slot device boundaries,13slot rejection, CPU service all-target
checks and public known-answer Rust certificate/wire fixtures. Host C++ syntax
and expanded all-target service checks pass; Rust/CUDA checks await the next
separately pinned build. V16 remains on its original source and has begun its
52-bit recovery controls after passing preflight.

### Decisions made
Read the supplied paper Section3.2 Lemmas9–10 and Section3.4 Figure2 before
choosing this extension. More slots can reduce total base columns but change
coverage and tuple work. ARITY.md records the concrete condition
L >= max(r^(1/m),10m); automatic thresholds preserve it. Smaller explicit
thresholds require empirical coverage accounting in a separate frozen panel.

### Open threads
Measure m10/m12 configurations only after correctness validation and a frozen
comparison. Continue existing v16 filter/63-bit panel without modification.

### Errors
No host check failure. New Rust and device tests are not yet executed.

## 2026-10-30 20:31 UTC — arity panel frozen

### What was done
Prepared/froze v17 at `d0c604c4d92ac0350f935547fe62247083698924c9c9e80eb68ceddbd39cc967`.
The native instruments compile with warnings denied; shell and schema checks
pass. ARITY-PANEL.md specifies all12cells, controls, bounds and threshold scope.

### Decisions made
Use one identical filtered service binary across m8/m10/m12 configurations.
Compare recovered scalars across arities and certificate bytes only within the
same arity. Upper-rung m10/m12 solves cap3600s each, preserving all four prescribed
cells within the18000s worker envelope. Launch only after current v16 closes.

### Open threads
V16 first63-bit target is running. V17 has no pod/data; source/package preparation
and fresh preflight remain. New twelve-slot tests still need Rust/device execution.

### Errors
No new failure. The current shared-host GPU is active; no duplicate task pod.

## 2026-10-30T20:56Z — V16 interruption and V17 collection repair

### What was done
V16 console recorded six52-bit completions and63-101-u at1148.950seconds. Provider GET then returned404; DELETE also404, and a fresh complete list had no task pods. S3 had only source-checkout.tgz, no results archive. Preserved console bytes/hash, receipts and controller exit22 under interrupted-v16. PR2996 merged at83720a2463b2; current CI checked separately.

### Decisions made
Do not promote console-only cells into audited recovery rows: certificates were not collected. V17 source/targets/caps remain frozen. A dated collection addendum adds60-second partial artifact copies and tolerates failed status reads without deleting a potentially healthy worker. No process restore. All clocks remain L0. Reviewed RunPod APIv2 overview and repository artifact/checkpoint rules.

### Open threads
Run V17 on one GPU after fresh absence/capacity/checkpoint gate; collect final or partial artifacts, independently verify actual certificates, then publish outcomes.

### Errors
Cause of provider disappearance unknown; no result archive exists. V16 is closed as interrupted, not silently retried.

## 2026-10-30T21:00Z — V17 allocation refused

### What was done
Created one V17 allocation request after checkpoint gate and fresh inventory/capacity checks. RunPod returned HTTP402 Payment Required: account balance too low. Complete reconciliation found zero matching pods; controller91. Zero experimental trials started. Recorded sanitized response, request template and source/protocol/collection hashes in launch-refused-v17.

### Decisions made
No blind POST retry and no payment changes. User notified that top-up is required. No new run report required for a refused allocation with no run directory; V16 report rendered and its one page inspected.

### Open threads
After account funding, reconcile live inventory and prepare a fresh bounded attempt with new signed artifact URLs. Twelve-slot device/Rust validation and the full arity panel remain queued.

### Errors
HTTP402 is an external funding dependency; no GPU allocated.

## 2026-10-30T21:10Z — Requested RTX5090 panel

### What was done
Owner reported account replenished and requested5090or4090. Selected32GB RTX5090 secure catalog1.19/hour; cap1.20. NVIDIA compute-capability table confirms5090 andPRO6000Blackwell both12.0, so kernels/build remain sm_120. Added explicitGPU selector, one-device and capability guards; five guardfixtures pass. Both controllerfault-injectionpaths pass. Protocolv18 frozen26446177ac80fae9ef001edef438e3a104c6bd73ae40bb0cf2a283463841038e; sourcee2cdde6d6f1fd5c316d2e8c5b5dfc24723e80eff. OriginalGit sourcearchive b597dd5d362d2fcd8327496e64ed5408d859a8f9c4bb9763a2ed902d1b5cef41 uploaded and freshGETbyteequal.

### Decisions made
Same52/63-bit targets,m8/m10/m12 and limits as unexecutedV17; lowerVRAM admission is part of this distinct hardwarepanel. No cross-run timing comparison. Prefer5090;4090wouldrequireseparatesm_89build/contract. Partial artifact copies are not restorecheckpoints.

### Open threads
Launch oneV18aftercheckpointgate; fullnewarityRust/device/sanitizerchecks precede data. PR2996 now has an evidence-audit shard8failure and other auditshards pending; inspect actual failure logs whenavailable, do not call its CIgreen. PR2997 carries current work.

### Errors
FirstV18draftliteratureentry failedschema; corrected before freeze,zero findings. Sourceindex-pack on sharedfilesystem failedbadfiledescriptor; local scratch rehearsal passed. Firstadministrativefaultfixture omittedrgfromPATH; corrected fixture passed; no provider call occurred in these tests.

## 2026-10-30T21:18Z — RTX4090 launched

### What was done
Owner-authorized4090fallback allocated oneNVIDIAGeForceRTX4090 pod25f10of8l9nl6i at21:17:08.633UTC,US-NC-1,0.89USD/hour. Sourcee8e1c819df8901100db0254a849271130e6cca90,archive95a046ed6b1a064109024976c775468e138ca96c045eebbe763f5c17bd7cd58d,protocolv19b58d0dfe47bc039a6160ea7912374d37ff30f595e2daea262490beef58ac5422. Original1149blobIDsverified andS3freshGETequal. Checkpointgatepassed20261030T211514Z-c7444548f1d1 beforeallocation.

### Decisions made
Use sm89 acrossvalidator and aritylauncher; fixedCUDA13.2image unchanged. Sixmodel/architectureguards pass andcontrollerhealthy/absentfaulttests pass. Preservealladmissionlimits/timeouts andsamearitycert/crossarityscalarchecks.

### Open threads
ControllerPID1255083,exec30852,admin/home/ec2-user/ecbench-e2e-20261030-v19. Do notrestart. Preflight/fullpanelpending. Partialuploads60s configured; verifyfirstactualupload. Collectfinalorpartialarchiveanddeleteownpod beforecompletion.

### Errors
V17balanceandV18capacityrefusals retained separately. PR2996auditshard8failedonunrelatedpencil_large_primecheckpointg9missinghost.json; no repairorclaimofallCIgreen.

### 2026-10-30 21:49 UTC — verified 4090 results and smaller-base preparation

What was done: V19 raw results committed e1db39a661f3; inspected two-page report
861a7fd27a89. Eight 52-bit recoveries replayed and four 63-bit memory/work limits
retained. Final archive verified; pod deleted and complete inventory empty.

Decisions: V20 uses m12 with three explicit thresholds, a bounded query-budget
API retaining default20, and combined full/partial-cache fallback. Cache
construction/storage stays charged. Native ladder/analyzer compilation and
shell syntax pass locally. New Rust/CUDA tests must pass on GPU before data.

Open threads: freeze V20, prepare original-Git source archive, run one4090,
collect and publish exact outcomes and PR. No task GPU active at this entry.

Errors: default system Python lacks the required jsonschema API; canonical
protocol validation passes using the existing tools venv. Initial draft lacked
the required draft-freeze block; corrected before freeze or data.

## V21 completion — 2026-10-30 16:12 PDT

### What was done
The smaller-base RTX4090 panel completed10/10 prescribed recoveries, including both63-bit prime-field targets. Fresh CPU replay and scalar+1 rejection passed for every cell. All29 Rust tests and27 query sanitizer checks passed before collection. The final archive was downloaded and SHA256 checked; the worker was deleted and provider absence verified. See `research/explicit_relations_e2e_20261030/results-v21` for exact rows, certificate audit and custody.

### Decisions made
Preserve all L0 clocks as descriptive; the completed63-bit rung is the scope of the result. At52 bits smaller bases needed more queries and longer observed cold time. Publish the exact10-cell audit alongside the canonical checker's five rectangular-coverage findings.

### Open threads
Publish the separate inspected V21 report and follow-up PR. The next affine-query specialization is documented but not implemented or measured. PR3003 merged; compile and Clippy pass. No task GPU remains active.

### Errors
PR3003 evidence-audit shards1/8 fail because unrelated pencil-study terminal-g18/checkpoint-g9 session directories lack host.json. Preserve these failures without changing unrelated evidence. V20's earlier preflight compile failure and V19's memory/work limits remain recorded.
