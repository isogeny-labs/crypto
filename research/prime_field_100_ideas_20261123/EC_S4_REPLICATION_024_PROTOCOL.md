# Frozen second-curve S4 relation-pipeline replication

Frozen 2026-11-25 (America/Los_Angeles), draft crypto PR #5165, before producer execution. This is a distinct public prime-order curve and target from the [first complete #24 pipeline](EC_S4_PIPELINE_024_PROTOCOL.md). The method, 16-point base size, 64-target cap per phase, exact S4/group checks, rank gates, and fixed-seed same-target Floyd rho control remain the same. An independent accepted-row qualifier is frozen as a separate source. Preserve a failed or censored attempt rather than changing the selected curve, target, base, cap, or seed.

## Source review and selection

The repository [prime-field technique inventory](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md) identifies Semaev S3/S4 relation collection as integrated; the prior [same-target S4 report](S4_SAME_TARGET_PROTOCOL.md) and [first full pipeline](EC_S4_PIPELINE_024_PROTOCOL.md) give the local arithmetic and measurement controls. The original [Semaev 2004 paper](https://eprint.iacr.org/2004/031.pdf), Theorem 1, gives the S3 formula, S4 resultant recurrence, and signed point-sum condition. KB status before selecting the curve: revision `750797a540fb4c38837c67b12df84d5b`, refreshed 2026-11-25T03:19:48Z, ready; lexical index current, hybrid index absent. Queries covered “Semaev S4 prime field index calculus relation collection rank public curve”, “S4 summation polynomial degenerate target false positives signed witnesses”, “Pollard rho same target prime field EC index calculus comparison”, and the original Semaev Theorem 1. Relevant KB source IDs were `source:4548c0e85d47f9299ddc900c047c266d` (repo technique inventory) and `paper:1cab0ae675996cdba6393b1e369a22113fd2e74bcf1223595743dc140631c7d6` (original Semaev paper). The source review confirms this is a replication of established algebra, not a novelty claim.

Exploratory **input selection only**, before measurement: scan `p=1019`, `a=1..12`, `b=1..20` in ascending order for nonsingular `E:y²=x³+ax+b` with prime `#E`. The first such pair is `a=1,b=20`, with `#E=1033` prime. Use that fixed `E/F_1019`, `G=(0,64)` (first liftable x=0), factor base the first 16 nonzero liftable abscissae in ascending order (ending x=36), and target `Q=(38,sqrt(38³+38+20))`, the next nonzero liftable abscissa with GP's deterministic root. The exploratory enumeration printed `(38,245)`; the producer recomputes Q from x=38 and checks membership/order. Q's logarithm was not used in selection, source, or relation generation. Input search is not charged to either attack because the public curve and points are fixed workload parameters; each attack separately charges curve/point setup.

A read-only post-selection check enumerated `a=1,b=1..20` with the discriminant tested modulo 1019: orders for `b=1..19` were all composite (`1052,1080,969,1020,1041,1011,1057,972,1050,1062,992,996,984,1035,1050,1046,1005,961,1048`); `b=20` had prime order 1033. This confirms the stated deterministic first-curve rule independently of the later producer outcome.

## Frozen experiment and limits

Use [producer source](EC_S4_REPLICATION_024_GP.gp), adapted from the first pipeline only by field, curve, point, base endpoint, and output label; the diff has no relation or rho algorithm change. Phase 0 scans `a=1..64,b=0` until factor-base rank 16; phase 1 scans `a=0..64,b=1` until rank 17. For every tried finite `R=aG+bQ`, compute exact resultant `S4(X,Y,W,x(R))`; check all 4096 ordered base triples and all eight sign patterns, require algebraic zero iff a signed point sum equals R, retain each newly independent `rel=[a,b,i,j,k,mask]`. Solve the 17-unknown system only at full rank; verify its residual, all 16 base logs, and the target. If capped, emit `ic_censored` and still run rho. Maximum 129 target points, 528,384 polynomial evaluations, 4,227,072 signed patterns. Positive controls are curve/group order, point membership, S4 multidegree, exact zero/signed agreement, modular rank, and final point checks.

Matched rho: separately initialize this same E, G, Q; start Floyd's partition-by-x-mod-3 walk at G+Q with coefficients `(1,1)`, cap 10,000 iterations, preserve degenerate/censored collision and verify any recovered log. Relation total clocks cover its cold curve/base setup, target construction, all resultant/grid work, rank, solve, and final point checks. Rho separately charges its setup and full walk/recovery check. Record CPU/wall milliseconds and exact counted operations; timer granularity and fixed arm order limit runtime inference. The [independent qualifier](EC_S4_REPLICATION_024_QUALIFIER.gp) parses every accepted relation, recomputes its exact signed group sum and S4 zero, modular rank/solution, and all point logs; it does not independently scan rejected grid triples. A full end-to-end scaling claim would require larger matched curves and controlled repeats.

The checkpoint gate remains failed (`.resume/RESUME.md` absent; earlier S3 `InvalidAccessKeyId`). The user's explicit local bypass and renewed public E(F_p) run instruction admit this bounded local experiment. No cloud spend or external target. GP can return shell exit zero after errors, so require empty stderr and completion sentinel. Stop on any producer mismatch or GP error; retain raw failure without silent repair. Run qualifier once only if producer reaches rank 17 with empty stderr. No output is admitted at protocol freeze.

Frozen GP 2.17.3 source SHA-256: producer `5e398ffe467b62feeb66d0f13881e5dca3201cac1c20501fdbc7195aa5701e50`; qualifier `b78dbbfc4d8895327e4b1706599d34e70fec39e09d01e6b48d49ac491a948308`. One-shot producer command from repository root:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_S4_REPLICATION_024_GP.gp > research/prime_field_100_ideas_20261123/EC_S4_REPLICATION_024.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_S4_REPLICATION_024.stderr.txt
```

Record the shell exit immediately in `EC_S4_REPLICATION_024.exit.txt`. If admitted and solved, run one qualifier:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_S4_REPLICATION_024_QUALIFIER.gp > research/prime_field_100_ideas_20261123/EC_S4_REPLICATION_024_QUALIFIER.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_S4_REPLICATION_024_QUALIFIER.stderr.txt
```

Record its exit immediately in `EC_S4_REPLICATION_024_QUALIFIER.exit.txt`.

## Observed one-shot local replication

The frozen producer and qualifier each ran once with GP 2.17.3, shell exit `0`, empty stderr, and their own completion sentinel. The producer confirmed `E/F_1019:y²=x³+x+20`, prime order 1033, `G=(0,64)`, `Q=(38,245)`, and the frozen 16-point base `x=[7,8,10,13,16,18,19,20,22,23,24,28,29,31,35,36]`. Neither relation collection nor rho read a planted Q logarithm. The 17 accepted rows solved `log_G(Q)=316`, and every base log and target point identity was checked. The qualifier independently replayed those 17 rows as signed group sums and S4 zeros, reconstructed rank 17, solved the modular system, and checked all point logs. `[pcc:unverified]`

| Relation phase | Frozen scan actually used | Rank after phase | S4-zero triples | Signed point witnesses |
|---|---:|---:|---:|---:|
| Factor-base phase, `b=0` | `a=1..10`, 10 points | 16 | 343 | 390 |
| Target phase, `b=1` | `a=0`, exact Q | 17 | 21 | 21 |
| **Total** | **11 points** | **17 / 17** | **364** | **411** |

The phase-0 rank transitions were `2,9,14,14,14,15,15,15,15,16`; the Q target raised it to 17. At `a=6,b=0`, the target abscissa was 20, which is itself in the factor base: 79 S4-zero ordered triples had 126 signed witnesses. The producer recorded both counts and checked the exact zero-versus-existence condition for each triple, so `364` and `411` are intentionally different totals. This degeneracy did not enter any row without an exact signed-point check. The complete 17 accepted `rel` records and every target summary remain in [raw producer stdout](EC_S4_REPLICATION_024.stdout.txt).

| Same public Q workload | S4 relation pipeline | Matched Floyd rho |
|---|---:|---:|
| Cold curve/point setup CPU / wall ms | 0 / 4 | 0 / 0, below GP's integer-ms resolution |
| Total CPU / wall ms | 600 / 637 | 0 / 0, below timer resolution |
| S4 resultants / polynomial evaluations | 11 / 45,056 | 0 / 0 |
| Signed patterns / candidate-sum additions | 360,448 / 720,896 | 0 / 0 |
| Target construction | 22 scalar multiplications + 11 additions | Literal fixed Q |
| Rho walk | 0 | 7 Floyd iterations, 21 step additions + 1 initial addition |
| Recovered target log | 316, independently qualified | 316, checked by scalar multiplication |

The relation total includes cold curve/base setup, all relation targets, rank work, solve, and final point checks. Rho separately initializes the same curve/G/Q and includes its walk and final check. The exact operation counts show far more work in this relation collector for this target. The rho clock reads zero at GP's integer-millisecond resolution, so no finite runtime ratio is assigned. The fixed order puts rho second in one GP process, and this two-curve panel cannot establish a scaling trend. No cap, curve, target, or rho seed was changed after the frozen source. There were no failed or censored arms in this replication.

The qualifier checks all **accepted** rows independently, not every rejected triple or every signed pattern; the producer's exhaustive 45,056-grid equivalence remains a self-check. The input-selection search was outside both algorithm clocks, as declared before the run. This is one additional public prime-order curve/target, not a cryptographic-size performance result.

| Preserved artifact | SHA-256 |
|---|---|
| [Producer source](EC_S4_REPLICATION_024_GP.gp) | `5e398ffe467b62feeb66d0f13881e5dca3201cac1c20501fdbc7195aa5701e50` |
| [Producer stdout](EC_S4_REPLICATION_024.stdout.txt) | `48d4b66dd0b0f56ad1d1e500bcc05c0952ce927d4fe0bbc5dbc49317d6c37db0` |
| [Producer stderr](EC_S4_REPLICATION_024.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Producer exit](EC_S4_REPLICATION_024.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [Qualifier source](EC_S4_REPLICATION_024_QUALIFIER.gp) | `b78dbbfc4d8895327e4b1706599d34e70fec39e09d01e6b48d49ac491a948308` |
| [Qualifier stdout](EC_S4_REPLICATION_024_QUALIFIER.stdout.txt) | `4b0fc8fccf3d21f95885d9d8a6ba090f691abd1e79f14d6e4cbab88624197c61` |
| [Qualifier stderr](EC_S4_REPLICATION_024_QUALIFIER.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Qualifier exit](EC_S4_REPLICATION_024_QUALIFIER.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
