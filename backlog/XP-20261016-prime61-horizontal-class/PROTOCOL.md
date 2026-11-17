# Prime61 horizontal class, every vertex: protocol

**Status: DRAFT, not frozen.** Nothing here is pre-registered until a
commit changes this line to "frozen", records `git hash-object PROTOCOL.md`
as the card's `protocol.blob`, and code items C1–C4 and C6 (§8) are merged.
Card: [`backlog/XP-20261016-prime61-horizontal-class`](README.md). Problem:
[`problems/prime61-horizontal-class-memory.md`](../../problems/prime61-horizontal-class-memory.md).
Earlier frozen work: [prime61 study](../../research/prime61_isogeny_20261016/README.md),
[disk-walk pilot](../../research/prime61_disk_walk_20261017/README.md).

Every statement carries one label: *recorded* (a committed row or a file in
`probes/`), *derived* (a calculation shown here or in the card README) or
*predicted* (stated before the runs).

## 1. Question

What are the factor-base results over every base-field isomorphism class in
the fixed ordinary trace class of `y^2 = x^3 - 3x + 111` over
`p = 2^61 - 1` (`r = #E = 2305843010818082053`, prime, `t = -1604388101`)?

## 2. Frozen inputs

| | value |
|:--|:--|
| field, order, `G`, `Q` | as in `research/prime61_disk_walk_20261017/spec.json`; public synthetic target `Q = [539365399]G`, never given to the collector |
| workload per vertex | `prime_fast` collector, native low-x three-inverse-pair base on that curve, 20 relations requested, 12 streams with seeds `2026101600..2026101611`, 100,000 target trials per stream (the pilot's run policy, unchanged) |
| traversal degree | ℓ = 7 only (`primes: [7]`), one database per coset |
| coset roots | [specs/coset-0.json … coset-7.json](specs/): pilot curves 1, 2, 5, 15, 19, 29, 66, 164, with their stored transported `G, Q` (paths: none; 3; 11; 61; 3·11; 3·61; 11·61; 3·11·61 by pilot edges 1; 4; 14; 1,19; 1,29; 4,74; 1,19,284) |
| binary | `isogeny_disk_walk`, `cargo build --release --no-default-features --example isogeny_disk_walk` at the commit that freezes this file; walker sources currently equal `b68341cd` |
| host | recorded before Stage 2 (`ecbench host` capsule), with `nproc`, `MemTotal`, free disk, and the binary's sha256 |

Each vertex's `G, Q` are the images of the source's `G, Q` along its
cycle-parent path (coset-root path, then ℓ = 7 steps). That path differs
from the pilot's full-degree first-parent path for most vertices, so a
vertex's workload rows are comparable with the pilot's only where the
paths coincide (§5, control K3). This is the one design change from the
pilot, made because the full-degree BFS needs 1,278 single-writer days
(card README) and a 5.37 TB database.

## 3. Stages

1. **Certificates** (C1, C2; native; seconds to hours).
   - C1 recomputes the genus vectors of the primes 3–61, checks rank 4,
     checks `A = (7,5,237475387793728201)` has exact order 68,162,026, checks
     the eight coset roots' genus vectors are distinct modulo that of `A`,
     and replays each root's path from the pilot dump to its spec file.
   - C2 computes `h(D)` unconditionally (count of reduced forms).
   - Any mismatch stops the card before Stage 2.
2. **Enumeration.** Eight shards, one per coset `k`, in parallel:
   `isogeny_disk_walk advance specs/coset-k.json OUT/k/walk.db 68162026`,
   preceded by `init`. A shard is closed when `status` reports
   `pending = 0`, `failures = 0`, `discovered = 68162026`,
   `edges = 136324052`. Any other end state is recorded and stops the card.
   Snapshot each closed database with `snapshot` and record its sha256.
3. **Measurement** (needs C3). Shards are curve-id ranges
   `[1 + 65536·i, 65536·(i+1)]` of one coset database, `i = 0..1040`
   (the last range ends at 68,162,026): 8 × 1,041 = 8,328 shards of at
   most 65,536 curves, 26.1 core-hours each (*derived*:
   `65536 × 1.435 s`). Each writes its own small database of run rows;
   it reads the closed coset database read-only.
4. **Verification** (needs C4, C6). Per coset: `verify` over the graph by
   id range; per measurement shard: re-validate every run row, re-add
   every relation, replay every verified solve and a deterministic 1% of
   capped streams (`curve_id mod 100 = 0`, all 12 streams). C6 checks that
   the eight j sets are disjoint and their union has 545,296,208 elements.

## 4. Sharding, checkpoints and spot interruption

- Every `advance` and `measure` transaction is durable before the next
  starts (`synchronous=FULL`). A process killed mid-transaction loses only
  that transaction. Rerunning the same argv resumes; a changed spec is
  refused (recorded pilot controls).
- A spot reboot or stop keeps the EBS volume: the loss is at most one
  transaction per running process. Relaunch every unfinished shard with
  the same argv; never truncate or delete an output database.
- A spot termination that loses the volume loses every shard not copied
  off-host. Closed Stage 2 databases (≈ 70 GB, ≈ 22 GB gzipped) and closed
  Stage 3 shard databases (≈ 134 MB, ≈ 42 MB gzipped, *predicted*) are
  copied off-host as soon as they close, when a route exists (S3 is denied
  to this instance role on 2026-10-18; Drive needs a live session). With
  copies, a termination loses at most the shards in flight, one per worker.
- A shard is complete only when its own status and range verification pass.
  Incomplete shards are reported as incomplete, never extrapolated.

## 5. Controls

- **K1, identity.** Before Stage 2, `measure` curves 65–76 of the pilot's
  `checkpoint-32` on the measuring host and binary: all 144 rows must equal
  the 2026-10-18 probe rows (`probes/`), and curves 1–64 under `verify 64`
  must replay the pilot's 768 rows exactly.
- **K2, small classes.** The four existing disk-walker tests
  (`cargo test --release --no-default-features --example isogeny_disk_walk`)
  pass at the frozen commit.
- **K3, path agreement.** In coset 0, database curves 2 and 3 are the
  pilot's curves 3 and 4 (root's ℓ = 7 neighbours, same parent edge): model,
  `G, Q` and all 24 run rows must be identical. Coset roots 0–5 must
  reproduce the pilot's committed rows for pilot curves 1, 2, 5, 15, 19, 29.
- **K4, cycle length.** Each closed cycle has exactly 68,162,026 vertices,
  the certified order of `A`; a different count is an instrument failure.
- **K5, interruption.** Kill one Stage 2 and one Stage 3 process with
  SIGKILL once each, resume, and require the final verification to pass;
  record the lost transaction.

## 6. Metrics and how they are reported

Per vertex (one row each, retained in the shard databases): coset, id, j,
EC1 id and UID, depth, the 12 stream statuses, target trials, relations,
rejected, verified scalar.

Class-wide, in one table, one unit (streams or vertices, stated per row):

| metric | unit |
|:--|:--|
| vertices covered / 545,296,208, per coset and total | vertices |
| streams run, capped, unresolved rank, verified solves | streams |
| target trials | trials |
| relations: total, histogram per stream, max per vertex | relations |
| vertices with ≥ 1 relation, with a solve | vertices |
| the top 100 vertices by relations, then by fewest trials to a solve | vertices |
| kernel failures, special-j failures | edges |

Every success (a relation or a solve) is listed with its replay. Zero
counts are reported as zero, with the population they cover. No cost,
`S`, rho-reference or speedup figure is produced; `full_cost` and
`speedup` stay null. Report, graph (relations per vertex across the
class, with coverage) and PDF follow the research-visuals workflow in a
new `research/` directory.

## 7. Decision rule, fixed now

| outcome | reading |
|:--|:--|
| all vertices covered, zero relations | class-wide null for this policy and budget: no isomorphism class of this trace class yields a relation in 1.2·10^6 targets; the source is not beaten by any isogenous model at this budget |
| some vertices have relations, none solve | rank the class by relations; report the leaders and their replays; no solve claim |
| some vertex solves | report every solving vertex with its replay; it is a relation-stage result, not an end-to-end gain |
| coverage incomplete | report covered counts only; no class-wide claim |
| C1/C2/K1–K5 failure | instrument first: no reading until explained |

*Predicted:* zero relations on every vertex. The pilot and probes have
0 relations in 912 streams (91.2·10^6 trials); the prediction has low
information and does not justify skipping the run.

**Inadmissible after Stage 2 starts:**
- changing the spec (field, `G, Q`, scalar, degree, runs, seeds, trials) or
  the coset roots;
- dropping a coset, a shard or a vertex, or replacing coverage with a sample;
- re-running a shard for a different answer (a lost shard is rerun whole
  with the same argv, and the lost attempt's files are kept);
- pooling the pilot's or the probe's rows into the headline population.

## 8. Code required before freeze

| item | what | where |
|:--|:--|:--|
| C1 | native class-cover certificate (genus vectors, rank, order of `A`, coset-root genus classes and paths) | new example; uses `src/isogeny/class_group.rs:513` `class_above_prime`, `:252` `pow`, `src/cryptanalysis/isogeny_degree_search/class_number.rs:209` `kronecker_symbol` |
| C2 | unconditional `h(D)` by counting reduced forms with square roots mod `4a`, `a ≤ 1.49·10^9`, range-parallel | new; the existing `enumerate_reduced_forms` (`src/isogeny/class_group.rs:334`) loops over every `b ≤ a`, about `|D|/3 ≈ 2.2·10^18` steps, so it cannot be used |
| C3 | `measure-range SPEC SOURCE_DB SHARD_DB LO HI`: source opened read-only, rows into a per-shard database binding spec, source digest and range; resumable | `examples/support/isogeny_disk.rs:593–621` holds `BEGIN IMMEDIATE` across `collect`; `:662–701` has no range command; `examples/support/disk_sqlite.rs:56` opens read-write only |
| C4 | range-restricted, resumable verification for graph and shard databases | `examples/support/isogeny_disk.rs:388–515` scans every table in one pass and is not resumable |
| C5 | optional: batch several sources per transaction (wall time only) | `examples/support/isogeny_disk.rs:281–363`, `examples/support/disk_sqlite.rs:74`; needs the interrupted-vs-uninterrupted test (`examples/support/isogeny_disk/tests.rs:172`) to pass unchanged |
| C6 | cross-database j disjointness and union count (8-way merge of `SELECT j FROM curves ORDER BY j`) and the class-wide summary over shard databases | new |

## 9. What was run before this protocol (disclosure)

- **Read, not run:** the pilot and study records, PROTOCOL.json, spec and
  committed dumps.
- **Run, sizing only, 2026-10-18,** on this host (32 vCPUs shared with a
  running experiment, load 15–37), binary sha256 `e46128bc…7b95aa`, at most
  one thread per process and 20 MB each, all outside the repository
  (`/home/ec2-user/probe-prime61-20261018`, not durable); timings in
  [probes/](probes/):
  - full-degree `advance` of a copy of the pilot `checkpoint-32` to 96, 97
    and 1,097 expanded sources (4,389 curves, 16,455 maps, 0 failures) and
    `verify` of the result (PASS);
  - `measure` of curves 65–76 on another copy: 144 streams, 14.4·10^6
    target trials, 0 relations, 0 solves (sizing only, not part of any
    result population; K1 reuses them as an identity reference);
  - an ℓ = 7-only spec from the pilot source: `advance` to 3,000 sources
    (3,002 curves, 6,000 edges) and `verify` (PASS);
  - `init` on each of the eight coset specs: all pass `Spec::validate`.
- The genus-vector table in the card README was computed at design time
  with a shell calculation of Legendre symbols mod ℓ ≤ 61; it is not
  evidence until C1 replays it natively.
