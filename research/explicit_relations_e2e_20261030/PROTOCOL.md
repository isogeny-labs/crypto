# Complete recovery on one RTX PRO 6000

The user requested the largest feasible complete recovery on one RTX PRO
6000. This ladder measures a bounded operational frontier, not a largest
possible field, a security claim, or a certified timing advantage.

The initial ladder uses frozen `protocol-v3.json`, hash
`36c8f17ea57cc552d84b076521f0b344fe2d2aa65cf41aa649228b0f110142b2`.
Version 1 failed before compilation because two wrapper commands were joined.
Version 2 passed the recovery/device preflight, then failed while compiling
the native chart analyzer, before any ladder data. Both archives and deletion
receipts remain under `failed-launch-v1/` and `failed-launch-v2/`.
Version 3 repairs the analyzer literal and compiles instruments with Rust
edition 2021 and warnings denied. The ladder code and inputs are unchanged.

The subsequent indexed-lookup panel is frozen separately in `protocol-v4.json`,
hash `29ff437233af158c060614d5e660a7b925d01a4edb6826bb0d945fe5cae333a6`.
It was designed after v3 completed through 52 bits and its first 56-bit cell
timed out; v3 continues unchanged. The index panel uses48/52/56/58/60-bit
fixtures, the same two seeds and limits. On48/52bits the retained binary
search (b) is compared with indexed search (u) and its identical repeat (x),
with reversed order by seed. Higher rungs use u/x. At most24executions run;
the first incomplete rung stops the ladder after all its cells are retained.
Every successful cross-arm certificate must be byte-identical.

The index stores lower-bound offsets for high x bits of the sorted full-point
table, at most65537u64 entries. Empty buckets, duplicate points, x=0,
last-field coordinates and infinity are explicitly tested. Infinity uses the
full-table lookup; finite buckets preserve the lowest original tuple id.
`ER_QUERY_BINARY_REFERENCE` disables the index; both measured services log
row counts and admission bytes with `ER_TABLE_DIAGNOSTICS`. Admission bytes
are a conservative allocation bound, not measured peak device usage.
[NVIDIA's lower-bound contract](https://github.com/NVIDIA/cccl/blob/main/thrust/thrust/binary_search.h)
and the existing full-key comparator informed this conventional indexing
change. No new algorithmic exponent or external-tool superiority is claimed.

The next separate panel is frozen as `protocol-v5.json`, hash
`554c7aff798beddcdbeade7b0107575a7c66059fbac6e030bd982a8a4e55b86f`.
It reuses two contiguous partial sums of right tuples across targets. Each
partial table must fit `chunk_rows`; otherwise the original query path is
used. At most2*chunk_rows*sizeof(Point) extra bytes are charged to admission.
The mixed-radix right id and lowest matching left id are unchanged, including
infinity and duplicate sums. `ER_QUERY_UNCACHED_REFERENCE` is the ablation.
This follows conventional reuse of invariant work, with explicit memory
accounting and full-workload testing, as discussed in
[NVIDIA's best-practices guide](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html).

V5 compares b/u/x at52bits for both seeds, then runs u on both targets at
56/58/60bits: at most12executions. The explicit row cap rises to2^30 and
work cap to2^31 for every arm; chunk remains2^20 and the GPU admission still
reserves sort space and leaves25%free. Solve caps are180s at52bits,300s at56,
and600s at58/60; worker4500s/controller4800s. The same complete-rung stopping
rule applies. The larger caps probe actual GPU admission beyond v4's planning
cap; they are not used to make cross-run timing comparisons. A/A timing
controls apply at52bits only, while every larger result still requires both
public targets, fresh replay and corrupt-scalar rejection. V4 remains pinned
and unchanged. No new panel runs concurrently with it.

## Implementation and mathematical checks

The standalone Rust adapter now accepts a nonzero point killed by the
declared prime group order. Deterministic u64 primality checks, the curve
equation, nonsingularity and the exact integer Hasse interval certify that
the full group order equals the declared order, provided it is the unique
multiple in that interval. Ambiguous tiny cases retain exhaustive counting;
unproved large cases are rejected. This follows the same argument used in
`crates/hashed-ecdl-tensor/src/curve.rs`, rather than trusting supplied orders.
Square roots use Tonelli–Shanks instead of an O(p) lookup, with exact parity
handling; intermediate products use u128 when needed.

The reduction still collects N+1 relations from independent slot-specific
hash bases and uses generic meet-in-the-middle decomposition on CUDA.
Elimination stores its factors and reconstructs the first dependency once
by triangular back-substitution, instead of updating N+1 provenance entries
at every pivot. An independent eager implementation checks exact dependency
coefficients in unit tests. Certificate replay accumulates the sparse
slot entries without constructing a dense matrix. The original tiny-curve
constructor and default 512-column limit are preserved. Larger column/work
limits are explicit CLI arguments, not silent overrides.

The supplied paper's §1.2/§3 separates factor-base construction, relation
collection, linear algebra and extraction. Its conditional asymptotic tensor
result does not give practical-curve speed evidence. This experiment does
not replace MITM with a tensor backend. Primary sources checked before work:
[Hasse and point-order background](https://www.hyperelliptic.org/tanja/teaching/isogeny-school21/csidh-sidh-week-3.pdf),
the repository's arbitrary-precision curve/arithmetic implementation, and
[PARI ellcard documentation](https://pari.math.u-bordeaux.fr/dochtml/html-stable/Elliptic_curves.html).

## Inputs, controls and stopping

Existing registry curves cover fields of 20,24,28,32,36,40 bits. A fixed PARI
rule generates 44,48,52,56-bit public fixtures before solve timing: at
`precprime(2^bits)`, choose the first nonsingular `a=-3,b=1..8192` curve with
prime full cardinality, then its first liftable x in 1..1000. The driver
independently certifies every order using the supplied nonzero generator.
Fixture-generation output and errors are retained. PARI receives no solver
results, and no curve is selected for observed recovery speed.

For each field, seeds 101 and 202 select public Q by the first even-y lift
from x=1009*seed. **No target scalar is generated or given to recovery.**
The same seed controls the solver's hash/trial RNG. Each seed runs GPU arms
u/x with identical binaries and inputs; their order reverses for seed 202.
CPU arm h runs on the 20/24-bit cases and must produce byte-identical
certificates. Every successful result also gets fresh-process CPU replay;
a copy with its scalar incremented modulo r must be rejected with exit 1.
Tiny known-answer and invalid-input tests run before the ladder.

All cases use eight slots, tau=2*ceil(r^(1/8))+2, max_rows=536870912,
chunk_rows=1048576, max_columns=16384, max_work=1073741824 and max_trials=16.
These are admission limits, not up-front allocation requests. GPU admission
still reserves sort workspace and leaves a quarter of free memory unused.
Independent inverse-hash replay has an explicit 2^24 candidate limit.

There are at most 44 executions. Each solve gets 300 seconds, each independent
replay 60 seconds, fixture generation 180 seconds, the worker 2400 seconds and
the controller 2700 seconds. Complete the prescribed cells of the current
rung, then stop higher rungs if any current cell times out or hits a limit.
Retain partial output and mark higher rungs unattempted. An incorrect answer,
accepted corrupt certificate, sanitizer error or unexpected process failure
invalidates the run rather than being reported as a scale frontier.

## Accounting and interpretation

`solver_seconds` starts before native public-target construction and ends
after the recovery driver exits: process startup, curve checks, GPU service
startup, all base trials, relation search, elimination, in-process replay,
serialization, flushing and child teardown are included. `wall_seconds`
continues through independent replay, the corruption check and certificate
comparison. Additional replay time is recorded separately. Raw driver logs
give base, target/check, oracle, elimination and in-process replay stages,
including discarded trials. Field-corpus construction and build/validation
costs are outside solve timing and retained in bootstrap logs.

Shared RunPod hardware earns L0 only. All wall_valid flags are false; no
statistical significance, fastest-tool or sub-rho claim is made. The native
frozen analyzer includes every attempted execution and draws incomplete
rows in red. Two fixed targets do not establish a general success rate.
The reported frontier means all prescribed cells at that rung completed
under these exact limits; a timeout is not a mathematical impossibility.

## Exact compact-row panel (v6)

Frozen `protocol-v6.json` has hash
`2ab44ea0799d563bac3c6a8c1f647b9e637cba1b625e4d68da7ab2c2feb9c450`.
It was designed while v5's larger cases were still running, after both 56-bit
recoveries and the matched 52-bit cache loss. The cache remains opt-in.

On one validated odd-prime curve, a canonical x coordinate and y parity
identify a point exactly, as specified by [SEC 1 v2, section 2.3.3](https://www.secg.org/sec1-v2.pdf).
The internal key is `2*(x+1)+(y&1)`, with zero reserved for infinity.
Under the service's `p < 2^62` bound, finite keys and index endpoints fit u64.
A key plus the original tuple ID occupies 16 bytes instead of the full row's
32 bytes. This representation is valid only for points on the same curve;
it is neither a fingerprint nor a comparator for arbitrary coordinate pairs.
Duplicate keys sort by original ID, preserving the existing witness rule.

`ER_COMPACT_ROWS` enables this representation; `make cuda-compact` exposes it
without changing the ordinary build. Both arms disable the partial-sum cache.
The existing full-coordinate index is the matched baseline at 52 bits, with
identical candidate A/A repeats and both seeds. Candidate-only probes cover
56/58/60 bits, for at most 12 rows. All curve/target rules remain fixed.
Every success requires fresh complete replay and corrupt-scalar rejection.

Every arm uses max_rows=2^31, max_work=2^32 and chunk_rows=2^20. Admission
reserves three buffers of the actual row type plus other allocations and
leaves 25% of free GPU memory unused. Solve limits are 180/300/900/1200 seconds
for 52/56/58/60 bits; worker 6900 seconds and controller 7200 seconds. These
are new explicit bounds, not a cross-host speed comparison. Finish the first
incomplete rung and leave higher rungs unattempted. No task GPU runs in
parallel: v5 must be archived and deleted before this panel starts.

New differential checks exhaust every valid point on a small curve against
full-coordinate lookup, with duplicate IDs, both signs, infinity, missing
keys and empty index buckets. Endpoint arithmetic is checked separately.
All existing query/reprepare/recovery tests run for full, cached and compact
variants; full and compact queries each receive memcheck/racecheck/synccheck.
Host syntax and native instrument metadata checks passed locally. CUDA
compilation and this panel's device results remain pending until executed.

### V6 failure and v7 pre-data correction

V6 pod `oazm8b2kvq0wvz` passed all three service builds, the Rust/reduction
checks and full/compact query sanitizers, then aborted on its first solve:
the service header still enforced max_rows <= 2^30. The frozen contract
requested 2^31. No ladder row was recorded; the failed attempt, complete
archive and verified deletion remain in `failed-launch-v6/`.

Frozen v7 hash
`b34f5721ad1b074e85e556fb2d8c7a92acaf2d42f6dc22bf03c385a7c6b7dd18`
authorizes a new run after making that explicit service limit consistent.
The planner and device use checked u64 cardinalities; memory admission and
sort reserve remain in force, and the cap does not allocate memory itself.
New tests cover the upper cap, zero, overflow and oversized chunks. The new
`run-compact-v7.sh` also sends the actual 2^31-cap service header to both
measured binaries before any ladder data. Inputs, arms, seeds, solver limits,
ladder and analyzer are unchanged. V6 remains a failed run with zero rows.

## Separate post-run coverage audit

The frozen analyzers verify unique cells and complete observed rungs, but
could accept a file truncated immediately after a successful earlier rung.
They remain unchanged. `check-frontier.rs` is a separate post-run audit that
requires either the entire prescribed plan or an explicit timeout/limit in
the final recorded rung, with no later execution. It also checks the exact
per-version cell set, seeds, replicates, flags, numeric fields and outcomes.

The unchanged v3/v4/v5 rows pass. Deliberate duplicate, missing-cell,
truncated-prefix, seed, replicate, warmup, arm and verification corruptions
are rejected. The source/input hashes and audit output are retained in
`frontier-audit-v3-v5.*`. This strengthens validation after collection; it
neither changes frozen statistics nor resolves the canonical checker's
rectangular-coverage findings. Run the same audit on v7 after collection:

```sh
rustc --edition=2021 -D warnings research/explicit_relations_e2e_20261030/check-frontier.rs -o /tmp/check-frontier
/tmp/check-frontier 7 research/explicit_relations_e2e_20261030/results-v7/rows.csv
```

## Wider prefix index, version 8

Version 8 retains 16-byte exact rows in both arms and changes only the
index ceiling from 2^16 to 2^24 buckets at the matched 52-bit rung. The
index consumes at most 128 MiB plus one u64 offset, charged to memory
admission, and its complete construction remains in cold setup. Larger
buckets still use the same exact comparator and duplicate-ID ordering.
The optional `cuda-wide-index` target enables `ER_WIDE_INDEX`; defaults
remain unchanged pending the recorded panel. NVIDIA's best-practices
guide section 3.2 discusses the throughput cost of random memory access;
this conventional indexing change narrows the random binary-search range.

The maximum-size offset array is checked element-by-element against CPU
lower bounds, with sparse keys, empty buckets, duplicates, infinity and
the 2^62 endpoint, for full and compact rows. The current merged CPU
arithmetic is identical across arms. This is not a cross-version timing
comparison. Fixtures, seeds, row/work caps and stopping rules are retained;
60-bit solve time is 1800 seconds, worker 9000, controller 9300. One GPU
may launch only after v7 archive collection and verified deletion.
