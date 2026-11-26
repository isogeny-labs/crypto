# Frozen same-target S4 relation pipeline and rho, proposal 24

Frozen 2026-11-25 (America/Los_Angeles), draft crypto PR #5165, before producer measurement. Semaev's `S3` and `S4` resultant give the exact abscissa condition for a signed three-point sum; the prior [same-target component](S4_SAME_TARGET_PROTOCOL.md) established a 16-point public-grid implementation but did not assemble enough rank to solve. This experiment attempts a complete bounded relation pipeline on one fixed public prime-order curve, with a matched same-target Pollard rho control. Producer results remain `[pcc:unverified]` until the separate accepted-row replay passes.

## Frozen input and relation algorithm

`E/F_1009:y²=x³+x+14`, `#E=1013` prime, `G=(0,425)`, and `Q=(42,sqrt(42³+42+14))` using GP's deterministic modular square root. Q is selected by its abscissa; neither the producer nor qualifier uses a planted Q logarithm to form it. Factor base: first 16 nonzero liftable abscissae in ascending order, deterministic GP roots, ending at x=40. Every point/order is guarded. Unknowns are the 16 factor-base logs and `d=log_G(Q)`. Each verified signed relation `Σe_iF_i=aG+bQ` gives linear row `[e_1,…,e_16,-b]·[f_1,…,f_16,d]^T=a` over `F_1013`. `[pcc:unverified]`

Phase 0 scans `a=1..64,b=0` in order until 16 independent rows; phase 1 scans `a=0..64,b=1` until 17 independent rows. Each attempted finite point constructs `S4(X,Y,W,x(aG+bQ))` by exact resultant, evaluates all `16³=4096` ordered abscissa triples, enumerates all eight sign patterns per triple, and requires `S4=0` iff at least one signed sum equals that exact point. It emits every newly independent relation as `rel=[a,b,i,j,k,mask]` and retains the full per-target zero/witness count and rank transitions. No independent row is selected from a claimed unknown log. Upon rank 17, solve the modular 17-variable system, check its residual, every factor-base point against the solved log, and `Q=[d]G`. If either phase cap is reached before its rank gate, print a censored state and still run rho; do not silently change base or cap. Maximum 129 target points, 528,384 polynomial evaluations, and 4,227,072 signed patterns, bounded public-curve work. `[pcc:unverified]`

## Matched rho control and cost

After the relation arm, independently initialize the same curve and same literal `G,Q` points for Floyd rho. Start at `G+Q` with coefficients `(u,v)=(1,1)`. Partition by `x(X) mod 3` (infinity in bucket 0): bucket 0 adds G, bucket 1 adds Q, bucket 2 doubles X, updating `(u,v)` consistently. One tortoise step and two hare steps per iteration, cap 10,000 iterations. A collision yields `d=(u_h-u_t)/(v_t-v_h) mod1013` if the denominator is nonzero; verify `[d]G=Q`. Degenerate collision or cap is censored and preserved, with no new seed. This baseline does not use the relation-arm solution.

Charge relation cold curve/base setup, each resultant, target construction (two small scalar multiplications and one addition), every S4 evaluation, signed sum (two group additions), rank tests, modular solve, and final point checks. Report GP CPU and wall milliseconds for relation cold setup, collection plus solve, and total. Rho separately charges its own curve/point setup, initial point addition, all three steps per Floyd iteration, and final point check; report its CPU and wall milliseconds. Some GP primitive costs are not instrumented separately, but both total clocks include them. The fixed arm order is relation then rho in one GP process; timings are a bounded one-target comparison, sensitive to warmup and millisecond resolution. A single success cannot establish scaling.

Independent qualifier: parse the producer's `rel` lines, reconstruct each signed EC sum and S4 resultant at its recorded target, independently build and solve the 17-column matrix, and verify all factor-base/target logs. This checks accepted rows, not all producer-scanned rejected triples; the producer's exhaustive S4/group equivalence remains a self-check. Preserve stdout, stderr, exit, source hashes, including failures.

The checkpoint gate remains failed (`.resume/RESUME.md` absent; prior S3 `InvalidAccessKeyId`). The user explicitly authorized local bypass and renewed public E(F_p) runs; this is a local-only exception, not a passed gate. No cloud spend or third-party target. Stop on GP stderr or failed guard even if GP returns exit zero or prints a later sentinel. Do not retry or change inputs silently.

Frozen producer [source](EC_S4_PIPELINE_024_GP.gp) SHA-256 `58b8a05b1722939e9bb3aecfd2c87741c054ac30d1f64fbdc15f0de2a48501b7`; frozen independent [qualifier source](EC_S4_PIPELINE_024_QUALIFIER.gp) SHA-256 `243952ae1c8af4040c1c13e60431d1a54b394313a7f3ba79342281c6cfc1cfde`. PARI/GP 2.17.3 on arm64 macOS. Producer invocation once from repo root:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024_GP.gp > research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024.stderr.txt
```

Record shell status immediately in `EC_S4_PIPELINE_024.exit.txt`. If producer is valid and reaches rank 17, run the qualifier once with:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024_QUALIFIER.gp > research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024_QUALIFIER.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024_QUALIFIER.stderr.txt
```

Record status immediately in `EC_S4_PIPELINE_024_QUALIFIER.exit.txt`. No result is admitted at this freeze point.

## Observed local same-target result

The frozen producer and independent qualifier each ran once with GP 2.17.3. Both shell exits were `0`, both stderr files were empty, and both completion sentinels appeared. This is a **complete bounded small-curve point-relation pipeline** on the one fixed public target, not an asymptotic or cryptographic-size improvement claim. The selected point was `Q=(42,259)`; neither producer collection nor rho used a planted logarithm. Both independently recovered `log_G(Q)=906` and checked `[906]G=Q`. The qualifier independently checked each of the 17 accepted rows by exact signed point addition and S4 resultant evaluation, reconstructed rank 17, solved the matrix, and verified all 16 factor-base logs. The producer additionally checked S4 zero equivalence against all signed sums on every scanned grid. `[pcc:unverified]`

| Relation phase | Deterministic target points tried | Rank reached | Exact S4-zero / signed-witness triples |
|---|---:|---:|---:|
| `b=0`, `a=1..6` | 6 | 16 factor-base columns | 186 / 186 |
| `b=1`, `a=0` | 1, target `Q` | 17 columns including target | 24 / 24 |
| **Total** | **7** | **17 / 17** | **210 / 210** |

The producer's accepted independent-row transitions were `4,13,15,15,15,16` through `a=1..6,b=0`, then `17` at `a=0,b=1`. The complete 17 accepted `rel=[a,b,i,j,k,mask]` records and each target's zero/witness count are in [producer stdout](EC_S4_PIPELINE_024.stdout.txt). The modular solution gave base logs `[868,129,828,470,398,186,241,659,886,690,71,582,826,915,60,300]` and target log `906`; all point checks passed in both producer and qualifier.

| Same Q workload | Frozen relation pipeline | Floyd rho |
|---|---:|---:|
| Curve and target | `E/F_1009`, `G=(0,425)`, `Q=(42,259)` | Identical |
| Cold setup CPU / wall ms | 0 / 4 | 0 / 0, below GP's integer-ms resolution |
| Total CPU / wall ms | 415 / 692 | 0 / 1, below CPU timer resolution |
| Resultant constructions | 7 | 0 |
| S4 polynomial evaluations | 28,672 | 0 |
| Signed candidate patterns | 229,376 | 0 |
| Candidate signed-sum additions | 458,752 | 0 |
| Target construction | 14 scalar multiplications + 7 additions | Fixed literal Q |
| Rho walk | 0 | 23 Floyd iterations, 69 step additions + 1 initial addition |
| Result | rank 17, independently checked exact rows, `d=906` | nondegenerate collision, checked `d=906` |

The relation total includes its curve/base setup, collection, rank tests, modular solve, and point-verification work; rho separately initializes and validates the same curve/G/Q and includes its initial state, walk, collision solve, and target check. The relation arm is much more expensive on this fixed tiny subgroup by the reported work counts and local wall time. The 1 ms rho total is at timer granularity, and rho ran second in one GP process, so the ratio of those two wall readings is **not** a stable speedup estimate. No target, factor base, scan cap, or rho seed was changed after seeing results. There was no censored arm or producer failure in this frozen attempt. `[pcc:unverified]`

The qualifier is independent for accepted relation correctness, rank, and solved logs; it does not independently repeat all 28,672 S4 grid evaluations or 229,376 signed-pattern checks. The producer's exhaustive zero/witness agreement is a self-check. The same-target rho arm is a single fixed-seed run, and no claim is made for larger curves or full-scale index calculus. `[pcc:unverified]`

| Preserved artifact | SHA-256 |
|---|---|
| [Producer source](EC_S4_PIPELINE_024_GP.gp) | `58b8a05b1722939e9bb3aecfd2c87741c054ac30d1f64fbdc15f0de2a48501b7` |
| [Producer stdout](EC_S4_PIPELINE_024.stdout.txt) | `841e51408bf25065b40c999467d81c2ee381e8d1552adb67bca62af4ebdcc865` |
| [Producer stderr](EC_S4_PIPELINE_024.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Producer exit](EC_S4_PIPELINE_024.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [Qualifier source](EC_S4_PIPELINE_024_QUALIFIER.gp) | `243952ae1c8af4040c1c13e60431d1a54b394313a7f3ba79342281c6cfc1cfde` |
| [Qualifier stdout](EC_S4_PIPELINE_024_QUALIFIER.stdout.txt) | `2f692c4e2776f5219277d9d1f3a395a72542438a944453776920971cd6f9ac6c` |
| [Qualifier stderr](EC_S4_PIPELINE_024_QUALIFIER.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Qualifier exit](EC_S4_PIPELINE_024_QUALIFIER.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
