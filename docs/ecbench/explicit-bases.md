# Explicit x-coordinate factor bases and GPU relation finding

Source implementation, 2026-10-29. Compilation, executable tests, device
validation and performance measurements are pending. This change stops before
opening a PR, as requested. The EKS image must eventually be built from the
merged `crypto` `main` commit.

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

## Source-only validation and builder handoff

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
