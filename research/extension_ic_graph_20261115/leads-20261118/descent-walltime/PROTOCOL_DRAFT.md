# DRAFT (not frozen, not dispatched): isolated descent wall-time A/B

Status: draft for an owner decision. No compute was allocated.
[pcc:unverified]

- **Question.** Does the local indicative descent wall ratio of production
  over traced (1.26–1.29x, `README.md`) hold on an isolated host?
- **Route.** `isolab-eks` (AGENTS.md §9a), one exclusive x86 node, CPU-pinned,
  1 thread. Before any run, `.resume/RESUME.md` must be filled and a
  `bench.record/v1` record opened.
- **Binaries.** `descent_walltime` (frozen linear stage), built by
  `build/build.sh` from the pinned sources. The source and binary SHA-256s are
  checked on the host before the first run.
- **Inputs.**
  - The Run50 deferred public fixture, seed 389, N = 3415.
  - nagao-e2e, seed 401, N = 1506.
  - At least one fresh public p1009 target and one larger-p target from
    `fresh-scaling/`. The stream size per target is frozen before data.
- **Arms.**
  - A = production staircase F4.
  - B = traced (C1).
  - A/A and B/B controls: the same binary, consecutive runs.
- **Order.** ABBA blocks with a derived per-block random start. 1 warm-up
  block, discarded and retained. 10 measured blocks per stream.
- **Primary metric.** The geometric mean over blocks of the paired
  A/B ratio of descent wall (`ms.descent_wall`), with a percentile
  bootstrap (10,000 draws) over whole blocks.
- **Secondary metrics.** F4 ms and linear-stage ms. The linear-stage ratio is
  a built-in positive control and must lie in [0.97, 1.03].
- **Gates.**
  - Identical `output_digest` across all runs of a stream.
  - Median A/A and B/B within [1/1.05, 1.05].
  - Zero rejected replays, or rejections reported with their charged work.
- **Not claimed.** Pipeline gain, rho parity, or any ranking of IC arms.
- **Owner decisions.**
  - Whether to run this at all (paid compute).
  - Whether the staircase certificate is acceptable or a pair certificate
    must be added and timed.
