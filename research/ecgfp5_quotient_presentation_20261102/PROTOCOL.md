# Pre-registration: presentation sweep of the EcGFp5 m = 5 quotient (L3)

Frozen 2026-11-02, before any cell in `results/`. It is committed and pushed before the first arm
starts. Changes after that point are amendments (§9) and never edit this text.

## 1. Question and baseline

- **Context.** The merged block-Wiedemann finish (`research/ecgfp5_quotient_bw_finish_20261102`,
  #3303) cut the whole-solve cost c of the m = 5 (Z/2)^4 ⋊ S5 quotient to 16,912 CPU-s at
  p = 4111 (1.83× under W0). F4 is now about three-quarters of c (12,456 CPU-s in that cell's
  msolve TIMINGS, against 10,349 in W0's run of the same system). The next factor has to come from
  F4.
- **Question.** Does a different presentation of the same decomposition problem lower the F4 cost
  (and so c) at m = 5? The levers come from the L2 audit
  (`research/ecgfp5_representation_audit_20261102`):
  - the grevlex **variable order** of the W0 system;
  - the **u = x/y ring form** (Pornin's u; FHJRV Prop. 13). It has the same supports (1287 + 792
    monomials, degrees 8,8,8,8,8,2) and D = 2^{(m−1)^2}, but its sixth equation is the two-term
    quadric s5 − w² (w = π = ∏u_i) in place of W0's 13-term w² − Q(s). Its factor base is
    u ∈ F_p, not x ∈ λF_p.
- **Scope.** This is a single-prime screen (p = 4111) of a per-system Gröbner stage cost at a toy
  prime. It is not an end-to-end ECDLP cost, has no matched rho reference and makes no security
  claim about EcGFp5.

## 2. Arms (all at p = 4111, m = 5)

| arm | system | variable order (first = largest in grevlex) | run |
|---|---|---|---|
| A0 | W0 `quot.ms` (merged, read in place) | s1,s2,s3,s4,s5,w | F4 only |
| V1 | same polynomials | s5,s4,s3,s2,s1,w | F4 only |
| V2 | same polynomials | w,s1,s2,s3,s4,s5 | F4 only |
| U1 | u-ring system from `src/qsolve.rs generate --m 5 --p 4111 --seed 1 --arms uring` | s1,s2,s3,s4,s5,w | whole solve |

- A0 is the same-session A/A control: the same binary on the same input as W0 and the bw cell.
- V1 and V2 change only the first line of `quot.ms`. The ideal is the same, so D = 65,536.
- U1 is a different ideal on the same curve and prime. Its generator checks are part of the gate
  (§4).

## 3. Method

- **F4 only** (A0, V1, V2): `msolve -v 2 -t 4 -g 1 -f <sys> -o <out>`, i.e. F4 plus the leading
  monomials only, under `qsolve run --rss-cap-gib 12 --wall-cap-s 14400`.
- **Whole solve** (U1): the frozen bw pipeline of the merged study, unchanged: msolve `-g 2` into a
  FIFO, `hnfsolve --dump-P`, `bwfinish --b 128 --threads 4 --seed 1 --planted`. The binaries are
  the ones built from that study's `src/` (`bin/` of
  `research/ecgfp5_quotient_bw_finish_20261102`).
- **Tools.** `/usr/bin/msolve` sha256 `c2722288…f759fa` (msolve 0.6.5, as W0). `src/qsolve.rs`
  is W0's generator plus the `uring` arm. Its source hash is recorded in the commit freezing this
  file.
- **Host.** Tier L: 4 vCPU Xeon @ 2.80 GHz, 15.7 GiB. One arm at a time, with nothing else
  running. The order is A0, V1, V2, U1 (`run_all.sh`).

## 4. Measurements and gates

- **F4 CPU** = msolve's `TIMINGS` `overall(cpu)` (the F4 phase only), as in W0's probe arms.
- **U1 c** = msolve process CPU + hnfsolve CPU (parse + staircase + dump) + bwfinish `total_cpu_s`,
  as in the merged bw study.
- **Gates.**
  - Every arm must exit 0 with no cap stop. Otherwise it is `not_measured`, never imputed.
  - U1 generator: H_R vanishes on 24/24 fresh decompositions, does not vanish on decompositions of
    R + N or at random points, and the planted point satisfies the descended system.
  - U1 whole solve: elim_degree = D = 65,536, `stab_fail=0` and `"planted": "recovered"`. If
    hnfsolve's stability check fails (exit 3), U1's c is `not_measured` and its F4 CPU is still
    reported.
- **A/A.** A0 F4 CPU is reported against W0 (10,349) and the bw cell (12,456). It is the reference
  for every arm in this session.

## 5. Decision rules (frozen)

- **F4 improvement** for an arm: arm F4 CPU ≤ A0 F4 CPU / 1.5, with the arm's gates passed. The
  1.5 bar is the same one used in W0's probes and the bw study.
- **Whole-solve improvement** (U1): c ≤ 16,912 / 1.5 = 11,275 CPU-s (the ratcheted per-system
  baseline at p = 4111), with every U1 gate passed.
- An arm between A0/1.5 and A0 is reported as a ratio, not an improvement. An arm above A0 is a
  regression for this solver.
- An improving arm triggers a follow-up at p = 262231 and 33554371 under an amendment, before any
  ratchet claim. One prime is a screen, not a pipeline improvement.
- G1 (CLOSE ≥ 2^46, PROMOTE ≤ 2^38, band [c/4, 4c]) is reported for U1's c, as in the bw study.

## 6. Pre-registered predictions

- V1 and V2: F4 CPU within ±25 % of A0. The system is symmetric in s1..s5 only up to the
  weighting of e_k, so the order changes tie-breaking, not the degree of regularity.
- U1: D = 65,536, and F4 CPU within ±25 % of A0. The supports are the same, and the shorter
  quadric changes only reductions involving w². No arm reaches the 1.5× bar.
- A0: within ±25 % of W0's 10,349 CPU-s.

## 7. Analysis

`analyze.py` reads only `results/*/` and the merged W0 and bw `analysis.json`. It writes
`analysis.json`, `analysis.md` and the figure. There is no other analysis path.

## 8. What may not be said

- Not "EcGFp5 is weaker than claimed", and not "the lead is reopened" (the atlas asks for
  c_cyc ≤ 2^38).
- A single-prime screen is not a pipeline improvement.
- U1 is a different factor base. A U1 gain is a gain of the u presentation, not a statement about
  the x-subline system.

## 9. Amendments

### A1 (2026-11-02, after the four frozen arms; committed and pushed before any A1 cell)

**What the frozen arms showed** (`analysis.md`).
- U1's F4 CPU is 4,729 against A0's 12,744: A0/U1 = 2.69×. That is an F4 improvement at
  p = 4111 under §5, with the generator gate passed.
- U1's whole solve is `not_measured`, because hnfsolve exited 3.
  - In grevlex the quadric s5 − w² has leading monomial w², so every standard monomial has
    w-degree ≤ 1.
  - That gives t = 32,768 chains of length ≤ 2, and the BNS22 stability check fails on
    32,767 heads.
  - Dev runs confirm this is structural: U1 also fails at m = 3 (`dev/`), while W0's m = 3
    system passes.
- V1 and V2 are within ±2 % of A0: no improvement, as predicted.

**A1 arms.** All use the same host, caps (12 GiB, 14,400 s wall) and binaries.

| cell | system | run |
|---|---|---|
| U3_p | U1 with w := w′ − c·s1 (`src/shear_w.py`, seed 1, c from SHA-256), renamed w | whole solve, bw pipeline exactly as U1 |
| A0_p | W0 `quot.ms` at p | F4 only (`-g 1`) |
| U1F_p | U1 generator at p, seed 1 | F4 only (`-g 1`) |
| U2_4111 | U1 with s5 := w² substituted and the quadric dropped (`src/subst_s5.py`): s1..s4, w, degree 16 | F4 only (`-g 1`) |
| U1P_4111 | U1 | whole solve by W0's finish: `msolve -P 1`, then `qsolve check --planted` |

- **Why the shear.** It is an invertible linear change of coordinates, so it gives the same
  points and the same D. The quadric's leading monomial becomes s1², so w is no longer killed
  at degree 2.
- **Dev check at m = 3, p = 4111 and 262231.** stab_fail = 0, D = 16, planted recovered by
  `bwfinish` (`dev/m3_*_u3`).
- **Fresh generation.** U3 at p ≠ 4111 regenerates the u-ring system with `qsolve generate`, and
  its generator checks are part of the gate.

**Order** (`run_a1.sh`). U3_4111, A0_262231, U1F_262231, A0_33554371, U1F_33554371, U2_4111,
U1P_4111. If U3_4111 passes its whole-solve bar, U3_262231 and U3_33554371 then follow.

**Gates.** As §4. For U3: shear exit 0 (the mapped planted point satisfies the system),
elim_degree = D = 65,536, `stab_fail=0`, planted recovered. For U1P: `check.json` has
elim_degree = 65,536, `planted_root_of_eliminant` and `planted_coordinates_recovered`.

**Decision rules.**
- **F4 stage improvement of the u ring.** U1F F4 ≤ A0 F4 / 1.5 at each of 262231 and 33554371,
  measured in-session, as well as at 4111 (already measured). It is reported as a stage
  improvement: it moves no whole-solve c by itself.
- **Whole-solve improvement** for U3 or U1P at a prime: c ≤ bw baseline / 1.5, i.e. 11,275,
  12,549 and 15,011 CPU-s at 4111, 262231 and 33554371.
  - c = msolve process CPU + hnfsolve CPU + bwfinish `total_cpu_s` (U3), or msolve process
    CPU (U1P).
  - It is a **pipeline improvement and ratchets the baseline** only if it holds at all three
    primes with every gate passed.
- An arm that hits a cap stop is `not_measured` and is never extrapolated.

**Predictions** (made before any A1 cell).
- U3: F4 between U1's and A0's (6,000–10,000 CPU-s at 4111), and c of 10,000–15,000. Whether
  it clears 11,275 is uncertain.
- A0/U1F ≥ 1.5 at both larger primes, and about 2.7×.
- U2: F4 above U1's 4,729, because the degree-16 equations raise the degree of regularity.
- U1P: msolve's change of order handles t = 32,768 dense columns against W0's 4,920, so it is
  predicted to cost more than W0's 31,004. A cap stop is likely.

**What may not be said** (adds to §8).
- An F4 stage improvement is not a pipeline improvement.
- A U3 or U1P pass at one prime is a screen.

### A2 (2026-11-02, after U3_4111; committed and pushed before any A2 cell)

**What U3_4111 showed** (`results/A1/U3_4111`).
- Every gate passed: D = 65,536, stab_fail = 0, planted point recovered.
- F4 CPU was 12,082, the same as A0. The basis has the same shape as W0's: t = 4,920 chains,
  longest 37, 306M terms.
- c = 12,291 + 134 + 4,186 = 16,611 CPU-s, against the 11,275 bar. So U3 is no improvement, and
  under A1 U3 is not run at the larger primes.
- Reading: the shear gives back the whole F4 gain. U1's gain comes from w² being a leading
  monomial, and the bw finish needs a variable with long chains.

**A2 arm U4.** The U1 polynomials unchanged, with only the variable order changed to
s1,s2,s3,s4,w,s5. s5 is last, so it is the eliminating variable.
- w² stays the leading monomial of the quadric, which is the structure behind U1's F4 gain.
- The chains are counted in s5.
- s5 = ∏u_i², and it separates the points if no two solutions share it.
- **Dev check at m = 3** (order s1,s2,w,s3; p = 4111 and 262231): stab_fail = 0, D = 16,
  planted point recovered (`dev/m3_*_u4`).
- **Run.** The bw pipeline exactly as U3 (`run_a2.sh`). The planted point is reordered to
  match. The gates are those of U3.

**Order change.** The A1 queue is stopped at the end of the cell then running (A0_262231). The
cell it had started next is moved, unrun, to `results/A1/_aborted_*`. Then U4_4111 runs, then
the remaining A1 cells in their A1 order.

**Decision rules.** The A1 whole-solve rule applies to U4: c ≤ 11,275, 12,549 and
15,011 CPU-s at 4111, 262231 and 33554371. If U4_4111 passes, U4_262231 and U4_33554371 run
(`run_a2.sh`) after the A1 cells. A pipeline improvement, and so a ratchet, needs all three
primes.

**Prediction.** V1 and V2 showed that the variable order does not move F4 for W0, so U4's F4 is
predicted within ±25 % of U1's 4,729. With a finish of about 4,300 CPU-s, c is predicted at
about 9,000–10,500. That passes the 4111 bar by a margin smaller than the ±2,000 A/A spread
read in the bw study.

### A3 (2026-11-03, after U4_4111 and the U1F replication; committed and pushed before any A3 cell)

**What the A1 and A2 cells showed.**
- **U4 (s5 eliminating).** F4 12,403 CPU-s, c = 16,679: no improvement. Its basis has W0's
  shape.
- **U1F (u-ring, w last, F4 only).** A0/U1F = 2.85× at p = 262231 (5,230 vs 14,888) and 2.72× at
  33554371 (4,834 vs 13,127), with the generator gates passed. With 2.69× at 4111, this is the A1
  F4 stage improvement at all three primes.
- **So the F4 gain exists only in U1's own basis.** Every leading monomial is a pure
  s-monomial, except w² (from w² − s5). The staircase is S × {1, w}, with |S| = 32,768.
- **s-stability.** In S, every one of the 2,460 monomials m with s5·m ∉ S has s5·m as a
  leading monomial. This holds at 262231 and 33554371 (from U1F's `lm.out`, by
  `stair_u1.py` logic).
- **So multiplication by w can be read straight off the reduced basis.** w·m = (w m), and
  w·(w m) = NF(s5 m). That normal form is a chain head (a unit entry) or the negated tail of
  one of 2,460 basis elements (a dense row). The bw finish's operator had 4,920 dense columns.

**A3 arm U5.** The U1 system unchanged (U1's `gen/` at 4111; regenerated with seed 1 at the
other primes, generator gates as U1). Whole solve:
1. `msolve -v 2 -t 4 -g 2` into a FIFO, with the same caps.
2. `src/hnfsolve_u.cpp --dump-P` checks the structure: every chain is {m, w m}, the element
   with LM w² is w² − s5, no other LM contains w, and s-stability holds. If any check fails it
   exits 3. Otherwise it writes the HNFP2 dump.
3. `src/bwfinish_u.cpp --b 128 --threads 4 --seed 1 --planted`. This is the merged bwfinish
   with the operator replaced by shift + unit axpys + one (D × 2,460)(2,460 × b) GEMM. Points
   come from a 2,460 × 2,460 kernel per rational root, using u(s5 m) = w0²·u(m). The generator,
   determinant and root code is unchanged.
4. `src/check_points.py`: every reported point satisfies every equation of the system.

**Dev check at m = 3, p = 4111 and 262231** (`dev/m3_*_u5`): structure checks pass (5 unit
rows, 3 dense rows), D = 16, the planted point is recovered, and all points satisfy the system.

**Measurement.** c = msolve process CPU + hnfsolve_u CPU (parse + staircase + dump) +
bwfinish_u `total_cpu_s`, as in the bw study.

**Gates.** All exit codes 0, no cap stop, U1's generator gates, elim_degree = D = 65,536,
planted point recovered, `check_points` all_satisfy.

**Decision rules.**
- A whole-solve improvement at a prime is c ≤ 11,275, 12,549 or 15,011 CPU-s (bw baseline / 1.5
  at 4111, 262231 and 33554371).
- At all three primes with every gate passed, it is a **pipeline improvement and the new
  per-system baseline (ratchet)**.
- At one or two primes, it is a screen.
- G1 is reported for each c, as in the bw study.

**Order.** Run as in A2's order change.
1. The A1 queue stops at the end of U2_4111.
2. U5_4111, U5_262231 and U5_33554371 run (`run_a3.sh`).
3. U1P_4111 runs last, as in A1.

**Predictions.**
- F4 is as U1/U1F (4,700–5,300 CPU-s).
- Printing the basis adds under 400 CPU-s, because it has half of W0's 306M terms.
- The sequence costs about half of the bw finish's (one limb about 1,800 CPU-s, two limbs
  about 3,300). With the generator, determinant and points, the finish is about 2,300 (one limb)
  or 3,800 (two limbs).
- So c ≈ 7,000–8,000 CPU-s at 4111 and 262231, and ≈ 8,500–9,500 at 33554371. That is
  2.0–2.4× under the ratcheted baseline, and an improvement at all three primes.
- G1 point estimate about 44.3–44.6 bits, with the band still straddling CLOSE (2^46) on its
  upper side.

**What may not be said** (adds to §8; A3).
- U5 changes the factor base to u ∈ F_p. A U5 ratchet is a ratchet of the u presentation of the
  same decomposition problem on the same curve and prime. It is not a statement about the
  x-subline system, and it is not an end-to-end ECDLP cost.

### A4 (2026-11-03, after U5_4111; committed and pushed before any A4 cell)

**Why.**
- A container restart (about 01:14Z) killed the first U5_4111 attempt. The queue was restarted
  at 01:16Z (`results/a3_restart.*`).
- Every A3 cell runs after the restart. The host reports the same class (Intel Xeon @ 2.80 GHz,
  4 vCPU, 15.7 GiB), but the physical machine may differ.
- U5_4111's F4 was 3,844 CPU-s against U1's 4,729 before the restart (−19 %). That is within
  the ±25 % A/A tolerance, but large enough to read.

**A4 cells.** These run after the A3 queue (U5 at three primes, then U1P_4111), with `run_a1.sh`
and the same commands as A1:
- `A0_4111`: W0's `quot.ms`, F4 only. This is the post-restart A/A against A0 (12,744).
- `U1F_4111`: the U1 system, F4 only. This is the post-restart A/A against U1 (4,729).

**Use.** A4 changes no decision rule. The A3 bars are absolute: the merged bw baseline / 1.5.
The report states each A3 c and its ratio to the baseline both as measured and corrected by
the post-restart drift of A0. If the corrected ratio at any prime falls below 1.5×, the
report says so.
