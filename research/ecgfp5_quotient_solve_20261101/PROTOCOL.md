# Pre-registration: one complete m = 5 solve of the EcGFp5 2-torsion quotient (D and c)

Registered 2026-11-01 by the commit that adds this file, before any cell below has
run at a protocol prime. Owner request 2026-10-31 ("yes, set it up and run it") for
the experiment specified in
[`research/ecgfp5_lead_evaluation_20261031/README.md`](../ecgfp5_lead_evaluation_20261031/README.md)
§5 (evaluator G1's frozen criteria, `cards/g1.md` §6). It answers the open problem
`crypto-autoresearcher:knowledge/open-problems/KN-OPEN-9b4a2b.md` as far as one
solver and one envelope can.

Every statement carries one label: *recorded* (a committed artefact), *derived* (a
calculation shown here) or *predicted* (stated before the protocol runs).

**Disclosure.** Development runs at the prime 331, which is not a protocol prime,
were made before this file was written: the m = 3 arms (D = 384, 64, 384, 64, 16, all
as predicted, planted point recovered, identity control passed) and a partial m = 5
quotient solve, stopped by hand during F4 degree 23 after 376 s wall and 1,178 CPU
seconds (2^41.6 cycles at 2.8 GHz) at 2.28 GB peak RSS, each finished degree from 17 to
22 costing 1.6–2.3 times the one before. They set the envelope tiers in §6. They are
not data for §7, and they stay in `dev/` with that label.

## 1. Question

For the F_p-rational (Z/2)^{m−1} ⋊ S_m quotient of the five-summand point-decomposition
system on EcGFp5-shaped curves (`KN-OPEN-9b4a2b`, statement), at m = 5:

- **Q1.** What is the ideal degree D?
- **Q2.** What does one complete solve (F4 plus the rational parametrisation) cost, c,
  in F_p operations, and at what peak RSS?

The tuned-base cost model of the lead evaluation (§3 there) puts EcGFp5's total at
about 2^{115.5 + log2(c)/3} word operations (G1). The verdict depends on c, not on D
(*recorded*, `g1.md` §6: "Measure c (not D)").

## 2. What is already fixed before the runs

- **D ≤ 2^16 at m = 5** (*derived*). The quotient system is five F_p-equations
  A_j(s) + w·B_j(s) = 0 (j = 0..4, the z-components of A + wB) of total degree 8 and
  one quadric w² − Q(s), in the six unknowns s_1..s_5, w. A has s-degree ≤ 8 because
  S_6 has degree 16 in each x_i, so after the Laurent shift by u_i^8 the t-degree is
  ≤ 8 and a symmetric polynomial of per-variable degree ≤ 8 is a polynomial of total
  degree ≤ 8 in the elementary symmetric functions; B = N/W has t-degree ≤ 7 and Q has
  degree ≤ 2. The affine Bézout bound for a zero-dimensional ideal is
  8^5 · 2 = 2^16 = 2^{(m−1)^2}. The "law" of `KN-OPEN-9b4a2b` is this bound, and it
  equals 5!·16^5 / (2^4 · 5!), the raw degree divided by the group order. So D can fall
  below 2^16 (solutions at infinity, non-free orbits) but cannot exceed it. G1's
  D-clauses (D ≤ 2^17, D ≥ 2^19) are therefore decided by zero-dimensionality alone.
- **Macaulay bound** (*derived*): the degree of regularity of a regular sequence of
  degrees (8, 8, 8, 8, 8, 2) in six variables is 5·7 + 1 + 1 = 37. An F4 run on this
  system can climb to about degree 37.
- **Per-target construction** is not part of c (*derived*). Specialising a symbolic
  A(s, X), B(s, X) (X-degree ≤ 16) at a new target costs about (1287 + 792) · 17
  F_q multiply-adds, about 2^{20} F_p operations, against c ≥ 2^{30}. The generator
  here interpolates per target instead (≈ 30 s at m = 5); that time is recorded as
  engineering and is not charged to c.

## 3. Instances

- **Field and curve** (as in the frozen design, EcGFp5 one size down in p only):
  F_q = F_p[z]/(z^m − c), c the smallest integer ≥ 2 that is not an m-th power mod p;
  E: y² = x(x² + 2x + b), b = b0·z with b0 the smallest positive integer for which b and
  4 − 4b are non-squares in F_q (EcGFp5 is b = 263z, m = 5, p = 2^64 − 2^32 + 1,
  z^5 − 3; Pornin 2022/274, *recorded*). β is the smallest non-square of F_p and λ the
  Tonelli–Shanks root of b/β in F_q; the factor base is {P : x(P)/λ ∈ F_p}.
- **Primes** (65551 replaced by 262231 in amendment A1, §10): p ∈ {4111, 65551, 33554371}, all ≡ 1 (mod 15), log2 p = 12.005, 16.000,
  25.000: they span 12.99 bits (*derived*) and straddle msolve's 16-bit and 32-bit
  arithmetic paths. 4111 is the prime of the earlier ladder (`EXP-GFPN-05ff43`); the
  third prime replaces 16777291, which spans only 11.995 bits from 4111.
- **Targets**: per (m, p), a **planted** target R = Σ ±P_i with P_i in the factor base
  (seed 1), so the planted decomposition (s*, w*) is a known solution (positive control).
  At m = 3 also one **random** target (seed 2), as in the red team's square analogue.

## 4. Arms

| m | arm | unknowns | equations | predicted D |
|---|---|---|---|--:|
| 3 | raw_x | x_i ∈ F_p, plus our separating variable a | S_4(x; X0) descended | 384 |
| 3 | sym_x | e_k(x) | G(e) descended | 64 |
| 3 | raw_u | u_i = x_i/λ ∈ F_p, plus a | S_4(λu; X0) descended | 384 |
| 3 | sym_u | e_k(u) | G(e) descended | 64 |
| 3 | quot | s_k = e_k(t), t_i = u_i + β/u_i, and w = Π(u_i − β/u_i) | A + wB descended, w² − Q | 16 |
| 5 | quot | s_1..s_5, w | A + wB descended (deg 8), w² − Q (deg 2) | ≤ 2^16, = 2^16 if no solutions at infinity |

No raw or S_5 arm at m = 5 (frozen in the lead evaluation: G1's dense bound for S_5
reaches 9.3 TiB at degree 76). `KN-OPEN-9b4a2b` resolution item 3 (D against the S_5 arm
on the same targets) is therefore not met by this protocol; the report says so.

The systems are built by `src/qsolve.rs generate` (one standalone Rust file, no crates):
Semaev's S_{m+1} by the recursive resultant definition with formal degrees, evaluated
exactly in F_q; every polynomial is obtained by exact interpolation at random points
and **verified at 24 further random points**; S_{m+1} is checked to vanish at genuine
sums (4/4) and to be symmetric; Q is checked to have coefficients in F_p; the planted
point is checked to satisfy every descended system it belongs to.

## 5. Solver and measurement

- msolve 0.6.5 (Ubuntu noble package `0.6.5-1build2`, the version of the earlier
  ladder), `msolve -v 2 -t T -P 1 -f SYSTEM -o OUT`, T = all CPUs of the tier, default
  options (DRL, automatic genericity handling).
- Run with `OMP_WAIT_POLICY=PASSIVE` (idle OpenMP threads sleep instead of spinning, so
  CPU time is work, not waiting) under `qsolve run`: wall time, user + sys CPU of the
  child (getrusage), maxrss,
  and VmRSS sampled every 0.2 s (logged every 1 s, then 10 s). The RSS cap is a
  watchdog on VmRSS, never an address-space limit.
- **D** = msolve's "Dimension of quotient" (log) and the parametrisation degree (output).
- **Positive control**: `qsolve check` finds the planted point among the parametrised
  solutions (eliminant root and every coordinate recovered). A cell without it is not a
  complete solve.
- **Identity control (m = 3)**: `qsolve orbits` maps every F_p-rational solution of
  raw_u onto quot and sym_u; it passes when every image is a solution, the orbits are
  free (size 24 onto quot, 6 onto sym_u), and every F_p-liftable quot solution is hit.
  The m = 5 cells are interpreted only if this passes at all three primes.
- **c, the unit.** Measured: c_cyc = (user + sys) CPU seconds × 2.8 GHz nominal clock
  (cycles, all threads; a higher turbo clock makes this an underestimate of cycles).
  msolve's F4 core (libneogb) uses AVX2, so its F_p multiply-adds per cycle are not 1:
  **c_ops ∈ [c_cyc/4, 4·c_cyc]** (*derived* band: 1/4 is G1's 4 instructions per
  operation at one instruction per cycle; 4 is a four-lane 32-bit vector multiply-add
  per cycle). Every c is reported as c_cyc and as both band ends. The 64-bit Goldilocks
  cost per operation is not measured here; the count of operations is what transfers,
  F4 matrix shapes being p-independent (*recorded*, review K6 of `EXP-GFPN-05ff43`).

## 6. Envelope tiers

| tier | host | RSS cap | wall cap per m = 5 cell |
|---|---|--:|--:|
| L | this session's container: 4 vCPU Intel Xeon @ 2.80 GHz (AVX2, AVX-512F), 15 GiB | 12 GiB | 4 h |
| E | isolab-eks `cpu` pool (≤ 32 vCPU, ≤ 128 GiB) | 120 GiB | 72 h |
| X | `crypto-eks-xmem` (512 GiB) | 480 GiB | 72 h |

The frozen envelope of the lead evaluation is tier X. At registration (*recorded*,
GitHub Actions on aburan28/crypto): `dreg-ckpt-job.yml`, the only workflow targeting
`crypto-eks-xmem`, has never been dispatched (23 scheduled runs, all skipped), and the one
`crypto-eks-highmem` job (`dreg-a3-eks-highmem.yml`, run 41245660145) has been queued
since 2026-10-31T21:53Z without a runner. Tier E needs a digest-pinned image with msolve
and the AWS launcher, neither of which this session has. Every m = 5 cell runs at tier L
first; a cell stopped by a cap is escalated only when a higher tier is reachable, and
otherwise stays as recorded. No pool cap or quota is raised to make room.

## 7. Cells and frozen criteria

m = 3: arms raw_x, sym_x, raw_u, sym_u, quot at each prime, planted and random targets
(30 solves). m = 5: quot, planted target, at each prime (3 cells), run one at a time in
the order 4111, 65551, 33554371 (`run_cells.sh m5all`).

Per m = 5 cell, G1's criteria (*recorded*, `g1.md` §6), with the unit band of §5:

- **PROMOTE**: complete solve, D ≤ 2^17 and c_ops ≤ 2^40 at the upper band end
  (c_cyc ≤ 2^38).
- **CLOSE**: c_ops ≥ 2^44 at the lower band end (c_cyc ≥ 2^46), or D ≥ 2^19.
- **not_measured**: anything else, and every cap or OOM stop, with RSS at the stop.

**Partial runs.** A cell stopped by a cap records not_measured (G1). Because the solver
is deterministic and a complete run costs at least its prefix, c_cyc spent before the
stop is a measured lower bound on msolve's c for that system. If it reaches c_cyc ≥ 2^46
the report states, as a separately labelled result, "CLOSE on a lower bound (msolve
0.6.5, DRL F4): this solver spends more than 2^44 F_p operations on one system". That is
a statement about this solver, not about every solver for the quotient system.

**Predictions** (*predicted*): m = 3 D = 384, 64, 384, 64, 16 at every prime and target,
both controls pass; m = 5 D = 2^16; G1 expected CLOSE with c_F4 ≈ 2^51–2^54 (dense proxy,
*recorded*); from the dev run, the tier-L cells stop on the 12 GiB cap or the 4 h cap
before the parametrisation.

## 8. What this does not say

Nothing here measures EcGFp5 itself (64-bit p), a 64-bit F4, or any solver other than
msolve 0.6.5 with default options; weighted orders and the symmetric-variable
techniques of Faugère–Gaudry–Huot–Renault could change c. A not_measured cell is not
evidence about D or c. No security level for EcGFp5 follows from this protocol; any
statement that it is below 128 or 142 bits routes to review-breakthrough
(`KN-OPEN-9b4a2b`, "What must NOT be said").

## 9. Files

`src/qsolve.rs` (generator, checker, orbit control, measured runner), `build.sh`,
`run_cells.sh` (the frozen commands), `results/<cell>/` (meta.json, the `.ms` systems,
msolve logs, `run.json`, `check.json`), `analyze.py` and `plot.py` (analysis and
rendering only), `README.md` and `report.pdf`.

## 10. Amendment A1 (2026-11-01T01:40Z, before any m = 5 cell and before any cell at 262231 or 33554371)

**What happened** (*recorded*, `results/`). The first execution of the card
(`cards/RUN_CARD.md`) ran `build.sh`, `run_cells.sh host` and `run_cells.sh m3`. The
m = 3 cells at 4111 (planted and random) completed with every prediction met
(`results/m3_p4111_*`, `results/m3.run1.stdout`). At 65551 msolve 0.6.5 died with
SIGSEGV on all five arms within its first F4 rounds (`results/m3_p65551_planted/*.run.json`,
signal 11). `run_cells.sh` then stopped under `set -o pipefail`, because its summary
`grep` found nothing; the executor reported that step as exit 0, which it was not.
Step d then started `m5all`; the main session stopped it 72 s into the 4111 solve (F4
degree 20, 214 CPU-s), before any m = 5 result, because the m = 3 validation had not
finished. That directory is kept, unread as data, under
`results/_aborted_launch_20261101T0129Z/`.

**The solver defect** (*recorded*, a two-variable toy system x² + y² − 5, xy − 2):
msolve 0.6.5 (Ubuntu `0.6.5-1build2`) exits with SIGSEGV for every tested prime from
65537 to 253229 (65537, 65993, 92681, 131101, 150571, 172969, 185363, 198673, 228211,
244589, 253229) and solves at 61151, 262147, 262231 and 33554371, with 1 or 4 threads.

**Changes.**
1. The middle prime 65551 becomes **262231** (≡ 1 mod 15, log2 = 18.0005). The span
   4111 → 33554371 (12.99 bits) is unchanged. `results/m3_p65551_planted/` stays as the
   record of the defect; 65551 is not a protocol prime any more.
2. `run_cells.sh`: summary lines can no longer stop the script; `m3` takes a prime
   list; the m = 5 order is 4111, 262231, 33554371; `launch-m5all` starts the chain
   detached and writes its own pid file.
3. The 4111 m = 3 cells of the first execution are protocol data (same frozen commands,
   unchanged prime). The remaining m = 3 cells run with `run_cells.sh m3 262231 33554371`.

Nothing else changes: arms, seeds, caps, units, criteria and predictions are as in §2–§7.

## 11. Amendment A2 (2026-11-01T02:05Z): weighted-order probes, before any weighted m = 5 run

**Request.** Owner, 2026-11-01: "try weighted ordering to lower the F4 cost".

**Prior work** (*recorded*, read in full: Faugère, Safey El Din, Verron, "On the complexity
of computing Gröbner bases for weighted homogeneous systems", arXiv 1412.7547, J. Symb.
Comput. 2016). Prop. 10: grevlex F4 on hom_W(F), the system after X_i → X_i^{w_i}, performs
exactly the reductions of W-grevlex F4 on F, so msolve needs no weighted order. §5.2: for an
affine system the gain comes from a W whose highest-W-degree components form a regular
sequence, avoiding degree falls; order the variables by decreasing weight. §6.2, Table 3:
the n = 5 Edwards decomposition system symmetrized by the 2-torsion ⋊ S_5 action (deg I =
65,536, our D), W = (2,2,2,2,1), F4 in Magma 56,105 s → 6,044 s (×9.3), F5 in FGb ×6.9; the
standard grading there had non-regular top components and many degree falls.

**What is already known here** (*recorded*, `results/m5_p4111_planted/quot.log`, read at
02:00Z while the cell runs): standard grevlex F4 on our quotient system has **no degree fall**
from degree 8 to 26, and its selected pairs peaked at degree 25 (2,800, then 2,720). The m = 3
system's Bézout bound is attained (D = 16). *Derived*: for W1 = (2,2,2,2,2,1) on
(s_1..s_5, w), the highest-W components of all six equations do not involve w (A_8 has W-degree
16 > 15 for w·B_7, and Q_2 has W-degree 4 > 2 for w²), so they are not a regular sequence; for
W2 = (1,2,3,4,5,5) every highest component is divisible by s_5^7.

**Prediction** (*predicted*): no weighted arm completes F4 with less CPU than the standard one;
W2 shows degree falls (as at m = 3, `dev/m3_weighted_p4111/`: D multiplied by Π w_i exactly,
6 degree falls for W2, none for W1).

**Arms** (m = 5, p = 4111, the baseline cell's own `quot.ms`, sha256 recorded per arm):

| arm | weights on (s1, s2, s3, s4, s5, w) | msolve variable order | D after substitution |
|---|---|---|--:|
| W0 | standard (the §7 cell) | s1 > … > s5 > w | 2^16 |
| W1 | (2, 2, 2, 2, 2, 1), the FSV/FGHR analogue | s1 > … > s5 > w | 2^21 |
| W3 | (1, 1, 1, 1, 1, 2) | w > s1 > … > s5 | 2^17 |
| W2 | (1, 2, 3, 4, 5, 5), t-degree weights | s5 > w > s4 > s3 > s2 > s1 | 2^16 · 600 |

Run with `msolve -g 1 -v 2 -t 4` (F4 only, leading ideal printed), `OMP_WAIT_POLICY=PASSIVE`,
12 GiB RSS cap, wall cap 1.25 × W0's F4 elapsed time + 60 s (4 h if W0's F4 did not finish), in
the order W1, W3, W2, by `run_weighted.sh drive`, which then resumes the two remaining §7 cells
(`run_cells.sh m5 262231`, `m5 33554371`). The chain driver of §7 was detached at 02:00Z for
this (the 4111 solve itself kept running); the driver runs that cell's frozen check unchanged.
`bin/qsolve-w` is built from the same `src/qsolve.rs` plus the `weight` subcommand;
`bin/qsolve`, which the §7 cells use, is not rebuilt.

**Measured per arm**: completion, F4 CPU (user + sys), F4 elapsed, max RSS, highest F4 degree,
number of degree falls (a round whose degree is below an earlier round's), and D after
substitution (must equal D_W0 · Π w_i).

**Criterion** (frozen): a weighted arm is a **measured improvement** only if its F4 completes
and its F4 CPU is below W0's by a factor ≥ 1.5 (the round times of the same F4 at the same
degree differ by up to 20 % between the dev run and the 4111 cell, so smaller ratios are not
resolved). An arm stopped by the wall cap has spent more than 1.25 × W0 and is recorded as
**no improvement**, with its CPU; one stopped by the RSS cap is not_measured. If W0's F4 did
not finish, a weighted arm that finishes is an improvement over W0's lower bound, reported as
such.

## 12. Amendment A3 (2026-11-01T02:20Z): signature-based arms, before any of them runs at m = 5

**Request.** Owner, 2026-11-01: "try F5 or a signature-based solver instead".

**Why it can matter here** (*recorded*, `results/m5_p4111_planted/quot.log`): msolve's F4
reduces most selected pairs to zero, 2,140 of 2,700 at degree 24 and 2,064 of 2,500 at
degree 27. For a regular sequence, F5's signature criteria discard every reduction to zero
(Faugère 2002; Eder and Faugère, "A survey on signature-based algorithms for computing Gröbner
bases", J. Symb. Comput. 2017, HAL hal-00974810; that id is the one found by search, the text itself was not read here), and the system shows the regular behaviour
of §11 (no degree falls). msolve 0.6.5 has no F5. The only signature-based implementation
installable here is Singular 4.3.2's `sba` (Ubuntu `1:4.3.2-p10+ds-1.1build1`), a
one-pair-at-a-time signature algorithm without F4-style linear algebra, single-threaded.
So the arms compare **F5's pruning in Singular** against **F4 without pruning in msolve**,
two implementations at once; a loss does not show that F5 cannot help, and a win cannot be
split between the algorithm and the implementation without a Singular `std` arm, which is
not run at m = 5 (§12 limits).

**Dev check** (*recorded*, `dev/m3_sba_p4111/`): on the 4111 planted m = 3 systems `sba`
returns vdim 16 (quot) and 64 (sym_u) in all three tested modes, as msolve does; on raw_u
it stops with "exponent bound is 65535".

**Arms** (m = 5, p = 4111, the baseline cell's `quot.ms`, converted by `ms2sing.sh`,
grevlex `dp`, same variable order, `option(redSB)`):

| arm | call | signature order | rewrite order |
|---|---|---|---|
| F5a | `sba(i, 0, 1)` (Singular's default) | induced Schreyer, non-incremental | JSC 2011 (doi 10.1016/j.jsc.2011.05.004) |
| F5b | `sba(i, 1, 0)` | position over term, incremental (classical F5) | JSC 2010 (doi 10.1016/j.jsc.2010.06.019) |

**Run** by `run_probes.sh drive`, which replaces `run_weighted.sh drive` (stopped at 02:15Z
while still waiting; it had run nothing, its empty log is under
`results/_superseded_driver_20261101T0215Z/`). Order: baseline check, F5a, F5b, then the A2
arms W1, W3, W2 unchanged, then `run_cells.sh m5 262231` and `m5 33554371`. Caps: 12 GiB RSS;
wall = 1.25 × W0's F4 CPU seconds + 60 (single-threaded, so wall ≈ CPU), or 4 h if W0's F4
did not finish.

**Criterion** (frozen, as in §11 but on CPU): an `sba` arm is a measured improvement only if
it completes, returns vdim equal to W0's D, and its CPU is below W0's F4 CPU by a factor
≥ 1.5. Stopped by the wall cap: no improvement (it has spent more than 1.25 × W0's CPU).
Stopped by the RSS cap: not_measured. Reported per arm: completion, CPU, max RSS, vdim,
basis size, Singular's syz and rewrite criterion counts.

**Prediction** (*predicted*): both `sba` arms are stopped by the cap (Singular's sba is
reported to need much more time than F4 implementations on large dense systems; its manual
advises `std`/`slimgb` when memory grows), so no improvement is measured; a positive result
would need a signature-based F4 (matrix-F5, GVW/F4-style) that this host lacks.

## 13. Amendment A4 (2026-11-01T08:05Z): container restart, queue reordered; no criterion changes

**What happened** (*recorded*). The session container restarted between 06:11Z and 07:52Z
(`uptime` 0 min at 07:53Z), most likely reclaimed while the session was idle. It killed
`run_probes.sh drive` and Singular during arm F5a. At 06:11Z F5a had run 7,179 CPU-s
(2 h 00 min, 4.63 GB RSS) without finishing. No run record was written, and its `sing.log`
is empty because the log is written when the run ends. The arm directory and the driver log
are kept, unchanged, under `results/_lost_restart_20261101T0753Z/`. Everything already
finished survived on disk, including the checked baseline 4111 cell. F5a's lost attempt is
not an outcome: it is neither a cap stop nor a completion, and the only fact it records is
that F5a needs more than 7,179 CPU-s on this system.

**Change.** `run_queue.sh` replaces `run_probes.sh drive` for the rest of the queue. Its
order is: `run_cells.sh m5 262231`, `run_cells.sh m5 33554371`, then F5a, F5b, W1, W3, W2.
The two baseline cells move to the front because they decide G1 at the other two primes
(§7). The probe arms are compared only against the 4111 cell, so their position does not
change what they measure. Arms, commands, caps (12,996 s for sba and 3,903 s for weighted,
the values `run_probes.sh` derived from the 4111 cell's `quot.log`) and every criterion of
§7, §11 and §12 are unchanged. To keep the container alive, the session holds a running
background task for as long as the queue runs.
