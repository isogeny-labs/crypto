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

None yet.
