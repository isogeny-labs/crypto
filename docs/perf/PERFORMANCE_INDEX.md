# The performance index

One number for "how much faster is this revision's code than that one's",
computed the same way every time, refused when the two revisions compute
different answers, and reported with its noise.

```bash
# perfindex is a Rust example (examples/perfindex); build it once.
cargo build --release --example perfindex
PI=target/release/examples/perfindex
# 1. Build both sides with the SAME harness (this tree's examples/perfbench).
$PI build --reference <base-rev> --out /tmp/pi-base
$PI build --current              --out /tmp/pi-cand
# 2. Wall time, paired and interleaved, one thread (the headline).
$PI compare --base /tmp/pi-base/perfbench --cand /tmp/pi-cand/perfbench \
    --rounds 5 --threads 1 --taskset 2 --out /tmp/pi-1t
# 3. The same at every core, and the deterministic instruction count.
$PI compare ... --threads 4 --out /tmp/pi-4t
$PI instr --base ... --cand ... --out /tmp/pi-ir
# 4. What the host and the rule can and cannot show, before trusting a verdict.
$PI null --bin /tmp/pi-base/perfbench --out /tmp/pi-null --rounds 5 --taskset 2
# 5. Hand a reviewer something that replays.
$PI bundle /tmp/pi-1t --dest benchmarks/<date>-<name>/perfbench
```

`compare` and `instr` refuse, instead of reporting a number, when the two
binaries were built from different harness sources, when they are the same
binary, when either build had uncommitted changes under `src/`, or when the
two sides do not have the same kernels (`--allow-missing` and
`--allow-provenance-mismatch` run anyway and record the problem; the result
is then marked not qualified).  A candidate that drops its slowest kernel
would otherwise raise the index.

## Progress over time: the history

The index above compares two revisions.  The **history** records every
measured revision so progress can be read off a chart instead of a chain
of PR descriptions: `scripts/perf/perfhistory.py` writes one snapshot per
commit (callgrind `Ir` and a wall-time reading for every quick kernel, plus
the frozen `ecbench exec` children in `docs/perf/ecbench-children.json`) and
renders `HISTORY.md`, `history.svg` and a machine-readable `history.json`
from them.  The snapshots and the rendered files live on the
[`perf-history`](https://github.com/aburan28/crypto/tree/perf-history/docs/perf)
branch, appended by `.github/workflows/perf-history.yml` on every push to
`main`.  Pull requests that touch measured code only test the history
machinery and compile both tools; the full instruction comparison of a branch
runs through a `check_only` manual dispatch.

Three pictures are rendered next to `HISTORY.md`:

* `history.svg`: the overall index and one line per area since the first
  snapshot.
* `history-panels.svg`: one small panel per area, each kernel a thin line
  and the area's index a thick one, so a single kernel that moved is visible
  without the others drowning it; a kernel flagged as a regression in the
  latest snapshot carries a red ring.
* `factor-bases.svg`: one bar per index-calculus solve in the
  `ic_factor_base` area (below), grouped by curve, its length the solve `Ir`
  on a log scale, its colour the symmetry its factor base is closed under,
  labelled with the base's size, next to the matched rho reference on the
  same curve as a grey dashed line.  The distance from a bar to that line is
  the distance still to go.

```bash
git fetch origin perf-history
git show origin/perf-history:docs/perf/HISTORY.md | less      # the tables
git show origin/perf-history:docs/perf/history.json > /tmp/h.json   # per-kernel series
# locally, for the working tree:
cargo build --release --example perfbench --bin ecbench
python3 scripts/perf/perfhistory.py check --binary target/release/examples/perfbench \
    --ecbench target/release/ecbench --history <checkout of perf-history>/docs/perf/history
```

The chained index takes no credit across a fingerprint change: a kernel
that computes something else is re-based, not sped up.  `Ir` is charted
because it is deterministic for one binary; across two builds of different
sources the untouched kernels still move by up to a few per cent (the
release profile's codegen units are partitioned afresh and cross-module
inlining follows), so one step under about 5 % on one kernel is build
noise and a trend or an area index is the signal.  The wall column is one
reading on the recording host and is informational.  Recording a snapshot
takes about fifteen minutes of callgrind on four cores.

### Regressions are surfaced, not enforced

Neither a snapshot on `main` nor a pull-request check fails on a slower
kernel (only a `check_only` dispatch with `PERF_HISTORY_FAIL_ON_REGRESSION`
set can).  After each snapshot on `main`,
`perfhistory.py regressions` compares every kernel with its own recent
range: the previous 5 snapshots with the same fingerprint.  A kernel is
flagged when its `Ir` rises more than 8 % above the highest of those (and
reported as improved when it falls more than 8 % below the lowest).  Against
the range rather than the previous snapshot, because several kernels flip
between two codegen levels from build to build (`ct_scalar_mul` by about
7 %): a plain 5 % step rule fires on 38 of the 74 recorded steps, the range
rule on 3, each of them a real change.  A flag appears as a GitHub warning
on the run, a table in its job summary, a callout at the top of
`HISTORY.md` and one comment on the merged pull request that introduced it.

```bash
python3 scripts/perf/perfhistory.py regressions --history <perf-history>/docs/perf/history
```

### Factor bases

The `ic_factor_base` area (weight 0: it never moves the overall index)
tracks frozen index-calculus solves whose factor bases differ, with the
matched rho reference on each curve.  Every row records the factor-base
spec, its symmetry class as `docs/ecbench/FACTOR_BASE_PROTOCOL.md` names it,
and the size the run's own report gives (columns, signed points):

| curve | factor bases | symmetry |
|---|---|---|
| Koblitz a = 1, n = 17 | `koblitz-orbit`, `compact-orbit-scan` | signed Frobenius |
| | `binary-subspace`, `koblitz-standard-subspace` | none |
| prime, 20-bit, generic | `prime-abscissa`, `glv-orbit` | negation (a generic curve has no larger automorphism) |
| prime, j = 0, p = 2345743 | `glv-orbit` `group=auto` | GLV (order 6) |
| | `glv-orbit` `group=negation` | negation |

The j = 0 curve and its arms are frozen in
`docs/perf/ic-factor-base-j0.spec.json` (the E2 curve of
`research/isogeny_conductor_gap_round2_20261011`, with an oracle that still
runs on prime curves).  At this size the order-6 fold builds three times the
signed points of the negation-only base for the same 32 columns (192
against 64) and costs about 4 % more `Ir` in all (34.6 M against 33.2 M at
`ca00c151`); on the larger j = 0 curve p = 22793761 it was about 9× cheaper
(25.9 M against 234.3 M `Ir`, measured once with an older `ecbench` build,
and too slow to repeat on every push).  Every index-calculus row is still
tens of times above its rho reference at these sizes: the chart shows that
distance, not a pass or fail.

### Separate F4/F5 groups

The [separate algorithm reports](https://github.com/aburan28/crypto/blob/perf-history/docs/perf/groebner/REPORT.md)
retain F4 and F5 as separate series for the shared Boolean matrix-step and
solver workloads, with instruction and diagnostic-runtime graphs. Complete-call
allocation and F5 criterion allocation have separate groups. Every group has a
PDF, and a combined PDF, SVGs, editable sources and native plot data are saved
together. The history workflow regenerates them when it records a revision.

The [frozen initial rendering](../../research/f4_f5_history_20261012/REPORT.md)
retains seven imported historical snapshots and the native allocation audit.
The [group registry](groebner-groups.json) and
[allocation history index](f4-f5-allocation-history.json) define the inputs;
[the native tool](../../tools/groebner-history/README.md) validates the raw
records before rendering. Historical CI timings are diagnostic, and the
recorded profiler scope has incomplete worker coverage. Lines break when the
fingerprint, host, compiler, harness or profiler changes. These displays make
no new runtime-speedup or complete-DLP claim.

The [native F5 hashing follow-up](../../research/f4_f5_hash_20261012/REPORT.md)
adds its own fixed eleven-system instruction-history and peak-live-allocation
groups, with SVGs, group PDFs and a combined PDF. Five paired A/B/A′ rounds
record 3.71% fewer complete-F5 instructions and 45.14% fewer criterion
instructions; F4 is neutral. Its merged-worker profiler contract, native
controls and live-byte intervals stay separate from earlier CI diagnostics.
Runtime and process RSS remain unmeasured.

The [native F4 indexing/F5 construction follow-up](../../research/f4_f5_fused_readback_20261012/REPORT.md)
records 5.28% fewer complete-F4 instructions and 11.02% fewer complete-F5
instructions on the same eleven-system suite. It adds paired instruction-history,
allocation-request and peak-live-byte graphs, inspected PDFs and both rejected
attempts. No final memory metric regresses. Three standalone criterion cases
use more instructions: the original all-arm gate fails, while the explicitly
clarified complete-call gate passes. The report retains that gate-scope change
and every case verdict. Historical and current matched pairs are disconnected
across intervening unrelated merges; no runtime, RSS or solver-speed claim is
made.

The [native F5 packing enhancement](../../research/f4_f5_dense_packing_20261012/REPORT.md)
measures 3.95% fewer complete-F5 instructions and 2.03% fewer requested
allocation bytes against the preceding candidate. F4 is neutral, exact outputs
and accounting match, and no primary or paired memory metric regresses.
Its gate was fixed for this F5-specific change before measurement. One standalone
criterion case uses 15.45% more instructions and is retained as a diagnostic.
The report adds a third matched history segment, per-case instruction intervals,
requested-byte plots, group PDFs and benchmark-index copies. These instruction
results do not establish elapsed-time, RSS or full-solver gains.

## What it measures, and what it does not

`examples/perfbench` is a registry of **kernels**: each runs a fixed,
seeded input through an existing public entry point of the library and
returns a 64-bit **fingerprint** of everything it computed (reduced
matrices, bases, models, recovered logarithms, point coordinates).  The
kernels are grouped into eight **areas**:

| area | covers |
|:--|:--|
| `gf2_la` | dense/sparse linear algebra over F₂ (Four Russians, echelon forms) |
| `bool_gb` | Gröbner bases over F₂: F4/F5, Macaulay/XL, crossbred, FES |
| `fp_gb` | Gröbner bases over F_p and extension fields: F4, F5/signature, tower F4 |
| `sat` | CDCL with native XOR clauses; Semaev and WDSat encodings |
| `pdp` | point decomposition: summation polynomials and decomposition oracles |
| `field_ec` | F₂ᵐ, F_p, F₃ᵐ arithmetic; point addition; scalar multiplication; batch inversion; the isogeny-walk `GF(p)` |
| `relation` | relation collection, index-calculus pipelines, relation-matrix algebra |
| `dlp` | generic discrete logarithm: Pollard rho, BSGS, kangaroo, Pohlig–Hellman |

These are **engineering** measurements in the sense of `AGENTS.md` §3.
A faster kernel lowers the *time per counted operation*; it does not
change the operation count `S`, a degree of regularity, a relation
yield, or any exponent, and it is never by itself an end-to-end method
speedup (§8).  A change that alters what a kernel computes changes its
fingerprint and is refused here: that is an algorithmic change and is
measured under §8's rules, not this index's.

## The formula

For kernel `k`, paired round `r`, baseline `A`, candidate `B`, and `T` the
median wall time of one harness invocation (warm-up discarded, `S` timed
samples):

```
d(k,r)  = ln( T_A(k,r) / T_B(k,r) )                 paired log-ratio
s(k)    = exp( median_r d(k,r) )                    kernel speedup
A(a)    = exp( (1/|a|) · Σ_{k∈a} ln s(k) )          area index (geometric mean)
PI      = exp( Σ_a w(a) · ln A(a) ),  Σ_a w(a) = 1  performance index
```

* **Validity gate.** If any two runs of a kernel — baseline, candidate, or
  a repeat — return different fingerprints, the comparison is INVALID and
  no index is reported.  Every kernel also checks, inside one process, that
  every sample returns the same fingerprint.
* **Weights.** `docs/perf/weights.json`; equal by default, so each area
  counts once however many kernels it has.  A reweighting is a new index
  and is reported as such, never mixed with the old one.
* **Why a geometric mean.**  It is unit-free (a kernel measured in
  microseconds and one in seconds count alike), symmetric (a 2× gain and a
  2× loss cancel exactly), and it is the only mean for which "the index of
  B over A times the index of C over B is the index of C over A".
* **Pairing.** Each round runs `A`, `B` and a second baseline `A'` for
  every kernel, back to back, with the arm order rotated each round and
  the kernel order shuffled (seeded).  Drift in the host's load lands on
  both sides of a pair instead of on one side of the comparison.
* **Noise (A/A).** `n(k,r) = ln(T_A / T_A')` is the baseline against
  itself.  `σ(k) = 1.4826 · median_r |n(k,r)|` (a robust standard
  deviation).  A kernel's verdict is **faster** only when the 95% bootstrap
  interval of `median_r d(k,r)` excludes zero **and**
  `ln s(k) > max(2σ(k), ln 1.02)`; symmetrically **slower**; otherwise
  **neutral**.  A difference inside the A/A spread is not a result.
* **Uncertainty of PI.** Rounds are resampled jointly (a bootstrap over
  rounds, 2000 resamples), since all kernels in a round share the host's
  state at that time.
* **Threads.** The headline index is at `RAYON_NUM_THREADS=1`.  The
  many-thread index is reported beside it; a multi-core gain must not
  regress the single-thread index (§10).
* **Instruction counts.** `perfindex instr` runs each kernel once under
  callgrind, restricted to the timed region (`perfbench_measured_region`),
  and applies the same formula to instruction counts `Ir`.  It is
  deterministic and immune to host load, and it is blind to memory stalls,
  to instruction latency and to instruction width: valgrind has no
  AVX-512, so it measures the scalar/AVX2 dispatch paths, and a 64-bit
  divide counts as one instruction, so replacing a `%` by a few
  multiplications *raises* `Ir` while the time falls (the index-calculus
  trial division in `cryptanalysis`: `Ir` 0.82×, wall 2.57× faster).
  Instrumentation is switched on and off by client requests in the
  measured region, so work on every thread counts, including private
  thread pools.  Report it next to wall time, never instead of it.
  `--jobs N` counts N kernels at once (the count does not depend on load,
  so this is safe on a shared host) and `--repeat K` counts each kernel K
  times and reports, rather than averages, any kernel whose count differs
  between runs; `instr.json` keeps every count and the valgrind version.

## From the index to a workload

The index weights areas equally; a particular pipeline does not.  If a
workload spends fraction `f(a)` of its baseline time in area `a`
(Σ f = 1, from a profile of that workload), its projected speedup is
Amdahl's

```
speedup_workload = 1 / Σ_a ( f(a) / A(a) )
```

— a projection, labelled as one, until the workload itself is rerun on
both revisions.  Where a workload has its own frozen benchmark (the
`ic-e2e-benchmark` rungs, `examples/pdp_bench`, the frozen WDSat suite),
that benchmark's paired rerun is the measurement and this projection is
only a cross-check.

## Rules for kernels

1. **Public entry points only.**  A kernel calls code the research
   pipelines call, so the baseline revision builds the same harness and
   the speedup is the speedup of that code.  A kernel that needs a new
   API has no baseline and is added after the change, with its own
   baseline from then on.
2. **Fixed inputs, fingerprinted outputs.**  Seeded generators only;
   fingerprint everything that is returned (not a length or a count
   alone), in a canonical order.
3. **Stable IDs.**  `area/name`.  An ID is never reused for different
   work; changing a kernel's input or size means a new ID.
4. **Size.**  1–300 ms per run at one thread on a 2 GHz x86-64, so a
   comparison of every quick kernel stays under a few minutes a round.
   Slower cells are `Tier::Full`.
5. **One-time costs outside the timer.**  Input construction happens in
   `setup`; per-sample restoration of consumed inputs in `prepare`.
   Lazy statics and thread pools are warmed by the discarded warm-up runs.
6. **Fingerprint the answer, and know what else the fingerprint covers.**  A
   fingerprint that also mixes in counted units (multiply-adds, pairs
   reduced) fails when a unit legitimately depends on the thread count.  Two
   `fp_gb` tower kernels do: their basis, step and row counts are identical
   at 1, 2 and 4 threads and only `muladds` moves (by under 0.3%).  They are
   listed under `thread_dependent` in `docs/perf/fingerprints.json`, with the
   reason; the A/B gate compares builds at one thread count, where it is exact.

## What a run records, and how to check it

`compare` writes, besides `compare.json` and `compare.md`:

| file | holds |
|:--|:--|
| `plan.json` | the frozen run: seed and schedule hash, thresholds, the area weights as they were, and for both builds the revision, harness hash, binary hash, lockfile hash, and the host's frequency governor, SMT, ASLR, THP and CPU mask |
| `raw.jsonl` | every harness row exactly as printed, appended and synced as it arrives, with the machine's load average and PSI at each round's start and end |
| `cost.json` | what the measurement itself cost (below) |

A crash, a timeout or `--budget-seconds` therefore keeps what was measured;
`--resume` continues the same plan and refuses to if the schedule or either
binary differs.  `perfindex replay DIR` recomputes `compare.json` from
`plan.json` and `raw.jsonl` alone and reports MATCH or MISMATCH: it checks
that the reported numbers follow from the raw rows by the formula above, that
every planned invocation is present once, and that nothing was edited.  It
does not re-time anything, so it is a check of the statistics, not of the
host.  `perfindex bundle` replays first and refuses a run that does not
match, then copies the five files with `SHA256SUMS` and a `README.md` of the
commands, ready to commit under `benchmarks/`.

### Cost accounting

Each harness row (schema 2) carries, beside the wall times, the process's
CPU time per sample, its peak resident set after setup and at the end, the
involuntary and voluntary context switches and the page faults during the
timed samples, the thread count the rayon pool actually had, how many
warm-up runs there were, and whether the time budget cut sampling short of
the requested count.  `cost.json` adds what `perfindex` measured around
each invocation: the whole process's wall time (start-up, setup, warm-up and
samples), summed by kernel, area and round, the share that was overhead
rather than timed samples, and the largest consumers.  `--dry-run` prints the
number of invocations before anything runs; `--budget-seconds` stops before a
round that would end past the budget (the first round always runs, and the
result is marked not qualified).  Peak RSS is per kernel and covers setup;
it is the process high-water mark, not a model of allocation.

### Contended rounds

Wall time is evidence only from uncontended runs (`AGENTS.md` section 10).  A
paired round is **contended**, and by default left out of the medians and the
index (and counted in the report), when an arm in it shows any of:

* at one thread, process CPU time below 90% of wall time (`--min-cpu-ratio`):
  the process was descheduled;
* optionally, more than N involuntary context switches per second of sampling
  (`--max-invol-per-s N`, off by default: kernels with a private thread pool
  pinned to one CPU switch constantly while measuring correctly, and a
  threshold of 100 excluded 441 of the 490 invocations of the null run);
* at least two scheduler ticks (20 ms) of hypervisor *steal* amounting to more
  than 5% of the invocation (`--max-steal`), read from `/proc/stat`.

`--keep-contended` pools them anyway; the count is reported either way.  What
this cannot see: a noisy neighbour that costs cache or memory bandwidth
without taking CPU time, and frequency changes.  Steal is summed over every
CPU the process may use unless `--taskset` names the CPU, so an unpinned run
can exclude a round for steal on a CPU the benchmark never ran on; pin it
(`--taskset N`, or reserve the core with `tools/isolated_bench.py reserve`)
for an exact account.  A kernel with fewer than three usable rounds gets the
verdict `insufficient`.

### Verdicts, and what five rounds can show

The verdict rule above is unchanged, so earlier tables stay comparable.  Three
things are reported beside it because the rule is weaker than it reads at the
five rounds `AGENTS.md` asks for as a minimum:

* An exact two-sided **sign test** cannot give p below 0.0625 with five
  rounds, and the distribution-free confidence interval for the median is the
  sample range at 93.75% coverage.  Both are in `compare.json`
  (`sign_test_p`, `speedup_ci_exact`, `ci_exact_coverage`); a percentile
  bootstrap of five values looks narrower than that.  Use six or more rounds
  to claim p below 0.05.
* The **best-of ratio** (fastest sample of each arm, over the used rounds) is
  not moved by a few slow samples.  A "faster" or "slower" whose best-of ratio
  does not lie beyond the 2% floor on the same side is marked `†` (not
  corroborated) and counted in the warnings: it is a statement about
  interference, not about the code.
* With no A/A spread (`--no-aa`, or fewer than three A/A pairs) a kernel's
  verdict is `unqualified-faster` or `unqualified-slower`, never a result.

`perfindex null --bin BIN` compares BIN with a copy of itself that differs
by one trailing byte, so every `faster`/`slower` it reports is false.  It is
the empirical false-positive rate of the whole method on this host at this
round count and load, and it should be run (and its `null.json` kept) before a
verdict on that host is cited.

### How the false-positive rate was measured

`null` was run on all 136 quick kernels, 5 rounds, one thread, pinned to a
single CPU of this 4-vCPU shared-tenancy host (2040 invocations): **2 of 136
kernels (1.5%) received a faster/slower verdict between identical binaries**,
both marginal (0.96x and 0.98x) with sign-test p = 0.125; the index was 1.005x
(95% CI 0.985-1.019), and the median A/A noise was 4.6% per kernel.  So on this
host a per-kernel verdict inside a few percent is not evidence, while the index
is good to about +-2%.  The rule is not exact: with 136 independent kernels
about two false verdicts are expected even for a well-calibrated rule, which is
why the report prints `†` and the sign test and why the index, not a single
kernel, is the headline.  `null` over a subset is a smoke test of the pipeline,
not a measurement of this rate.

### The Python tool

`scripts/perf/perfindex.py` is the earlier implementation and is kept
unmodified because `scripts/perf/perfhistory.py` and its tests import it.  New
work should use the Rust tool, which has the same formula and thresholds (the
two were cross-checked on the 2040 recorded rows of the null run: the summaries agree, and a 1e-6 perturbation of any reported number is detected) and adds the
raw record, replay, contention classification, the resumable schedule and the
sparse-checkout build.  Its `build --ref` fails on recent revisions because the
sparse list it writes lacks the crate's path dependencies; `perfindex build
--reference` reads them from the manifests of the revision being built.

### Fingerprints

Beyond the A/B gate, `perfindex fingerprints --bin BIN --threads 1,2 --golden
docs/perf/fingerprints.json` runs every kernel once at each thread count and
checks (a) that the fingerprint does not depend on the thread count and (b)
that it equals the recorded one.  A kernel whose fingerprint moves computes
something else: it needs a new ID (rule 3 below), not an edit of the golden
file.

## Reporting a change

A performance change carries, in its commit or PR: the host manifest
(`compare.json` records it), the A/A spread, the per-kernel table with
fingerprints, the single- and many-thread indices, the instruction-count
index, and the changes that were tried and rejected with their numbers
(§10).  It is classified **engineering** (§3).

The raw evidence belongs in the repository, not on the author's machine: a
reviewer cannot weigh a ratio whose rows they cannot open.  Run `bundle` on
each `compare` and commit the result under `benchmarks/`.  State whether
`compare.json` says `qualified`; if it does not, say which warning, and treat
every ratio as an unverified observation.  Run `null` on the same host first
and quote its false-verdict count, so a reader knows what a "slower" row
there is worth.

## Build settings tried

| setting | result | decision |
|:--|:--|:--|
| `lto = "fat"`, `codegen-units = 1` (2026-09-26, crypto, bool_gb + gf2_la kernels, 3 paired rounds) | no kernel reliably faster; `gf2_la/rref_random_2048` 0.77× (slower); area indices 1.005× / 0.769×; build 3 min | rejected: the default profile stays |
