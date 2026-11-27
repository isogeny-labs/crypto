# Frozen EC graph and coefficient rank check (#35–39)

**Status:** V1 failed acceptance; separately admitted V2 passed the local exact checks. Both attempts and their raw output are retained below. Conductor tasks `T-12177`, `T-12198`, and `T-12205` reserved preparation, V1, and V2 work respectively.

## Question and controls

Can a graph cycle or a topology derived solely from relation support determine the rank of a prime-field **elliptic-curve** index-calculus relation matrix? The support-only control is the identical triangle graph in both arms. The coefficient-aware baseline is exact rank over the subgroup order, with every row independently checked in the curve group.

Public synthetic curve: `E/F_1009: y²=x³+x+14`, group order `r=1013`, generator candidate `P=(0,425)`. The script checks the group cardinality, primality, and point equation before evaluating rows. Factor-base points are `F1=P`, `F2=2P`, `F3=3P`. They are generated from known logs to make the row witnesses inspectable; this is a rank-mechanism test, not a relation-collection benchmark.

The frozen matrices and right-hand sides are

```text
A = [1 1  0]   b_A = [ 3]
    [0 1  1]         [ 5]
    [1 0  1]         [ 4]

B = [1 1  0]   b_B = [ 3]
    [0 1  1]         [ 5]
    [1 0 -1]         [-2]
```

Each row asserts `Σ_j C_ij F_j = b_i P`; the GP source checks all six group equations. The last row sign is the only changed coefficient, so the support graph and its single cycle are the same. `det(A)=2` is nonzero modulo 1013, while `det(B)=0` and its first two rows have a unit 2×2 minor. The prediction is ranks **3 and 2**. A support-only rank rule necessarily gives the same answer for both and fails on one; the exact modular baseline distinguishes them.

## Launch and acceptance

The original gate required a passing S3 checkpoint before running the command below. The dated admission amendment records the user's explicit local-bypass authorization after that gate returned HTTP 403.

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_GRAPH_RANK_GP.gp
```

Capture stdout and stderr separately in a new, scope-checked raw-output file. Reject any `***` GP error line or nonzero exit code. Require the printed curve/order, six checked rows, equal support, and the two rank lines exactly as stated. Do not select another curve, coefficient, or control after viewing output. Budget: one local GP process, under 60 seconds and 100 MiB expected; stop rather than retry automatically if it exceeds either limit. No external target or paid resource is involved.

The check can falsify the literal topology-only rank claim. It cannot establish relation yield, independent-rank accumulation rate, a successful target descent, or a complete-solver cost advantage. An affirmative path for ideas #35–39 would require a coefficient-aware topology statistic that predicts a useful outcome on held-out, independently collected EC relations, followed by a full-cost comparison with the existing large-prime filter or exact rank baseline.

## Admission amendment — 2026-11-24, 20:34 PST

The parent reported that the repository S3 checkpoint gate returned HTTP 403. After being told this, the user explicitly authorized: “Run locally with bypass.” This authorizes this one local public synthetic check despite that reported gate failure; it does not authorize a cloud run or change any scientific acceptance condition. The GP source SHA-256 before launch is `23e127616127232577ac68bc98dbdb607954fe81ee27e1cc4d489ee8b675d22f`. The frozen input, six EC row witnesses, matrix controls, subgroup modulus, 60-second limit, 100 MiB expected memory ceiling, and one-attempt allowance above remain unchanged. The inner command remains `gp -q < research/prime_field_100_ideas_20261123/EC_GRAPH_RANK_GP.gp`, wrapped only by `timeout 60` for the time cap. Standard output, standard error and exit code will be preserved in separate files. The reported HTTP status is parent-provided context, not an independently repeated request by this workstream.

## Execution attempt 1 — 2026-11-24, 20:35 PST

The one allowed local invocation completed in approximately 0.02 s and the shell recorded exit code `0`, but **failed the frozen acceptance rule** because GP printed `***` syntax/type errors to standard error. The multiline `for(i=1,3,...)` and nested support loop were parsed as incomplete top-level statements, so the six relation-row checks and support equality check did not execute correctly. The later determinant checks and print statements still produced the predicted lines, but their presence cannot certify the intended experiment. No automatic retry or source change was made. Raw evidence: [`EC_GRAPH_RANK_GP.stdout.txt`](EC_GRAPH_RANK_GP.stdout.txt), [`EC_GRAPH_RANK_GP.stderr.txt`](EC_GRAPH_RANK_GP.stderr.txt), and [`EC_GRAPH_RANK_GP.exit.txt`](EC_GRAPH_RANK_GP.exit.txt). Source hash at invocation remained `23e127616127232577ac68bc98dbdb607954fe81ee27e1cc4d489ee8b675d22f`. The next attempt requires a separately admitted source revision using GP block syntax, with the same frozen curve, matrices, witnesses and controls; it must receive a new attempt record and explicit authorization under the one-attempt rule.

## V2 parser-repair admission — 2026-11-24, 20:37 PST

The parent explicitly authorized a separately recorded V2 parser repair and one V2 run under the user's local-bypass instruction. [`EC_GRAPH_RANK_GP_V2.gp`](EC_GRAPH_RANK_GP_V2.gp), SHA-256 `4234f8c06f4acb764e59314ec85d72348925f6ee4c19907bcd605f5d4402b87f`, moves the two row-check loops, support loop and determinant checks into `verify_all()`. The curve, point, factor-base points, matrices, right-hand sides, six group-equation checks, support check, determinant checks and printed predictions are byte-for-byte unchanged in substance. V1 source and all V1 raw outputs are retained. Run `timeout 60 gp -q < research/prime_field_100_ideas_20261123/EC_GRAPH_RANK_GP_V2.gp` once locally; preserve V2 stdout, stderr and exit separately. The same 60-second, expected 100 MiB, no paid resource limits apply. Accept only if stderr contains no `***`, exit is zero, all checks execute and the printed rank/support lines match. No automatic retry follows a V2 failure.

## V2 execution — 2026-11-24, 20:38 PST

The single V2 invocation passed its local exact acceptance rule. [`stdout`](EC_GRAPH_RANK_GP_V2.stdout.txt) printed group order `1013`, factor-base points `P=(0,425)`, `2P=(991,641)`, `3P=(162,510)`, equal triangle support, six completed row checks, `det(A)=2`/rank 3 and `det(B)=0`/rank 2. [`stderr`](EC_GRAPH_RANK_GP_V2.stderr.txt) has zero bytes and [`exit`](EC_GRAPH_RANK_GP_V2.exit.txt) contains `0`. The source SHA-256 at execution was `4234f8c06f4acb764e59314ec85d72348925f6ee4c19907bcd605f5d4402b87f`; stdout SHA-256 is `1a2f0f09992538dd789f38fda43a5bd20765580f952dba7181e3e99b81e59f83`. V1 stdout has that same hash despite its parse errors; only V2 completed the guards, so stderr and source identity are necessary evidence. The tool call returned in about 0.25 s, including wrapper overhead; peak memory was not measured, so the 100 MiB expectation is unverified.

The result rejects the **support-only rank inference** on these two public EC relation matrices: identical support topology yields different modular ranks. It does not show that a coefficient-aware topological feature cannot help relation search or matrix filtering, and it does not measure any EC discrete-log solve advantage. The actual factor-base points were generated as known multiples of `P`, making the group-equation checks a controlled synthetic diagnostic rather than a relation collector test. Repository proof-carrying verification has not signed this result.
