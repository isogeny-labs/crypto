# CUDA release CI and stale N83 history repair — 2026-10-27

## Corrected history integration

An earlier audit treated matching tree hashes as proof that PR histories had been integrated. The review finding was correct: the reviewed one-parent commit did not have either source PR head as an ancestor, so that state did not preserve their histories. The tree comparison alone was insufficient.

The history was then integrated with ordinary merge commits. Commit `ed41a8469649de8dbc336008da8c8b984b00956e` has parents `62c166aaa9e96e077c638b4828ef4c2ebac80e6b` and PR #2124 head `c502f34973924022985f5876786423cfc98b09ce`. Commit `1da1e8ff379a3c854e12e561a4eccb7162668ed2` has parents `ed41a8469649de8dbc336008da8c8b984b00956e` and PR #2138 head `09270bccae13cab796c9bdba2783b4934bb48e98`. Both merge commits have tree `3dd26d063f7e7121aafafaa90afddda4ac1937c7`, equal to the tree of main `62c166aaa9e96e077c638b4828ef4c2ebac80e6b`; their source PR heads are ancestors of the current #2712 head. These ancestry claims are directly checkable with `git rev-list --parents` and `git merge-base --is-ancestor`.

Relative to their shared base `7e271266f86968843a9323e4a074f9a874448eb2`, each source PR changes only the same benchmark README row; no N83 source or data change remains in those diffs. The two source PRs are still open until this integration branch lands on main. No scientific protocol, result, counter, or evidence file was changed by the history merges.

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

## Match the original half-trace candidate format

The mirrored IC metadata validator now supports the exact original format emitted by cryptanalysis's half-trace candidate producer: `summand_count: 2` plus its exact `htsolver.py` implementation path. This fixes a real cryptanalysis CI failure without rewriting ten frozen content-addressed candidates. Modern explicit solver-family records still must match their stage codes. New controls reject a changed stage, arity, implementation, Boolean arity, or incomplete mixed format.

Both validator and test copies are byte-identical across the two repair branches. Crypto's test run passes 11 controls with 2 existing cryptanalysis-only skips; cryptanalysis passes all 37 catalog tests, and the cross-repository metadata check reports zero errors. The initial crypto diagnostic was missing a local sparse registry fixture; restoring tracked docs/curves made it pass. These checks establish metadata consistency only.

## Additional contained histories and daily journal

PR #2165 head 2d0b3a2a997614410ffef4f9dbebfb4ca85f9647 has no remaining file delta. A normal merge preserves its ten-commit author history with a complete tree identical before and after; the exact proof is retained alongside the earlier #2124/#2138 proofs. No research result is rerun or reinterpreted.

PR #2697 head 0e50ff68ccc2ce897b88212265e340c9b942d3fc now contributes only five artifact-wiring regression tests; the implementation and workflow repairs are already on main. Those tests are retained and all 27 artifact tests pass. The original author history is preserved rather than reconstructing it with cherry-picks.

PR #2694's journal and timestamped session note are retained exactly as an early-day historical observation. The session-log conflict retained both that note and main's subsequent Lean-build entry; no later observation is overwritten. These additional merges alter only artifact tests and journal/session documentation relative to the dry-run source at 9918decb2. CUDA and production source are unchanged.

## Dry-run result and bounded publisher checkouts

The feature-branch Release dry run [40564308149](https://github.com/isogeny-labs/crypto/actions/runs/40564308149), at source 9918decb2, passed its CUDA compile job 122401506011 on the restored Ubuntu 24.04 image. The exact source-tree IDs and hosted job receipt are retained. This establishes compilation, not GPU execution. Other build jobs were still running at this checkpoint.

The same run exposed a separate asset-check failure: checkout spent 4m59s fetching the full source tree, then the five-minute job was cancelled before either bootstrap or the publication test ran. The asset-check and final publisher checkouts now request only `.github/scripts`, the existing curve bootstrap inputs, `tools/artifacts` and `.artifacts`, using blob filtering. Both existing timeouts, all publication/checksum controls and the main-only publication guard are preserved.

All nine existing publication controls pass locally, also in an isolated fixture containing only the two publisher script files. Actionlint, all 501 workflow budgets and generated artifact wiring pass. A fresh hosted dry run is required to verify the bounded checkout; the earlier cancelled job is retained as a failed attempt rather than called successful.
