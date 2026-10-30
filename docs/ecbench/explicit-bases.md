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
