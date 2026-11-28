# One finite local matched-component call

This registration supersedes no prior result. All earlier H1/H2/trace
registrations remain consumed. Execute the following worker exactly once
after this protocol, source, inputs and binary manifest are published.

Question: what does the qualified degree-7 template cost at the actual
root/materialization boundary compared with the existing plain and
shape-checked traced six-coordinate F4 paths? This is a descriptive study
of two exposed development residuals, not a fresh-target cohort or a full
IC/DLP comparison. There is no predicted win or novelty claim.

## Frozen computation

- Worker SHA256: `f7d6ad8ae02736953bc6a1a2a3405385667ba129ae719b102e1e85de6291d8d5`.
- Public fixture: `fixture/public.json`, SHA256
  `cca80f4d1443f8f750f72a46f8678da945a93ad4c0c9ce07ae4ca6aedeb6f4fa`.
- Separate expected roots: `fixture/expected.json`, SHA256
  `f368eb2bcf075ddf07d38ab9dabda9337b3689bba9dc6bdb59c801a1c7cc4f37`.
- Exact sources, counted checker, Root41 dependency and native isolation
  helper: `study-source-and-binary.sha256`. Current whole-root compilation
  is not asserted. Rust1.100.0, opt-level3, one Rayon thread.
- All arms use fresh spec/context/factor base (seed13), root RNG389 per
  residual, grevlex/maxdegree24, Fast root stage and the same materializer.
  A/B: staircase64 plain/traced F4. C: fresh unrestricted F4 on residual0,
  compile H2 degree7 (rows3072, columns2048, cells16M, degree limit16),
  materialize residual0 from that basis, then exact H2 plus explicit
  traced fallback on residual1. A residual's cooperative budget is60s,
  shared by its preparation, root stage and fallback; legacy inner loops
  are not hard interruptible. A process watchdog supplies the hard bound.
- Counted checker enabled; partial native bitop hooks disabled uniformly.
  Record original F4/root/Nagao fields, explicit H2/checker work, compilation
  counts, statuses and errors. Field counters remain partial. Do not sum
  overlapping context and Nagao fields or promote them to a full native total.

## Fixed schedule and decisions

One common traced warm-up, two fresh traced A/A intervals, then five
three-arm rounds: ABC, BCA, CAB, ACB, CBA. A=plain, B=traced, C=H2.
Each interval has cold algorithm caches in the same warm process. Charge
fresh development work, compilation, checks, all attempted/fallback work,
original divisor verification and cleanup. Report common warm-up and total
experiment cost separately; never call this a cold-process attack.

After each interval, compare returned roots to the separate saved validator
roots, require resolved status, verify every materialized divisor and match
the factor-base/roots/decompositions fingerprint to the warm-up. The solver
accepts only public data. Correctness failure in warm-up/A/A stops the panel.
A/A max/min above1.10 stops it too, with all remaining positions not-run.
Panel arithmetic failures remain in the record while other prespecified
arms continue. Do not drop failed samples or retry to obtain a better gate.

Retain every sample and partial prefix on timeout/OOM/error. For a fully
completed correct panel, report per-arm median/minimum wall time and all
field-count ranges. Compare wall differences with the A/A interval. No
uncontended wall conclusion if the native receipt marks contention. PSI
is unavailable on this container, so local wall observations retain that
coverage gap even if the helper's subgate passes. External solver comparison
is not run; no fastest-tool claim is allowed.

## Resource envelope and exact command

Root: `/workspace/ic-local-runs/template-cost-20261124-01`.
AMD EPYC9V74 x86_64, Linux6.18.57; allowed CPUs0–4, cgroup CPU quota4,
cgroup memory16GiB; reserve CPU0 (its visible sibling list is0).
8GiB virtual-memory limit, worker watchdog600s, no paid resource.
Native helper holds an exclusive lock, moves other movable threads away,
settles2s, requires other CPU<=0.10core and observed PSI<=5, records
post-run conditions. Do not terminate its parent while affinities are held.

```sh
RAYON_NUM_THREADS=1 F4_FP_THREAD_OPS=1 JV_LIN=fast \
  ./isolated-bench run --cpus 0 --out conditions.jsonl \
  --settle 2 --max-other-cpu 0.10 --max-psi 5.0 \
  --label template-cost-20261124-01 -- \
  prlimit --as=8589934592 -- ./worker run public.json expected.json output
```

Use exclusive new root/output/launch marker and pre/post hashes. Capture
stdout/stderr/native status unchanged. A native preflight refusal consumes
this registration without launching the child; preserve all18 not-run
positions. No unchanged repeat. Source qualification smoke uses /bin/true
and is not a scientific sample.

Checkpoint gate currently fails: no AWS profile or upload token. The user's
explicit local continuation uses published Git/source/input hashes and a
local read-only worker, as in the preceding local H2 protocols. Preserve
the failure and LOCAL custody; this is not a passing S3 checkpoint, signed
execution, full L2 admission or canonical benchmark/leaderboard promotion.
Canonical PCC prefix reconciliation remains open without ledger rewriting.

Sources checked and limitations are in WORKER_DESIGN.md, QUALIFICATION.md
and the prior H2/trace packets. No new field/model assumption is used.
