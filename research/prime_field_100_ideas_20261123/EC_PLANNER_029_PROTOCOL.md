# Frozen adaptive EC relation-block planner, proposal 29

Frozen 2026-11-25 (America/Los_Angeles), draft crypto PR #5165, before producer run. Proposal #29 calls for deep reinforcement learning to choose sieve intervals. This experiment instantiates a **finite-state reward-driven bandit** over exact signed `E(F_p)` relation candidates. It is an adaptive EC planner, not a deep-RL implementation or a full index-calculus solver. It tests whether paid training labels and online feedback improve one-block-per-target collection on a disjoint held-out curve at equal candidate budgets.

## Sources and frozen data split

The [earlier #29 block screen](EC_LEADS_001_034_PROTOCOL.md) measured exact signed hits for four equal-work blocks on one curve, with a four-target training/four-target holdout split. The [bit-feature control](EC_CLASSIFIER_001_034_PROTOCOL.md) showed label cost and false-negative risk. [Semaev 2004](https://eprint.iacr.org/2004/031.pdf), Theorem 1, supplies the signed point-relation condition; here candidate verification uses exact `elladd` rather than a polynomial oracle. KB status: ready, revision `750797a540fb4c38837c67b12df84d5b`, refreshed 2026-11-25T03:19:48Z, lexical index current; searches for EC relation scheduling, bandit block yields, and held-out curve controls returned related internal programs but no source that changes this small public-curve protocol. KB source `source:4548c0e85d47f9299ddc900c047c266d` points to the repository's technique inventory. No novelty claim is made.

Training curve: `E/F_1009:y²=x³+x+14`, prime order 1013. Held-out curve: `E/F_1019:y²=x³+x+20`, prime order 1033. Each uses its **own** first 16 nonzero liftable x-coordinates as base with GP's deterministic square root, then its next eight liftable x-coordinates as public target points. Training target x are fixed `[42,43,44,45,48,49,54,55]`; held-out target x are `[38,39,41,46,49,53,54,56]`. The code asserts curve orders, every point order, and those target sequences. The held-out curve/targets are absent from policy training. The two groups have different prime orders and base points; this is an out-of-curve transfer test, not a random within-curve split.

The candidate space for each target consists of ordered triples of base points and eight signs. Partition by the first base index into four contiguous blocks of four; every block contains `4×16²×8=8192` signed patterns, each requiring two point additions and one exact target comparison. A hit emits a 24-column signed row `[base_1..base_16 | -target_1..-target_8]`; row rank is computed modulo the held-out curve's order. Count targets with at least one exact hit. Rank and coverage are separately reported because duplicate witnesses need not add independent rows.

## Frozen planner and equal-work controls

Training labels: evaluate **all four blocks** on all eight training targets, charging `262,144` signed patterns, `524,288` candidate-sum additions, and `262,144` target comparisons. Initialize each block value `q_b` to its eight-target mean exact-hit count. The adaptive policy processes held-out targets in their fixed x order, choosing the block with highest current `q_b` (ties to lower index), observing exact hits **only from that chosen block**, then updating `q_b ← (q_b + reward)/2`. This is a deterministic exponential moving-average bandit; later choices can depend on earlier held-out rewards, but no unchosen held-out block label is read by this policy. It uses no target bit features, target logs, or hidden labels.

Controls, each replayed independently on the same eight held-out targets with exactly one block per target: (1) fixed `train_best`, the highest eight-target training mean (same paid training labels as adaptive); (2) `static_1`, always block 1 and no training labels; (3) `seeded_random`, LCG seed `20261125`, update `s=(1103515245s+12345) mod 2³¹`, choose `1+s mod4`, with no training labels. Each held-out arm pays `65,536` signed patterns, `131,072` candidate-sum additions, and `65,536` target comparisons. The adaptive and train-best **cold** candidate work includes the full `262,144`-pattern training charge plus their own held-out work; static/random cold work includes only their own held-out work. Training and held-out curve/base setup CPU/wall are printed separately, along with each arm's elapsed time. The four arms share a process for evaluation, so per-arm wall readings are diagnostic, not isolated-run speed estimates.

Positive controls: all selected point triples and target points lie in the asserted prime-order group, each emitted row is tied to an exact point-sum equality, and the row rank is computed over the correct subgroup order. The source prints per-target chosen block, reward, and updated adaptive `q` to make feedback dependence auditable. Stop on GP stderr, failed guards, missing summaries or sentinel; preserve failures and do not silently retune. No cloud spend or external target. The checkpoint gate remains failed (`.resume/RESUME.md` missing; earlier S3 `InvalidAccessKeyId`); the user's explicit local bypass admits this bounded run without claiming the gate passed. Producer results are `[pcc:unverified]` pending independent replay.

Frozen GP 2.17.3 [source](EC_PLANNER_029_GP.gp) SHA-256 `70ff65790999288e465b72c75b7cefaa535fe1a97dabac20d0a09f091046c4b7`. Invoke once from repo root:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_PLANNER_029_GP.gp > research/prime_field_100_ideas_20261123/EC_PLANNER_029.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_PLANNER_029.stderr.txt
```

Record shell status immediately in `EC_PLANNER_029.exit.txt`. No output is admitted at this freeze point.

## V1 observation and random-control amendment

The frozen V1 producer ran once: shell exit `0`, empty stderr, final sentinel present. Its training on the F1009 curve gave block totals `[37,37,51,49]`, hence initial `q=[37/8,37/8,51/8,49/8]` and fixed train-best block 3. The adaptive arm on the disjoint F1019 curve made feedback-dependent choices `[3,4,1,1,1,1,1,1]`, with 54 exact hits, target coverage 7/8 and 24-column rank 20; train-best found 54/rank 21, static block 1 found 61/rank 22. The V1 seeded LCG random choice sequence was `[3,4,1,2,3,4,1,2]`, yielding 70/rank 22. Its lower two LCG bits impose a four-cycle, so this is a **weak random comparator**, even though it was frozen without labels. Retain V1 unchanged; the adaptive/static findings are still admitted at their precise scope. V1 [source](EC_PLANNER_029_GP.gp), [stdout](EC_PLANNER_029.stdout.txt), [stderr](EC_PLANNER_029.stderr.txt), and [exit](EC_PLANNER_029.exit.txt) SHA-256 are respectively `70ff65790999288e465b72c75b7cefaa535fe1a97dabac20d0a09f091046c4b7`, `33a5710d9d013331c928e6b720e4478e248e723a5cc6c84c2e83c1e18c75fe9e`, `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`.

Before any V2 run, freeze a **random-control-only** amendment: keep the LCG state update and seed, but choose `1+((s div 65536) mod 4)` from higher state bits. The source diff changes exactly that choice expression; the train/held-out curves, targets, policy, static controls, budgets, rank method and stop conditions remain identical. The amended random arm is still one deterministic seeded sample, not a distribution estimate. [V2 source](EC_PLANNER_029_GP_V2.gp) SHA-256 `011276216a83aae8b89b2506fbabf33044c558761bb85cf71ff44fbefb380bce`; GP 2.17.3. Invoke V2 once under the same local checkpoint bypass, with no silent further tuning:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_PLANNER_029_GP_V2.gp > research/prime_field_100_ideas_20261123/EC_PLANNER_029_V2.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_PLANNER_029_V2.stderr.txt
```

Record shell status immediately in `EC_PLANNER_029_V2.exit.txt`; stop on stderr, failed guard, or missing summaries. Both attempts remain in the PR.

## Independent scalar replay frozen after V2 producer

V2 ran once with exit `0`, empty stderr and a complete sentinel. The training totals and adaptive/train-best/static arms matched V1 exactly; the amended random arm chose `[1,1,3,1,4,1,1,1]`. To check exact relation labels and ranks by a different arithmetic path, freeze [qualifier source](EC_PLANNER_029_QUALIFIER.gp) SHA-256 `8aa848e4084e65961c07f9108b39cd7d9744194702162805b8403d8a45a79d7a`. The qualifier first constructs every training/held-out base and target point from the same public x rules, anchors each as a scalar multiple of an asserted generator by a bounded 1..r search, then counts signed triple congruences modulo r and recomputes the 24-column row ranks. It hardcodes the V2 arm choices and expected totals only as assertions after the policy run; it supplies no logs or held-out labels to the policy. This independent replay checks the full selected candidate streams and training totals, but shares the same deterministic input rule and matrix definition. Invoke once with GP 2.17.3 under the same local bypass:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_PLANNER_029_QUALIFIER.gp > research/prime_field_100_ideas_20261123/EC_PLANNER_029_QUALIFIER.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_PLANNER_029_QUALIFIER.stderr.txt
```

Record shell status immediately in `EC_PLANNER_029_QUALIFIER.exit.txt`. Stop on GP error, missing sentinel or failed scalar/rank guard; preserve a failure without silently changing the replay.

## Observed V2 policy and independent replay

V2 and its scalar qualifier each exited `0` with empty stderr and a completion sentinel. The qualifier anchored all 24 points on each curve to a verified generator, recomputed the training totals and every selected signed candidate using scalar congruences rather than the producer's elliptic-curve `elladd`, and reproduced each held-out row rank. This is an independently replayed bounded EC relation-collection result `[pcc:unverified]`; it does not train a neural network or solve a target logarithm.

Training on the F1009 curve yielded block-hit totals `[37,37,51,49]` after evaluating all eight targets and all four blocks. The frozen policy initialized `q=[37/8,37/8,51/8,49/8]` and selected block 3 first. Its first held-out reward was 2, lowering block 3's q to `67/16` and causing a switch to block 4; the next reward of 2 lowered block 4's q to `65/16`, causing a switch to block 1. Later updates retained block 1. These choices depend on rewards already collected, with no unchosen held-out label observed by the policy.

| Held-out policy | Block choices for x=`[38,39,41,46,49,53,54,56]` | Exact hits | Targets covered | Row rank / 24 | Training labels charged? |
|---|---|---:|---:|---:|---|
| Adaptive EMA bandit | `[3,4,1,1,1,1,1,1]` | 54 | 7/8 | 20 | Yes |
| Fixed training-best | `[3,3,3,3,3,3,3,3]` | 54 | 7/8 | 21 | Yes |
| Static block 1 | `[1,1,1,1,1,1,1,1]` | 61 | 7/8 | 22 | No |
| Seeded upper-bit LCG sample | `[1,1,3,1,4,1,1,1]` | 64 | 7/8 | 22 | No |

Every held-out arm spent exactly `65,536` signed patterns, `131,072` candidate-sum point additions, and `65,536` target comparisons. Training cost `262,144` additional patterns, `524,288` additions, and `262,144` comparisons; thus the **cold** candidate-pattern charge is `327,680` for adaptive/train-best versus `65,536` for static/random, plus curve/base setup and the small policy arithmetic. Target x=56 yielded zero hits for every selected arm; coverage is 7/8. The adaptive arm found 7 fewer witnesses and two fewer independent rows than free static block 1 despite paid training. This is a negative result for this frozen policy on this disjoint curve, not a general limit on adaptive scheduling or deep RL. The earlier within-curve block ranking did not transfer as a guaranteed advantage.

GP 2.17.3 CPU/wall milliseconds in V2: training `391/1254`, held-out setup `0/0`, adaptive `96/144`, train-best `100/526`, static `102/453`, seeded sample `104/270`. V1 had substantially lower wall times in the same code paths (training `365/387`, adaptive `91/94`, static `94/103`), showing host contention/order sensitivity; the matched candidate-work counts are the cleaner comparison. The four held-out arms ran serially in one GP process. Neither V1 nor V2 supplies an isolated wall-time speed claim. The V1 low-bit random sample and V2 upper-bit sample are both retained; one seeded sequence is not a random-policy distribution.

| Preserved V2 or qualifier artifact | SHA-256 |
|---|---|
| [V2 source](EC_PLANNER_029_GP_V2.gp) | `011276216a83aae8b89b2506fbabf33044c558761bb85cf71ff44fbefb380bce` |
| [V2 stdout](EC_PLANNER_029_V2.stdout.txt) | `cf329bd5e883bf8bcd5252873df82e15d6b39244e7f2444b31fc49651684198e` |
| [V2 stderr](EC_PLANNER_029_V2.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [V2 exit](EC_PLANNER_029_V2.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [Qualifier source](EC_PLANNER_029_QUALIFIER.gp) | `8aa848e4084e65961c07f9108b39cd7d9744194702162805b8403d8a45a79d7a` |
| [Qualifier stdout](EC_PLANNER_029_QUALIFIER.stdout.txt) | `64b8f9bf40bdd9d3174397894d8a6b56a6e8016c3b3aaed92866c899156343c4` |
| [Qualifier stderr](EC_PLANNER_029_QUALIFIER.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Qualifier exit](EC_PLANNER_029_QUALIFIER.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
