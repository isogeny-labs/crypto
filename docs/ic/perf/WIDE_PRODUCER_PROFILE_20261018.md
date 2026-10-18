# Wide producer profile: `scan_state128` at n = 73

Task T03 of `RESEARCH_IC_LEADS_ASSESSMENT_20261018.md`. Class: measurement
(profile), not an improvement; no baseline moves. Purpose: tell T01 (parallel,
batch-inverted `build_index`) and T02 (blocked Bloom filter, slot prefetch,
single canonicalisation per hit) where the instructions of the wide relation
producer `examples/koblitz_orbit_dlp_fast.rs` go.

## Host and tool

- CPU: Intel Xeon (model string `Intel(R) Xeon(R) Processor @ 2.80GHz`), KVM
  guest, 4 cores, 1 thread per core, L1d 128 KiB (4x), L2 4 MiB (4x), L3 33 MiB;
  AVX2, BMI2, PCLMULQDQ present.
- **No hardware PMU** (`/sys/bus/event_source/devices` lists only breakpoint,
  msr, power, software, tracepoint, uprobe). `perf stat` cycles/IPC are
  unavailable, so this report counts instructions (Ir) only and states no IPC.
- valgrind 3.22.0 (callgrind), rustc 1.97.0, source commit `e0fc6f2bf`.
- Build: `CARGO_TARGET_DIR=/home/user/wt/t03/target CARGO_PROFILE_RELEASE_DEBUG=1
  cargo build --release --example koblitz_orbit_dlp_fast` (release profile of
  the repository plus line tables).

## Instrumentation (disclosed)

The example has no state cap, and its rank stage ends only when every column is
fixed (about 5.7e7 probes per relation at K = 600, 18,398 s of rank wall in
the landed n = 73 run; `docs/ic/RESEARCH_TRANSFER_AWARE_IC_20261010.md`). To
stop inside the scan I applied a 14-line local patch, kept as
`WIDE_PRODUCER_PROFILE_COUNTER_PATCH_20261018.patch` and **not committed to the
example**: `KIC_PROF_MAX_STATES=<S>` makes `extract128` print
`PROF states probes hits index_states` to stderr and exit after S scanned
states, plus a relaxed atomic counter on the table-hit path. The patched and
unpatched hot loops are identical apart from one atomic add on a path that was
taken 0 times. The counters are per `extract128` call (`probes`), which equals
the whole run here because no relation was found within the cap.

## Commands

Fixture: constructed base, n = 73, a = 1, K = 40 columns, empty target file
(rank stage only), seed 1. n = 73 is the target size. K is smaller than the
landed K = 600 so that the callgrind run fits; the per-probe work does not
depend on K, but the table is smaller (see caveats).

```
B=target/release/examples/koblitz_orbit_dlp_fast
# native, 3 runs
KIC_PROF_MAX_STATES=40000 $B construct:73:1:40 empty.txt 1 out.jsonl
# native, setup only
KIC_PROF_MAX_STATES=1     $B construct:73:1:40 empty.txt 1 out.jsonl
# callgrind
KIC_PROF_MAX_STATES=40000 valgrind --tool=callgrind --dump-instr=yes \
  --collect-jumps=yes --callgrind-out-file=callgrind.out \
  $B construct:73:1:40 empty.txt 1 out.jsonl
callgrind_annotate --inclusive=no  callgrind.out > ..._EXCLUSIVE_20261018.txt
callgrind_annotate --inclusive=yes callgrind.out > ..._INCLUSIVE_20261018.txt
```

Raw `callgrind.out` (637 KB) is not committed. The two annotate outputs are
committed whole (188 KB each, under the 200 KB limit):
`WIDE_PRODUCER_CALLGRIND_EXCLUSIVE_20261018.txt`,
`WIDE_PRODUCER_CALLGRIND_INCLUSIVE_20261018.txt`.

## Native run (same arguments)

| quantity | value |
|:--|--:|
| index states | 116,800 |
| states scanned (cap) | 40,000 |
| probes (partner roots canonicalised and looked up) | 11,680,000 (292 per state) |
| table hits | 0 |
| relations | 0 (no relation expected: about 1e9 probes per relation at K = 40) |
| wall, full run (3 runs) | 14.59 s, 13.34 s, 13.52 s |
| wall, setup only (cap = 1; 2 runs) | 8.71 s, 8.94 s |
| scan wall (mean difference) | about 5.0 s, i.e. 125 us per state, 0.43 us per probe, 2.3 Mprobe/s |

Wall time is a context figure only (shared 4-vCPU guest, no isolation tier, one
thread); the measured quantity is Ir.

## Callgrind totals

Total 124,355,947,799 Ir (callgrind wall 9 min 40 s, 42x native).

| phase | Ir | share of run |
|:--|--:|--:|
| `KoblitzCurve::new` -> `factorise` (setup) | 83,564,941,344 | 67.20% |
| `extract128` (the scan; includes `scan_state128` inlined) | 39,519,040,551 | 31.78% |
| `build_index128` | 1,168,293,154 | 0.94% |
| everything else (base construction, 2048-point self-check) | about 104,000,000 | 0.08% |

Per unit of the scan (39,519,040,551 Ir): **3,383.6 Ir per probe**, **987,976
Ir per state** (292 probes per state).

`scan_state128` is `#[inline(always)]` into `extract128`, so callgrind reports
the scan under `extract128`; its callees that stay out of line are
`RootBatch128::solve`, `Gf2_128::mul`/`mul_fold`/`clmul_u64`, `Gf2_128::sqr`.

## Where the scan's instructions go

Shares of the scan inclusive cost (39.52 G Ir). Line-level attribution comes
from the annotated source of `koblitz_orbit_dlp_fast.rs` (exclusive).

| component | Ir | share of scan | Ir per probe |
|:--|--:|--:|--:|
| canonicalisation: the 72-step rotate/compare loop of `NormalBasis128::canonical` (lines `current = ((current << 1) ...` 11.17 G and `if current < best` 5.15 G) | 16,314,624,000 | **41.3%** | 1,396.8 |
| `RootBatch128::solve` (S3 root solve for the batch), inclusive | 15,439,640,000 | **39.1%** | 1,321.9 |
|  of which field multiplication (64.52 M calls, 159 Ir each, `mul_fold` + `clmul_u64`) | 10,258,680,000 | 26.0% | 878.3 |
|  of which squaring (80 k calls) | 15,360,000 | 0.04% | 1.3 |
|  of which solve self (batch inversion chain control, quadratic/trace logic, loop) | 5,165,600,000 | 13.1% | 442.3 |
| `Linear::apply` byte-table XOR (`to_normal` per partner, `to_poly` per candidate) | 5,421,793,525 | 13.7% | 464.2 |
| Bloom hash mixing and bit test (listed lines only) | about 431,000,000 | about 1.1% | about 37 |
| remainder: loop control, `index.shifted` loads, table probe on Bloom pass, `StateStore::get` | about 1,910,000,000 | about 4.8% | about 164 |

Notes. There is no separately named inversion function: the batch inversion is
inlined into `solve`, so its multiplications are inside the 10.26 G of `mul`
and its control in solve self. Multiplications per probe: 64.52 M / 11.68 M =
5.5. Field squarings are negligible (the Frobenius structure is a rotation in
the normal basis). Table-hit lift (`lift128`) cost is not measured: hits = 0 in
this run (see caveats). The remainder row is the arithmetic residual and
includes the table lookup taken after the Bloom filter passes; the Bloom pass
probability is not counted here.

## Top 25 functions, exclusive (callgrind rows, as reported)

Callgrind reports a function once per source file its inlined code comes from,
so one function appears in several rows (for example `extract128` in
`uint_macros.rs`, `range.rs`, `cmp.rs`). `factorise` and `__umodti3` rows are
the setup, not the scan.

| # | Ir | % | file:function |
|--:|--:|--:|:--|
| 1 | 32,486,885,927 | 26.12 | `src/cryptanalysis/koblitz_index_calculus.rs:koblitz_index_calculus::factorise` |
| 2 | 19,421,534,548 | 15.62 | `examples/koblitz_orbit_dlp_fast.rs:koblitz_orbit_dlp_fast::wide::extract128` |
| 3 | 18,563,934,796 | 14.93 | `lib/core/src/num/uint_macros.rs:koblitz_index_calculus::factorise` |
| 4 | 13,922,968,104 | 11.20 | `lib/compiler-builtins/compiler-builtins/src/int/specialized_div_rem/mod.rs:__umodti3` |
| 5 | 9,281,979,076 | 7.46 | `lib/compiler-builtins/compiler-builtins/src/int/specialized_div_rem/asymmetric.rs:__umodti3` |
| 6 | 9,281,979,076 | 7.46 | `lib/compiler-builtins/compiler-builtins/src/macros.rs:__umodti3` |
| 7 | 6,367,515,372 | 5.12 | `src/cryptanalysis/semaev_decomp.rs:<semaev_decomp::Gf2_128>::mul_fold` |
| 8 | 3,667,360,000 | 2.95 | `examples/koblitz_orbit_dlp_fast.rs:<koblitz_orbit_dlp_fast::wide::RootBatch128>::solve` |
| 9 | 1,729,061,825 | 1.39 | `lib/core/src/num/uint_macros.rs:koblitz_orbit_dlp_fast::wide::extract128` |
| 10 | 1,595,377,248 | 1.28 | `src/cryptanalysis/semaev_decomp.rs:semaev_decomp::clmul_u64` |
| 11 | 1,393,284,207 | 1.12 | `src/cryptanalysis/semaev_decomp.rs:<semaev_decomp::Gf2_128>::mul` |
| 12 | 1,050,793,605 | 0.84 | `lib/core/src/iter/range.rs:koblitz_orbit_dlp_fast::wide::extract128` |
| 13 | 860,753,462 | 0.69 | `lib/core/src/cmp.rs:koblitz_orbit_dlp_fast::wide::extract128` |
| 14 | 797,688,624 | 0.64 | `lib/core/src/../../stdarch/crates/core_arch/src/simd.rs:semaev_decomp::clmul_u64` |
| 15 | 667,823,542 | 0.54 | `examples/koblitz_orbit_dlp_fast.rs:koblitz_orbit_dlp_fast::wide::build_index128_with_symmetry` |
| 16 | 516,680,000 | 0.42 | `lib/core/src/slice/iter/macros.rs:<koblitz_orbit_dlp_fast::wide::RootBatch128>::solve` |
| 17 | 508,080,000 | 0.41 | `lib/core/src/slice/iter/macros.rs:koblitz_orbit_dlp_fast::wide::extract128` |
| 18 | 398,844,312 | 0.32 | `lib/core/src/../../stdarch/crates/core_arch/src/x86/pclmulqdq.rs:semaev_decomp::clmul_u64` |
| 19 | 187,680,332 | 0.15 | `./string/../sysdeps/x86_64/multiarch/memset-vec-unaligned-erms.S:__memset_avx2_unaligned_erms` |
| 20 | 187,640,000 | 0.15 | `lib/alloc/src/vec/mod.rs:<koblitz_orbit_dlp_fast::wide::RootBatch128>::solve` |
| 21 | 128,880,000 | 0.10 | `lib/core/src/ptr/mod.rs:<koblitz_orbit_dlp_fast::wide::RootBatch128>::solve` |
| 22 | 119,240,000 | 0.10 | `lib/core/src/ptr/non_null.rs:<koblitz_orbit_dlp_fast::wide::RootBatch128>::solve` |

## Top 25 functions, inclusive (rows after the 12 entry/runtime wrappers)

The first 12 inclusive rows are `_start`, `main`, `lang_start`, ... `wide::run`
at about 100% each and are omitted.

| # | Ir | % | file:function |
|--:|--:|--:|:--|
| 1 | 83,564,941,344 | 67.20 | `src/cryptanalysis/koblitz_index_calculus.rs:<koblitz_index_calculus::KoblitzCurve>::new` |
| 2 | 83,564,941,333 | 67.20 | `src/cryptanalysis/koblitz_index_calculus.rs:<koblitz_index_calculus::KoblitzCurve>::subfield` |
| 3 | 83,537,709,075 | 67.18 | `src/cryptanalysis/koblitz_index_calculus.rs:koblitz_index_calculus::factorise` |
| 4 | 51,050,821,957 | 41.05 | `lib/core/src/num/uint_macros.rs:koblitz_index_calculus::factorise` |
| 5 | 39,519,040,551 | 31.78 | `examples/koblitz_orbit_dlp_fast.rs:koblitz_orbit_dlp_fast::wide::extract128` |
| 6 | 32,486,928,041 | 26.12 | `lib/compiler-builtins/compiler-builtins/src/macros.rs:__umodti3` |
| 7 | 15,439,640,000 | 12.42 | `examples/koblitz_orbit_dlp_fast.rs:<koblitz_orbit_dlp_fast::wide::RootBatch128>::solve` |
| 8 | 13,922,968,104 | 11.20 | `lib/compiler-builtins/compiler-builtins/src/int/specialized_div_rem/mod.rs:__umodti3` |
| 9 | 10,546,178,753 | 8.48 | `src/cryptanalysis/semaev_decomp.rs:<semaev_decomp::Gf2_128>::mul` |
| 10 | 9,281,979,076 | 7.46 | `lib/compiler-builtins/compiler-builtins/src/int/specialized_div_rem/asymmetric.rs:__umodti3` |
| 11 | 9,152,894,546 | 7.36 | `src/cryptanalysis/semaev_decomp.rs:<semaev_decomp::Gf2_128>::mul_fold` |
| 12 | 2,791,910,184 | 2.25 | `lib/core/src/../../stdarch/crates/core_arch/src/simd.rs:semaev_decomp::clmul_u64` |
| 13 | 1,729,061,825 | 1.39 | `lib/core/src/num/uint_macros.rs:koblitz_orbit_dlp_fast::wide::extract128` |

## Observations

1. **Setup is 67% of this run, and it is not the probe loop.**
   `KoblitzCurve::new` -> `subfield` -> `factorise` is trial division
   (`while d * d <= v`) in u128 using `__umodti3`: 83.5 G Ir, 2.32 G
   `__umodti3` calls, about 8.7 s native of the 13.8 s. It is a fixed cost
   per process at n = 73 (independent of K), outside the T01 and T02 scopes
   (it is in `src/cryptanalysis/koblitz_index_calculus.rs`). At the landed
   K = 600 and 18,398 s of rank wall it is negligible, but it dominates every
   short run and every test of the wide path.
2. **The scan costs 3,384 Ir per probe, and 41% of that is canonicalisation.**
   The default `canonical_selected` is the full 72-iteration rotate and compare
   loop (about 19.4 Ir per iteration), run on every partner root before the
   Bloom filter is consulted. The sparse zero-gap variant exists in the code
   (`KIC_SPARSE_CANONICAL=1`) but was not profiled here.
3. **The root solve is 39%, of which two thirds is field multiplication**
   (5.5 mul per probe at 159 Ir each; PCLMUL fold with 6 clmul per mul).
4. **The two linear maps are 13.7%** (byte tables, 10 lookups per 73-bit word).
5. **The Bloom hash and bit test are about 1% of the scan.** The filter is
   cheap in instructions; what it saves is DRAM misses, which callgrind
   (instruction counts, no cache model run) cannot show.
6. `build_index128` is 0.94% at K = 40 (1.17 G Ir, about 0.12 s). Its cost grows with
   the pair count, about K^2: arithmetic scaling to K = 600 is about 225x,
   about 263 G Ir, an extrapolation, not measured.

## What T01 and T02 should attack

T02 (probe path), ordered by measured share of the scan:

1. **Canonicalise less than once per probe: up to 41.3% of the scan
   (1,397 Ir per probe).** Filter on a rotation-invariant key first (or
   compute the minimal rotation with the cheaper zero-gap method, or keep the
   rotation cost off the miss path) so that the full canonical form is
   computed only for probes that pass the filter; "single canonicalisation per
   hit" as specified would remove essentially all of this row, because hits
   are rarer than relations (one per 5.7e7 probes at K = 600). Expected upper bound of the saving is the full
   row, minus whatever invariant key replaces it.
2. **The Bloom filter and slot prefetch (blocked Bloom, prefetch): at most
   about 1.1% of Ir in the hash mixing plus the table-probe share of a 4.8%
   remainder.** Instructions are not where this gain is; it can only be
   judged on wall time with cache behaviour, which this host cannot give
   (no PMU) and callgrind did not model. Do not expect an Ir reduction from
   it; measure it by wall time against a same-binary control.
3. Not in T02 but in the same probe loop and measured here: root-solve
   multiplications at 26.0% of the scan and `Linear::apply` at 13.7%.

T01 (`build_index`): 0.94% of this K = 40 run, so it matters only through the
K^2 scaling (extrapolated about 225x at K = 600); profile it at the K it will
be used at before reading a gain from it. One more measured fact for T01: the
index here has 116,800 states (730,000 at K = 100), and index setup at K = 100
was 10.3 s of wall for the process against 8.8 s at K = 40, so a K = 100 build
is on the order of 1.5 s of wall and K = 600 is not measured.

Outside both: the `factorise` setup (item 1 under Observations) is the largest
single cost of any short run; a better factoriser (the order is a 73-bit-class integer) would remove 67% of the instructions of this run. That is a
different file and a separate task.

## Caveats

- K = 40 not K = 600; Ir per probe is K-independent in the loop, but the cache
  behaviour, the Bloom pass rate and the table size are not. Instruction
  counts here say nothing about DRAM stalls, which are the target of
  prefetch and blocked-Bloom work.
- hits = 0: the lift and hit-path code (`lift128`, `unpack`) is not profiled.
  At the landed shape a relation costs about 5.7e7 probes, so the lift should stay
  negligible, but that is arithmetic, not measured here.
- One callgrind run, one fixture, one seed; callgrind Ir is deterministic for
  a fixed binary and input, so no repeat spread is reported. The native wall
  times are 3 and 2 runs on an unisolated shared guest.
- Callgrind inlining attribution is by source line; the split of `Linear::apply`
  between `to_normal` and `to_poly` is not separated because both inline the
  same line.
- Class of this deliverable: measurement. It changes no baseline and claims no
  speedup or rho gap.

## Files

- `docs/ic/perf/WIDE_PRODUCER_PROFILE_20261018.md` (this report)
- `docs/ic/perf/WIDE_PRODUCER_CALLGRIND_EXCLUSIVE_20261018.txt`
- `docs/ic/perf/WIDE_PRODUCER_CALLGRIND_INCLUSIVE_20261018.txt`
- `docs/ic/perf/WIDE_PRODUCER_PROFILE_COUNTER_PATCH_20261018.patch`
