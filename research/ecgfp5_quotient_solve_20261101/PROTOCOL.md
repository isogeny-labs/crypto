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
- **Primes**: p ∈ {4111, 65551, 33554371}, all ≡ 1 (mod 15), log2 p = 12.005, 16.000,
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
