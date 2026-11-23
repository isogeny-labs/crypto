# VERIFY: LEAD-A-bilinear-lift-size-degree (skeptic pass, 2026-11-12)

An independent skeptic checked this result: reruns plus a cross-check against the legacy Python
closure. Every number below is `[pcc:unverified]`. The reruns were single-threaded on the same
shared container, so this is not a second host.

## Bottom line

**The reported verdict, "partial", holds up.**

- The key numbers reproduce bit for bit with a fresh build of the native tool.
- The key cells also reproduce with the independent legacy closure `lfd_small.closure`.
- The verdict follows the lane file's falsifiers (Lead A5, `lane_A_falldegree.md` l.375-424), and
  no threshold was moved.
- The failed negative control is reported honestly, not explained away.

I found nothing that refutes the result. Four caveats follow at the end.

## What I re-ran

### 1. Build and native reruns

`gcc -O3 -march=native -o $SCRATCH/polyclose polyclose.c -lm4ri -lm` produces a binary
byte-identical (`cmp`) to the committed `polyclose`. I reran 16 cells with
`python3 emit_lift.py ARGS | polyclose D 8192 0 0`. All agree with `out/main.jsonl` and
`out/table.md`:

| cell | result (W_dims) |
|---|---|
| orig n15 s1 D3 / D4 | stall [0,1,45,570] / fall [1,17,137,697,2517] |
| bil n11 s3 D2 / D3 | stall [0,17,710] / fall, rank 9,570 of 18,473, 0.6 s |
| bil n15 s1, s5 D2 | stall [0,23,1643] (falsifier 2 cell) |
| bil n15 s1 S={0} D3 | fall (24 vars, 2,325 cols) |
| bil n17 s1, s2 D2 | stall [0,26,2321] |
| bil n17 s1, s2 S={0,1} D3 | fall (36 vars, 7,807 cols) |
| bil n17 s1 S={0} D3 | stall [0,2,192,3014] |
| rand n15 r28 D3 / D4 | stall / fall (d_F(rand) = 4) |
| randbil n11 r10 D3 | fall, W_3 rank 15,176 |
| randbil n15 r28 S={0,1} D3 | stall [0,0,249,4614] |

### 2. Independent cross-check against the legacy Python closure

I piped the emitted systems through `crosscheck.py`, which runs the Python `lfd_small.closure` and
the native tool on the same system. All 8 cells agree:

| cell | verdict | dimensions |
|---|---|---|
| bil n15 s1 S={0} D3 | fall | identical |
| bil n15 s5 S={0} D3 | fall | identical |
| bil n15 s1 D2 | stall | identical |
| bil n17 s1 D2 | stall | identical |
| bil n17 s2 S={0,1} D3 | fall | identical |
| bil n17 s1 S={0} D3 | stall | identical |
| randbil n15 r28 S={0,1} D3 | stall | identical |
| orig n17 s1 D4 | fall | differ in the last entry (4,043 vs 4,048); expected, see note |

Note on the last row: dimensions at a fall are order-dependent, because both tools stop early.

### 3. Records and default-unchanged evidence

- The committed records in `research/lfd_lower_bound_20261111/out/n15.txt` and `n17.txt` give
  dF = 4 and W_3 = [0,1,45,570] and [0,1,51,731]. These match the native reruns.
- `git diff` is empty for `lfd_small.py` and `ext_check.py`.
- No tracked file was changed by this lead. The tracked modifications in the checkout (`examples/jv_cover.rs`, `src/...`) belong to
  other agents. The default-unchanged claim is therefore real: nothing shared was edited.

## Logic checks

**Falsifier 1** (d_F(F_bil) = 4 at n = 15): not fired.

- 1 ∈ W_3 on the direct full lift for seeds 1 and 5 (logged).
- I independently confirmed the cheaper S = {0} lift falls at D = 3 with both tools. By
  monotonicity this also gives 1 ∈ W_3(F_bil).

**Falsifier 2** (d_F = 2 at n ≥ 11): not fired. The D = 2 stalls at n = 15 and 17 were
confirmed with both tools.

**Monotonicity lemma** (W_D(F_S) ⊆ W_D(F_{S'}) for S ⊆ S', D ≥ 2): the argument is correct.

- Each F_S generator equals an F_{S'} generator plus a sum of degree-2 axioms, so it lies in
  W_D(F_{S'}).
- W_D(F_{S'}) is closed under multiplication by a superset of the variables.

So the n = 17 conclusion d_F(F_bil) = 3 rests on two exact, cross-checked cells:

- S = {0,1} falls at D = 3;
- the full lift stalls at D = 2.

The killed direct run is correctly labelled inconclusive.

**Negative control:** reported as FAILED (6/6 randbil instances drop 4 → 3), which is the honest
reading of "must not drop".

## Caveats

1. **The control's construction is an interpretation.** The lane says "a random n-dimensional
   subspace of the linear forms in place of the descent span". The emitter instead draws n
   independent random bilinear-plus-affine forms. These are not hardness-matched: d_F(rand) = 4
   even at n = 11 and 13, where the descent has 3, and they have no linear consequence in W_1. This
   cannot rescue the descent-specific claim, since a negative control that drops already shows
   non-specificity. It does mean the post-hoc "descent needs a smaller lift" comparison is
   confounded by the different baseline difficulty.
2. **The lead's own sketch fails.** The lane claimed that a degree-2 certificate in F_bil would
   imply a Nullstellensatz-type certificate, so d_F(F_bil) ≥ 3. The randbil drop from 4 to 3 shows
   the lift can lower d_F by one even where the original has d_F = 4. The ≥ 3 part remains
   empirical only.
3. **A/A is the bare minimum.** It uses 2 instances per n. The size-curve points are
   deterministic in S, not sampled.
4. **The verification is incomplete.** There is no second host, replay certificate or research-kb
   receipt, because AWS, S3 and EKS were rejected and the knowledge base is not in the toolset. I
   did not rerun the 155 s n = 15 full-lift D = 3 cells. The n = 15 fall is instead confirmed by
   the S = {0} lift plus monotonicity.
