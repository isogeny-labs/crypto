# Shared point index: matched native panel bridge

Status: driver implementation under review; no benchmark has run and no runtime gain is established. [pcc:unverified]

Lossless development logs, the exact source digest and local failure history are in `development.tar.xz`; `development-archive.sha256` pins its bytes. The archive was extracted and compared to its source directory before publication. [pcc:unverified]

`build-pair.sh` is an unprivileged build stage for a later trusted publisher. It accepts three exact clean Git source checkouts (old collector, shared-index collector, panel driver) and the committed options JSON, builds static x86-64 musl binaries with one Rust 1.78 toolchain and flags, checks architecture/linkage, and emits per-binary hashes and a build manifest. It does not publish an image or submit an experiment. The build has not completed in this executor because its cached Rust 1.78 toolchain lacks the musl target; a hosted exact-source build and review are still required. [pcc:unverified]

`publish-pair.sh` and `Dockerfile.panel` are a proposed trusted publisher for that exact bundle. The publisher checks the three source revisions, all four payload hashes, checksum-list bytes and the intended options policy before any AWS access. Its offline mock test confirms that mismatched bundles cause no AWS/Docker calls and that supplied executables are never run during publication. It is **not wired into a trusted main-branch workflow**; no image was built or pushed. The source/test receipt is under `publisher-checks-20261127/`. A reviewed main workflow must build the pair without AWS credentials, pass the artifact as data to this publisher, checkpoint and read back the exact source, then admit native L2 execution and independently verify S3 outputs. [pcc:unverified]

The existing 84-case native panel calls `solve`, which does not enter the optimized two-slot collector changed by the shared point index. The driver on this branch adds `--optimized-options FILE`: both arms then call `solve-hashed-optimized` with the **same** options file. Each row records its SHA-256 digest, and the analysis checks that both binaries and the options file are unchanged at the end. The legacy command remains the default. Certificate replay and the independent panel oracle remain mandatory for each completed row. [pcc:unverified]

The initial isolated comparison is the old collector at `57110758a23985975b943a3597ea153518c2279f` against the shared-index collector at `fde23fd73445463c692f59312dcf319d525de923`. These are provisional pins until the implementation PR's final source is known. Compile both `hashed-ecdl` binaries with identical release flags and a common compiler, and bind each binary digest, build log and source tree to the run. Build the panel driver from this bridge commit. The only intended native behavior change between arms is the complete factor-base indexing path; review the exact final diff before admission. [pcc:unverified]

`optimized-options.json` selects the default prepared joint control: no walk or alternative witness policy. The hash table is constructed once per complete base. Both arms still charge factor-base validation and all per-target group subtractions, failed relation attempts, matrix work, extraction and final replay. A reduction in map count alone is not an end-to-end result. [pcc:unverified]

After admission, the frozen driver invocation will have this shape. `BASELINE_BIN`, `CANDIDATE_BIN`, `PANEL_BIN`, `SEED`, and the output directory must be replaced by the source-pinned build and protocol values before dispatch; this is not a launch command yet. [pcc:unverified]

```text
PANEL_BIN run --panel research/hashed_ecdl_speed_20261029/inputs \
  --inputs-sha256 research/hashed_ecdl_speed_20261029/inputs/SHA256SUMS \
  --baseline BASELINE_BIN --candidate CANDIDATE_BIN \
  --optimized-options research/unified_point_index_20261127/panel-bridge-20261127/optimized-options.json \
  --out NEW_OUTPUT --seed SEED --case-limit-secs 30 \
  --deadline-secs 1800 --aa-rounds 5 --ab-rounds 5
```

## Run design, pending admission

Use the unmodified known-answer panel at `research/hashed_ecdl_speed_20261029/inputs/`, including all 60 development and 24 holdout inputs and its SHA256SUMS. Use five A/A and five interleaved A/B rounds, the panel's independent target oracle, a fixed public scheduling seed, one thread and a 1,800-second original deadline. Retain every row, timeout and error. A fresh independent holdout panel and complete seed manifest under `docs/experiment-seeds.md` must be frozen before any new randomized measurement. Do not label these already published 24 cases as fresh holdouts. [pcc:unverified]

Record cold wall time and resource use for the **complete** solve, including setup. Separate indexed-base build, target generation, relation collection, graph or matrix work, extraction and verification using exclusive phase accounting. Check that every completed scalar satisfies `[k]P=Q` and that the certificate replays. Compare equal verified workloads, report cold cost first, and show any incomplete paired rows. The repository's operation-accounting contract and independent rho reference remain required for a broader speedup claim; unpriced development event counts cannot replace them. [pcc:unverified]

Only after exact source plus frozen spec/protocol are published, upload and independently read back the current-source S3 checkpoint. Admit a digest-pinned image containing both exact binaries and the driver, authenticated native L2 execution, replay and A/A, and verified S3 result payloads before timing. The existing `isolab-source-checkpoint.yml` can archive a published source and frozen JSON inputs, but `isolab-experiment.yml` currently builds an `ecbench` image for a single source revision; it does not establish a two-binary native panel execution route. The local executor has neither `aws` nor `isolab`. No workflow was dispatched or timing substituted. [pcc:unverified]

Stop on a failed admission, original deadline, missing completion, certificate failure or source mismatch; preserve the failure and do not extend the panel to obtain a positive result. A positive claim needs a complete paired panel, a 95% interval excluding no improvement, total-cost ratio greater than one under the parent contract, and independent holdouts. A negative or inconclusive result is equally reportable. [pcc:unverified]
