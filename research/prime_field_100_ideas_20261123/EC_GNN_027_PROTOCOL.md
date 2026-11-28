# Frozen one-hop EC relation message-passing screen, proposal 27

Frozen 2026-11-25 (America/Los_Angeles), draft crypto PR #5165, before compile or producer execution. The submitted #27 graph has integer/factor nodes joined by shared prime factors, which does not directly define exact `E(F_p)` point relations. This experiment uses an explicit EC adaptation: candidate-tuple nodes, public signed factor-base point nodes, and public target-point nodes. A one-hop typed message aggregator learns a candidate relation score from four point neighbors. It is a **minimal message-passing classifier**, not the full original heterogeneous number-factor GNN, and it does not claim a complete target solver.

## Sources and fixed public split

The prior [#27/#31 classifier control](EC_CLASSIFIER_001_034_PROTOCOL.md) used bit-Hamming nearest neighbors on exact EC block labels and charged label acquisition. The [#29 disjoint-curve planner](EC_PLANNER_029_PROTOCOL.md) fixed the two curve/target rules used here. The repository [EC technique inventory](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md) and KB revision `750797a540fb4c38837c67b12df84d5b` (ready, refreshed 2026-11-25T03:19:48Z) were checked for existing graph/prime-field relation mechanisms; searches included “graph neural network elliptic curve relation graph signed point sums factor base missing edges” and “message passing GNN ECDLP index calculus relation prediction held out curve false negatives rank”. KB hits included `source:4548c0e85d47f9299ddc900c047c266d` (repository technique inventory) and related internal graph proposals; these are pointers, not evidence of a solved classifier. [GraphSAGE](https://arxiv.org/abs/1706.02216) motivates inductive neighborhood aggregation, and [R-GCN](https://arxiv.org/abs/1703.06103) motivates distinguishing edge/node roles. The implementation below is a fixed small specialization, not a replication of those papers' architectures or results.

Train only on public `E/F_1009:y²=x³+x+14`, prime order 1013; evaluate only on disjoint public `E/F_1019:y²=x³+x+20`, prime order 1033. For each, use the first 16 nonzero liftable x-coordinates and the next eight liftable x-coordinates as targets. Unlike GP's square-root convention, the native Rust code chooses the **smallest nonnegative y** satisfying the curve equation for each x; the fixed target-x sequences are `[42,43,44,45,48,49,54,55]` and `[38,39,41,46,49,53,54,56]`. Assert a full-order generator on each curve, the target lists, and every selected row's exact point sum. No target log or held-out label enters training, features, initialization, or model scoring.

## Graph, model, labels, and training

Each `(i,j,k,mask,target)` candidate is a node with three signed edges to base-point nodes and one target-role edge to the target node. Point-node public features are normalized affine x and y. A signed edge uses the y-coordinate of the negated EC point when its sign bit is one. The fixed six-component candidate aggregate is `[1, mean(base x/p), mean(signed base y/p), target x/p, target y/p, mean((base x/p)^2)]`. Three base edges share the same aggregation rule; the target edge has a separate role. A learned width-8 tanh hidden layer and learned linear output map the one-hop aggregate to a score. Weights have a fixed small arithmetic initialization (source), with three deterministic epochs of SGD at learning rate 0.01. Each epoch alternates every exact positive training candidate with one deterministic hash-selected negative from the **training curve only**; the source checks that the negative label is zero. This deliberately small balanced training set is not a deep network or a tuned architecture.

Exact labels are formed by two EC additions per signed triple to compute all 32,768 candidate sums once per curve, then comparison with eight target points (`262,144` target comparisons). The train-label generation charge is `65,536` group additions plus `262,144` comparisons, feature construction, and all SGD updates. The disjoint held-out full-grid labels are generated only for **retrospective audit**, after training; their `65,536` additions and `262,144` comparisons are reported separately and do not inform model selection. The audit is not free evidence, but it is excluded from online model inference cost because an online classifier would not know rejected candidates' exact labels.

For each held-out target, score all 32,768 candidates and select the top `K=8192` (tie to lower ID). Compare four equal-verification-budget arms: (1) message-passing score; (2) cheap public feature `-popcount((x_i xor x_j xor x_k) xor x_target)`; (3) static first-index block 1 of exactly 8192 candidates; (4) deterministic SplitMix64 seeded hash ranking with seed `20261125`. Cheap/static/random use no training labels. Every arm then **recomputes** the exact EC sum for all `8×8192=65,536` selected candidates, charging `131,072` additions and `65,536` target comparisons. Model inference additionally scores `8×32,768=262,144` candidates through the 8-by-6 hidden map, and sorts each target's list; cheap/random also score/hash and sort, static does not. The producer prints full-grid audit hit/rank/coverage and each selected arm's exact hits, false-positive checks, missed true relations, target coverage, and modular relation-row rank over the held-out group order. A relation row has 16 base coefficients and eight target columns. False-negative rank loss is full-grid rank minus selected rank, not a claim that each missed row is independent.

The model is a ranking filter; every accepted relation still needs exact group verification. Equal `K` controls the expensive candidate verification work, while model training and full-grid inference are additional paid components. The experiment cannot infer a runtime gain from a positive top-K precision count alone. Holdout is one curve with eight targets, so any empirical ranking difference has that scope only. The full-grid held-out audit is a separate retrospective cost, not an online advantage.

The checkpoint gate remains failed (`.resume/RESUME.md` absent; earlier S3 `InvalidAccessKeyId`); the user's explicit local bypass and renewed public EC run instruction admit this bounded local run. No cloud spend or external target. Stop on compile failure, Rust panic, missing sentinel, or nonzero exit; preserve stdout/stderr/exit and no silent retuning. Binary is local-only under `/tmp`, never staged. Results are producer findings `[pcc:unverified]` pending independent audit.

Frozen Rust 1.99.1 [source](EC_GNN_027.rs) SHA-256 `83181f42ba9a50aa7afab877fee0d69a3acb828bcfc8d67df25db6930243be1f`. One compile and run from repo root:

```sh
TMPDIR=/tmp rustup run 1.99.1 rustc -O --edition 2021 research/prime_field_100_ideas_20261123/EC_GNN_027.rs -o /tmp/ec_gnn_027.bin
/tmp/ec_gnn_027.bin > research/prime_field_100_ideas_20261123/EC_GNN_027.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GNN_027.stderr.txt
```

Record both compile and run statuses in `EC_GNN_027.exit.txt`, and source, binary, raw-stream hashes below. No output is admitted at this freeze point.

## Observed bounded result

The frozen Rust 1.99.1 source compiled once and ran once; both statuses were `0`. The compiler emitted one nonfatal `unused variable: label_ms` warning; it is preserved in the tool transcript, while the runtime [stderr](EC_GNN_027.stderr.txt) is empty. The output has all eight target rows per arm and the completion sentinel. The local binary remains under `/tmp` and is not staged. These are local producer results `[pcc:unverified]` pending an independent model-output replay.

Training generated exact labels for `262,144` candidate-target pairs from `32,768` distinct signed sums (`65,536` group additions) and found 174 positive relations. Three balanced SGD epochs made `1,044` weight updates. The measured combined label-generation/training time was 4 ms in optimized Rust on this host. The disjoint held-out **retrospective audit** generated another `262,144` labels from `65,536` additions in 3 ms. Model, cheap-feature and hash controls each scored `262,144` candidates; the combined scoring and sorting phase took 49 ms. These audit labels were generated after training and were not supplied to the model or selection policies.

| Held-out arm, top 8,192 per target | Exact verified hits | False-positive verification checks | Missed true hits | Targets covered / 8 | Row rank / 24 | Rank below full grid |
|---|---:|---:|---:|---:|---:|---:|
| One-hop message passing | 51 | 65,485 | 172 | 4 | 9 | 13 |
| Cheap x-coordinate XOR/popcount | 61 | 65,475 | 162 | 5 | 13 | 9 |
| Static first-index block | 61 | 65,475 | 162 | 7 | 22 | 0 |
| Seeded hash sample | 60 | 65,476 | 163 | 7 | 22 | 0 |
| Full-grid retrospective audit | 223 | — | 0 | 7 | 22 | 0 |

Each selected arm verified exactly `65,536` candidates by recomputing the point sums (`131,072` group additions and `65,536` target comparisons). Model inference paid an additional `262,144` width-8/feature-6 forward scores and per-target top-K sorts; cheap/hash controls paid their own simpler ranking and sorts; static needed neither. Model training paid the full training-label collection above, feature construction, and 1,044 SGD updates. The source measured 95 ms total for training, held-out audit, selection and all four arm verifications/rank audits in one process; it did **not** isolate a per-arm wall clock. Thus no wall-time speed comparison is inferred. The static block's exact per-target hit counts `[9,2,10,2,6,12,20,0]` and rank 22 agree with the independently implemented [#29 planner control](EC_PLANNER_029_PROTOCOL.md) on the same held-out x sequence, though this run uses a separately defined point-root convention.

The message-passing model was worse than the cheap and static controls at this one frozen split: it discarded 172 of 223 exact hits and 13 of the full grid's 22 independent row dimensions, and lost coverage of three targets that the full grid covered. Its top-K exact-hit rate `51/65,536` was also below the full-grid rate `223/262,144`. The selected rows remain exact because every one was group-verified; the model score itself does not certify a relation. This outcome is evidence about this small one-hop architecture and feature map only. The original proposal's integer shared-prime-factor graph has no literal EC edge definition, and deeper learned architectures or other feature maps are not tested.

The public input and audit group orders were checked through full-order generators and fixed target-x sequences. An independent replay of all model scores/selection IDs was not performed; only the static-arm reward/rank cross-check noted above exists outside this producer. No failed or censored experimental arm occurred after compilation. The compiler warning had no effect on the binary, model, or labels.

| Preserved artifact | SHA-256 |
|---|---|
| [Source](EC_GNN_027.rs) | `83181f42ba9a50aa7afab877fee0d69a3acb828bcfc8d67df25db6930243be1f` |
| [Stdout](EC_GNN_027.stdout.txt) | `c3fd4aaeaa2ae0e9304c6cf3594a76a940127a4cfa96765e2b24eed17360d437` |
| [Runtime stderr](EC_GNN_027.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Compile/run exit](EC_GNN_027.exit.txt) | `6ae53545e4b9562543543c003678fd596b448a77bc6940dfe87010b843763960` |
| `/tmp/ec_gnn_027.bin` (local only) | `be4084ae3d09493259251513f9edf66df28ec7eeae427b9160524aee4b709ecd` |
