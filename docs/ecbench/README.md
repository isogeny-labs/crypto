# ecbench: one harness for every ECDLP method

The experimental `ic.trace_line_2lp` method compares actual embedded subfield
lines and trace-selected multiplicative shifts on polynomial-basis binary
instances with N=4d. It runs the exact bounded four-summand MITM/2LP collector,
retains every accepted relation, and exposes construction, lookup, LP and LA
work as uncharged counters. Its GAE is a lower bound, not complete C1 cost.
It does not implement an algebraic Gaudry–Diem trial-cost measurement. Parameters,
prospective controls, replay and remaining gates are in the
[trace-line study](../../research/trace_c1_2lp_20261027/README.md).

`ecbench` measures Pollard rho (in its variants), baby-step giant-step,
the kangaroo, and index calculus **behind one interface, on the same
single-target workloads, in the same unit, under the same isolation, into
the same records**. A number from one method can therefore be read
against a number from another without first asking what each one
counted, where it ran, or which target it solved.

It is native code (`src/cryptanalysis/ecbench/`, `src/bin/ecbench.rs`),
as AGENTS.md requires of measurement work. It does not replace the
repository's accounting rules; it is the machinery that applies them to
every method at once.

For F4/F5/F6-IC, custom root finding, SAT and full backend combinations, open
the [backend frontier leaderboard](../backend-leaderboard/index.html) and its
[measurement and agent workflow](BACKEND_FRONTIER.md). It shows exact-target
count frontiers, cold/online timing diagnostics and missing field-size coverage.

```
spec.json ──ecbench run──► session/                ──ecbench verify──► audit receipt (replay certificate)
                           ├─ spec.json            ──ecbench compare─► comparison.json
                           ├─ plan.json            ──ecbench table───► the one-unit table (AGENTS.md §2)
                           ├─ host.json            ──ecbench db sql──► SQLite (docs/ecbench/schema.sql)
                           ├─ session.json
                           ├─ records.jsonl        ◄── one sealed record per execution, failures included
                           └─ exec/NNNNNN.stderr
```

## Contents

1. [Quick start](#1-quick-start)
2. [Where it sits](#2-where-it-sits)
3. [What a measurement is](#3-what-a-measurement-is)
4. [Names and identities](#4-names-and-identities)
5. [Confounders, and how each one is controlled](#5-confounders-and-how-each-one-is-controlled)
6. [Isolation levels](#6-isolation-levels)
7. [The unit, and what each method is charged](#7-the-unit-and-what-each-method-is-charged)
   - 7a. [Curves of higher genus and transfers](#7a-curves-of-higher-genus-and-transfers)
8. [Statistics](#8-statistics)
9. [Reproduction and the independent runner](#9-reproduction-and-the-independent-runner)
10. [The database](#10-the-database)
11. [Adding a method, a curve family or a factor base](#11-adding-a-method-a-curve-family-or-a-factor-base)
12. [Limits](#12-limits)
13. [GPU stage runner](#13-gpu-stage-runner)

Factor bases (build with `ecbench fb`, publish to Git/`store:`, run
`ic.pipeline` with a matching plug-in string):
[FACTOR_BASE_PROTOCOL.md](FACTOR_BASE_PROTOCOL.md).

### Every subcommand, and where it is described

| `ecbench …` | does | described in |
|---|---|---|
| `methods` | list every method with its parameters | §7, §11 |
| `host` | print the host capsule | §5 |
| `plan` | validate a spec and print what it expands to; an arm its curve cannot run is refused here | §1, §3 |
| `run` | run a session | §1, §5, §6 |
| `exec` | the measured child (`run` spawns it; not for hand use) | §3 |
| `verify` | audit a session from its own files, with exact replays | §9 |
| `compare` | compare arm B against arm A, paired bootstrap intervals | §8 |
| `table` | every arm on every curve, mean `S`, ratio to the floor and to the reference arm | §3, §8 |
| `band` | the pricing band of AGENTS.md §5a: unpriced native work charged at each weight | AGENTS.md §5a |
| `resources` | resource vectors for all measured attempts | §7 |
| `trend` | fit how a paired ratio moves across curves from `compare --save` files | §8 |
| `calibrate` | A/A interval coverage and false admissions across quiet and contended sessions | §8 |
| `price-units` | measure the full-pricing units on this host | §7 (`*_full` methods) |
| `profile-input`, `callgrind-ir`, `callgrind-online-ir` | export one measured child's input; sum Callgrind instruction counts inside the solve markers | §9 |
| `fb`, `fbm`, `store` | build a factor base; seal an FBM1 measurement identity; push, pull and index large files | [FACTOR_BASE_PROTOCOL.md](FACTOR_BASE_PROTOCOL.md) |
| `claim` | a `vs_rho` claim: one IC run against the strong rho on a public target | §9 "Claims" |
| `db` | the SQLite database | §10 |
| `isolab-job` | an `isolab.job/v1` for an independent lab worker | §9 |
| `bound`, `frontier`, `challenge` | sealed cost bounds, the frontier of unbeaten methods, and the frozen paired challenge that moves it | [../bounds/README.md](../bounds/README.md) |
| `gpu-stage`, `gpu-match`, `gpu-match-verify` | the separately labelled GPU stage and its CPU recheck | §13 |
| `wordram-prices` | print the fixed word-RAM reference prices of the `*_wordram` methods; nothing is measured | §7 |

`ecbench <command> --help` describes every option of every command, with
its default.

## 1. Quick start

Rolling `main` release archives ship `ecbench` beside `ic` for Linux (glibc
and static musl), Apple silicon, and Windows. A source build is:

```bash
cargo build --release --bin ecbench
```

```bash
./target/release/ecbench plan --spec docs/ecbench/specs/smoke-prime.json
```

The exact binary-field correctness fixture pairs `ic.large_prime` with
the matched `rho.signed_frobenius` on the same explicit Koblitz curve and targets:

```bash
./target/release/ecbench plan --spec docs/ecbench/specs/smoke-binary-large-prime.json
```

The companion `ic large-prime` command imports and validates every curve,
base point, and challenge point in an external ECBench corpus before solving
one selected challenge. `--summands n-1` is literal and a run is refused when
its exact combination table would exceed `--max-states`. The JSON report binds
the raw manifest bytes with `manifest_sha256` and `manifest_bytes` and refuses
duplicate family ids:

```bash
ic --json large-prime --manifest corpus/manifest.json --family koblitz-n11-l4 \
  --summands n-1 --large-primes 2
```

```bash
./target/release/ecbench run --spec docs/ecbench/specs/smoke-prime.json --out /tmp/ecbench/smoke-1
```

```bash
./target/release/ecbench verify --dir /tmp/ecbench/smoke-1 --replay 8
```

`verify` prints a receipt and exits 1 when the audit finds a problem (2
when it cannot run at all); `--exit-code` is still accepted and ignored.
`--replay 8` replays eight runs chosen at random from the deterministic
ones, with a seed drawn from the operating system and recorded in the
receipt (`--replay-seed N` repeats a draw, `--replay-seq N` replays one run
by sequence number, `--replay-all` replays every one).

```bash
./target/release/ecbench compare --dir /tmp/ecbench/smoke-1 --a rho-neg --b bsgs-neg
```

```bash
./target/release/ecbench table --dir /tmp/ecbench/smoke-1
```

```bash
./target/release/ecbench db sql /tmp/ecbench/smoke-1 | sqlite3 -bail ecbench.db
```

Wide registered curves whose coordinates do not fit the measured harness's
`u64` group types can still carry canonical factor-base inventories.  The
native P-256 Dickson builder writes the same object layout and FB1 preimage,
with its unavoidable wide integers labelled explicitly:

```bash
./target/release/p256_factor_base \
  --curve icv1-fp256-t89188191154553853111372247798585809583-f188c491 \
  --factor-base dickson-torus:depth=18 \
  --out /tmp/icv1-fp256-t89188191154553853111372247798585809583-f188c491.factor-base.json \
  --sql-out /tmp/icv1-fp256-t89188191154553853111372247798585809583-f188c491.factor-base.sql \
  --relation-length 17 \
  --verify
```

The standalone builder keeps the sealed `ecbench` measurement path unchanged.
Its `ecbench.factor_base_dump/v1-wide` inventory and SQL companion use the
same factor-base tables, but neither is an `ic.pipeline` measurement.

`ecbench methods` lists every method and its parameters; `ecbench host`
prints the host capsule; `ecbench claim build` turns an IC run and a
strong-rho run on a public target into a checked `vs_rho` claim (§9). On Linux `run` reserves a whole core by default
(`--cpus auto`); on macOS there is no affinity control, so it records
every run at L0 and says so. Operation counts are unaffected.

The skills in [`.agents/skills/`](../../.agents/skills) walk an agent
through a measurement (`ecbench-measure`), an independent run
(`ecbench-independent-runner`) and an extension (`ecbench-extend`).

## 2. Where it sits

| tool | language | what it is for | relation to `ecbench` |
|---|---|---|---|
| `tools/isolated_bench.py`, `src/bin/isolated_bench.rs` | Python, Rust | pin one command, keep others off its core, record conditions | `ecbench` takes the **same lock** (`/tmp/crypto-bench.lock`), refuses the same things and uses the same preflight defaults, natively, per measured child |
| ICMS (`tools/icms/`, `docs/ic/measurement/`) | Python | index-calculus configurations through producer adapters | `ecbench` keeps its vocabulary (windows, isolation levels L0–L3, env class, never-overwrite sessions, A/A arms) and extends it to every ECDLP method in native code. ICMS sessions stay valid evidence |
| `isolab/` | Python service | an independent lab worker: cgroup partitions, NUMA, IRQs, PSI gates | `ecbench isolab-job` writes the job; `ecbench` runs inside it with `--cpus inherit` (§9) |
| `taskq/` | Python service | the Kubernetes fleet queue | unchanged; a spec runs there the same way, as a command |
| boundary autolab, IC tournament | Python | their own frozen protocols | unchanged; `ecbench` does not redefine historical protocols |

AGENTS.md forbids Python for new measurement work, and that is the reason
`ecbench` is native. The Python tools above remain where they are and
keep their own evidence. New cross-method measurements use `ecbench`.

## 3. What a measurement is

The question is AGENTS.md's: **what does it cost to solve one previously
unseen target**, cold, with every phase charged, verified.

- **One target per workload.** A workload is one curve, one prime-order
  subgroup, one generator and one target point. `targets_per_curve: 8`
  makes eight workloads, eight rows, never one eight-target run. Nothing
  is shared between workloads.
- **Cold.** Each execution is a fresh process. Nothing a method builds
  (a jump table, a baby-step table, a factor base, a pair table)
  survives to the next execution, so its cost is inside every row.
- **Two windows.** `S` is `cold_end_to_end`: set-up, search, relation
  collection, linear algebra and the method's own internal verification,
  all charged. Beside it every record carries the **one-target online
  window** of the IC measurement rules (AGENTS.md "IC measurements"):
  from the first target-dependent computation to the verified recovery,
  with reusable target-independent set-up outside it.
  - For index calculus the window opens after the factor base and the
    oracle's tables. It splits into the five exclusive phases the
    `vs_rho` claim schema names: `target_query`, `target_PDP`,
    `target_relation_check`, `target_descent` and
    `target_recovery_check`, measured by the repository's phase clock
    (`ic_measurement`) and summing exactly to the window. A phase the
    method never enters is zero with a recorded reason. For example, the
    oracles return exact decompositions, so the classic pipeline has no
    separate relation check.
  - For rho the window is the target-dependent solve. For the strong
    reference (`rho.signed_frobenius_strong`), whose jump table `[a]G` is
    target-independent, the window starts at the first walk start
    `[c]G + Q`. The stages it includes are `walk`, `collision` and
    `recovery_check`. `rho.signed_frobenius_strong_legacy` retains the
    literal pre-2026-10-14 canonicalization only as a trajectory-identical
    implementation control; it has the same boundary and accounting and is
    not a second mathematical reference.
- **Verified, by someone else.** No solver receives the planted scalar.
  The runner checks `[k]G = Q` and `k = planted` in its own process, and
  `ecbench verify` checks it again from the files. A run that does not
  verify is a row with status `wrong_answer`, `exhausted`, `error`,
  `timeout`, `crashed`, `interrupted` (the session's own interruption
  killed the child) or `spawn_error` (the child could not be started or
  pinned; the session stops there). It is kept, never dropped, and it makes
  any comparison involving it `incomplete`.
- **Planted or public targets.** By default a target is `Q = [k]G` for a
  derived `k` (`uniform_scalar_sha256_v1`), kept for the check above. With
  `"target_kind": "public"` a target is hashed to the subgroup
  (`hash_to_subgroup_v1`: an abscissa from SHA-256, lifted, the cofactor
  cleared), so nobody knows its logarithm and `[k]G = Q` is the only
  check. That is what the IC claim rules mean by a previously unseen
  public target, and `ecbench claim` accepts nothing else. The law is in
  the workload id, so the two kinds never share one.

### Boundaries

Every record carries the curve's generic floor in `S`,
`floor_s = √(π / 2A)`, with `A` the automorphisms a generic algorithm may
use, computed from the curve (`workload::Instance::facts`): 2 (negation)
on any curve; `2m` on a Koblitz curve over `GF(2^m)` (signed Frobenius),
whether constructed or given as an explicit binary curve with `a ∈ {0,1}`,
`b = 1`; 6 on a `j = 0` prime curve (`a = 0`, `p ≡ 1 (mod 3)`) and 4 on a
`j = 1728` one (`b = 0`, `p ≡ 1 (mod 4)`), when the rational order-3
(order-4) automorphism acts on the order-`r` subgroup as a scalar (the
structure `rho.cubic` and `rho.quartic` need, checked with their own
constructors). The ratio `S / floor_s` is in every record and every table.

The count is versioned: a record carries `boundaries.floor_rule`
(`automorphisms/v2`). A record without the field was written under the first
rule, `A = 2` on every prime curve, and the audit recomputes it under that rule
(`workload::legacy_automorphisms`), so a committed record keeps the floor it
was written with. Under the first rule a `j = 0` curve's `rho.cubic` read
0.91× a floor it was entitled to beat (`A = 6` makes that floor
`√(π/12) = 0.51`, and the same run about 1.6× it). Apart from
[`research/ecbench_accounting_v2_20261023`](../../research/ecbench_accounting_v2_20261023/REPORT.pdf),
which measures this change on the registered `j = 0` curve
`icv1-fp25-t9734-bcfe9f51`, the committed sessions that use a `j = 0` or
`j = 1728` prime curve are the class set-up study's
(`research/prime_rho_automorphism_20261015`, `j = 1728`) and the other
pre-fix producers' (below), and every main-binary `rho.cubic` /
`rho.quartic` record. Their records carry the first rule's `A = 2` and used
6 or 4; the audit holds a record under the first rule (no `floor_rule`) to
the count rebuilt from the spec (`audit::automorphisms_bound`), whichever
binary wrote it, and a record under `automorphisms/v2` to its own figure. The explicit Koblitz curves in
`research/ghs_curve_comparison_20261009` already carry `A = 2m`, because
`binary_explicit_instance` certifies them as Koblitz. The audit checks that
no record's `automorphisms_used` exceeds the `A` available. The *reference* is an arm of the session, normally the matched
rho: `rho.negation` on a prime curve, `rho.signed_frobenius` on a Koblitz
curve. `ecbench table` divides every arm by it.

## 4. Names and identities

Every object is named by a hash of what it is. A label never enters an
identity, and two objects that hash alike are the same object.

| object | id | hashed from |
|---|---|---|
| curve | ICV1 slug `icv1-…` ([ICV1](../curves/ICV1.md)), EC1 alias when the registry holds this generator | the curve model (AGENTS.md §11) |
| workload | `W` + 12 hex | ICV1 string, subgroup order, cofactor, generator, **target point**, target law, seed, index, `target_count: 1`, `cache_policy: cold` |
| method | `ECM1h` + 12 hex | registry id and every parameter, defaults written out; an integer-valued parameter is written in canonical decimal (`64`, `064` and `+64` are one method) |
| factor base | `FB1h` + 12 hex | curve slug, family, parameters, column count, point count, SHA-256 of the sorted point keys |
| spec | `ECS1h` + 12 hex | the spec without its label and description |
| host class | `ECBENV3h` + 12 hex (`ECBENV2h` before version 3, `ECBENV1h` before version 2) | the stable host facts (§5): topology shape, memory in whole GiB, so a reboot keeps the class; version 3 leaves the compiler out (below) |
| session | `ECBS1h` + 12 hex | spec, host and plan hashes, start time |
| record | `ECR1h` + 12 hex | the exact record line, with the id empty |
| run | `<method_id><workload_id>R<n>` | the repository's `{candidate}W{workload}R{number}` convention; `n ≥ 1` counts executions of the pair in the session |
| comparison | `ECC1h` + 12 hex | both arms' sessions and methods, the resampling request |
| claim candidate | IC1 label `IC1N…C…fb…PDP…RC…LA…TD…ISO0h` + 12 hex | the tournament's candidate record (`research/ic_candidate_tournament_20260915/identity.py`): the curve, the factor base's audited inventory, every IC stage resolved, the IC sources' hashes |
| claim workload | 12 hex | the tournament's workload record: EC1 curve id, the target point, its law and seed, the algorithm seed, the resource envelope |

Canonical JSON is sorted, compact and ASCII-escaped, as
`docs/curve-identities.md` hashes it. Floats never enter an identity. The
host-class prefix is deliberately not ICMS's `ENV1h`: the two hash
different fact sets. The two claim identities are hashed the way
`identity.py` hashes them (UTF-8, not ASCII-escaped) and are pinned
against its values in `claim.rs`'s tests, so a claim built here and a
tournament candidate with the same inputs carry the same id.

Ordinary factor-base dumps encode group keys as eight-byte big-endian
integers and coefficients as `u64`.  `factor_base_dump/v1-wide` preserves the
table and identity layout but stores curve integers and coefficients as
decimal strings.  Its family declares its fixed-width key encoding as an
identity-bound parameter; the P-256 Dickson family uses 33-byte big-endian
`((x+1)<<1)|sign` keys.  A wide dump may use an FB1 identity because the FB1
preimage is unchanged; it may not claim the ordinary v1 dump schema.

## 5. Confounders, and how each one is controlled

Every row of this table is something that has moved a figure in this
repository or in the literature without the algorithm changing. Each is
**controlled** (removed by construction), **gated** (checked per run, and
a run that fails the check loses its isolation level), or **recorded**
(kept beside the figure so a later reader can see it).

| confounder | how it moves a figure | what `ecbench` does | kind |
|---|---|---|---|
| Which target | rho's and BSGS's cost depend on the target; a lucky target flatters a method | targets are derived by SHA-256 from (curve, seed, index), every arm solves the same ones, and intervals resample workloads (§8) | controlled |
| Algorithm randomness | one rho walk is one draw of a wide distribution | one seed per (workload, round) is shared by every arm, so arms are paired; rounds vary it | controlled |
| Order and drift | heat, a filling page cache, a background job ramping up | arms interleave every round (`alternate` reverses order on odd rounds, counting the warm-up rounds in the parity; `alternate_measured` counts the measured rounds only, so the first measured round is always in spec order; `shuffle` permutes by seed); warm-up rounds are run and marked. An odd number of measured rounds cannot be balanced by either alternating mode: `plan` and `run` print a warning (existing specs expand exactly as before) | controlled |
| One-time costs | page faults, lazy statics, allocator warm-up land on whoever runs first | every execution is a fresh process; nothing is cached across executions | controlled |
| Allocator state inherited from set-up | the child builds its workload (curve, target, tables) before the timed solve, and on glibc that leaves the fastbins full of freed chunks; the first sizeable allocation inside the solve (a distinguished-point table, a jump table) then pays `malloc_consolidate` for all of it. At small sizes that is 3–6 ms against a solve of 0.1–0.2 ms, it differs with how much garbage each method's set-up left (an arm-dependent bias, not just noise), and it is invisible to Callgrind, which replaces `malloc` | the child calls `malloc_trim(0)` after the workload is built and before any clock, counter or window opens, so every solve starts from a consolidated allocator; each record's `allocator` block says what ran (name and version, whether the pre-solve trim ran, its return value) and the host capsule's `build.allocator` names the allocator. Where the platform has no such call (macOS, musl) the block says the trim did not run. This changes wall time only, never a count | controlled, recorded |
| Another benchmark | two timed jobs on one machine measure each other | the shared lock `/tmp/crypto-bench.lock`, the one `isolated_bench` takes; `--wait` queues. The lock is taken **first**: planning (a prime search can burn minutes of CPU), the host capture and the choice of CPUs all happen inside it, so a queued session neither disturbs the one running nor chooses its CPUs from a mask that session narrowed. `gpu-stage` and `gpu-match` take it too | controlled |
| Other processes on the core | preemption, cache eviction | whole-core reservation; every movable thread moved off, and put back when the session ends however it ends (threads the evicted ones spawned meanwhile included, recycled thread ids excluded; a thread that exits between the affinity read and the write is counted as gone, not as unmovable); interrupt affinity (`/proc/irq/*/smp_affinity_list`) is moved off the reservation where the kernel permits it (root) and restored the same way, with the number moved, the number that would not move and the first reason recorded; the runner itself leaves the reservation; busy ticks on the run CPU not explained by the child are measured per run; before and after every run the reservation is scanned for user threads that could run on it (a thread that appeared after the eviction, or got a wide mask back) | gated (L1, L2) |
| SMT sibling | a neighbour on the sibling shares the pipeline and caches | a CPU is reserved only with all its siblings; under an inherited placement a sibling outside it costs the run L1; every sibling's busy ticks are checked per run, reserved or not | gated (L1, L2) |
| CPU 0 | the kernel's housekeeping and many IRQs land there | `auto` never picks CPU 0's core; a run on it loses L1 | gated (L1) |
| Migration | a process moved mid-run pays cold caches | pinned between fork and exec; the child reads its own mask back and reports the CPU it started and ended on | gated (L1) |
| NUMA | memory on the far node costs every miss | memory is bound to the run CPU's node with `set_mempolicy(MPOL_BIND)` in the child; the child reads the policy back with `get_mempolicy` and counts its anonymous pages per node in `/proc/self/numa_maps` after the solve | gated (L1, multi-node hosts) |
| Run-queue delay | time runnable but not running is not the method's | the child's `/proc/self/task/*/schedstat` summed over its threads, across the solve; above 0.5 % of the solve and above 100 µs (one wake-up), L2 is lost | gated (L2) |
| Preemption | each one costs cache and branch state | involuntary context switches over **the solve** (`getrusage` read by the child just outside the timed window); one is allowed, beyond that 50 per second of solve | gated (L2) |
| Hypervisor steal | invisible to a process tick count | per-CPU steal ticks from `/proc/stat` around every run; one tick (the counter's resolution) is allowed, beyond that 1 % of the run | gated (L2) |
| Interrupts and foreign slices on a short run | `/proc/stat` ticks are 10 ms: a few-millisecond solve cannot see foreign work in them | the child reads the run CPU's `/proc/interrupts` and `/proc/softirqs` columns (devices, the local timer, IPIs and other named lines, softirq work and tick-driven softirqs) and, where the kernel has `CONFIG_SCHEDSTATS`, its `/proc/schedstat` line, around the solve and records the differences; non-tick interrupts above a floor of 2 plus 100 per second of solve, or more than one foreign task slice on the CPU during the solve, cost L2. The scheduler tick is never an intrusion | recorded, gated (L2) |
| Too short to time | at a few tens of microseconds one wake-up or one interrupt is a visible fraction of the figure, whatever the gates allow | a solve under 1 ms cannot admit wall time: an L2 blocker `solve too short to admit wall time` records it explicitly, so the run is graded L1 and its wall time is descriptive | gated (L2) |
| Memory pressure | reclaim stalls a run | PSI memory `some` total around every run | gated (L2) |
| A busy host at start | everything above, before the first run | a settle-window preflight (busy ticks per reserved CPU, PSI); refused unless `--allow-busy`, which records it and caps runs below L2. `--settle` below 2 s, `--max-other-cpu` above 0.10, `--max-psi` above 5 (or a looser preemption limit) loosen the preflight, and a run under such thresholds carries an L2 blocker that names them | gated (L2) |
| Frequency and turbo | the clock is the denominator of wall time | governor, `no_turbo`, `boost`, `isolcpus`, `nohz_full` and SMT control recorded per CPU; required for L3 | recorded, gated (L3) |
| Instruction count | wall time moves with frequency and contention, instructions retired barely do | user-space instructions and cycles across the solve from the CPU's counters (`perf_event_open`), where the host exposes a PMU; elsewhere the record says why not. The two events form one group with `inherit` set (threads the solve spawns count; the record says whether the kernel accepted it), and when the PMU was shared (`time_running < time_enabled`) the record keeps the raw counts and adds `instructions_scaled`/`cycles_scaled` (`raw × enabled / running`) and `multiplexed`; sums over runs use the scaled counts | recorded |
| Virtualisation | a guest cannot see or control most of the above | CPU `hypervisor` flag, DMI, `systemd-detect-virt`, `kern.hv_vmm_present` recorded; bare metal required for L3 | recorded, gated (L3) |
| Heterogeneous cores | Apple P and E cores differ by about 2× | performance levels recorded; with no affinity control the runs are L0 | recorded |
| Threads | four Rayon threads on one pinned CPU wait on each other | `RAYON_NUM_THREADS=1`, `OMP_NUM_THREADS=1`; the child counts its own threads (`/proc/self/task`) before and after the solve, and more than one loses L1 (CPU time over wall above 1.05 still does, but a pinned child's CPU time can never exceed its wall time, so that check alone cannot fire) | controlled, gated (L1) |
| Environment leakage | an engine knob in the operator's shell changes the search | the child's environment is cleared and rebuilt: `PATH`, `HOME`, a pinned locale and thread counts, nothing else; its hash is in the session | controlled |
| Build | a debug build or another binary is another experiment | the binary's SHA-256, the source commit embedded at build time and its declared dirty state, any runtime worktree commit as a diagnostic only, `rustc -Vv` on `PATH` as a diagnostic outside the host class (from class version 3; it moved with the shell and a toolchain file in the current directory), compiler flags in the environment; the binary is hashed from `/proc/self/exe` at the very start of `run`, so a rebuild during planning cannot null or swap the hash, and each child execs `/proc/self/exe`, so a rebuild mid-session cannot swap the code (elsewhere a changed binary stops the session); a debug build loses L1 | recorded, gated (L1) |
| Host | the same binary differs by 17–20 % across hosts | the host capsule's stable facts hash to the class id; wall-clock ratios are formed only inside one session. Class version 3 (`ECBENV3h`) leaves the compiler out: it used to be `rustc -Vv` of whatever `PATH` and the current directory named, so a `sudo` run landed in another class on the same machine. Capsules of versions 1 and 2 still recompute under their own definitions | recorded, controlled |
| What each method counts | one tool counts additions, another wall time, another instructions | one `CountedGroup` ledger for every method; native work the unit does not charge is counted as `*_uncharged` and makes the total a lower bound (§7) | controlled |
| Pricing drift | per-run measured ratios repriced identical runs by up to 8 % | IC native work is priced only at the repository's pinned ratios (`docs/ic/calibration.json`); a unit without one stays unpriced, never host-measured; an algebraic solver's wall-priced work is taken back out | controlled |
| Selective reporting | dropping a failure, rerunning until it looks good | every execution writes a sealed record before the next starts; the output directory is claimed with an atomic `mkdir` under the lock, so a session queued for a used directory fails instead of writing over it; an interrupted session is marked, never deleted; a child that cannot be started or pinned is a `spawn_error` record and the session stops there with the reason recorded, not a silent end; a child killed by the session's own interruption is `interrupted`, not `crashed`; `timeout` is recorded only when the watchdog's SIGKILL actually ended the child (a child that exited by itself in the instant the deadline passed keeps its answer) | controlled |
| Interruption | a killed runner leaves narrowed masks behind and a child solving on an unprotected core | SIGINT, SIGTERM, SIGHUP and SIGQUIT are taken by one thread, which kills the child's process group; the session is marked `interrupted`, the eviction restored, and the runner exits `128 + signal`; the child dies with the runner (`PR_SET_PDEATHSIG`). `session.json` is replaced atomically (temporary file, `fsync`, rename) after **every** record, with `records_written`, `last_seq` and the per-status counts, and `stop` (`kind`: `interrupt`, `spawn_error` or `error`; the `seq` it stopped at; the signal or message) once the session stops early. A runner killed outright (SIGKILL, host loss) cannot mark the session, which keeps reading `running` with the counts of the last record it wrote; `ecbench verify --allow-interrupted` audits such a session as interrupted (`interruption: never_finalised`, `records_bound: false`), with every per-record check (seal, sequence, plan, answer, grading) still applied. No whole-file records hash was ever written, so the file is explicitly unbound: truncation or appended records are seen only through the unkeyed per-record seals, and such a session must not be cited as complete; `verify` checks that the records end at `last_seq` (a killed session at most one record past it) and at `stop.seq`, and reports the stop in the receipt | controlled |
| Tampering and transcription | a figure edited by hand, or copied wrong | every file is hashed into `session.json`; `verify` re-derives identities (the session id from its preimage), the plan, the answers, the seals, every derived figure and each record's internal consistency (§9), and replays randomly chosen runs; a failed audit exits non-zero | controlled |

## 6. Isolation levels

A run earns the highest level whose checks, and every lower level's,
passed. Its record lists every check it failed as a blocker
(`"L2: 3 ticks of hypervisor steal on the run CPU"`), so a level is never
a bare label.

| level | requires |
|---|---|
| **L0 recorded** | the run executed under a saved host capsule. Enough for operation counts, which do not depend on contention |
| **L1 pinned** | a reservation (`--cpus` not `none`) inside this process's cpuset; a release build; no SMT sibling of the run CPU outside the reservation; the child's own affinity read back as exactly the run CPU, and the run CPU at its start and end; the run CPU not on CPU 0's core; the runner off the run CPU (under an inherited placement with no other CPU it may sleep on an idle sibling, where the sibling check sees it); no user thread left unmovable on the reserved CPUs (a non-root run usually fails this), and none that appeared on the reservation since (the per-run scan); a single-threaded child (its thread count read from `/proc/self/task`; child CPU time over wall ≤ 1.05 as before); on a multi-node host, the memory policy read back as `bind:<node>` of the run CPU and at least 99 % of the solve's anonymous pages on that node |
| **L2 quiet** | L1, and: a quiet preflight, run with the default or tighter thresholds (a loosened `--settle`, `--max-psi`, `--max-other-cpu` is an L2 blocker that names it); a solve of at least 1 ms (`solve too short to admit wall time` otherwise); steal within one tick or 1 % of the run; foreign busy time on the run CPU within max(2 ticks, 1 % of wall); the run CPU's siblings and the rest of the reservation within the same allowance; over the solve alone, run-queue delay ≤ max(100 µs, 0.5 % of the solve), involuntary switches ≤ max(1, 50 per second of solve), non-tick interrupts on the run CPU ≤ 2 + 100 per second of solve and at most one foreign task slice; memory stall ≤ max(1 ms, 0.5 % of wall) |
| **L3 isolated** | L2, and the host configured for measurement: run CPU in `isolcpus` or `nohz_full`; `performance` governor; turbo known off; bare metal known |

A spec's `isolation_required` (default L2) is the level a **wall-clock**
figure needs to be admitted. Operation counts never need one.

**Grading versions.** The rules above are version 3 (`grading_version` in
`session.json`). Version 2 judged a few-millisecond solve by figures taken
over the whole process (exec, set-up and exit included): on 216 smoke
records one extra context switch, one wake-up or one 10 ms steal tick cost
27, 14 and 17 runs their L2, the foreign-busy gate could not see anything
under about 2 s, and the thread gate could not fire on a pinned child.
Version 3 measures the solve, gives every gate an absolute floor so a single
event cannot fail a short solve, scales the allowance with the duration,
counts threads, reads interrupt and scheduler counters, scans for intruders
and refuses to admit wall time from a solve under 1 ms. A session is
regraded under the version it was graded with (`grade_for_version`; version 2
is frozen), so historical sessions regrade identically.

`ecbench compare` also reports a **measurement quality score** from 0 to
100. This is an operational rubric, not a probability that the result is
correct and not a confidence interval. For each measured pair, an
unverified result gets zero points; a verified pair gets the lower of its
two isolation levels: L0=0, L1=40, L2=75, L3=90. The average is taken
over every paired attempt. Quality admission requires at least L2, even
if a diagnostic spec requests L0 or L1. A further 10 points are earned
only when an L2-or-higher A/A control has a 95% interval containing 1 and
wholly inside `[0.9, 1.1]`. The comparison records the level counts, verified and
admitted pair counts, 95th percentile foreign CPU busy fraction, 95th
percentile run-queue delay, and whether telemetry was missing. The
thresholds in the L0–L3 gate still decide whether wall time is admitted.
The separate bootstrap 95% interval measures sampling uncertainty over
the stated targets and seeds; a high quality score does not narrow it.

What hosts earn, as recorded so far:

- **macOS:** L0, because there is no affinity control.
- **A two-node QEMU guest** (Ubuntu 6.8 kernel, 2 sockets × 2 cores × 2
  threads, [`research/ecbench_numa_vm_20261002`](../../research/ecbench_numa_vm_20261002/README.md)):
  L2 on about 80 % of runs, the rest stopped at L1 by run-queue delay.
  Pinning and NUMA binding were confirmed there, with policy `bind:<node>`
  and every anonymous page on the bound node. An emulated guest reports
  no steal by construction, so its L2 does not certify quiet hardware.
- **A GitHub-hosted Linux runner, run as root** (AMD EPYC 9V74, Azure VM,
  4 vCPUs as 2 cores × 2 threads): L1 on 72 of 72 runs of the CI session,
  with every child's affinity read back as its run CPU. Its operation
  counts matched the macOS session of the same spec to the last digit, IC
  included. It is a VM, so L3 is out of reach. Each CI run's summary
  carries the current level and blocker counts.

L3 needs a lab host, which `isolab` prepares (§9).

## 7. The unit, and what each method is charged

> **Every record is also priced in `bitop/v1`** (`docs/units/BITOPS.md`):
> two-input gates per primitive, under frozen formulas, with nothing left
> unpriced or converted by a host. It is the unit in which methods,
> repositories and curve families compare.
>
> - `ecbench work <sessions…>` prices committed records from their own counts.
>   `--summary` gives one line per method configuration.
> - New records carry a `work` block. It holds:
>   - the total and per-phase gates, and the primitive counts;
>   - the ratio to the floor in additions;
>   - the reusable/per-target split;
>   - `complete`.
> - `ecbench bound fit --unit bitop/v1` fits an arm in gates, and also in
>   additions: gates over one addition's gates at each size, where generic
>   methods have exponent ½. An incomplete pricing is inadmissible in this
>   unit.
> - `ecbench audit-ir --spec S --seq N… --out D` replays executions under
>   Callgrind and sets instructions beside counted gates. An arm far above
>   same-curve rho's ratio is doing work its counters miss.
> - `ecbench amortize` compares an arm with batch rho (Kuhn–Struik) per curve
>   and reports the window of target counts in which the arm costs less.
>
> Counters added for `bitop/v1` live in `PhaseRecord.work_native` and in the
> solver block's `bitop_*` keys. A replay of a committed record ignores
> both, so committed sessions replay exactly.
> `research/bitop_unit_20261016/` is the first study in the unit.
> The group-addition unit below is unchanged, and every committed figure in it
> stands as recorded.

```
S = total group-addition equivalents / √r
```

An addition and a doubling are one each. A scalar multiplication is
charged as the additions and doublings it performs. `S ≈ 1.25` is plain
rho's expectation `√(π/2)`; the curve's floor is `√(π/2A)` (§3).

### The registered methods

`ecbench methods` prints the live registry with every parameter and its
default; the ids are:

| id | family | what it is |
|---|---|---|
| `ic.aps_v1` | ic | bounded APS polynomial instrument, exclusive cold/online wall phases and retained failures; only independent-check group work is priced, so native operation totals remain incomplete |
| `rho.frozen_reference` | rho | the frozen r-adding walk every ledger row was priced against through §17 (A = 1); the historical "before" mark |
| `rho.plain` | rho | tuned r-adding walk on points (A = 1), distinguished points, stride starts |
| `rho.negation` | rho | tuned walk on `{P, −P}` (A = 2), Wiener–Zuccherato look-ahead and cycle escape; the matched reference on a prime curve |
| `rho.negation_strong` | rho | strong negation-map walk on `{P, −P}` (A = 2) over `F_{(2^m)^4}` (`f2m4_torsion` only): 32 jumps, escape of fruitless cycles by doubling, distinguished points, `lanes` walks per Montgomery-batched `F_{q⁴}` inversion; charged in counted `F_q` multiplications, the unit `ic.pipeline` + `gaudry-d4-bin` is charged in on those curves |
| `rho.quartic`, `rho.cubic` | rho | the same walk on `{±ι^i P}` (A = 4, j = 1728) and `{±ζ₃^i P}` (A = 6, j = 0); refused at plan time on a curve without the structure |
| `rho.negation_v2`, `rho.cubic_v2`, `rho.quartic_v2`, `rho.signed_frobenius_v2`, `rho.signed_frobenius_strong_v2` | rho | the same walks under the `_v2` accounting: Brent cycle detection beside the 16-point window, lost-walk and distinguished-point table counters, the cubic/quartic class set-up in its own `automorphism_setup` phase, and for the strong reference the scaled `auto` shape with charged starts and a `verification` phase (Accounting v2 below) |
| `rho.signed_frobenius` | rho | tuned walk on signed Frobenius orbits `{±φ^t P}` (A = 2n), Koblitz only; the matched reference there |
| `rho.signed_frobenius_strong`, `…_legacy`, `…_escape` | rho | the strong single-target reference (lockstep walks, batched inversion), its trajectory-identical pre-2026-10-14 implementation control, and the variant that escapes fruitless cycles by doubling |
| `rho.signed_frobenius_full`, `rho.signed_frobenius_strong_full`, `ic.pipeline_full` | — | the base method repriced with every native operation charged (`price-units`) |
| `bsgs.textbook`, `bsgs.interleaved`, `bsgs.negation` | bsgs | baby table then giant steps (mean 1.5√r); Pollard's interleaving (mean 4√r/3); classes `{P, −P}` with stride 2m + 1 (mean about √r) |
| `bsgs.textbook_v2`, `bsgs.interleaved_v2`, `bsgs.negation_v2` | bsgs | the same, plus one charged final `[d]G = Q` in a `verification` phase |
| `kangaroo.vow`, `kangaroo.vow_v2` | kangaroo | van Oorschot–Wiener tame/wild kangaroo on `[0, r)`, distinguished points; the frozen id charges a re-placement after a same-kind collision to `search`, the `_v2` id to its own `restarts` phase and adds the charged final check |
| `claw.pair_table`, `claw.pair_table_v2` | claw | the four-point signed-Frobenius pair claw (cryptanalysis#175): known-log orbit base, target-independent quotient pair table; the `_v2` id adds the final check and counts the `recover` phase in the `max_adds` budget |
| `ic.pipeline`, `ic.pipeline_counted` | ic | the index-calculus pipeline (factor base, oracle, relations, linear algebra, verification), and the same with the relation charge rebuilt from counted work |
| `ic.large_prime`, `ic.large_prime_v2`, `ic.shared_rank` | ic | the large-prime variation and the shared-rank adapter on binary curves; the `_v2` id charges the target's `[r]Q` membership check and the final `[d]G = Q` in the `verification` phase |
| `ic.koblitz_descent` | ic | the ledger §23 one-target Koblitz index calculus as `ic price --single-target` runs it (subgroup-orbit base, folded pair table, aimed m = 3 collection, sparse logs, walked m = 2 descent), every stage charged by counted group operations; odd `n < 64` Koblitz curves |
| `ic.koblitz_batched` | ic | the candidate tournament worker's pair-table pipeline (`examples/ic_tournament_worker.rs`: subgroup-orbit base, folded pair table, relation collection in `batch_trials` queries with a rank check after each batch, sparse logs, one-target descent), every stage charged by counted group operations; exists so the tournament's `batch_trials` candidates can be measured under L2, replay and A/A; odd `n < 64` Koblitz curves |
| `ic.k4_gaudry` | ic | Gaudry's k = 4 index calculus on `E(F_{p⁴})`, `fp4_translation` curves only |
| `ic.hyperelliptic_thread` | ic | the hyperelliptic thread's index calculus (ADH/Gaudry: degree-1 places folded by ±, factor-base walk, sparse elimination mod `r`, optional single large primes) on a genus 2–4 Jacobian, `hyperelliptic_*` curves only |
| `ic.cm_folded_native_v1`, `ic.cm_folded_recurrence_v1`, `ic.cm_folded_native_d43_v1` | ic | the CM scalar-orbit signed MITM2 controls with a native backend, prime curves only; frozen: their records carry `total_gae` 0 |
| `ic.cm_folded_native_v2`, `ic.cm_folded_recurrence_v2`, `ic.cm_folded_native_d43_v2` | ic | the same three controls with the replay's counted operations charged (2026-10-28): `independent_replay` carries them as GAE, `total_gae` is the phase sum, and `unpriced` names the `native_*_uncharged` counters instead of a sentence |
| `ic.hyperelliptic` | ic | Adleman–DeMarrais–Huang / Gaudry index calculus on a genus-2 Jacobian (`genus2_*` workloads): factor base of degree-1 places, smoothness by gcd, sparse elimination mod r; frozen (verification uncharged, factor-base scan untallied) |
| `ic.hyperelliptic_v2` | ic | `ic.hyperelliptic` with the verification `[k]D₁ = D₂` charged as Jacobian operations (phase `verification`) and the factor-base scan's `F_p` multiplications tallied (`factor_base_fp_muls_uncharged`, phase `factor_base`), 2026-10-28 |
| `transfer.isogeny` | rho | carry the instance across a rational ℓ-isogeny (ℓ = 2, 3; Vélu), solve on the isogenous curve with a generic inner method, check `[k]G = Q` on the source; prime curves |
| `transfer.quartic_cover` | rho | pull the instance back along the double cover `V⁴ = x³ + ax + b → E` into the Jacobian of a smooth plane quartic (non-hyperelliptic, genus 3), solve there, check on `E`; prime curves, `29 ≤ p < 2³¹`; frozen (the square roots of the point searches are neither charged nor tallied) |
| `transfer.quartic_cover_v2` | rho | `transfer.quartic_cover` with the `F_p` multiplications of the point searches' square roots and `f(x)` evaluations tallied as `root_search_field_muls_uncharged` and listed unpriced (2026-10-28); charged work identical |
| `transfer.ghs_cover` | rho | carry a `ghs_fp6` instance by the GHS conorm–norm map into the Jacobian of its genus-3 hyperelliptic cover over `F_{p²}`, solve there, check on `E` |

Curves of higher genus and the transfers are described in
[§7a](#7a-curves-of-higher-genus-and-transfers).

| family | charged | counted, not charged (`*_uncharged`) |
|---|---|---|
| `rho.*` | jump-table set-up, walk starts, every step, cycle escapes, look-ahead additions, internal verification | canonicalisations (negation, signed Frobenius), Frobenius maps, distinguished-point table queries and inserts (counted in the `_v2` ids) |
| `rho.quartic`, `rho.cubic` (frozen ids) | as `rho.*`, plus the class set-up: the scalar multiplication that fixes ι (resp. ζ₃) on the subgroup, in the `setup` phase and in the total. Records written before 2026-10-18 carried it in the `setup` phase only, so their `total_gae` and `S` fall short of the phase sum by that set-up (21 GAE on the 17-bit crater curves of `research/prime_rho_automorphism_20261015`); the phase sum is the true total. `verify` replays such a record exactly, reporting it as reproduced under the pre-2026-10-18 accounting, with the set-up the replay itself measured subtracted from the total (`audit::legacy_class_setup`), but only when the record names one of the six pre-fix producer binaries in `audit::PRE_CLASS_SETUP_BINARIES` and carries no `detail.class_setup`; any other record, every one the current harness writes included, must replay exactly. The census of affected sessions and the corrected headline ratios are in [`CLASS_SETUP_AUDIT.md`](CLASS_SETUP_AUDIT.md). | as `rho.*` |
| `rho.quartic_v2`, `rho.cubic_v2` | as `rho.*` under the `_v2` walk (Brent cycle detection, charged lost-walk and distinguished-point table counters), plus the same class set-up scalar multiplication, charged in the total in its own `automorphism_setup` phase instead of the frozen ids' `setup` | as `rho.*`; the `_v2` ids also count the distinguished-point table queries and inserts |
| `rho.negation_v2`, `rho.signed_frobenius_v2`, `rho.signed_frobenius_strong_v2` | as the matching first-rule id under the `_v2` accounting (the Accounting v2 paragraph below); the strong id's jump table, lanes and start multiples are charged and its candidate checks are a `verification` phase | as the first-rule id; the `_v2` walk also counts the distinguished-point table queries and inserts |
| `rho.signed_frobenius_strong` | group additions exactly; each scalar multiplication (jump table, walk start stride, candidate checks) at `1.5·log₂ r` additions, `ic_boundary::signed_frobenius_rho`'s convention | canonicalisations, partition hashes, distinguished-point table queries and inserts |
| `rho.negation_strong` | every `F_q` multiplication on the tower's counter (`Gf2q::muls`): the jump table and start points (`setup`), every step with its share of the batched inversion, the start additions, the escape doublings and unbatched additions (`search`), each canonicalisation at one `F_q` multiplication (`canonicalisation`: four `F_q` additions and a comparison, no multiplication, so one is an upper bound), the candidate checks (`check`); each phase's GAE is its multiplications over the curve's `fq_muls_per_add`, the divisor `solve_ic_f2m4` prices IC with | partition hashes, distinguished-point tests, table queries and inserts |
| `rho.signed_frobenius_strong_legacy` | identical to `rho.signed_frobenius_strong`; comparison-only implementation control | identical to `rho.signed_frobenius_strong` |
| `rho.multitarget_pooled` | the whole batch of `L = batch` targets: the shared jump table (full-size multiples of `G`), each target's `G + Q_i` and start stride, every walk start and step, look-ahead and escapes, and the internal `[d]G = Q_i` check of every recovered log; phases `setup`, `search`, `internal_verification` | canonicalisations; distinguished-point table inserts and probes |
| `bsgs.*` | baby steps, the giant stride, giant steps (the `_v2` ids add the final check, phase `verification`) | table inserts and lookups |
| `kangaroo.vow` | jump-table set-up and the two starts (phase `setup`); every jump and any re-placement after a same-kind collision (phase `search`; before 2026-10-18 a re-placement was charged to `setup`, total unchanged; no committed record ever restarted, so every kangaroo record replays exactly; `generic::tests::a_kangaroo_restart_is_charged_to_search` is the pinned control on a curve where every seed restarts). `kangaroo.vow_v2`: the same walk, but a re-placement is charged to its own `restarts` phase (not to `search`), and the final `[d]G = Q` check is a charged `verification` phase | table inserts and lookups |
| `claw.pair_table` | the known-log base's seed scalar multiplications, every table addition `P_i + F_b`, both query additions `Q − (F_k + F_l)`, the addition that rebuilds a hit's table sum; phases `base`, `table`, `search`, `recover` | Frobenius maps, canonicalisations, table inserts and probes |
| `ic.pipeline` | factor base, oracle set-up (pair tables), relation trials, linear algebra, verification, each a phase; native work (lookups, row operations, square roots, Artin–Schreier solves, …) at the pinned ratio where the repository has one | native work with no pinned ratio for this curve; algebraic-solver operations |
| `ic.pipeline_counted` | the same pipeline with the relation phase charge rebuilt from counted work and pinned conversions for deterministic SAT/PDP replays | algebraic-solver operations without a pinned conversion, including SAT conflicts; these runs remain lower bounds |
| `ic.known_log_mitm2_v1` | every known-log base scalar multiplication, folded pair-table addition, failed query, pair witness check and final scalar verification; separate setup and five online phases | base sampling and indexing, table insertions and probes, negations and scalar combinations; native work is counted and unpriced |
| `ic.tensor_contraction_v1` | every non-trivial point addition and doubling (neither input O) the toy tensor-contraction index calculus of `crates/ecdl-tensor-ic` performs, from the crate's per-phase tally: the relation targets `[u]P + [v]Q` (`relation_target`), every point addition evaluated inside the exact sparse contraction and the witness check (`relation_search`), the final `[ξ]P = Q` check (`recovery_check`); `factor_base` and `linear_algebra` charge 0. Prime-order curves only (cofactor 1) | contraction-table rows (`relation_search.contraction_rows_uncharged`), the factor-base listing, which enumerates the group (`factor_base.fb_points_listed_uncharged`) and evaluates every SHA-256 membership hash (`factor_base.fb_membership_evals_uncharged`), and elimination multiplications mod r (`linear_algebra.linalg_mulmod_uncharged`); priced across the §5a band. Exponential by construction: no [Ope26] solver |
| `ic.hyperelliptic` | Jacobian operations: the factor-base walk's step precomputation (`setup`), every relation-search step (`relations`). Not the verification: `d1.scalar_mul(k)` runs twice on a success and increments no counter (frozen; the older text of this row said it was charged) | the smoothness oracle's `F_p` multiplications, elimination row operations (phase `linear_algebra`, 0 GAE); priced across the §5a band. The factor-base scan's `F_p` work (`f(x)` and a square root at every `x` it visits) is not tallied at all |
| `ic.hyperelliptic_v2` | as `ic.hyperelliptic`, plus every Cantor doubling and addition of both verifications (`verification`, `⌊log₂ k⌋ + 1` doublings and one addition per set bit each, in the total); `ic.hyperelliptic_v2` minus `ic.hyperelliptic` is exactly that phase | as `ic.hyperelliptic`, plus the factor-base scan's `F_p` multiplications (`factor_base_fp_muls_uncharged`, phase `factor_base`, counted by `hyper_ic::factor_base_scan_muls` over the same `x`: Horner `f(x)`, Euler test, Tonelli–Shanks); the record is a lower bound at `w = 0` |
| `ic.cm_folded_*_v1` | nothing: `total_gae` and every phase's `gae` are 0 (the replay's operations are in `independent_replay.adds/doubles/scalar_mults` only), so `S` is 0, below the floor, and the prose `unpriced` entry names no counter. Frozen | the native backend's work (`native_*_uncharged`) |
| `ic.cm_folded_*_v2` | the Rust replay's counted operations (`independent_replay`: base scalar replays, relation sums, `[α]G`, the final `[d]G = Q`) as GAE; `total_gae` is the phase sum | `native_*_uncharged` (each listed in `unpriced` by name), and, for `mode = velu`, the map derivation's group operations (`cm_map_derivation_group_ops_uncharged`); a lower bound |
| `transfer.isogeny` | the inner method's phases (`target:*`, on the isogenous curve, the same unit), the source check `[k]G` (`verify`) | the kernel search and the two point maps, `F_p` multiplications and inversions (`transfer`, 0 GAE) |
| `transfer.quartic_cover_v2` | as `transfer.quartic_cover`, charges unchanged | `root_search_field_muls_uncharged`: the `F_p` multiplications of `f(x)` evaluations and Tonelli–Shanks square roots on `E` that search for the cover's rational points (`cover_setup`) and the base point `B` (`transfer`), which run outside `plane_quartic`'s counter; the record is a lower bound |
| `transfer.quartic_cover`, `transfer.ghs_cover` | everything but, for the quartic, those square roots (frozen: its records say `unpriced: []`, which understates it; `transfer.quartic_cover_v2` tallies them): the cover's construction (`cover_setup`, quartic only), the two pullbacks (`transfer`, with the two additions `G + B`, `Q + B` on `E` for the quartic), the inner solve (`target:solve`), the check on `E` (`verify`); Jacobian work counted in `F_p` multiplications and divided by `c_E`, measured in the run (§7a) | nothing |
| `ic.large_prime` | cofactor projection, exact combination-table group additions, target probes, and relation witness checks; the external runner verifies the returned scalar | combination states, MITM lookups, large-prime merge operations, and modular row operations; the record is therefore a lower bound, not a speed claim |
| `ic.koblitz_descent` | every point addition and doubling each stage performs, counted in the arithmetic itself (`group_op_count`: `binary_ecc` `point_add`/`point_double`, `FastCurve`'s affine, batched, lazy, AVX-512 and López–Dahab paths, `koblitz_fast_arith`'s 64- and 128-bit curves; a batch of `k` counts `k`, a ladder its actual steps): the factor-base cofactor projections (`factor_base`), every folded pair sum (`oracle_setup`), the anchors, strides, walk steps, window scans, hit recoveries and witness checks (`relations`), the relation re-checks and column-log checks (`linear_algebra`), the descent's stride multiples (`descent_setup`), the walks' starts and steps, pair recoveries and the recovery ladder (`descent`, the online window), and the final `[d]G = Q` (`verify`) | canonicalisations, pair-table filter probes, bucket and point-index reads, Frobenius coordinate maps, factor-base lifts and rebuilds, sparse row operations; field arithmetic outside the group law, hashing, the descent's modular combination, and the forward pass of a vector batch that declined a degenerate block (its lane count is in `detail`, host-dependent) are listed unpriced without a replayed counter |
| `ic.koblitz_batched` | the same counted operations, stage by stage, as `ic.koblitz_descent` (`factor_base`, `oracle_setup`, `relations`, `linear_algebra`, `descent_setup`, `descent` the online window, `verify`), over the worker's calls | the same unpriced native work as `ic.koblitz_descent` |
| `rho.plain`, `rho.negation`, `rho.frozen_reference`, `bsgs.*`, `kangaroo.vow` on a hyperelliptic Jacobian | as on a curve: one Cantor composition-and-reduction (`hyperelliptic_word::HCurve`) is one addition or one doubling, whatever its degree; the walk's set-up, starts, steps, look-ahead, escapes and internal verification as `rho.*` | as `rho.*` (`canonicalisations_uncharged`, …) and `bsgs.*`; the `F_p` work inside a Cantor step is neither charged nor tallied (`counts_field_ops` is `false`) |
| `ic.hyperelliptic_thread` | every Cantor addition on `HCurve`: the walk start `D₁ + D₂` (phase `walk_setup`, 1 GAE) and one addition per relation trial (phase `relations`), and the verification `[k]D₁ = D₂`, one counted scalar multiplication (phase `verification`, `⌊log₂ k⌋` doublings and `popcount(k) − 1` additions, in the total; before 2026-10-28 it was left to the runner and uncharged, fixed in place because no committed record names this id); the factor base (`factor_base`) and the elimination (`linear_algebra`) are phases with no group operation | every other operation, exactly: `F_p` multiplications building the factor base (`factor_base_fp_muls_uncharged`) and in the smoothness oracle — splitting `u`, evaluating `v` at its roots, inversions and square roots included (`oracle_fp_muls_uncharged`); column lookups and `±` canonicalisations per root; the walk's random draws (`step_draws_uncharged`); multiplications and inversions mod `r` in the Markowitz elimination (`linalg_mulmods_uncharged`, `linalg_inversions_uncharged`); with `large_primes = 1`, partial-table lookups and the combination's mulmods. The record is a lower bound at `w = 0` |

**Hyperelliptic Jacobians** (`ecbench::hyper`, family `hyperelliptic`).
`{"kind": "hyperelliptic_thread", "p": 41, "genus": 2}` builds the
hyperelliptic index-calculus thread's instance
(`hyperelliptic_ic_bench::thread_instance(p, genus)`: `y² = f(x)`,
`deg f = 2g + 1`, `g ∈ {2, 3, 4}`, `p < 256`, its first `c` and its
generator `D₁` of prime order `r`), so the generic arms run on the very
Jacobians the thread's IC is compared on. `#J = h·r` comes from the
L-polynomial: its first `g − 1` coefficients from point counts over
`F_{p^k}`, `k < g`, leave one unknown, and of the multiples of `r` in that
window and the Hasse–Weil interval exactly one must kill 32 random
divisors, or the build fails (`HCurve::jacobian_order`). Measured children
rebuild from `hyperelliptic_explicit` (`f`, `#J`, `r`, the generator's
`gu`/`gv`), so they repeat neither the thread search nor the counts. A
divisor is two hex words: word 0 is `deg u` (bits 0–7) and the non-leading
coefficients of `u` (bytes 1–4), word 1 the coefficients of `v` (bytes
0–3). The group key, which the distinguished points, the jump partition,
the BSGS tables and the `{D, −D}` choice all read, is injective:
`Σ_{e<deg} p^{2e}` plus the base-`p` index of `(u, v)` among degree-`deg`
pairs. The name keeps ICV1's shape with a `hyp` tag
(`icv1-hypg<g>p<p>-n<#J>-<8 hex>`) and is never registered; targets are
planted only (no public-target law), and every IC method but
`ic.hyperelliptic_thread` and every endomorphism walk is refused at plan time.
`ic.hyperelliptic_thread` (`ecbench::hyper_ic`) is the thread's index calculus
(`hyperelliptic_index_calculus`, `RelationSearch::FactorBaseWalk`, sparse
solve) run on the counted Cantor group with every search parameter required
(`fb_size`, `large_primes`, `extra_relations`, `branches`, `d_step_in`); on a
run that solves in its first round its trials, relations and group
operations equal the thread's `hec_index_calculus_dlp` with the same seed
(tested), and where the rows leave `k` free it keeps walking instead of
restarting from the seed as the thread does. It refuses an instance with
`r² | #J`, where relations mod `r` need not have a solution.
`A = 2`: the hyperelliptic involution is the negation; a curve with more
automorphisms is not detected.

`rho.multitarget_pooled` is registered `applies: prime_only` (single-word
prime-field curves, the only instance family it implements): the planner
refuses a binary or extension workload before a session starts, and `solve`
refuses it again. It is the one method whose run solves more than its
record's target. The record's target is solved last, after `L − 1` companion
targets hashed to `⟨G⟩` by the public-target law (indices `2^40 + j`, the
algorithm seed as the law's seed), whose logarithms nobody holds; the runner
verifies the record's target as for every method, each companion's log is
checked inside the run and written with its point under
`detail.per_target` / `detail.companions`. `S` is the whole batch over `√r`,
so it is compared with `L` single-target runs (`L·S_rho.negation`) or with the
Kuhn–Struik reference `Σ_{i<L} C(2i,i)·4^{−i}·√(π/4)`; its `expected_s` is
unset because it depends on `L`. A batch keeps one distinguished-point table
for all targets (memory grows with `L`).

`ic.koblitz_descent` reproduces `ic price --single-target` stage for stage
(`src/bin/ic/price.rs`, test `the_ecbench_koblitz_descent_pipeline_is_this_pass`:
the same logs, trials, relations, summands scanned, units and descent trials on
the rule v8 `icv1-f2m41-tm2308219-7f48b14a` recipe at three seeds). Its parameters are the rule v8
parameter files' fields; each defaults to `rule_v8`, the file's value at the
six §23 sizes (icv1-f2m41-tm2308219-7f48b14a, icv1-f2m53-tm56619371-dac20a85, icv1-f2m57-tm747311035-c1f545af, icv1-f2m61-t158598901-ab42b6c5, icv1-f2m47-t22705043-f4e44623, icv1-f2m59-tm943548413-98844ecc), and the planner
refuses `rule_v8` anywhere else, where every value must be given. `seed = run`
takes the run's algorithm seed; by default the recipe's seed 201 fixes the
base, the probes and the descent walks, so only the target varies between
runs. The stages keep no `GroupOps` ledger of their own: the method runs them
in a one-thread Rayon pool and charges each by the difference of two
thread-local counter snapshots taken on that pool's only worker
(`src/cryptanalysis/group_op_count.rs` defines one count). Its completeness
test (`koblitz_descent::tests`) checks every phase against an independent
reconstruction: exact for `factor_base` (cofactor ladders), `oracle_setup`
(one operation per stored pair), `linear_algebra` (relation and column-log
re-checks), `descent_setup` and `verify`; a lower bound for `relations` and
`descent`, whose hit recoveries are data-dependent. The pair table is the
memory side of the trade: `|F|²/(4n)` folded entries at about 4.5 bytes each.

The counted IC method has its own method identity so frozen SAT/PDP sessions
retain their exact parameter and accounting contract. It removes solver wall
pricing from the reported group-addition equivalents. Solver conflicts and
other work without a pinned conversion stay visible as unpriced counters;
the recorded `S` is therefore a lower bound on the complete pipeline cost.

The standing strong-rho implementation applies paired normal-basis maps and
paired coefficient reductions during signed-Frobenius canonicalization. The
[2026-10-14 optimization study](../../research/ecbench_rho_optimized_20261014/README.md)
records identical trajectories and counts over 240 matched pairs and a
complete-solve Callgrind ratio of 0.802155 against the same-binary legacy
path. That is implementation evidence in `callgrind.Ir`; it does not change
the generic-group floor or turn uncharged canonicalization into charged GAE.

`ic.pipeline` defaults to `linalg=incremental-gauss`, which stops when the
target scalar is pinned; the factor-base logs can still be underdetermined.
`linalg=incremental-gauss-full-rank` uses the same matrix and relation stream
but reports success only at rank `factor_base.columns + 1`. It records the
first target-pin rank, trial and relation as well as final rank. A trial-cap
exit after the early pin is an exhausted full-rank attempt. This distinct
parameter value changes the method identity and preserves old replays.
`linalg=incremental-gauss-full-rank-checked` follows the same relation
stream and stop rule, then verifies one nonzero-coefficient point per base
column by checking `[coefficient × column_log]G = [cofactor]P`. It charges
both scalar multiplications and records checked, missing and failing
columns in the verification phase. The existing full-rank mode keeps its
historical accounting and replay identity.

`linalg=sparse-cado` runs the large-matrix pipeline from
`koblitz_sparse_la`: CADO-NFS-style filtering (duplicates, singletons,
clique purge, then a Markowitz merge that combines rows along a spanning
tree) followed by block Wiedemann on the core. That pipeline was measured
1.93× faster than CADO-NFS 3.0's own `purge`/`merge-dl`/`replay-dl`/`bwc`
at 262,144 columns
([`research/la_engines_20261014`](../../research/la_engines_20261014/REPORT.pdf);
the leaderboard's linear-algebra section). It is a batch solver: it
buffers the relations and re-solves once every column appears and the rows
number at least the columns, then again every `max(1, columns / 32)`
relations. It answers only at full column rank, so it runs under the
full-rank stop rule and its trials compare with
`incremental-gauss-full-rank`, not with the target-pinned default. A
solution is accepted when two independently seeded solves return the
same vector. `solve_sparse_system` checks every row but does not certify
rank, and in a rank-deficient system two seeded solves differ by a random
kernel element. Work is charged in the same `row_ops` unit as the
eliminations: one per entry multiply-subtract in merges, folding, sparse
products (non-zeros × block lanes), checks and reconstruction, summed
over every solve tried. The `block × block` arithmetic of the approximant
basis is not counted, just as the dense matrix's column scans are not.
The recorded `S` prices those `row_ops` with the calibrated
`ns_per_row_op`. The same plug-in is selectable as
`ic bench --linalg sparse-cado`, and
[`specs/smoke-prime-sparse-cado.json`](specs/smoke-prime-sparse-cado.json)
runs it beside `incremental-gauss-full-rank` on one relation stream (a
plumbing check, not a performance claim).

`compact-orbit-scan:columns=N,raw_x_cap=M` builds a Koblitz factor base
from a bounded raw-abscissa scan. Its cofactor projections, subgroup checks
and Frobenius eigenvalue checks are counted group operations. The factor-base
phase also records scan, lift, field, hash and vector-storage counters;
unpinned work keeps the reported `S` a lower bound. The
`mitm-frobenius-counted:m=3` oracle uses the existing folded table and
probe but adds canonicalisation, Frobenius-map, lookup, entry and
representative counts to `oracle_setup`. Use `mitm-frobenius:m=3` on the
same base as its accounting control.

`mitm-fiber:fiber=closed|lifts|blind`, `mitm-frobenius-fiber:fiber=closed|lifts|blind`
and `descent-algebraic-fiber:fiber=combined|lifts|blind` (with a `solver`) are
the cofactor-fiber-aware oracles of `ic_framework::fiber`
(`research/isogeny_conductor_gap_ic_20261010/`).  A subgroup target `T` is
decomposed as `Σ P_i = T + K` for any `K` in the rational `h`-torsion, which
the relation loop's `[h]` projection makes an exact row: `closed` holds
`P_i + P_j + K` for every `K` in the pair table (one probe, `h` times the
entries), `lifts` probes or descends every lift (`h` calls), `combined`
descends one system with the target abscissa free and the fiber polynomial
adjoined, and `blind` is the landed oracle.  The fiber the oracle found is
in `oracle_setup` (`fiber_size`, `fiber_complete`); with `h = 1` every mode
is the blind oracle.  All three take `fiber_gen=legacy|coset` and
`fiber_scale=d` (`research/isogeny_conductor_gap_round2_20261011/`).
`legacy`, the default, is the round-1 subgroup closure, kept so that
committed sessions replay.  It overcounts set-up when `E(F_q)[h]` needs
several generators.  `coset` stops the closure at the first multiple already
in the subgroup.  `fiber_scale=d` (coset only) uses the subgroup
`[d]E(F_q)[h]` instead of the whole fiber.

`koblitz-standard-subspace:dimension=d` with
`pdp3-koblitz:m=3,engine=inherited-f4|f6-ic,degree=D,node_budget=N` runs
PR #1333's Groebner decomposers unmodified inside `ic.pipeline`, with no
default for any parameter that changes the search. The base is every point
whose abscissa lies in `span(1, z, …, z^{d−1})`, folded by negation only. It
is the base #1333's workers used.

- **Charged:** every point addition the decomposition performs, including
  F6-IC's `geometric_group_additions` (support closure, residual arithmetic
  and witness replay), plus the framework's sign lift of each witness.
- **Counted, not charged:** the Boolean solver's word XORs, under the
  inherited-F4 adapters' unit string, so `S` is a lower bound and names
  that unit in `unpriced`. Reductions, splits, propagations and every F6
  gate counter ride in the record's `solver.extra`.

Both engines are exact on the same base and seed, so the two arms issue the
same queries until one exhausts its node budget. The base and oracle live in the
`ecbench` binary (`src/bin/ecbench/pdp3_koblitz.rs`) and reach `ic.pipeline`
through `methods::register_binary_plugins`. The library cannot name them:
frozen-source replays rebuild it against pre-F6 snapshots of
`koblitz_index_calculus.rs`, `Cargo.toml` is pinned by other frozen
evaluations, and the tournament admits no root build script. It also
refuses to run while a `KIC_*`, `F4_*` or `SOLVER_*` tuning override is set.

`structured-s4-base-v1:dimension=L,family=geometric,seed=S` and
`structured-s4-oracle-v1:m=3,form=symmetric,degree=2,preclose=1,node_budget=20000,max_solutions=4096,max_cols=30000`
run the geometric/product-space follow-up in the complete native pipeline.
All search parameters are mandatory. `form=direct,preclose=0` is its direct
S4 control; `mitm:m=3,negation_folded=1` is the compatible complete pair-table
control on the same base. The standard-subspace-only `pdp3-koblitz` adapter
refuses a different geometric base. Extraction above 21 variables is refused.

The new adapter charges signed lifts through GroupOps and reports closure,
readback and hybrid word operations in the inherited-F4 unit. Symbolic
construction and cubic-split field arithmetic are not completely counted:
S remains a lower bound, and the common native-work band does not calibrate
those omissions. See the [protocol and accounting scope](../../research/ic_algebraic_crossover_20261023/PIPELINE.md).

`prime-kummer:t=T[,g=G]` and `prime-isogeny:t=T[,seed=S]` are Petit,
Kosters and Messeng's tower bases for prime curves
(`ic_framework::prime_tower`): every point whose abscissa lies in a set `V`
of `2^t` elements that is the zero set of `t` composed degree-2 maps, both
signs, one column an abscissa.  The Kummer base needs `2^t | p − 1` and
costs `2^t` Legendre symbols and square roots.  The isogeny base,
`V = x(R' + ⟨Q⟩)` for `Q` of order `2^t` on an auxiliary curve `E'/F_p`,
needs no condition on `p`; `E'` is searched from the public seed, and the
search is **charged to the factor-base phase**: on every curve tried, a
baby-step giant-step over the multiples of `2^t` in the Hasse interval
filters out curves with `2^t ∤ #E'`, and the survivors' orders are computed
by Mestre's baby-step giant-step; with `Q`, `⟨Q⟩`, `R'`, `V` and the Vélu
chain's kernels, every addition and doubling on `E'` is charged in the unit
(a curve over the same field).  The phase records `aux_curves_tried`,
`aux_search_gae`, `aux_count_gae` and `aux_tower_build_gae` apart; the
Legendre symbols and square roots of point sampling join the phase's
`legendre_symbols` and `sqrt_solves` (unpriced, as for every prime base),
and the `2t` Vélu evaluations are `velu_evaluations_uncharged`
(`research/pkm_isogeny_tower_ecbench_20261015/`).

`rootfind-mitm:m=3` and `rootfind-pairs:m=3` are the root-finding
decomposition oracles of `research/pdp_root_finder_20261012` as plug-ins
(`ic_framework::rootfind`), on any base and both tracks. `rootfind-mitm`
builds the negation-folded pair table once (`full_table=1` for every
`i ≤ j` pair) and, per target, computes the points `R ∓ P_k` — one
representative `P_k` an abscissa — in blocks of 32 abscissae, probes each
and stops at the first hit;
`rootfind-pairs` keeps no table and tests every candidate `U ∓ P_j` of
every row point `U = R ∓ P_i` for membership in the base: the enumeration
floor, the memoryless control.

- **Charged:** every point addition, one each: the table build, the
  additions a target (`2A` for a refutation, the count `mitm:m=3` charges
  over the `2A` signed points; on a hit at most one block more than
  `mitm`'s early exit), the probe's confirming addition on a key hit, and
  every candidate of `rootfind-pairs`.  The cost is complete: on a binary
  curve the group computes a block with one shared inversion
  (`FastCurve::add_many`), which is how the charged additions are done,
  not work beside them; on a prime curve the default path inverts once an
  addition.
- **Counted, not charged:** table lookups and membership tests
  (`lookups`).  What the new ids change against `mitm` is therefore the
  arithmetic behind the same charged additions, which wall time sees and
  `S` does not; and the memoryless three-summand arm, which the harness
  had not had.

**The root-finding decomposition engine.**
`RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` replaces the Gröbner solve of the
decomposition step by root finding, and two oracle ids run it here on the
bases the Gröbner arms use, so the engines meet on one query stream
(`research/ecbench_rootfind_engine_20261012/`):

- `pdp3-koblitz:m=2|3,engine=rootfind` on `koblitz-standard-subspace` is
  `DecompositionStrategy::PairTable`, the strategy `koblitz_index_calculus_dlp`
  runs by default: the base's pair sums `P_i + P_j` tabulated once
  (`PairSumTable::build_full_within`, the summand-storing tier), a target
  decomposed by one table probe (`m = 2`) or one per `R − P_k` (`m = 3`).
  It takes no `degree` and no `node_budget` and refuses them. **Charged:**
  the table's `|F|(|F|+1)/2` pair additions, counted from the base's size
  because the batched builder carries no ledger (`|F|` of them doublings);
  for `m = 3` every `R − P_k` on the ledger until the first hit; and the
  re-addition of every hit in the group before it is returned.
  **Counted, not charged:** one `lookup` per probe (priced only where the
  curve has a pinned ratio, like every table oracle's) and the table's
  entry count in `oracle_setup`. The record has no `solver` block.
- `pdp3-koblitz:m=3,engine=rootfind-window` is the same table and set-up
  charge with the cyclic third-summand window of
  `PairSumTable::decompose_fast_window`: `⌈|F|/3⌉` probes a target, started
  at `x(R) mod |F|` (deterministic in the public point). It is a different
  search from full `rootfind` (and from inherited-F4), so it is a separate
  engine id and is not compared on query-stream identity. The native
  counters record `pair_table_points` and `scan_window_len`. Improve-loop
  evidence: `research/ecbench_rootfind_improve_20261013/`.
- `rootfind-subtract:m=2|3` on a prime curve is `prime_fast`'s
  direct-subtraction root finding: the Semaev `S₃` root in the third
  variable is `x(R ∓ F_i)`, so every candidate of a target comes from one
  Montgomery batched inversion over the base's abscissae, each looked up
  in the base (`m = 2`) or probed against the negation-folded pair table
  (`m = 3`). It is `subtract`'s search, candidate for candidate in the same
  order, so the two arms issue the same queries and find the same
  relations; it is charged as `subtract` is (one addition per candidate
  point) plus one re-addition per hit, so their `S` differ by one addition
  per relation and nothing else. The difference the unit does not see, one
  inversion a target against `2|F_x|`, is in the `inversions` counter, the
  record's field-operation tally and wall time.

Read a root-finding arm against a Gröbner arm on the resource vector and on
wall time: the root finder's `S` is a complete count of group additions,
while a Gröbner arm's `S` is a lower bound (its word XORs are counted and
not charged), and the ratio of a complete count to a lower bound is not a
speedup. Against `mitm`, whose generic pair table does the same search in
the framework's own arithmetic, it is an *engineering* comparison in the
sense of AGENTS.md §3: the same work in a faster constant. `ecbench` has no
prime-curve Gröbner oracle (the note's Buchberger baseline terminates only
at `|F| = 6`, too small for a DLP), so the prime side is compared with the
existing `subtract` and `mitm` oracles.

**Accounting v2.** Some rules above had gaps that a registered method's id
freezes: counts are compared on replay, so a fix is a new id, and the old id
keeps its records and says what it leaves out. The `_v2` ids below are the ones
to read new work against; every phase sum equals the total for them
(`accounting_tests::the_listed_methods_phases_sum_to_their_totals`).

| old id (frozen) | new id | what changes |
|---|---|---|
| `rho.cubic`, `rho.quartic` | `rho.cubic_v2`, `rho.quartic_v2` | the class set-up `[κ]G` (`CubicClasses::new` / `QuarticClasses::new`) is in the total under both. The frozen ids have charged it since 2026-10-18, in the `setup` phase (older records put it in the `setup` phase only, not in `total_gae`, and replay through `audit::legacy_class_setup`; see [`CLASS_SETUP_AUDIT.md`](CLASS_SETUP_AUDIT.md)); the `_v2` ids charge it in its own `automorphism_setup` phase, together with the `_v2` walk of the next row. Before 2026-10-18 the frozen ids put the ledger in `search` after reading `gae`, so those records' phases sum to 26 GAE (3 % of `S` at 2^20) more than their total |
| `rho.negation`, `rho.signed_frobenius`, + the two above | `rho.negation_v2`, `rho.signed_frobenius_v2` + the two above | Brent cycle detection (one stored checkpoint at powers of two, the cycle left from its least-key member) beside the 16-point window, so a cycle longer than the window is escaped and not run to the walk cap (`20·2^dp + 64`); `lost_walk_steps` (and by cause: `steps_in_walks_capped`, `…_abandoned_in_a_cycle`, `…_ending_at_the_identity`, `…_out_of_budget`) are charged counters (the work is in `walk_operations`), `cycles_longer_than_window` counts the detections, and the distinguished-point table's `…_table_queries_uncharged` and `…_inserts_uncharged` are counted. On the review's n = 37 runs 4 of 80 hit capped walks and the mean rose 14 % |
| `rho.signed_frobenius_strong` | `rho.signed_frobenius_strong_v2` | lanes, distinguished-point bits and the jump table scale with `r` (`auto`, see below); the jump table is `[s]G` for distinct scalars from one table of `[2^i]G`, every doubling and addition counted (`1.5·log₂ r` a jump in the frozen id); the `lanes − 1` start multiples are charged (not in the frozen id); candidate checks are a `verification` phase; an exhausted solve records the work it spent (the frozen id records total 0 and no phases) |
| `bsgs.textbook`, `bsgs.interleaved`, `bsgs.negation` | `…_v2` | one charged final `[d]G = Q` (phase `verification`, about `1.5·log₂ r` GAE, 2.7 % of `S` at 2^20), which rho and index calculus already paid for their answer |
| `kangaroo.vow` | `kangaroo.vow_v2` | the final check, and restarts in their own phase `restarts`. The frozen id charges a restart to `search` (since 2026-10-18; to `setup` before, total unchanged) and runs no final check; the `_v2` id's `search` holds jumps only |
| `claw.pair_table` | `claw.pair_table_v2` | the final check; the `max_adds` budget counts the `recover` phase too (the frozen id lets a run spend past it by its recover additions) |
| `ic.large_prime` | `ic.large_prime_v2` | the target's `[r]Q` subgroup-membership check, which the frozen id ran on a ledger nobody read, and the final `[d]G = Q` are counted scalar multiplications in the `verification` phase |

*One rule for verification.* A method that returns an answer charges one final
`[d]G = Q` scalar multiplication, and every failed candidate check, in a phase
named `verification` (rho: `internal_verification`). It is a property of the
method id and not a harness phase added to every arm: rho and index calculus
already pay it inside their frozen ids, so a harness phase would charge them
twice unless it knew which methods verify (a per-method flag again), and it
would have to be versioned through the plan, the record and the audit's
`S = total / √r` recomputation, each owned elsewhere and each a new way for a
historical record to stop recomputing. A `_v2` id carries the rule in the
identity the audit, the replay and the run ids already hash, and one
registry-wide test pins it.

*The strong reference's `auto` shape.* With `ideal = √(πr/2A)` expected steps:
`dp_bits = round(log₂(ideal)/2)` (1 to 20), `lanes` the power of two at most
`ideal / 2^(dp_bits+3)` (1 to 32), and `jumps` the largest of 32, 16, 8 whose
fixed set-up (`jump table + 3·log₂ r + 1.5·log₂ r`, the two start
multiplications and the final check) is at most a tenth of `ideal`, never
fewer than 8; the table repeats its distinct jumps to the reference's 32
entries. Where `r` is too small for that (below about 2^31) the set-up share is
not hidden by a worse walk: it is in each record's `detail.setup_share`.
`methods::setup_share`, `row_flags` and `calibrated_log2_r` compute the
flag a table row should carry (fixed set-up above 10 % of `S`, or `log₂ r`
below the calibrated range: prime `r < 2^25`, Koblitz `r < 2^39`), and
`ecbench table` prints it in its last column, `flags`. A row pools its verified
runs: the share is the summed `setup` and `automorphism_setup` GAE over the
summed totals (`methods::pooled_row_flags`). At the review's sizes the frozen reference was 58× (n = 13) and 15.5×
(n = 19) theory, almost all of it 32 full-size jump scalars; at n = 13, with
about eleven expected steps, even eight jumps cost more than the search.

*The factor-base inventory.* An index-calculus run names its factor base in
the record (`factor_base`: the `FB1h…` id, size and description). The measured
child used to rebuild the base for that after `run_pipeline`, before closing
its scheduler, hardware-counter and Callgrind windows, so those, and
`max_rss_kib`, included a second build that is neither the solve nor counted
in `S`. `ic.pipeline` now builds the base once and describes the base the run
used (`run_pipeline_with_base`): no second build, but the description (hashing
the base) still falls after the timed solve (`wall_ns`, the online phases) and
inside those process windows. `ic.shared_rank` still rebuilds its inventory, so
`methods::solve_measured` returns without it and the child calls
`methods::complete` after every window has closed (`methods::solve` calls both
for any other caller). No count changes: a replay of the committed
`ic.pipeline` and `ic.shared_rank` sessions is identical.

**Calibration.** The unit has been checked against theory in
[`research/ecbench_calibration_20261002`](../../research/ecbench_calibration_20261002/README.md).
Over 2 384 verified runs, preregistered and replayed, every generic
method's mean `S` reaches its analysed constant within its 95 % interval
at `r ≈ 2^25.4` (prime) and `r = 2^39` (Koblitz): plain rho
`√(π/2)`, negation rho `√(π/4)`, signed-Frobenius rho `√(π/4m)` (at 1.006
of it), BSGS 1.5, 4/3 and 1, kangaroo 2. Round 1 missed two
preregistered bands for sample-size reasons, and that record is kept.

Two consequences to read every table with:

- **BSGS's `S` omits its memory.** BSGS stores about `√r` points and
  probes a hash table every step. Neither is a group operation, so a
  BSGS row can sit below rho in `S` and still be the slower and far
  larger method at any interesting size. The records keep `max_rss_kib`
  and the lookup counts. This is the known time–memory trade, never a
  finding.
- **The pair claw's `S` omits its memory too.** `claw.pair_table`, ported
  from `aburan28/cryptanalysis#175`, holds `M = table_scale·√(r/n)` classes.
  Every base logarithm is known, so it is a generic method (a randomised
  BSGS on signed Frobenius classes) whose mean is about
  `(c + 1/c)/√n` at `c = table_scale`, at best `2/√n` against rho's
  `√(π/4n)`. It is in the `claw` family, not `ic`, and it is not an
  index-calculus candidate for a `vs_rho` claim.
- **A lower bound is marked.** `cost.lower_bound` is set whenever
  anything is unpriced, and comparisons carry `bounded: true`. A rho with
  the negation map is a lower bound by this rule (its canonicalisations
  are counted, not charged), exactly as in the rest of the repository.
  The ratio of two such lower bounds is **not** a bound on the ratio of
  full costs. Read it as a counted-work diagnostic until both arms have
  complete, calibrated costs; do not infer an end-to-end speedup from it.

- **Full pricing is opt-in.** `ic.pipeline_full`,
  `rho.signed_frobenius_full` and `rho.signed_frobenius_strong_full` run
  their base method unchanged, then price every native counter (field
  multiplications, squarings and inversions, Artin–Schreier solves, hash
  probes, canonicalisations, row operations, word XORs) from
  `src/cryptanalysis/ecbench/full_prices.json`, one host's measured ratios to an affine
  addition, written by `ecbench price-units --curve a:n …`.
  `src/cryptanalysis/ecbench/full_price.rs` gives each counter an explicit
  rule. A counter without a rule stays unpriced, so the record is still
  marked a lower bound. The pinned `docs/ic/calibration.json` and every
  other method are unchanged. `ecbench trend --compare …` fits how a paired
  ratio moves with `ln r` and `ln n` across curves. First use:
  `research/ecbench_frobenius_orbit_vs_rho_20261014/`.

- **Word-RAM reference pricing is the host-independent alternative.**
  `ic.pipeline_wordram`, `rho.signed_frobenius_wordram` and
  `rho.signed_frobenius_strong_wordram` run the same base solvers and
  charge the same counters by the same rules as the `*_full` methods.
  They take each unit's price from fixed formulas, not from one host's
  timings. The scheme is
  [CurveSmash's](https://github.com/zac-williamson/curvesmash/blob/main/docs/COST_MODEL.md)
  (after HashSmash). The curve's affine addition is the reference operation
  and costs 1. Its budget `C`, in operations on a 256-bit word RAM, is fixed by
  formula. Every other unit costs its own word-operation budget divided
  by `C`. For a `k`-bit field in `s` words: `A = 128(s+1)`,
  `M = k(2A+64) + 128(s+1)` and `C = I + 16M + 32A + 4096`.
  Prime fields use CurveSmash's Fermat inversion,
  `I = 2k(M+64) + 128(s+1)`, unchanged; at `k = 256` this gives its
  secp256k1 price, `C = 78,037,248`, and a unit test holds both published
  constants. In `GF(2^m)` a squaring is linear, so `S = 8k + 128(s+1)`
  and `I = k(S+M+128) + 128(s+1)`. Frobenius maps, signed-Frobenius
  canonicalisations, half-trace Artin–Schreier solves, hash probes,
  row operations modulo `r` and word XORs are built from those budgets
  (`src/cryptanalysis/ecbench/wordram_price.rs` documents each one).
  The record's `detail.wordram_pricing` gives every budget, `C` and the
  charge by counter. `ecbench wordram-prices --curve 0:31 …` prints the
  table for any Koblitz curve; nothing is measured.
  These are conservative portable allowances that define the unit, not
  timings. The word RAM charges one operation per memory transfer
  whatever the cache level, so a hash probe is much cheaper here than in
  `full_prices.json`. Native work is priced in the same unit on every
  host and needs no `price-units` run, but its rate is a modelling
  choice: §5a's band (0, 0.1 and 1 GAE per native operation) still
  decides any ranking, and a word-RAM total is one point to quote beside
  it, not a replacement for it. A counter with no rule stays unpriced,
  and the record stays marked as a lower bound.

`ecbench resources --dir SESSION --out resources.json` reports, for every
measured attempt including failures, the native integer counters by phase,
charged GAE and clocks by phase, the target's exclusive online window,
method counters, fork-to-reap and solve time, CPU time, peak resident memory,
and solve-wide instructions and cycles when the host exposes a PMU. A PMU
sum is null unless **every** measured run in that arm has the counter.
These are separate units: a SAT conflict, table lookup and byte of memory
do not have universal GAE prices. The resource vector shows where the
counted `S` is incomplete. A full runtime comparison still requires
matched L2/L3 runs and the A/A noise gate in §8; the Mac's L0 timings
remain descriptive.

## 7a. Curves of higher genus and transfers

Four workload kinds and four methods carry ecbench past elliptic curves
over one field. Every one is charged in the same unit as rho, verified by
`[k]G = Q` outside the method, and replayed exactly by `verify`.

| `kind` | the group | element encoding | name |
|---|---|---|---|
| `genus2_search` `{p_bits 5–16, seed, max_cofactor}`, `genus2_explicit` | `J(F_p)` of an imaginary genus-2 curve `y² = f(x)`, `deg f = 5`; `#J = h·r` by point counts over `F_p`, `F_{p²}` | Mumford `(u, v)` as `[deg u·p² + u₁p + u₀, v₁p + v₀]` | `jcv1-g2-fp<bits>-<sha8>` |
| `plane_quartic_search` `{p_bits 5–11, seed, max_cofactor}`, `plane_quartic_explicit` | `J(F_p)` of a smooth plane quartic (non-hyperelliptic genus 3), Khuri-Makdisi-style linear-algebra group law; `#J = L(1)` from point counts over `F_p`, `F_{p²}`, `F_{p³}` and a BSGS for `a₃` | a class as the 3×10 echelon basis of `H⁰(3K − E)`, two 120-digit hex strings | `jcv1-pq-fp<bits>-<sha8>` |
| `ghs_fp6_search` `{p, seed}`, `ghs_fp6_explicit` | `E : y² = x(x − α)(x − σα)` over `F_{q³}`, `q = p²`, `α ∉ F_q`, `#E = 4ℓ` (Joux–Vitse curves, `jv_cover`); prime `11 ≤ p ≤ 1600`, `p ≠ 3` | six base-`p` digits per coordinate, one word | `ghs-fp<p>x6-<sha8>` |

None of these is an ICV1 curve, so none is registered (`registered:
false`, no EC1 alias); `ecbench run`'s note about unregistered slugs counts
`icv1-` slugs only.

**The unit on a Jacobian.** One Jacobian group operation (a Cantor or a
linear-algebra addition, reduced) is one GAE, as one point addition is on
a curve. A genus-2 or plane-quartic row's `S` is therefore in Jacobian
operations. It is not in additions on some elliptic curve. Compare it with
rho on the same Jacobian (the generic methods all run there), not with an
elliptic-curve row. On a plane quartic, negation is a flip against a fixed
section and costs about one group operation, so `rho.negation` and
`bsgs.negation` are refused at plan time; use `rho.plain` or
`bsgs.textbook`.

**The unit in a transfer.** A transfer's row is a row of the *source*
curve, in its unit:

- `transfer.isogeny`: the isogenous curve is an elliptic curve over the
  same field, so the inner solve's GAE are the source's GAE. The kernel
  search and the maps are field work, recorded unpriced.
- `transfer.quartic_cover` and `transfer.ghs_cover`: the Jacobian's
  operations are not additions on `E`. The method counts every `F_p`
  multiplication of the cover, the pullbacks and the inner solve with the
  arithmetic's own counter, and divides by `c_E`. `c_E` is the
  multiplications of one affine addition on `E`, measured in the same run
  on 64 additions with the same counter (`jv_cover::unit_costs`'
  convention). An inversion is counted as what it costs in that arithmetic
  (Fermat on `F_p` for the quartic; one `F_p` inversion at 16 for the GHS
  tower). `detail.{quartic_cover,ghs_cover}` records `c_E`, the
  multiplications by phase and `e_adds_per_jacobian_op`.

**What to expect.** These transfers move the problem into a group with
the same `r`-part, so a generic inner method pays rho's `√r` times the
cost of one Jacobian operation in `E`-additions. On the committed smoke
sessions that factor was about 6.4 for the genus-3 GHS cover over
`F_{19²}`, and about 13,000 for the plane-quartic cover at 16 bits, whose
group law is dense linear algebra. A transfer pays off only when the
target group admits an algorithm that is asymptotically better than rho:
index calculus on the Jacobian. `ic.hyperelliptic` is that algorithm in
genus 2. The Joux–Vitse decomposition index calculus on the genus-3 GHS
cover is in `jv_cover` and its binaries, not yet an inner method here.
No index calculus on a plane quartic (Diem's) is implemented. Until one
is, a transfer row measures what the route costs today. It does not show
the route is cheaper.

```json
{"schema": "ecbench.spec/v1", "label": "GHS cover vs rho on E(F_{p^6})",
 "workloads": {"curves": [{"kind": "ghs_fp6_search", "p": 19, "seed": 1}],
               "targets_per_curve": 3, "target_seed": 7},
 "arms": [
  {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
  {"name": "ghs", "role": "candidate", "method": {"id": "transfer.ghs_cover",
                                                   "params": {"inner": "rho.negation"}}}],
 "measurement": {"rounds": 2, "warmup": 0, "seed": 1, "isolation_required": "L0",
                 "timeout_seconds": 300}}
```

Every arm of a session runs on every curve, so a session holds one
family; `plan` refuses an arm its curve cannot run, with the reason (no
rational ℓ-isogeny, a negation the Jacobian does not have, a family the
method does not take).

## 8. Statistics

- **The mean, not the median.** The floor `√(πr/2A)` is an expected
  value, so the comparable statistic is the mean `S`.
- **Paired, and a ratio of totals.** Runs are matched by (workload,
  round). Within a session the two arms of a round share an algorithm
  seed; across two sessions of one spec (a baseline binary and a
  candidate) they do too, and `same_seeds` says so. The ratio is
  `Σ S_B / Σ S_A` over the matched pairs, AGENTS.md §8's
  `baseline_total / candidate_total` in `S`. It is never a mean of
  ratios, which is biased whenever the arms spread differently (rho's
  cost varies with its seed, BSGS's does not).
- **The primary interval is linearised (`ecbench.comparison/v2`).** With
  workloads as clusters, `A_k = Σ S_A` and `B_k = Σ S_B` over cluster `k`,
  `R = ΣB/ΣA`, the influence values are `z_k = (B_k − R·A_k) / mean(A)`,
  the standard error is `sd(z)/√K` and the interval
  `R · exp(± t(K−1, 0.975) · se/R)`, the same linearisation read on the
  log scale so that a ratio of costs never gets a non-positive end (on the
  ratio scale, `R ± t·se`, the coverage is the same: 0.957, 0.948, 0.947
  at K = 2, 4, 8 against 0.956, 0.946, 0.950; the quantile is computed in
  `stats.rs` and tested against the published table). The reason is coverage: the percentile two-stage bootstrap of
  the ratio of totals covers 85 % at two workloads and 92–93 % at three
  or four (it over-covers at eight or more), where the linearised interval
  covers 94–96 % at every `K`; `stats::tests` pins this with a seeded
  Monte-Carlo (rho-like Rayleigh costs against BSGS-like per-target costs).
  `ci_method` is `linearised_t`; the bootstrap interval of the same
  statistic stays beside it as `ci95_bootstrap` (`cluster`, or `within`
  with one workload).
- **Few workloads are indicative.** Below five workloads `ops.indicative`
  is true and the verdict says `INDICATIVE`: the t quantile at one or two
  degrees of freedom is 12.7 or 4.3, so the interval is honest and wide.
  With one workload the clusters are the pairs (`ci_method: within_t`,
  `interval_scope: within_workload`): the interval covers seeds on that
  target and says nothing about other targets. Runs without a partner
  make the comparison `partial`.
- **Censored runs bound the ratio.** Ratios are not formed over verified
  pairs alone: a timeout or an exhausted budget drops the arm's most
  expensive draw and biases the survivors low. An exhausted run enters at
  the cost it spent (its record carries it); a timed-out run carries no
  cost, so it enters at the cost its wall time implies (process wall minus
  the arm's largest fork-and-setup overhead, divided by the arm's largest
  ns per GAE over its verified runs, so it is a lower bound; `0` when the
  arm has no verified run to convert with). The pair is `censored`,
  `ops.censored_pairs` counts them, and `ops.bound` says which way the
  ratio is bounded: `lower` when only B is censored (Σ S_B is a lower
  bound), `upper` when only A is, `indeterminate` when both are. The
  verdict says `BOUND` whenever any pair is censored, the status stays
  `incomplete` (not admissible), and `verified_only_ratio` keeps the old
  figure. The interval of a bounded ratio is that of the bounded
  statistic, not a coverage statement about the true ratio.
- **Tail diagnostics.** Each arm reports max/median `S` and the largest
  run's share of the arm's total (`tail`). Above 25 % the verdict adds
  `heavy tail; more runs needed`: the mean is carried by one run.
- **The pricing band (AGENTS.md §5a).** Each arm lists every unpriced
  counter with its count (`unpriced_counts`, summed over verified runs;
  `null` is a counter with no count). `ops.band` prices that native work,
  and any pinned-calibration work, at 0, 0.1 and 1 GAE per operation on
  every arm alike (the same function as `ecbench band`) and reports the
  ratio of totals at each weight; `order` names a winner only when it
  holds at every weight and otherwise says `unresolved inside the band`
  (or `unrankable` when a counter is uncounted). It is over the verified
  pairs, with censored pairs excluded.
- **Sessions must be complete.** `compare` refuses a session whose status
  is not `complete`. `--allow-interrupted` compares it anyway as
  descriptive evidence: the comparison records the statuses and the flag,
  sets `admissible: false` and the verdict says so.
- **Per curve.** A ratio is reported per curve (one size each) as well as
  pooled. A claim about scaling reads the per-curve rows and fits
  exponents over at least four sizes (AGENTS.md §5).
- **An A/A in operations is exactly 1.** Two arms running one method on
  the same seeds do identical work, so the ops A/A is a check of
  determinism, not of noise. The noise in operations comes from targets
  and seeds, and the intervals above already carry it. The A/A arm is
  for wall time, where it estimates the session's position bias and noise.
- **Wall time is gated, and is a cost per operation.** It is compared
  only within one session, only over pairs where both runs reached
  `isolation_required`, only with at least five such pairs; anything less
  prints as `descriptive`, never as a result. The statistic is each arm's
  `ns per GAE = Σ solve_wall / Σ total_gae` over the admitted pairs
  (`ns_per_gae_a`, `ns_per_gae_b`) and its ratio B/A (`wall.ratio_b_over_a`),
  with a linearised `t(n−1)` interval (log scale) whose clusters are the pairs (the
  timing replicates). It answers what operation counts cannot, the constant
  factor, and does not inherit the work variance of rho: in the deep dive's
  simulation (5 workloads × 3 rounds, 3 % jitter, B truly 5 % slower per
  operation) the per-pair median ratio spans [0.56, 1.54] and the ns/GAE
  ratio [1.025, 1.075]. The v1 headline, the median of per-pair wall
  ratios, is a different estimand from the ops ratio (biased about +6 % for
  a deterministic arm against a random one) and stays only as
  `median_ratio`. The ratio of total wall times (`total_wall_ratio`, with
  `total_wall_ci95`) is the secondary "end-to-end wall" and keeps the work
  variance. When an arm's GAE is a lower bound (unpriced work,
  `ns_per_gae_upper_bound_arms`), its ns per GAE is an upper bound on its
  cost per operation, and the verdict says so.
- **The noise test corrects for bias.** The A/A pair is the pair of arms
  running one method that includes arm A (else one including the
  reference arm, else the first alphabetical; `aa_selection` and `aa_arms`
  record the choice). Its ns/GAE ratio (`aa_ratio`, interval `aa_ci95`;
  needs five admitted pairs) is the systematic bias between two identical
  arms, positions included. `corrected_ratio` is `ratio_b_over_a /
  aa_ratio`, with a `t` interval from the two standard errors combined as
  independent on the log scale (conservative when the A/A shares arm A)
  and Welch–Satterthwaite degrees of freedom; `outside_noise` is whether
  the corrected interval excludes 1. A comparison of two arms that run one
  method is itself an A/A (`self_aa`), with nothing to correct. The v1
  rule (the median interval excludes 1 and does not overlap the A/A
  median interval, roughly p < 0.005 and counting noise twice) is kept as
  `legacy_outside_noise`, with `legacy_ci95`, `legacy_aa_arms` and
  `legacy_aa_ci95`; `legacy_comparison_id` is the v1 id of the same inputs.
  The measurement-quality score reads the A/A interval (the control's own
  interval for a `self_aa` comparison) for its +10 bonus.
  This `compare` wall field uses the whole solve, including IC's reusable
  setup. For the primary one-target IC question, read the separate online
  windows and `ecbench claim`'s same-point interval; a cold wall ratio is
  never an online speedup.
- **Schema.** New comparisons are `ecbench.comparison/v2`; v1 files are
  still read (every added field is optional) and keep their meaning:
  there `ci95` is the percentile bootstrap and `wall.ci95` is the median's.
  `ecbench table` prints each arm's mean `S` with the same linearised
  interval (marked `indicative` below five workloads), flags a mean that
  leaves unverified runs out (`survivors only`) and a heavy tail.

A speedup in the sense of AGENTS.md §8 is still
`baseline_total_operations / candidate_total_operations`, over the whole
cold method with every phase priced. `ecbench compare` reports the ratio.
Classifying the change (advance, engineering, relabelling, accounting)
stays with the author.

## 9. Reproduction and the independent runner

**Exact replay.** Operation counts depend only on the code, the workload
and the seed, never on the host. `ecbench verify --replay N` re-executes
N measured runs and requires the same answer, total, phase counts,
counters, unpriced work and factor base, bit for bit (`--replay-all`
replays every one). Run on another machine, it is an independent check of
the figure. Which runs are replayed is the auditor's choice, never the
record's:

- Whether a run is deterministic is taken from the method registry (the
  method's id and parameters), not from the `cost.deterministic` the record
  declares; a record that declares itself nondeterministic under a
  deterministic method fails the audit. Every measured run of a
  deterministic method is a candidate, whatever its outcome: a `verified`,
  `wrong_answer` or `exhausted` run must reproduce as such (an exhausted
  run must replay to exhausted), and an `error` run must replay to the same
  error. Three committed sessions recorded failed outcomes under code whose
  failure paths have since changed (`audit::FAILED_OUTCOME_REPLAY_EXEMPT`,
  with the reason for each); their failed runs are checked statically only.
  A `timeout` is checked without a replay: it has no total, was
  killed by SIGKILL (`signal 9`) and ran at least the spec's
  `timeout_seconds`; a `crashed` run has no total and a signal or nonzero
  exit; `interrupted` and `spawn_error` runs have no total.
- `--replay N` draws the N runs uniformly at random, from a seed that
  defaults to the operating system's entropy; the seed, the number of
  candidates and `replay_selection` are in the receipt, and
  `--replay-seed` repeats a draw. A session's author cannot know which runs
  will be replayed. (Receipts written before this read
  `candidates[j·len/n]` and have none of these fields.)

Records written before 2026-10-05 by an `ic.pipeline` arm whose solver work
was wall-priced (the descent-algebraic arms of
`research/ecbench_yield_sweep_20261004/sessions/koblitz` and the other
committed sessions listed in `audit::LEGACY_ROUNDING_RECORDS`) carry gae
figures rounded to the binade of that wall term, because the term was
removed by subtraction. The audit reproduces that rounding from the solver
term and pre-removal total the record carries, and reports the replay as
`identical (legacy gae rounding, …)`, only for those exact sealed
`(session id, record id)` pairs, and only when the difference is within a
few ulps of the pre-removal total (and a millionth of the exact total).
A record that is not on the list must replay exactly. New records carry
the exact figure.

The audit also recomputes every derived figure from the record's own
counts (`S = total/√r`, the floor, the ratio, the lower-bound flag) and
checks each record against what needs no replay: phases sum to the total
(except a class-set-up record from one of the named pre-fix producers,
`audit::pre_class_setup_record`, whose replay checks the shortfall),
`automorphisms_used` does not exceed
`automorphisms_available`, `role` is the spec arm's, `env_class_id` and
`binary_sha256` are the host capsule's, `field_ops` is present where the
method counts field operations on a prime curve, and `status_counts`,
`executions_planned` and the `session_id` (recomputed from the spec, host
and plan hashes and the start time) are what the records and plan give.
Every run is regraded from its recorded observations under the grading
version the session records (`regraded` is a check: an unsupported version
fails the audit; version 1 is reproduced from the current rules). A figure
edited by hand and resealed is caught.

An `interrupted` session accepted with `--allow-interrupted` is reported
in the receipt (`warnings`, `interruption_point`, `planned_executions`) and
on stderr as a prefix of its plan that must not be cited as complete, with
the runner's `stop` (kind, seq, message); the records must end at the
runner's `last_seq` (`last_seq` + 1 records; a killed session at most one
more) and at `stop.seq` (that execution recorded or not yet run), and a
complete session has no `stop`. A torn final line of
`records.jsonl` is a problem in the receipt (under `--allow-interrupted`,
for an interrupted or killed session, a warning, with the intact prefix
audited). A record line with a duplicate key, or a number such as `1e400`
that is not a finite float, is refused. The receipt names every file by
SHA-256, and its own SHA-256 is the replay certificate a claim cites.

What CI replays (`.github/workflows/ecbench.yml`):

- **every pull request** that touches ecbench: every committed session,
  found by schema wherever it lives, is integrity-audited (hashes, seals,
  plan, answers, derived figures, grading) without replays; a fresh
  smoke session replays 8 runs; and four sessions written before the
  class set-up fix replay 8 runs each, which must pass through the
  pre-fix reconstruction ([`CLASS_SETUP_AUDIT.md`](CLASS_SETUP_AUDIT.md));
- **weekly, and on demand**: the same integrity audit, plus every
  measured run replayed for the sessions under `research/ecbench_*/sessions`,
  the binary-curve comparison and the n37 compact-orbit notes.

A replay that disagrees reports what differs (`differs in total_gae,
phase counts, …`). One that agrees reports `identical`, or names the
historical reconstruction it needed:

| replay detail | meaning |
|:--|:--|
| `identical` | every compared figure is bit for bit the record's |
| `identical (legacy gae rounding, …)` | an `ic.pipeline` record from before 2026-10-05 whose solver wall term was removed by subtraction, reproduced from the record's own solver term |
| `identical (pre-2026-10-18 phase accounting, reproduced from the replay)` | a `rho.quartic` / `rho.cubic` record from one of the six pre-fix producers, or a `rho.gls` / `rho.frobenius_orbit` record from one of the two written before 2026-10-28, whose total left out the class set-up (the string keeps its original date) |


**A method's counts are frozen by its id.** Changing what a registered
method counts breaks the replay of every committed session that used it,
and CI fails. That is by design: register a new method id
(`kangaroo.vow2`) instead. The records' hashes are unkeyed, so someone
who rewrites every observation and reseals every hash could forge a
quieter run. Git history and the replay on another host are the
witnesses against that, as with ICMS.

**Legacy accounting.** The rule was broken before it was enforced: after
sessions were recorded, several merges changed what the same
`ic.pipeline` / `pdp3-koblitz` ids count, and sessions of both
generations are committed under identical ids and parameters, so the id
cannot say which accounting a record has. `src/cryptanalysis/ecbench/legacy.rs`
holds the named profiles of the past; a replay that does not reproduce
under the current accounting is tried under each in turn and accepted
only if the **whole** record then reproduces (answer, totals, phases,
counters, native counters, factor base, unpriced list). The receipt's
replay detail names the profile (`pre-2043`, `pre-2043-priced`, `pre-1646`, `pre-A1 endomorphism counters`). New
sessions are recorded and replayed under the current accounting only.
The changes: #1774 (`c8b1594f6`) priced the qualified F4 word-XOR unit
by count (before, a wall-priced solver term was taken back out and the
run was a lower bound); linear harvest on by default (`d1d034d94`,
`9e69f57ef`) and F6-IC gate-first on by default (`d72e8b04d`) changed the
F4/F6-IC search and word count; #2043 (`d72e8b04d`) made every aarch64
host charge the eager rewrite that x86-64 already charged, where before
it rewrote lazily and charged only the rows read (the Apple-silicon
records of 2026-10-13/14); #1646 (`e49afb98f`, 2026-10-11) sent the root's echelon form through the Four Russians kernel and had a child ask the node oracle before specialising (the F6-IC ladder of 2026-10-05, the yield sweep); #1640 (`31dd89b67`) added the `koblitz/icv1-f2m23-t5197-69e76b73` row to `docs/ic/calibration.json`, so native units of that curve were unpriced before; amendment A1 of `research/ext_endo_rho_20261023` changed the uncharged endomorphism counters of `rho.gls` and `rho.frobenius_orbit` in place (no `endomorphism_field_muls_uncharged`, three applications per GLS step), reproduced from the replay by `legacy::pre_endomorphism_a1`. The `oracle_setup` descriptors
`pair_table_kind`, `pair_table_points` and `window_div` were added to
`pdp3-koblitz`'s set-up counters later; a record without one is compared
without it. `engine=rootfind-window-negfold` without `window_div` is the
`⌈|F|/3⌉` window it was before the divisor became a parameter.
A change of accounting still needs a new id: do not add a profile to
excuse one.

**Accounting `_v2` ids (2026-10-28).** An audit found four methods whose
counts left charged-looking work out. Where a committed record exists the
old id is frozen and the fix has a new one: `ic.hyperelliptic_v2`
(verification charged, factor-base scan tallied),
`transfer.quartic_cover_v2` (square-root multiplications tallied) and
`ic.cm_folded_{native,recurrence,native_d43}_v2` (replay operations
charged; the `_v1` records' `total_gae` is 0). `ic.hyperelliptic_thread`
had no committed record (no `spec.json` in `origin/main`'s history
names it), so it was fixed in place: its `verification` phase is new. Each `_v2` has a test
that the frozen id's counts are unchanged and that `v2 − frozen` is exactly
the added charge or counter.

**Plan-time refusals.** `applicable` now raises what `solve` raised on every
run: `rho.gls` / `rho.frobenius_orbit` on a curve of the other family (and
every method without a GLS / subfield implementation on those curves),
`transfer.isogeny` with a non-generic `inner` (or `ell` outside 2, 3),
`rho.multitarget_pooled` with `batch` outside 1..=65536, the CM folded
controls on a curve the adapter refuses (cofactor, base size, query cap,
watchdog, mode, no CM orbit action), and it looks through
`wordram_price::base_of` as it does through `full_price::base_of`. Refusals
change no counts.

*Who may use a profile.* A legacy accounting excuses a difference, so it
is offered only to a record whose `binary_sha256` is on that profile's
closed list in `legacy.rs` (the same pattern as `PRE_CLASS_SETUP_BINARIES`),
and only to an index-calculus record (`ic.*`) carrying no `bitop/v1` work
counters (every binary since 2026-10-16 writes them). A record from any
other binary, including everything the current harness writes, replays
exactly; the lists were found by scanning the committed sessions that
needed each profile, and each entry names its binary SHA-256, the
`git_commit` its session names (informational: matching is by binary hash)
and its sessions:

- `pre-2043`: `1e972df4…fe531` (rootfind_engine claim-n13, claim-n19,
  smoke-koblitz, abandoned/claim-n13-interrupted); `394e0c1b…ec973`
  (rootfind_improve claim-n13, select, window-d4); `1622e3fa…372dc`
  (rootfind_parity claim-n13, claim-n13-v2, cycle2, cycle4, cycle4b, cycle5);
  `de2e66bc…20c` (rootfind_parity cycle1); `b978f0e4…cccdf` (yield_sweep
  koblitz, da-f4 arms); `712a58bb…ebad` (f6 ladder n7, exhausted runs);
  `9bb9a1a6…b8b68` (factor_bases koblitz17-algebraic, exhausted runs).
- `pre-2043-priced`: `27fd2653…b76c` (rootfind_faster claim-n13-v3, cycle1,
  cycle3).
- `pre-1646`: `712a58bb…ebad` (f6 ladder n13, n19); `b2596e9d…d3845`
  (all_candidates koblitz); `b978f0e4…cccdf` (yield_sweep koblitz).
- set-up descriptors and `window_div` absent: `1e972df4…fe531`,
  `394e0c1b…ec973`, `de2e66bc…20c`.
- pre-A1 endomorphism counters: `87bac344…148f` (ext_endo_rho
  `sessions/{gls-fp2,subfield-fp3}`; `sessions-a1/` replays exactly).

The full digests and commits are in `legacy.rs`; a test checks they are 64-
and 40-hex, and that the same committed record with an unlisted binary is
not excused, for every profile.

*Still failing.* `research/f6_ic_ecbench_ladder_20261005/sessions/n23` does
not replay: its `ic-f6` records (4 of 6 sampled replays; the rho records
reproduce) fail under every profile above. The nearest accounting,
`pre-1646`, gives `solver_ops` 9,189,627,347 against 9,189,892,368
recorded (0.003 % apart), but the search is not the same one: recorded
`splits` 72,409 against 71,936, `reductions` 169,304 against 163,371,
`infeasible_branches` 71,890 against 66,430 and `geometric_refutations`
0 against 4,987, while the gate's other counters (support checks 133,842,
witnesses 100, residual lookups 235, group additions 914) match. Another
change to the F6 node oracle between 2026-10-05 and 2026-10-11, or an
uncommitted recording binary, is the likely cause; it has not been found,
and no profile that only nearly matches is offered for it.

**Independent runner (isolab).** `ecbench isolab-job` writes an
`isolab.job/v1` that runs a spec on a lab worker:

```bash
./target/release/ecbench isolab-job --spec docs/ecbench/specs/smoke-prime.json --binary target/release/ecbench > job.json
```

isolab places the job on whole cores of one NUMA node (`smt: isolate`,
`numa_node: single`), moves threads and IRQs off, sets frequency policy,
gates on PSI, and runs `ecbench run --cpus inherit`. Inside, `ecbench`
pins its children to the placement it was given and grades every run
from its own observations, as anywhere else. After the clock stops, the
job's `verify` step runs `ecbench verify --replay 3 --exit-code` on the
worker. The session comes back as hashed artifacts.

- `--binary` (preferred) uploads a binary built for the worker's
  architecture. The job needs no network and the binary's hash is its
  identity.
- `--git-url URL --commit SHA` builds on the worker. The repository does
  not track `Cargo.lock`, so dependencies are resolved there and the job
  needs network. The job labels this, and every record still carries the
  binary hash a comparison checks.

### Claims

A **`vs_rho` claim** is the report `docs/ic/boundary_targets.json`
defines for the primary IC question: one index-calculus run against one
strong-rho run (`rho.signed_frobenius_strong`) on one public target,
compared on their one-target online windows (§3), keyed by
`(candidate_id, workload_id, run_id)` in the repository's IC1 convention.
The ledger calls its one-run ratio `online_speedup`; it is the ratio of the
one round the claim carries, and it stays under that name because the ledger
(and `boundary_autolab.py`) require it. A claim built from this version on
also carries `online_speedup_single_run` (the same number, named for what it
is), `online_speedup_single_run_round`, and `paired_rounds`: **every** measured
round of that workload in which both arms verified and recorded an online
window, with how many (`n_pairs`), each pair's ratio, the aggregate
`Σ rho ms / Σ IC ms`, and a percentile-bootstrap interval over the rounds
only from five pairs (one target, so run-to-run variation only). The round the
claim is built on must be among them. The checker adds `speedup_basis`
(`single_run` below five pairs or without an interval, `paired_aggregate_with_interval`
otherwise) and a note to its output for such a claim, and validates the block;
a claim without it is checked, and answered, as before. A checker `PASS`
certifies the report's pairing, timing fields and provenance conditions,
not a statistically established runtime gain. That requires at least five
admitted matched pairs, an A/A noise interval, and a paired 95% interval
excluding no improvement (§8). The cold end-to-end operation comparison
is a separate requirement.

```bash
./target/release/ecbench claim build --dir SESSION --ic ic --rho rho-strong --workload W... --out claim.json --exit-code
```

```bash
./target/release/ecbench claim check --report claim.json
```

`build` takes the IC and rho records of one workload and round (by
default the first measured round both arms ran) and assembles every
field the schema requires, from the session's own files:

- **The candidate identity** is the tournament's
  (`identity.py`, ported in `claim.rs` and pinned against its values):
  the curve record and EC1 id, the factor base rebuilt and inventoried
  under the cofactor convention (usable base `[h]B` without the identity,
  columns its sign-and-Frobenius orbits), every stage of the method
  resolved, and the IC sources hashed into the binary at compile time. A
  claim is therefore built only by the binary that measured the session.
  The claim also carries the complete canonical candidate and workload
  records beside their hashes, so a reader can audit the compact IDs.
  `ecbench`'s Koblitz bases fold the raw lifted points, so a raw orbit
  whose cofactor image is the identity (the 2-torsion point above
  `x = 0`) is a column with logarithm zero; the record discloses such
  columns under `factor_base.construction.solver_columns` rather than
  counting them.
- **The workload identity** needs a public target: a planted one is
  refused, as `identity.py` refuses it.
- **The windows** are the records' online windows, the IC one split into
  the five exclusive phases, both arms' start and stop events named.
- **Wall isolation.** A `vs_rho` wall comparison requires both runs to
  earn at least L2, even if the session spec lowers `isolation_required`
  for diagnostic runs. A stricter L3 spec still requires L3. The builder
  writes the effective floor and the native checker validates it, so an
  L0 session cannot become a passing wall claim by supplying a replay
  receipt. Operation-count comparisons remain available at every level.
- **The envelope** is the session's (binary, host class, CPU, NUMA node,
  one thread, timeout), identical for both arms by construction.
- **Independence.** The schema requires `independent_validation` and a
  replay certificate for each arm. The certificate is the SHA-256 of an
  `ecbench verify --replay-all` receipt that reproduced both runs;
  `--independent-receipt FILE --pointer WHERE` supplies it. The auditor
  must have a different measurement environment **and** a different
  hardware profile class. The latter omits mutable compiler, kernel and
  governor facts: changing `rustc` on `PATH` on one Mac was enough to
  change `ECBENV2` while staying on the same machine. A receipt from the
  session's own hardware class, one lacking the new auditor hardware
  class, one for other bytes, or one that did not reproduce both runs is
  refused. These self-reported fingerprints cannot by themselves attest
  physical independence; retain the external runner's job and custody
  receipt with the pointer. Without an admitted receipt the
  claim is built with null certificates, the checker fails it on exactly
  those fields, and the verdict says it is not yet a claim.
- **Late receipt attachment.** If the measuring binary is no longer
  available when the other-host receipt arrives, `ecbench claim attach`
  reconstructs the entire previously saved diagnostic report from the frozen
  session and the current claim sources. It requires exact equality before
  adding the receipt; a changed source, target, time, candidate, or other
  report field is refused. The new report retains the original binary hash.
  `claim build` still requires the measuring binary itself. Archive both the
  original report and the attached one.
- **Admissibility.** The verdict states the speedup `rho / IC` only when
  both runs earned the spec's isolation level; otherwise it is marked
  descriptive. The levels are in the report either way.

`check` is `boundary_autolab.py claim-check --stage vs_rho` natively: the
same required fields and aliases, the same validations in the same order,
the same output object. CI builds a claim from
`docs/ecbench/specs/claim-koblitz.json` and requires the two checkers to
agree on it.

## 10. The database

[`schema.sql`](schema.sql) is the schema, compiled into the binary.

```bash
./target/release/ecbench db sql research/ecbench_smoke_20261002/sessions/* | sqlite3 -bail ecbench.db
```

The database is an **index**, rebuilt from session directories,
comparison files, factor-base dumps and claim files; it is never the
record of a measurement. `db sql` audits each session directory
(`verify --replay 0 --allow-interrupted`) before it emits any of its rows:
a tampered or unsealed record, a records file that does not hash to the
session's, or a session that fails any other integrity check is skipped
with the reason on stderr, the other paths still load, and the exit status
is 1. Loading is idempotent in one way only:
an insert tolerates a conflict on its table's primary key when it is the
same row again, and nothing else. A second row claiming an existing run
id is a UNIQUE error. A key arriving with a different identity (a curve
slug on another ICV1 string, a workload, method, factor-base or host id
on another hash, an arm on another role or method, a run with other
figures, a comparison id with other content, a complete session reloaded
as running or with other records) is stopped by the schema's identity
triggers. The views read `complete` sessions only; an `interrupted` or
`running` session is in the tables with its status.

| table | holds |
|---|---|
| `curves`, `curve_representations`, `curve_constructions` | ICV1 slug, ICV1 string, family, sizes, floor; the generator measured and its EC1 alias; the constructor calls that produced it |
| `workloads` | one curve, generator and target, with the known answer |
| `algorithms` | every method configuration (`ECM1h…`): rho, BSGS, kangaroo, IC |
| `factor_bases`, `factor_base_points` | every factor base a run used, by `FB1h…`; with its points when an `ecbench fb` dump is loaded |
| `hosts`, `sessions`, `arms` | the host class and capsule, the session's build and reservation, its arms |
| `runs`, `phases`, `run_counters`, `isolation_blockers` | one row per execution, its phases, its counters, and why it did not reach a higher level |
| `run_solver` | the decomposition solver's statistics for an algebraic or SAT IC run (the record's `solver` block): the system's variables, equations and semi-regular degree, the solving degree reached, Macaulay rows, columns, degree and rank where the solver reports them, SAT variables, clauses, conflicts, decisions and propagations, and every solver-specific counter verbatim. Absent for table oracles. Informational: outside `S` beyond the relations phase's `solver_*` counters, and outside the replay comparison |
| `store_objects`, `store_tags`, `store_superseded` | the object store's index (`store/index.json`, loaded like any other input): each large file's name, SHA-256, size, kind, and, for `factor-bases/<curve>/<FB1h…>.json` names, the curve slug and `fb_id` parsed from the name. A load replaces these three tables with the index it is given, so the tables mirror the index and never accumulate removed names |
| `comparisons` | every saved comparison |
| `claims` | every loaded `vs_rho` claim, keyed by its IC1 `run_id`, with the checker's status at load time; a claim rebuilt with an independent receipt replaces its earlier form |

Views: `store_factor_bases` (the stored factor-base dumps, with `loaded` set where the
database also holds that `fb_id`'s row), `arm_workload_summary` (the AGENTS.md §2 table, per workload),
`method_by_curve` (each method's mean `S` and ratio to the floor per curve,
across sessions), `ic_phase_split` (each IC run's phase shares with its
factor base), `ic_yield` (the yield ledger: one row per IC run with its
curve, target, factor base, oracle, trials, relations, `yield` =
relations / trials, lookups, lift failures, matrix rank and rows, and the
`run_solver` statistics; `ic.pipeline`'s `relations` and
`ic.shared_rank`'s `hits` are read as the same count).

```sql
SELECT fb_family, oracle, count(*), round(avg(yield), 4), round(avg(lookups * 1.0 / relations), 1)
FROM ic_yield WHERE status = 'verified' GROUP BY fb_family, oracle ORDER BY avg(yield) DESC;
```

The same ledger, joined with the curve registry and the tournament's
rounds, is the Yield view of the lab browser (`docs/browser/`,
AGENTS.md §7c). A yield is a stage diagnostic: it ranks bases and oracles
on the same targets, and only the whole-pipeline `S` decides speed.

Paths given to `db sql` may be `store:NAME` or `sha256:DIGEST`; the file is
fetched into the verified cache first, so a stored factor-base dump loads
without a manual `store pull`:

```bash
./target/release/ecbench db sql store/index.json \
  store:factor-bases/icv1-f2m17-tm101-00378d4e/FB1h585d0da5e4b8.json | sqlite3 -bail ecbench.db
```

```sql
SELECT method, curve_slug, verified_runs, round(mean_s, 3), round(mean_ratio_to_floor, 2)
FROM method_by_curve ORDER BY curve_slug, mean_s;
```

## 11. Adding a method, a curve family or a factor base

- **A method:** an entry in `methods::registry()` and an arm in
  `methods::solve`. Once a session that uses it is committed, its counts
  are frozen (§9). A changed algorithm is a new id. Charge every group operation through `CountedGroup`,
  count everything else under a name ending `_uncharged`, never read the
  planted scalar, and return the recovered value unverified. Give it a
  unit test that recovers known logarithms on a prime and a Koblitz curve.
  The `ecbench-extend` skill walks through it.
- **A curve construction:** a variant of `workload::CurveSpec` that calls
  an existing constructor. Register any new curve before citing its slug
  (AGENTS.md §11). `conductor_gap` (`{"kind": "conductor_gap", "ell": 263,
  "t": 241, "level": "crater" | "floor", "seed": 1}`) builds CM pairs over
  `GF((t² + 19ℓ²)/4)` of prime order `q + 1 − t` whose endomorphism
  conductor is 1 at the crater and the prime `ℓ` at the floor, certified
  without computing an isogeny; it is the construction behind
  `research/conductor_gap_cm_pairs_20261010/`. `hyperelliptic_thread`
  and `hyperelliptic_explicit` build genus 2–4 Jacobians over `F_p`,
  `p < 256` (§7, "Hyperelliptic Jacobians").
- **A factor base or oracle:** add it to `ic_framework` as a plug-in;
  `ic.pipeline` reaches it through its name. Dump it with `ecbench fb` to
  store its points.

Explicit x-coordinate lists use `explicit-x:xs=X0;X1;...` on prime and narrow
binary curves. The plug-in lifts exactly those abscissae, rejects duplicates
and missing roots, and preserves input order as column order. Binary linear
bases separately use `binary-subspace:dimension=D,basis=B0;B1;...`.
Both specs work in `ecbench fb` and an `ic.pipeline` arm. See
[explicit bases](explicit-bases.md) for identity/accounting rules and the
[standalone CUDA relation service](../../gpu/explicit-relations/README.md)
for signed per-slot bases and the paper reduction. GPU work is not priced as
CPU GAE or registered as a measured ecbench method.

## 12. Limits

Stated so that nothing here is read as more than it is:

- **The Jacobian and transfer rows are not speedups.** Each moves the
  logarithm into a group with the same `r`-part and solves it there with
  a generic method or, in genus 2, ADH/Gaudry index calculus (§7a). The
  orders of genus-2 and plane-quartic Jacobians come from exact point
  counts. A GHS curve's `ℓ` comes from `jv_cover`'s BSGS order search,
  trusted the way an explicit curve's order is.

- **A claim's identity covers Koblitz curves with `5 ≤ n ≤ 61`.** The IC1
  candidate identity is the tournament's (`identity.py`), whose adapter
  admits those curves and no others; a prime-curve IC run has an online
  window and no claim. A claim's independence rests on the receipt's
  recorded host class and file hashes: `ecbench` checks that the receipt
  is a passing audit of these exact files, run elsewhere, that
  reproduced both runs, and a reader checks the receipt itself at the
  pointer the claim cites.
- **Wide curves run two methods.** Prime curves need `p < 2^62`. Binary
  curves run every method for `m ≤ 62`. Koblitz curves with `64 ≤ m ≤ 127`
  (`koblitz_wide`, over the registry's modulus, so the m = 83 gate curve
  `icv1-f2m83-tm6151469093347-debefd74` of AGENTS.md §8a builds) run only
  `rho.signed_frobenius_strong` and `claw.pair_table`. Both are the one-word
  code made generic over the word, and the committed sessions' replays pin
  that the one-word walk did not change. Every other method refuses a wide
  curve. A subgroup order past `2^64` is written as a decimal string
  (`canonical::compat_u128`) and gets the `_v2` target laws
  (`uniform_scalar_sha256_v2`, `hash_to_subgroup_v2`). Of the registered
  wide Koblitz curves only m = 83 and m = 97 have a prime `#E/h`, so the
  cheapest wide solve is m = 83 itself: about `2^37` strong-rho steps,
  some 20 single-core hours at the 0.49 µs per step measured on Apple
  silicon. The claw's cost-minimising table there is about `2^37` entries,
  out of reach of one host's memory. `p256_factor_base` only constructs
  and inventories a factor base on the registered wide prime curve; it
  does not make that curve runnable by `ic.pipeline` and produces no
  operation-count or speed claim.
- **NUMA binding has met a two-node kernel, not two-socket hardware.** In
  a two-node QEMU guest the policy read back as `bind:<node>` and every
  anonymous page sat on the bound node, and that test found and fixed a
  wrong read-back (`research/ecbench_numa_vm_20261002`). Remote-memory
  latency, the confounder the binding exists for, is not emulated, so a
  two-socket host remains the test of its effect.
- **Hardware counters need a PMU.** Instructions and cycles are read
  through `perf_event_open` on Linux. macOS, the QEMU guest and most cloud
  VMs expose no hardware counters, and their records carry the reason
  instead of a count.
- **A separate Callgrind solve count is available on Linux x86-64.** Export
  a frozen child input with `ecbench profile-input --spec SPEC --seq N`, run
  `ECBENCH_CALLGRIND_SOLVE=1 valgrind --tool=callgrind --cache-sim=no
  --branch-sim=no --callgrind-out-file=PREFIX ecbench exec < INPUT.json`,
  then read it with `ecbench callgrind-ir --prefix PREFIX`. The parser counts
  every numbered part after the pre-solve reset through the post-solve reset,
  including internal phase dumps, and rejects missing markers. This is a
  user-space **implementation** instruction count for the one solve, **under
  Valgrind's allocator**; it has its own `callgrind.Ir` unit. Valgrind replaces
  `malloc`, so the glibc allocator's own work (`malloc_consolidate`,
  `_int_malloc`, `free` bookkeeping) never runs under Callgrind and is not in
  the Ir: it is not a complete count of what the real binary executes, and it
  cannot see the allocator confounder of §5 at all. It is not a PMU count or
  an isolated native wall-time result. Keep its binary, Valgrind version, CPU feature
  dispatch, input, child output, and raw parts together. The
  [n37 protocol](../../research/ecbench_callgrind_solve_20261004/PROTOCOL.md)
  freezes a same-target IC/rho census using this path.
- **A target-only Callgrind interval is opt-in for the shared-rank IC and
  strong signed-Frobenius rho.** Set `ECBENCH_CALLGRIND_TARGET=1` alongside
  `ECBENCH_CALLGRIND_SOLVE=1`, then run `ecbench callgrind-online-ir
  --prefix PREFIX`. The parser requires one target interval inside the
  complete solve and reports pre-online, online and post-online Ir with
  their sum checked against the complete solve. It rejects profiles
  missing either boundary. This is simulated instruction attribution,
  not an isolated online wall claim; the
  [preregistered n37 gate](../../research/ecbench_n37_online_ir_20261004/PROTOCOL.md)
  defines its first comparison.
- **Rho's distinguished-point table is not charged.** Its probes and stores
  happen once per distinguished point, which is not a vanishing fraction of
  steps at the walk lengths the references use: 1 in 16 at `dp_bits = 4`, so
  6.5 % of the steps at n = 37 and 16 % at n = 13 for
  `rho.signed_frobenius_strong`, which counts them as
  `table_queries_uncharged` and `table_inserts_uncharged` (the tuned rho's
  `_v2` ids as `distinguished_point_table_queries_uncharged` and
  `…_inserts_uncharged`; the frozen tuned ids do not count them). The strong
  `_v2` reference's `dp_bits` scales with `r`, which brings the fraction down
  to about `1/√ideal`; §5a's pricing sees the counts either way. BSGS and the
  kangaroo count their table operations.
- **One thread.** Parallel collision search is out of scope until a
  multi-thread run can be graded (a parallel change must be measured at
  one thread and at many, AGENTS.md §10).

## 13. GPU stage runner

`ecbench gpu-stage` runs the repository's existing CUDA ECC driver
(`gpu/ecc/bench`) with a device self-test before a bounded benchmark. Build
the CUDA binary on a GPU host with `make -C gpu/ecc bench`, then run, for
example:

```bash
ecbench gpu-stage --binary gpu/ecc/bench --out /tmp/ecbench-gpu-rho-1 \
  --mode rho --timeout-seconds 600 -- --iters 256 --fold 2
```

The command creates a new output directory and writes `selftest.stdout`,
`selftest.stderr`, `benchmark.stdout`, `benchmark.stderr`, and
`gpu-stage.json`, plus a private byte copy of the binary. The receipt binds
the binary SHA-256, `CUDA_VISIBLE_DEVICES`, exact argument
vector, exit code, timeout, timestamps, and output hashes. A failed device
self-test prevents the benchmark. The directory is never overwritten.
`--solve` is excluded from this stage command so a large-group solver does
not run indefinitely under a throughput request.

Both GPU commands take the shared benchmark lock (`--lock`, default
`/tmp/crypto-bench.lock`; `--wait` queues) before they create anything, as a
CPU session does, and write the host capsule beside the receipt as `host.json`
(its SHA-256 and class are in the receipt, and `gpu-match-verify` checks the
hash). The receipts' `elapsed_ns`/`elapsed_ms` run from spawn to reap, so
they include process start, CUDA runtime and context initialisation, and
teardown (`elapsed_scope` says so); they are not the benchmark's time. For
`gpu-match`, `process_overhead_seconds` is that elapsed time less the driver's
own `host_seconds`, so the overhead outside the benchmark's clock is reported
separately; a stage's driver output is not parsed, so its elapsed time stays
labelled whole-process.

This entry point runs real CUDA code, but its current fixed curves and
targets do not share `ecbench run`'s workload identity or independently
replayed answer and operation ledger. The stage receipt therefore stays
separate from `records.jsonl`, `ecbench compare`, and the L0–L3 CPU score.
It has no GPU contention sensor or admitted CPU/GPU interval yet. To make
a GPU ECDLP method fully comparable, the CUDA driver must accept the
frozen EC Bench workload and seed, emit exact phase counts and recovered
answer, expose GPU device and contention telemetry, and pass independent
answer replay under the same record contract. The specific acceptance test
and remaining device-validation gap are tracked in
[the GPU matched-workload problem](../../problems/ecbench-gpu-matched-workload-contract.md).

### Frozen finite-extension replication methods (three leads, 2026-10-18)

`CurveSpec::ThreeLeads` records the complete source model, field representation,
certified order, prime subgroup and generator. It validates the field,
singularity, Hasse bound and subgroup; smaller witnesses require exhaustive
point counts. Larger quadratic witnesses use a prime-order point with r>4p,
which determines the unique group order in the Hasse interval. The fixture
manifest separately retains the certificate and common generation work.

The `three_leads` adapter shares its native arithmetic with the independent
study driver. `rho.three_leads_{plain,negation,jacobian}_v1` charge initialization,
all walk/collision/escape/restart work and post-solver verification. The
Jacobian arm also charges explicit Scholten transfer and destination setup.
These memoized rho implementations use O(sqrt(r)) collision memory, retained
beside the index-calculus graph's memory; they are not memory-free rho timings.

`ic.three_leads_genus2_v2` charges the cover, both transfers, walk initialization,
Mumford arithmetic, root finding, signed large-prime graph elimination and
restarts. It discards repeated split divisor classes up to sign and restarts
on any repeated walk divisor, including unsplit states. It does not extract
scalars from those walk collisions. The independent and unfiltered modes
are separately specified controls. v1 preserves the diagnosed unsplit-cycle
exhaustions and remains available for replay.

`ic.three_leads_decomposition_v2` charges building/projecting a subfield or
Kummer/Dickson base, full cofactor-offset walk initialization, actual binary
linearized/S3 roots, rational point lifts, point-sum verification, and
incremental elimination of unknown factor-base logarithms. Each step makes
a counted seeded jump choice and includes initial/restarted relation targets.
v1 preserves the earlier fixed-partition sampler, including its exhausted
small-sumset attempts. Methods never receive the planted logarithm.

All primitive M/S/I counters are actual ledger observations. Odd quadratic
operations expand into F_p operations. Binary operations use the named ambient
field unit. Recorded GAE uses the declared conversion S/M=0.75, I/M=10 and a
counted generic source addition; it is not a host-time calibration. The study
also reports all twelve cells S/M in {0,0.5,1}, I/M in {1,10,100,1000}, plus
symmetric native-operation sensitivity. Native counters denote logical field
additions, RNG/scalar/graph operations and lookup calls, rather than physical
hash buckets, allocations or CPU instructions. They do not establish a complete
wall cost. Count-only L0 sessions make no timing or fastest-implementation claim.

Protocol, derivation, raw sessions, replay receipts and claim verdicts:
[`three_leads_e2e_20261018`](../../research/three_leads_e2e_20261018/README.md).

### Seeded P+M / P-M selection model (v1)

`rho.plus_model_v1` and `rho.best2_model_v1` are prime-field diagnostic model
arms, with required `jumps`, `walks`, `dp_bits`, and `cap_multiple` parameters.
They charge setup, starts and every group operation (two complete candidate
additions for BEST2), including cycle-seek steps and escape doublings. Hash,
canonicalisation, DP/table probes and coefficient work have explicit
`_uncharged` counters; apply the standard sensitivity band to both arms.
The runner independently verifies the returned scalar. These arms use hashed
group keys rather than CUDA coordinate limbs and do not measure the shared
field inversion or actual GPU solve cost. Existing rho IDs remain unchanged.

The [frozen comparison spec](../../research/nist_best2_small_batch_20261027/ecbench-model.json)
and [audit/status](../../research/nist_best2_small_batch_20261027/seeded-audit-v1/README.md)
record the unexecuted integration/session/replay requirements. No model result
may be multiplied by standalone GPU throughput to claim measured NIST savings.

### Bounded APS instrument (`ic.aps_v1`)

This native method exposes the existing APS zero-sum relation route with six
encodings, exact/legacy root controls, exhaustive/first-informative recovery
and certified cubic-orbit line pruning. Every search parameter is explicit;
planning rejects unsupported combinations and p>65521. It takes the actual
single target, never its known logarithm.

The cold solver wall interval includes setup and all attempted bases, failed
sampling/construction/solver work, equation export binding, roots, signed
recovery and separate ordinary-residue witness/scalar checks. The five online
phases start before target validation/sampling and sum exactly to their window.
Final report aggregation/serialization is outside the solver clock. Observer and export
construction overhead is included and must be assessed with controls.

GAE prices only the independent check's counted group operations. F4/root
multiplication diagnostics are retained; core group/field work, polynomial
construction, allocations and hashing remain incompletely counted. Thus S and
bitop totals are lower bounds, not admissible complete-cost rankings. The method
is nondeterministic under a cooperative wall deadline; a returned witness is
not a certificate of enumeration completeness, execution history or cost.

[The instrument study](../../research/aps_measurement_20261029/README.md) has
control specs and the remaining protocol/export/accounting/launch prerequisites.
Those specs are development controls, not a frozen held-out scaling experiment.
No benchmark or EKS job has run; the complete-cost research panel remains open.
