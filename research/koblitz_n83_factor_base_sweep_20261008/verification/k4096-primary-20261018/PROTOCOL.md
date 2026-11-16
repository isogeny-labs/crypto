# Primary N83 K4096 factor-base construction and S3 gate

Freeze one declared `v2-size-frontier` cell before executing the producer:
curve `a=0`, subgroup order `2417851639230796216685689`, public-x hash
policy, seed `2026100801`, 4,096 signed-Frobenius orbit columns and the
study's pinned polynomial basis. A successful full-orbit object must have
679,936 point records. The exact destination is the existing
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/`
prefix, under the compressed object's BLAKE3-addressed name. The producer
must not overwrite the prior K2048 panel or object.

The source-pinned static Linux exporter has SHA-256
`21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed`.
The native arm64 replay/upload exporter has SHA-256
`5881dc22b3367b6076ef5c3d452eb270bfa3c4013ec63782a0276b4670cdc094`.
The construction image is the locally verified Git-containing immutable
Linux arm64 image
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
Before launch, require a clean committed worktree and verify the compiled
exporter source matches the checked-out file. Retain the source commit,
binary hashes, exact command and input design hash. The Linux binary runs
under emulation; its wall time is a construction resource observation, not
a native index-calculus timing.

1. Run `v2_object_supervisor.py` once into a fresh scratch directory with
   one CPU, 4 GiB hard memory, zero swap, no network, read-only repository
   mount and a 900-second outer wall cap. Retain config, raw stdout/stderr,
   worker manifest or cap, outer status, input/source/binary checks and
   container-cleanup receipt. Stop the sequence if it does not return
   `PASS_construction_only`.
2. Run the native `v2-replay-one` on that exact object under a separate
   1,800-second process cap, with its matching internal cap. The generic
   multi-limb backend must check every point and representative, subgroup
   membership, Frobenius labels, closure and point-set hash. Retain raw logs
   and a stage receipt. Stop on a cap or failed replay.
3. Only after replay passes, run native `upload` under a 600-second process
   cap. The existing uploader must re-read the object, publish it and
   download it for compressed BLAKE3 equality. Independently issue a fresh
   S3 HEAD and GET and compare BLAKE3, SHA-256, content length, metadata and
   exact key. Retain the upload and fresh-GET receipts. A failed upload is
   a storage/infrastructure outcome, not a completed candidate.

The conservative stage envelope is 3,300 seconds plus build/preparation;
report observed stage walls separately. Preserve all first-attempt failures
and censored outcomes. No relation search is authorized by this construction
gate itself. The nominal five-summand multiset count divided by subgroup
order rises from approximately 15.653 at K2048 to 500.882 at K4096, but
that count is not a measured success rate, useful-rank rate or runtime.
Keep `total_index_calculus_runtime_ms` and `selected_best_total_runtime`
null. A later source-pinned search and matched full cold comparison are
separate gates.
