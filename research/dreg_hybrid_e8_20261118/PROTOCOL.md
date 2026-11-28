# Protocol: stretch cell E8, (m, n, ℓ) = (3, 22, 8). How many R1 guesses restore degree 4?

Lead: `LEAD-ICE-hybrid-guess-determine-l7` (crypto-autoresearcher `knowledge/leads/ecdlp/ic-engine/`).
Parent preregistration: `research/dreg_hybrid_guess_20261024/PREREGISTRATION.md`, tier 6 ("stretch E8"),
which did not run. Frozen 2026-11-18 by `PROTOCOL.sha256`, before any measured run of this study.
Class: **stage diagnostic** (exact GF(2) algebra on frozen draws). No `S`, no rho ratio from measurement,
no speed claim, no leaderboard row. Every number this study produces is `[pcc:unverified]` until validated.

## 1. Question

At (m, ℓ) = (3, 7) the parent found k\* ∈ {1, 2} on 10 of 10 unsatisfiable draws (κ(7) = 2) under rule R1.
The parent's decision for the m = 4 row hinges on ℓ = 8:

- κ(8) = κ(7) keeps the optimistic **S_const** scenario alive (m = 4 row +8.43 bits over rho at κ = 2);
- κ(8) ≥ κ(7) + 1 favours **S_lin** (+30.7 to +58.3 bits), and closes the hybrid route for the m = 4 row.

Q1. For each unsatisfiable (3, 22, 8) draw, what is k\*: the smallest k such that all 2^k branches of the
nested rule R1⁺ (below) refute at degree ≤ 4 under the MutantXL closure?
Q2. Does the unspecialised (k = 0) degree-4 closure fail to refute at ℓ = 8, as assumed but never observed?
Q3. At k\*, is the summed branch cost below the projected unspecialised degree-5 cost?

## 2. Prior information (disclosed; this study is not blind to it)

All on the frozen draws `research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl` (seed 20261022;
unsatisfiable draws 0, 3, 5, 6; satisfiable draws 1, 2, 4 with 6, 6, 18 solutions):

- `dreg_closure_guess3_20261023`: R1 at k = 3 (variables 7, 15, 23 = this study's R1⁺ k = 3), branch 0 does
  **not** refute on draws 0, 3, 5 (closure ranks 129,221, 128,213, 129,893 of 136,698 columns); all 8 branches
  refute on draw 6. Its G6 set {6, 7, 14, 15, 22, 23} (= R1⁺ k = 6) refutes on branch 0 of all four draws.
- `dreg_closure_guess_20261022`: the set {7, 15, 23, 24} refutes on all 16 branches of all 4 draws (k\* ≤ 4 for
  that set, which is not R1⁺).
- `dreg_guess_scaling_20261106`: invariant-coordinate guess sets give k\* ≤ 4 at ℓ = 8 and ≤ 5 at ℓ = 9.
- Never measured: the k = 0 degree-4 closure at ℓ = 8 (179,447 columns); R1 at k = 1, 2; R1⁺ at k = 4, 5;
  all branches of R1⁺ k = 6; any fresh (3, 22, 8) draw.
- Hence under R1 as the parent defined it (k ≤ 3), κ(8) > 3 is already implied on the frozen draws if the
  closure is monotone under further fixing. This study measures it directly and pins k\* with R1⁺.
- No sizing run was made on any draw used here.

## 3. Draws

- **Frozen (F):** unsatisfiable draws 0, 3, 5, 6 of `research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl`.
- **Fresh (R):** `dreg_ladder --cells 22:8:3 --m 3 --ffd-max 3 --seed 6029471183 --shard I/8 --first-draw 0
  --max-draws 8` (8 processes, one draw each, draws 0–7 of the one deterministic stream), concatenated and
  sorted by `draw` into `draws/m3-22-8-fresh.jsonl`. The fresh panel is the **two lowest-index unsatisfiable
  draws** among 0–7. If fewer than two exist, the panel is what exists (listed). The seed 6029471183 occurs in
  no file of crypto, ml-cryptanalysis or cryptanalysis (code search, 2026-11-18). crypto-autoresearcher is
  vault-encrypted, so a code search cannot see its contents; its plaintext was not searched.
- **Positive control (P):** satisfiable draw 1 of the frozen file (6 solutions).

## 4. Rule R1⁺ (fixed now)

Summand i's coordinates are variables `iℓ … iℓ+ℓ−1`; ℓ = 8. R1⁺ is the ordered list

    v = [7, 15, 23, 6, 14, 22]

the top coordinate of each summand in turn (exactly the parent's R1 for k ≤ 3), then the second-top
coordinate of each summand in turn. At level k the first k entries are fixed; branch b ∈ [0, 2^k) assigns
`v[j] = (b >> j) & 1`. The rule is nested, uses no draw-specific information, and is not changed after any
outcome. k ranges over 0..6; k\* > 6 is reported as censored.

## 5. Instrument and branch command

`examples/dreg_closure` from this PR's head (unchanged source), built `--release`; its SHA-256 is recorded
before the first run (`runs/binary.sha256`). One process per branch:

    F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=T \
    prlimit --as=24000000000 timeout 5400 dreg_closure --to 4 --max-cols 200000 --draw D [--fix v=b,...] DRAWS

(`--satisfiable` added for P). Recorded per branch: refutes (closure_degree not null) / saturates / unmeasured
(killed or timeout), columns and closure rank at each degree, `closure_word_ops`, `closure_products`, wall
seconds and peak RSS (VmHWM from the log). Columns are deterministic: `C(46 − k, ≤ 4)` = 179,447, 164,221,
149,986, 136,698, 124,314, 112,792, 102,091 for k = 0..6.

## 6. Escalation and scheduling (frozen)

Per unsatisfiable draw:

1. k = 0 baseline. If it refutes, k\* = 0 and the draw stops.
2. Then k = 1, 2, …, 6 in order. At level k, all 2^k branches are queued. The first non-refuting branch
   **fails** the level: queued branches of that level that have not started are cancelled (unmeasured);
   branches already running finish and are reported. A failed level escalates to k + 1. A level whose
   branches all refute gives k\* = k and the draw stops.
3. Worker pool: W concurrent branch processes with T threads each (W·T ≤ vCPUs, W·12 GB ≤ RAM).
   Draws interleave in the queue in the order F0, F3, F5, F6, R(first), R(second), P.
   Order affects only wall time, never k\*.
4. A branch killed by timeout, memory or pod loss is **unmeasured**. It is rerun once from scratch. A level
   with an unmeasured branch and no failed branch is **unresolved**, and so is the draw.
5. Wall budget: the pod is deleted at 5 h from its start or when the queue is empty, whichever is first.
   Anything not reached is **unmeasured** and reported as such, with no extrapolated stand-in.

## 7. Controls

- **A/A (reproduction, N1b analogue).** R1⁺ k = 3 branch 0 on F0, F3, F5, F6 and (if reached) k = 6 branch 0
  must reproduce the committed `dreg_closure_guess3_20261023` records: outcome, columns and closure rank equal,
  `closure_word_ops` within 1 %, `closure_products` within 2 % (the parent's Amendment 1 tolerance, since
  products depend on thread scheduling). Failure means instrument or harness drift: the study is
  **uninformative**, with every run shown.
- **Positive control P (soundness).** `dreg_hybrid plant` enumerates the 6 solutions of frozen draw 1 and checks
  them against the built system. All 8 R1⁺ k = 3 branches are run with `--satisfiable` (degree ≤ 4 only, by the
  column cap). **Pass:** no branch whose fixed values agree with some solution refutes. **Fail:** any such
  branch refutes. The study is then **void**. Solution-free branches may refute or not and carry no verdict.
- **Monotonicity check (descriptive).** If k = 1 or k = 2 succeeds on a draw whose k = 3 branch 0 fails (or the
  reverse pattern at any pair of levels), it is reported. Q1 is defined by the escalation and does not need monotonicity.

## 8. Statistic, decision rule and predictions

- κ(8) = max k\* over the unsatisfiable draws that resolved (F and R), with censored draws reading "> 6".
- Registered reading (the parent's): **κ(8) ≥ κ(7) + 1 = 3 → S_lin favoured over S_const; the hybrid route does
  not improve the m = 4 row.** κ(8) ≤ 2 → S_const not rejected (one more point on a flat line).
- Also reported: the per-draw k\*, its distribution, and the slope κ(8) − κ(7) next to the parent's model
  rows (`dreg_hybrid model --kappa κ(8)`), always as bits **above** rho (distance remaining).
- Q2: the k = 0 closure at degree 4 does not refute: reported per draw.
- Q3 cost, unit `closure_word_ops`: `H(d) = Σ_b ops` over the 2^{k\*} branches; `H_tot(d)` adds every failed
  and beyond-stop branch. Degree-5 projection `D5(d, e) = ops4(d) · (C(46, ≤ 5)/C(46, ≤ 4))^e`, ratio 8.639,
  e ∈ {3.0, 4.1, 4.25}, `ops4(d)` the draw's own measured k = 0 degree-4 ops.
  §5a band: `cost_g = ops + g · Σ products · ⌈cols/64⌉` at g ∈ {0, 0.1, 1}, applied to both arms.
  "Hybrid cheaper" only if the order holds at every (e, g). Otherwise unresolved.
- **Predictions (recorded, not tuned to):**
  - A/A and P pass: 0.9.
  - k = 0 does not refute on every draw: 0.9.
  - Frozen draws 0, 3, 5: k\* ∈ {4, 5, 6} on each (0.75). k\* = 4 on at least two of them (0.5).
  - Draw 6: k\* ≤ 3 (0.95).
  - κ(8) ≥ 3, so S_lin favoured: 0.95.
  - Q3: hybrid at k\* cheaper than projected degree 5 at every (e, g): 0.6 (2^{k\*} branches at k\* ≥ 4 is
    16–64 degree-4 closures against a factor 8.64^e ≥ 645).

## 9. Compute

RunPod CPU pod (`cpu5c`, `cpu3c`), SECURE, not interruptible, image `ubuntu:24.04` (the binaries are built
here against glibc 2.39). Spend cap about $10. Timings are cloud-VM L0 readings, never a metric.
Inputs (binaries, draws) and outputs (records, logs, tarball after each branch) move through the pod. The
transport is recorded in RESULTS.md.

## 10. What this cannot establish

Nothing at m = 4 or at n = 131 is measured: the m = 4 row stays a derivation from the parent's model. One
guess rule is measured: a different set may need fewer guesses. Four frozen draws plus two fresh is a small
panel, and n = 22 has subfields GF(2^2), GF(2^11), which the random subspaces do not use (AGENTS.md §8b).
