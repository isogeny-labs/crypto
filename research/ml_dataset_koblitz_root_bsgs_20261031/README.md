# Frozen Koblitz n23 root and BSGS pilot

The frozen `experiment.protocol/v1` record is `protocol.json`. The pinned official checker at `crypto-autoresearcher@f755085ff951888dbe64682c210c9ef7d167b40c` returned zero findings on the draft and frozen files. Its protocol body SHA-256 is `02e47cf1c4a4076b203e185888d3b64a15224c9332b4829431122e00763a06fa`; the full frozen file SHA-256 is `10b168bafeb6f32d7c9c01734cf3342ae8124539c1488d12acf8b79096a455d6`. The record was frozen before affected data and binds source commit `c41bec150e16e4c4f9a00634002b633b821de262` and clean binary SHA-256 `b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.

The native plan has one Koblitz n23 curve seed, m2, two generator indices, two key indices, paired twists, and `root,bsgs` methods: 16 cells. The experiment exporter expects four arms (`root`/`bsgs` by twist) and four shared instance IDs (`g0k0`, `g0k1`, `g1k0`, `g1k1`) in each arm, also 16 rows. Matching root and BSGS cells on one twist share the same public curve, generator, and target. The corresponding IDs across twists balance the index grid; they do not identify the same public curve. The export checker computes expected measured rows as arms × instances × replicates (`tools/experiment_protocol.py:476-495`).

Root artifacts are bounded binary S3 distinct-root and group-lift/PDP evidence. The controller separately retains checked partial query evidence, promotes a completed-root certificate only when every root query completed without censoring, and promotes an independently checked positive PDP witness even when that witness stops the search early. Only BSGS can provide an independently recovered ECDLP scalar in this pilot, and that label requires native verification. Timed values are operational diagnostics; the protocol makes no solver speed comparison, whole-worker-instruction-count claim, or exact first/last fall-degree claim. An unattempted or timed-out cell remains in the 16-cell denominator.

After the protocol commit, create a native plan with the pinned binary and the **frozen file** SHA-256:

```sh
crypto dataset plan --out "$work/plan.json" --n 23 --m 2 \
  --families koblitz --solvers root,bsgs --curve-seeds 1 \
  --generators 2 --keys 2 --seed 20261031 \
  --factor-base-size 8 --degree-bound 3 --work-limit 20000 \
  --protocol-sha256 10b168bafeb6f32d7c9c01734cf3342ae8124539c1488d12acf8b79096a455d6
```

Run `dataset preflight` against that plan and the frozen protocol, then process each shard index 0 through 15 once with `--max-cases 1 --shard-count 16 --seconds-per-case 30 --memory-mib 2048 --trace-mib 64` and the 90-second outer cap. Check actual manifest configs against the native mixed-radix plan. The intended local output is XFS under `/var/tmp`; retain native verify, report, export, complete restricted archive, S3 PUT, and fresh GET/hash receipts per attempted cell. No generation or cloud submission belongs to this protocol-preparation stage.

Source scope checked before freezing: `src/ml_dataset/solvers.rs:326-346,2510-2523`, `src/ml_dataset/binary_roots.rs:1-21,68-89`, `src/ml_dataset/mod.rs:261-294,325-435`, `docs/ml-dataset/DESIGN.md`, `docs/ml-dataset/OPERATIONS.md`, and `research/ml_dataset_local_generation_20261031/RESULT-01.md`. This mechanical pilot does not assert methodological novelty or performance against external tools.

The administrative `campaign.py` has `check` (hash/XFS/preflight admission only) and `run` modes. `run` is a one-attempt, bounded 16-shard controller: it writes a durable attempt marker before each native invocation, refuses a live or ambiguous retry, enforces the 4 GiB XFS floor, then captures verify/report/export outcomes separately. Every attempted outcome, including censored root calls and verify failures, is eligible for complete restricted archive and pointer custody with fresh S3 GET/hash checks. Git-ready per-cell receipts are written under `checkpoints/`; the campaign operator must commit/checkpoint those receipts routinely. A read-only `check` does not launch generation or contact S3.
