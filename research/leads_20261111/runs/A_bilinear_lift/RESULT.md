# LEAD-A-bilinear-lift-size-degree (lane A, Lead A5): result, 2026-11-12

Lead: `research/leads_20261111/lane_A_falldegree.md`, Lead A5. Every statement below is
`[pcc:unverified]`. One agent produced all of it, single-threaded, on a shared 4-vCPU container.
There is no replay certificate and no second host.

## Verdict (against the preregistration)

| Preregistered item | Outcome `[pcc:unverified]` |
|---|---|
| Point prediction: d_F(F_bil) = 3 at n = 11, 13, 15, 17 | **Held** at all four n, two instances each. n = 11, 13, 15 come from direct runs. At n = 17 the result follows from a partial lift plus monotonicity (proof below); the direct run is in the n = 17 section. |
| Falsifier 1: d_F(F_bil) = 4 at n = 15 | **Not fired.** 1 ∈ W_3(F_bil) on both instances (seeds 1 and 5). |
| Falsifier 2: d_F(F_bil) = 2 at some n ≥ 11 | **Not fired.** 1 ∉ W_2(F_bil) on all 8 instances (n = 11, 13, 15, 17). |
| Positive control: the Lemma 2 lift (`ext_check`) gives d_F = 2 at n = 7, 9, 11 with the native tool | **Reproduced on 6/6 instances.** The variable and axiom counts are those in `out/ext.txt`. |
| Negative control: F_bil built from a random n-dimensional span "must not drop" | **FAILED.** The lift of a random system of the same shape drops from d_F = 4 to 3 at n = 11, 13 and 15 (2 + 2 + 2 instances; the direct full-lift run at n = 15 is on 1 instance). |
| A/A: two instances per n | Done. The two instances agree on every verdict at every n. |

**Overall: partial.**

- The lead's numerical claim held. d_F(F_bil) = 3 at n = 15 and n = 17, against d_F(F) = 4.
- The preregistered negative control failed, so the drop cannot be attributed to the Weil-descent
  structure. Lifting the bilinear part lowers the last fall degree of *random* bilinear systems of
  the same shape just as well. What the data support is the generic statement "d_F is
  presentation-sensitive for bilinear systems". The data do not support a descent-specific mechanism.
- Post hoc, not preregistered: the descent needs a smaller lift than random systems do. At n = 15,
  lifting one row (|S| = 1) suffices for the descent, while random systems still stall at
  |S| = 1 and 2 and fall at |S| = 4. Two instances per arm, so this is an observation, not a
  result.

**This is not a cost improvement.**

- The full lift at n = 15 needs a closure over M(80,3) = 85,401 columns. F needs M(16,4) = 2,517.
- The cheapest lifts found by the exploratory scans:
  - n = 15: 20 variables, 1,351 columns at D = 3, against 2,517 at D = 4.
  - n = 17: 33 variables, 6,018 columns, against 4,048 at D = 4. At n = 17 the lift costs more
    columns than it saves.
- Nothing here bears on ECDLP cost, on rho, or on any leaderboard row.

## What was run

The native tool is `polyclose.c` (new, in this directory).

- It implements the generic degree-≤D closure W_D of `lfd_small.closure`: the least subspace of
  B_{≤D} containing F and closed under f ↦ x_i f for f ∈ W_D ∩ B_{<D}.
- It reads an arbitrary Boolean system. Monomials are index lists, up to 1000 variables.
- The M4RI batch/RREF core `insert_batch` is copied unchanged from the A3 tool
  `runs/A_fall_effective_variable/fallclose.c`.

The instance emitter is `emit_lift.py`.

- It imports the legacy `lfd_small.make_instance` / `descend` / `reduced_basis` and
  `ext_check.build_ext`. The instances are exactly those of the NOTE.md tables.
- It writes the original system in (A,C) coordinates, X1 = A and X2 = A + C. Every f_s was checked
  to be bilinear a^T M_s c, plus a linear part, plus a constant. The emitter aborts otherwise.
- It also writes:
  - the bilinear lift: y_ij replaces a_i c_j for i ∈ S, with axioms y_ij + a_i c_j;
  - the Lemma 2 lift;
  - the random negative-control systems: n random affine forms of the same shape, meaning random
    M_s, random linear part and random constant. Their solution count is computed by brute force
    over 2^{2k} and must be 0.

No file outside this directory was edited. `git status` shows only
`research/leads_20261111/runs/A_bilinear_lift/` as new.

```
cd research/leads_20261111/runs/A_bilinear_lift
gcc -O3 -march=native -o polyclose polyclose.c -lm4ri -lm
./run_crosscheck.sh                      # native vs legacy lfd_small.closure: out/crosscheck.jsonl
# every cell: ./run_one.sh TAG D FULL ROWCAP -- <emit_lift.py args>   (appends to out/main.jsonl)
./run_one.sh orig_n15_s1_D4 4 0 0 -- orig --n 15 --seed 1            # reference d_F(F)
./run_one.sh bil_n15_s1_D2  2 0 0 -- bil  --n 15 --seed 1            # falsifier 2 cell
./run_one.sh bil_n15_s1_D3  3 0 0 -- bil  --n 15 --seed 1            # the informative cell
./run_one.sh part_n15_s1_S01_D3 3 0 0 -- bil --n 15 --seed 1 --S 0 1 # partial lift |S|=2
./run_one.sh ext_n11_s3_D2  2 0 0 -- ext  --n 11 --seed 3 --D 3      # positive control
./run_one.sh rand_n15_r28_D4 4 0 0 -- rand --n 15 --rseed 28         # negative control, unlifted
./run_one.sh randbil_n15_r28_D3 3 0 0 -- randbil --n 15 --rseed 28   # negative control, lifted
./run_one.sh bil_n17_s1_D3 3 0 110000 -- bil --n 17 --seed 1         # direct n=17, row cap 110k
python3 summarize.py > out/table.md                                   # full table of all runs
```

Instances:

- Descent instances: the first unsatisfiable `lfd_small` seeds per n, namely n = 11: 3, 4;
  n = 13: 2, 11; n = 15: 1, 5; n = 17: 1, 2.
- Random-shape controls: the first two unsatisfiable rseeds per n, namely n = 11: 10, 15;
  n = 13: 2, 32; n = 15: 28, 35.

## Tool validation `[pcc:unverified]`

1. **Native against legacy.** I compared `polyclose` with `lfd_small.closure` on 49 small systems,
   covering every emitter mode (orig, bil, partial bil, rand, randbil, ext) at n = 7, 9 and
   D = 1..4. All 49 agree on the verdict. On the stall runs they also agree on every cumulative
   dimension; `lfd_small` stops early on a fall, so dimensions are compared on stalls only.
   I also checked n = 15 seed 1, |S| = 2, D = 3: both say "fall". Outputs:
   `out/crosscheck.jsonl` and `out/crosscheck_n15.jsonl`.
2. **Reference d_F(F) against the committed records.** The native closure of the emitted (A,C)
   system reproduces the committed `lfd_small` records exactly:
   - n = 11 and n = 13: d_F = 3, W_2 dims (0,1,22) and (0,1,26)
     (`out/n7-11.txt`, `out/n13.txt`);
   - n = 15: d_F = 4, W_3 dims (0,1,45,570) (`out/n15.txt`);
   - n = 17: d_F = 4, W_3 dims (0,1,51,731) (`out/n17.txt`).

   Because W_D is invariant under the affine change (a,b) → (a,c), this also checks the coordinate
   change. These lfd_small records are the "default unchanged" control; no shared code was modified.
3. **Positive control.** The Lemma 2 lift gives 1 ∉ W_1 and 1 ∈ W_2 on all 6 `out/ext.txt`
   instances. The sizes match that file: 92, 175 and 298 variables; 280, 540 and 924 axioms.

## Results `[pcc:unverified]`

d_F is the least D with 1 ∈ W_D. "Stall at D" means 1 ∉ W_D, and the run went to its fixed point.

| n | k | instance | d_F(F) (native = lfd_small record) | F_bil vars | 1 ∈ W_2(F_bil) | 1 ∈ W_3(F_bil) | d_F(F_bil) | W_3 rank at fall / cols | sec (D=3) |
|---|---|---|---|---|---|---|---|---|---|
| 11 | 6 | seed 3 | 3 | 48 | no | yes | **3** | 9,570 / 18,473 | 0.7 |
| 11 | 6 | seed 4 | 3 | 48 | no | yes | **3** | 9,578 / 18,473 | 0.6 |
| 13 | 7 | seed 2 | 3 | 63 | no | yes | **3** | 25,765 / 41,728 | 10.4 |
| 13 | 7 | seed 11 | 3 | 63 | no | yes | **3** | 25,681 / 41,728 | 10.1 |
| 15 | 8 | seed 1 | 4 | 80 | no | yes | **3** | 59,076 / 85,401 | 155.5 |
| 15 | 8 | seed 5 | 4 | 80 | no | yes | **3** | 59,222 / 85,401 | 157.1 |
| 17 | 9 | seed 1 | 4 | 99 | no | yes, via S={0,1} + monotonicity (direct: see below) | **3** | — | — |
| 17 | 9 | seed 2 | 4 | 99 | no | yes, via S={0,1} + monotonicity | **3** | — | — |

Peak RSS of the n = 15 full-lift run: 1.63 GB.

**Monotonicity, used at n = 17.** Let S ⊆ S'. The generators of the S-lift lie in the degree-≤2 span
of the S'-lift's generators: a partial-lift equation is the full equation plus a sum of axioms
y_ij + a_i c_j. The S-lift ring is a subring of the S'-lift ring, and closure is monotone in the
generators and in the allowed multipliers. Hence W_D(F_S) ⊆ W_D(F_{S'}) and
d_F(F_{S'}) ≤ d_F(F_S). The same argument with S = ∅ gives d_F(F_bil) ≤ d_F(F).

### Partial lifts (the preregistered size curve, |S| = 2, 4, 8)

All partial-lift runs are at D = 3. The size points are (number of variables, d_F), with F_S
written for the lift over S.

| n | instance | S | vars | 1 ∈ W_3 | size point (vars, d_F) |
|---|---|---|---|---|---|
| 15 | seeds 1, 5 | ∅ (F itself) | 16 | no | (16, 4) |
| 15 | seeds 1, 5 | {0,1} and {6,7} | 32 | yes (W_3 = all of B_{≤3}) | (32, 3) |
| 15 | seeds 1, 5 | {0..3} and {4..7} | 48 | yes | (48, 3) |
| 15 | seeds 1, 5 | {0..7} (full) | 80 | yes | (80, 3) |
| 17 | seeds 1, 2 | {0} | 27 | **no** (stall) | — |
| 17 | seeds 1, 2 | {0,1} | 36 | yes (W_3 = all of B_{≤3}) | (36, 3) |
| 17 | seeds 1, 2 | {0..3} | 54 | yes | (54, 3) |

For each partial lift, d_F = 3 exactly, because the full lift already stalls at D = 2 and
d_F(F_{S'}) ≤ d_F(F_S).

Exploratory scans, not preregistered (tags `xpl_*` in `out/main.jsonl`):

- **n = 15, one lifted row.** S = {0} with 24 variables already falls at D = 3 on both seeds.
  Lifting only the first t pairs (0,j):
  - t = 1 or 2: stall on both seeds;
  - t = 3: falls on seed 1, stalls on seed 5;
  - t ≥ 4: falls on both seeds. That is 20 variables and 1,351 columns at D = 3, against
    2,517 columns for F at D = 4.
- **n = 17, row 0 plus the first t pairs of row 1.**
  - t ≤ 5: stall on both seeds;
  - t ≥ 6: falls. That is 33 variables and 6,018 columns at D = 3, against 4,048 columns for F
    at D = 4.

  Only this one nested order was scanned. It is not a minimum over all choices of lifted pairs.

### Negative control: random systems of the descent's shape

| n | rseed | d_F(rand) | 1 ∈ W_2(randbil) | randbil S={0} D=3 | S={0,1} D=3 | S={0..3} D=3 | full randbil D=3 | d_F(randbil) |
|---|---|---|---|---|---|---|---|---|
| 11 | 10 | 4 | no | – | – | – | yes | **3** |
| 11 | 15 | 4 | no | – | – | – | yes | **3** |
| 13 | 2 | 4 | no | – | – | – | yes | **3** |
| 13 | 32 | 4 | no | – | – | – | yes | **3** |
| 15 | 28 | 4 | no | stall | stall | yes | yes (165 s, rank 61,053) | **3** |
| 15 | 35 | 4 | no | stall | stall | yes | (≤ 3 via S={0..3}) | **3** |

At n = 11 and 13 the random systems are harder than the descent: d_F = 4 for random against 3 for
the descent. Their lift still drops by one. The preregistered requirement was "must not drop". It
dropped in 6 of 6 instances.

## n = 17 direct full-lift run

I ran the full lift at n = 17, seed 1, D = 3 directly. The settings were 161,800 columns, a row cap of
110,000 (about 2.2 GB for W, about 2.8 GB RSS observed) and a 1,180 s timeout.

- **Outcome: killed by the timeout.** It stopped after batch 15: 121,770 products, rank 91,604 and
  a queue of 3,378 rows. 1 was not yet in W. The log is `out/log/bil_n17_s1_D3.log`, and the
  `main.jsonl` row has rc = 143.
- **The direct run is therefore inconclusive.** The batches grew to about 150 s each, against
  17 s at n = 15. That is consistent with the lane's own estimate that this cell is borderline
  locally.
- **The n = 17 verdict does not depend on the direct run.** It rests on two runs:
  - S = {0,1} falls at D = 3 on both seeds, which by monotonicity gives d_F(F_bil) ≤ 3;
  - the full lift stalls at D = 2, which gives d_F(F_bil) ≥ 3.

  Both runs are exact closures.
- A direct confirmation needs a larger host: a dense 162k × 162k W, about 3.3 GB, and roughly
  30–40 min single-threaded by extrapolation.

## What is blocked or not done

- **Infrastructure.** AWS, S3 and Iso Lab EKS were not used: the AWS identity is rejected. There is
  no `bench.record`, no S3 raw-data copy, no replay certificate and no second host. The
  research-kb MCP server is not available in this agent's toolset, so no knowledge-base receipts
  were taken. The crypto-autoresearcher repository (VLT1) was not touched.
- **Sample size.** Two instances per n is the preregistered A/A minimum. The post-hoc observation
  that the descent needs a smaller lift than random systems needs more instances.
- **The lower-bound sketch.** "d_F(F_bil) ≥ 3" is confirmed only empirically, as the D = 2 stall
  at each n tested. The atom-independence argument in the lane file remains a sketch.

## Next step

1. Re-preregister the negative control as a comparison rather than a "must not drop" gate. The
   quantity would be the minimal lift size s*(n) that brings d_F from 4 to 3, measured on the
   descent and on random systems of the same shape. Use ≥ 10 instances per arm at n = 15, 17
   and 19, with a greedy and a randomized pair order.
2. The observable that could carry descent-specific information is the growth of s*(n) for the
   descent. From these two instances per n: at most 4 pairs at n = 15, and 15 pairs in the nested
   order at n = 17.
3. Test the generic version first. Does a minimal lift of size s*(n) lower d_F by one for every
   bilinear system? If yes, the lead reduces to the known bilinear-system theory (Faugère,
   Safey El Din and Spaenlehauer) and should be closed as not descent-specific.
4. The size-degree curve should be reported in closure columns, not in variables. At n = 17 the
   cheapest lift found already costs more columns (6,018) than F at D = 4 (4,048).
