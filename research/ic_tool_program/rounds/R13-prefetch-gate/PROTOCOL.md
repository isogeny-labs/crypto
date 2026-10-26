# R13: size gate for the admitted-run prefetch pass

**Status:** declared before any R13 timing. This is a Table A, cold,
single-target Koblitz index-calculus tournament round. The comparator is the
v8 code at `261482646ee89b332dca69e2f296b0774c25d2c2`; the candidate
changes only the R12 run-prefetch pass in
`src/cryptanalysis/koblitz_index_calculus.rs`. Both arms are compiled with
the same pinned `Cargo.lock`, toolchain and release settings on the run host.
Their binary SHA-256 values and the candidate commit must be recorded in the
round's host manifest before the first timed process.

## Prior and fixed candidate

R12's retained panel measured cold base/candidate ratios of 0.971, 0.981,
and 0.956 at `log₂ r = 27.8, 36.6, 38.0`: its extra pass cost time there.
The three largest measured tables benefited, at 1.055, 1.055, and 1.063.
The corresponding stored-pair counts in R12's `runs.tar.xz` are 2,664,
150,024, 237,120, 3,707,880, 3,881,728, and 6,265,920. R12's other
small tables have at most 265,680 stored pairs. The candidate executes the
same prefetch pass only when `PairSumTable::len() >= 1_000_000`.
This threshold is fixed from R12 before R13's holdouts or timing; it is
target-independent. No other algorithm, recipe, target law or accounting
field changes.

## Frozen panel

- **Primary sizes:** `K_0/GF(2^37)`, `K_1/GF(2^47)`, and
  `K_0/GF(2^57)` from suite v1, identified by their ICV1 slugs in
  `suite/v1/`. These are the three R12 sizes where the pass cost time.
- **Suite rows:** M1's two rows at all eleven sizes, plus all eight
  suite rows at each primary size. One previously unseen public target
  per `ic price --single-target` process; IC and matched rho use the
  identical point, with a verified scalar in each arm.
- **Holdouts:** recipe seeds 242–245, two public hash-to-curve targets
  per seed, `T175`–`T182` at each primary size. The native
  `icprog holdouts` output and `SHA256SUMS` are committed before timing.
  These seeds and targets are outside earlier rounds' holdouts.
- **Order:** pin candidate against v0's 90 retained outputs, five-round
  A/A using byte-identical baseline binaries, then five ABAB paired
  rounds on suite rows and fresh holdouts. Extend a set to ten rounds
  only when its declared 95% interval half-width exceeds 3%.
- **Reference and unit:** same-process signed-Frobenius rho at the
  canonical batched policy (`KIC_RHO_CANON` unset, as rule v8), priced in
  the same process-local batched affine-addition unit as IC. Report
  setup-inclusive cold IC and rho,
  target-dependent online intervals separately, the §5a 0/0.1/1 GAE
  band, and the equal-advice rho boundary separately.
- **Host:** an x86_64 host meeting `REFERENCE_CLASS` in `icprog`, one
  isolated CPU, `RAYON_NUM_THREADS=1`, with a recorded CPU model, feature
  list, memory/disk envelope, no concurrent benchmark on the reserved
  CPU, and an isolation receipt for every timed process. The planned
  host is the already-running `m8i-flex.8xlarge` spot instance. Its
  `avx512vbmi2` absence means its result is a distinct CPU subclass
  from R12's Emerald Rapids host. Both arms are rebuilt and paired on
  this host; historical wall times are not pooled with R13's.

## Decision and limits

### Pre-timing pin compatibility addendum (2026-10-26)

The first untimed R13 pin against R01 v0 returned `held=false` for all 90
rows. Every row's sole differing top-level output was `counts`. A field-by-field
check of all 90 raw pairs showed that the only differences were two additive
`row_ops` counters at `counts.pass.logs.linear_algebra.filter.row_ops` and
`counts.pass.logs.linear_algebra.row_ops`, introduced after the v0 archive.
All counts shared with v0, statuses, recovered scalars, rho counts, and
verification flags matched. The native pin comparator now omits one of these
two counters from the *comparison* only when v0 lacks that exact field; if v0
has it, equality is still required. Raw candidate records retain both. The
failed strict pin is preserved as `pin-strict-v0.json` beside the corrected
pin and in the S3 checkpoint. This is a historical schema compatibility
correction made before timed work, not a change to the candidate, panel,
acceptance threshold, or measurement protocol. The corrected native pin must
still pass all 90 rows before A/A starts.

### Pre-timing SMT isolation addendum (2026-10-26)

The first A/A launch produced no timed process. `isolated_bench` refused the
first row five times because CPU 2 shares a physical core with CPU 18 on the
Xeon 6975P-C host; its refusal log is retained in `runs/aa/refusals.log`.
The native runner now accepts `ICPROG_BENCH_CPUS` for timed isolation calls.
For this host R13 sets it to `2,18`, reserving both logical siblings of one
physical core. `RAYON_NUM_THREADS=1` and the one-process-per-target protocol
are unchanged. The isolation tool's per-process receipt must record that CPU
set and a clean run. The untimed pin and both `ic` binaries are unchanged.
No timed A/A, suite, or holdout record preceded this addendum. A future host
uses its actual `thread_siblings_list`; a missing or changed topology stops
the run until its CPU set and manifest are recorded.

### Pre-timing dedicated-host addendum (2026-10-26)

The shared `m8i-flex.8xlarge` host was unavailable for R13 timing: another
validation job held `/tmp/crypto-bench.lock`, and its CPU work continued.
The corrected A/A process was stopped while waiting for that lock; it
produced zero timed price files. Its refusal and wait receipts remain
separate from the measurements below.

The round moves before any timed sample to a dedicated
`m8i-flex.4xlarge` Spot instance (`i-06ef80569878fa0d5`, us-east-2a).
It reports the same Intel Xeon 6975P-C model and required AVX512F,
PCLMULQDQ, VPCLMULQDQ and GFNI features, with 16 logical CPUs and 61 GiB
of memory. CPU 2's SMT sibling is CPU 10, so set
`ICPROG_BENCH_CPUS=2,10` on this host. Rebuild both `ic` arms from the frozen
base and candidate source with the same Rust toolchain and `Cargo.lock`,
and repeat the native host-class, 24-holdout and 90-row pin checks before
A/A. Run all five-round suite and holdout comparisons on this one host;
never pool its times with the shared host. Preserve its public IP, binary
digests, isolation receipts, and an S3 copy of raw runs. The six-hour host
budget starts at its 2026-10-26 20:43:31 UTC launch; terminate it by
2026-10-27 02:43:31 UTC. The threshold, targets, rho rule, acceptance
criterion and conditional extension remain fixed.

The native `icprog analyse r13` computes geometric mean paired cold
base/candidate ratios and 95% intervals by size. Accept the candidate
for this CPU subclass only if every primary size's suite and holdout
interval lies above 1.01, all pinned outputs and scalar certificates
match, A/A stays inside its recorded band, and no M1 size regresses
beyond its A/A band. A timeout, incomplete row, failed scalar check,
changed host class, or failed isolation check is retained and stops
promotion. A rejected or inconclusive result is reported with its raw
records. A qualifying result on this host does not automatically move
Table A's Emerald Rapids rows: reproduce the candidate on that host
class and pass the leaderboard `VALIDATION.json` gate first.

The R13 run tree, command log, source/binary digests, independent replay,
validator, matched-rho reference and visual report (SVG, PNG and PDF)
will be filed beside this protocol. New performance results are also
copied into `benchmarks/` after the validation gate. Every outcome names
the next candidate or discriminating control without changing this
protocol or R12's frozen evidence.
