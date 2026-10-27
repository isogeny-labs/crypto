# CUDA release CI and stale N83 history repair — 2026-10-27

PRs #2124 and #2138 are titled as retained N83 factor-base studies. Their only remaining merge delta against main 62c166aaa is the same unrelated deletion of the existing 20261016-isogeny-round5 benchmark row. No N83 source or data change remains in either merge result. Preserving that unrelated row integrates both author histories without changing main's content.

Each merge was performed normally, with only benchmarks/README.md restored from main. The complete Git tree after each merge equals main 62c166aaa9e96e077c638b4828ef4c2ebac80e6b, before this audit note and the session-log entry are added. The recorded tree proofs below can be checked with git rev-parse. No scientific protocol, source, result, counter, row or evidence file changes.

[
  {
    "pr": 2124,
    "head": "c502f34973924022985f5876786423cfc98b09ce",
    "base": "62c166aaa9e96e077c638b4828ef4c2ebac80e6b",
    "tree": "3dd26d063f7e7121aafafaa90afddda4ac1937c7",
    "same_tree_after_preserving_unrelated_row": true
  },
  {
    "pr": 2138,
    "head": "09270bccae13cab796c9bdba2783b4934bb48e98",
    "base": "62c166aaa9e96e077c638b4828ef4c2ebac80e6b",
    "tree": "3dd26d063f7e7121aafafaa90afddda4ac1937c7",
    "same_tree_after_preserving_unrelated_row": true
  }
]

## Restore the CUDA release runner pin

After obsolete unassigned main runs were cancelled, Release run [40559504617](https://github.com/isogeny-labs/crypto/actions/runs/40559504617), on main cdc8503334bd0eb608ca469c11b0421cb9a23520, actually executed. Its [CUDA job](https://github.com/isogeny-labs/crypto/actions/runs/40559504617/job/122387230575) ran Ubuntu 26.04 and failed installing both pinned CUDA 12.8 packages.

The CUDA job now uses `ubuntu-24.04`, matching the existing documented pin in `docs/RELEASES.md`. NVIDIA's Ubuntu 24.04 package index still advertises `cuda-nvcc-12-8` 12.8.93-1 and `cuda-cudart-dev-12-8` 12.8.90-1. The package versions, architectures, build, smoke checks, release dependencies and permissions remain intact. Hosted CUDA compilation is required before this repair can be called verified. No GPU runtime claim is made.

Source checked: [NVIDIA Ubuntu 24.04 package index](https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2404/x86_64/Packages.gz), alongside the repository release guide and the failed job's actual runner image and apt output. The repository-index excerpt and queue-state proof are retained under [ci-repair-queue-evidence-20261027](ci-repair-queue-evidence-20261027/).

## Queue cleanup

Four old main runs held concurrency groups while their remaining jobs waited for unavailable runner labels. Immediately before cancellation, each run was checked against the latest main SHA and every unfinished job had status `queued`, an empty runner name and zero steps. Completed jobs and artifacts remain retained. No running research process was stopped. Current Release and Rust lint jobs started afterward; that observation exposed the real CUDA install failure above.
