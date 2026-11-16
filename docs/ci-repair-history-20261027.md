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

## Refresh the native geometry catalog against its actual registry

Release staging failed its native cover-link check because the canonical registry contains 20,592 curves (SHA-256 e7e8cd2ce2570e6e6931674ff653fbaf0cd513b447738364dbb5440a0974a420), while covers.json and cover-links.yaml still bind the earlier 15,791-curve input. The unchanged native checker source hashes to 741c19d1566bf1a54f860b2d598bd7cbbea6c34b4fac2a97d63a5e3e3e1b7f62 and regenerates both files with 20,592 verified, zero unsupported and zero invalid models.

Every one of the 15,791 existing report rows, every existing graph curve row, all 15,789 existing cover objects and all 15,791 existing map objects are unchanged. Exactly 4,801 rows/maps/covers are added from the already-present registry. Root --check and a cold installed layout both pass, including construct/verify, identity arithmetic, catalog export and fixed-F8 order 486. These establish the existing native geometry contract; subgroup transfer and ECDLP advantage are not established.

The exact semantic delta, before/after hashes, binary hashes, commands and outputs are retained in the evidence directory. These current generated metadata files are explicitly kept in Git by .artifacts/config.json and exempted as delta-compressible catalog text by scripts/check-large-files.sh; neither policy changes. Frozen experiment records and historical source snapshots are untouched. The separate transfer-assessment builder still binds its older cohort and is handled as a distinct source-bound update.

## Keep the dated transfer cohort and repair its derived reports

The transfer assessment explicitly declares catalog commit
728304718cb0ab6df70da620a908a5f76f175293. Its builder and independent verifier
now read the three catalog inputs from that immutable commit, while ordinary
source/evidence artifacts continue to bind current working-tree bytes. A missing
Git snapshot fails rather than falling back to the live catalog. Three snapshot
controls and six stale-catalog refusal controls pass. The existing merge gate
now rejects stale registry/checker bindings without waiting for Release.

All 17 scientific and contract sections outside source_artifacts are exactly
equal before and after generation, including profiles, memberships, typed
bindings, overlays, coverage, statuses and open obligations. All six independent
authoritative semantic digests remain unchanged. The complete independent
verifier reports zero problems; deterministic generation and the pinned Draft
2020-12 schema check pass. This preserves the 15,791-map/979-EC1 assessment and
does not apply its conclusions to the 4,801 later maps or 15 later EC1 bindings.
The problem record and report explicitly retain that coverage gap.

The three archived visual/report assets were retrieved through authenticated
manual workflow 40576338037 and checked against every original size and SHA-256.
This exposed a stale PDF describing 2,572 maps. Both diagrams and the PDF were
rebuilt from the current DOT/Markdown sources. The original PDF remains under
store digest 1f6cde8011714af43814bd28aca117e048f3d6cf3f58eba9c444712328cbed8e.
The existing policy explicitly keeps PDFs in Git, so its obsolete store pointer
is removed and the newly derived PDF is committed; PNGs use the ordinary trusted
artifact migration. No store object is overwritten or removed.

Tectonic 0.17.0 was verified against its official release digest, but its bundle
endpoint returned HTTP 403. The installed XeLaTeX engine rendered the same
Markdown successfully. The metadata helper records the actual engine and pypdf
version, and the documented fallback preserves this provenance. The PDF's exact
coverage Subject is correct and its first page was visually inspected. The
new/old hashes, exact section-equality proof, successful and failed logs and
renderer details are retained with the earlier native evidence.

Manual transfer job 122439966503 separately exhausted its five-minute timeout in
full checkout. Its checkout now materializes the assessment's source/evidence
surface while retaining Git history for the snapshot. The timeout and every
validation remain unchanged. The bounded replacement job retrieved the replay
assets by 20:16:40 UTC. Final current-head hosted checks remain necessary.

## Refresh the lab browser without losing candidate or session identities

The canonical browser still described 15,791 models. It is regenerated from the
current 20,592-model registry and refreshed native catalogs. All 248 old session
summaries and their raw-record hashes are exactly unchanged; four committed
sessions are added. Every old curve model and every old candidate's semantic
fields remain equal. The current index has 252 sessions, 220 methods, 672 factor
bases, 277 candidates and 30 rounds. Registry provenance/retired aliases refresh
from the canonical source; existing scientific records are untouched.

The first regeneration exposed four missing candidate IDs. Their sole source
was the required leaderboard, which has grown to 9,268,417 bytes and crossed the
general eight-MiB citation-scan limit. The required, already-parsed leaderboard
now always contributes candidate IDs; the limit remains for general research
notes. A regression control exercises both the oversized core input and refusal
to scan an oversized general note. All four browser tests pass. The corrected
regeneration retains every old candidate and adds six currently named only by
the leaderboard. Production --check passes with its no-lazy-fetch guard after
explicitly retrieving the exact missing committed Git objects.

The exact retained/added identity comparison and successful/failed browser logs
are retained with the native/transfer evidence. This index refresh does not
re-run solvers, change measurements or extend the dated transfer assessment.


## Remove an obsolete workflow expression inherited from main

The final-head workflow parser job [122491560066](https://github.com/isogeny-labs/crypto/actions/jobs/122491560066)
parsed all 503 workflow YAML files, then actionlint correctly rejected
`steps.peer.outputs.available` in `ic-semantic-metadata.yml`: that step ID no
longer exists. The same three-line fallback is present on main. Remove the
obsolete duplicate local-validation step. The preceding mandatory
`validate_semantics.py --peer cryptanalysis-peer`, authenticated fetch and
registry freshness check all remain intact; a missing token still fails closed.
This changes no trigger or required-check policy. Conductor is unavailable in
this runtime (`conductor check`: command not found); the documented absent-binary
fallback applies, and the change is isolated to this repair branch.
