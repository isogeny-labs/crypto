experiment.protocol/v1  (preregistration, NOT YET RUN; [pcc:unverified])
lead: chained_transfer_next O4  (genus-3 degree-2 non-hyperelliptic quartic line-section IC)
author-session: transfer-lead sweep, 2026-11-16
frozen-before-data: yes. gated measurement (isolab-eks / native L2); cannot run in this container.

QUESTION (falsifiable)
  Does the line-section index calculus on C: v^4=x^3+ax+b --(x,v)->(x,v^2)--> E,
  restricted to the transferred subgroup S=c^*(E(F_p)), solve the order-r ECDLP
  on E in fewer counted operations than the matched negation Pollard rho on E,
  at any point of the AGENTS.md §5a pricing band, with the gap NOT growing in p?

ARMS (same instrument, interleaved, masked labels, derived seeds)
  Q  : line-section IC (solve_quartic_ic). rows = split line sections, folded
       norm-image columns. counted unit = ecbench group-op equivalent; native
       F_p muls/sqrs/invs and F_r muls/adds/invs counted by class (A1/A13).
  D  : control IC, same factor base, rows from elliptic additions of 3 columns
       (solve_direct_ic). isolates "does the quartic source save group ops" vs
       "the F_r linear algebra dominates".
  R  : matched negation Pollard rho on E (full_jacobian_ic::rho_full_ec_tallied),
       primary baseline; plain rho as context. base = muls+sqrs+w_inv*invs.

MATCHED-RHO REFERENCE
  per prime, same E, same r; rho seeds 1000+j+100k, 16/instance (stage2: 8).
  arms ranked only where the order holds across the WHOLE band.

COUNTED UNIT + §5a PRICING BAND (decisive)
  price every native op at 0, 0.1, 1 GAE (and calibrated 0,1,10 w_cal).
  rank Q vs R at each corner. The protocol's prediction: Q's counted F_r
  linear-algebra stream alone is >= n(p) MulR, and n(p)=Theta(p) (measured
  locally: exponent 1.019 over p in {53..3209}, prime-order r>=0.85p), while
  rho = Theta(sqrt r)=Theta(sqrt p) (measured exponent 0.487). So even at the
  Q-favorable 0-GAE corner (native F_p work free) Q's counted cost >= Theta(p)
  > Theta(sqrt p) = R. The band moves constants, not this exponent gap.

SELECTION (frozen in PROTOCOL.md Amendment 1; binds r to p)
  candidate_ab(p,i,u): a_u,b_u = mix64 of (0xA4B1/0xB7C3 +101 i +u) mod p,
  u=0..3999; require nonsingular, a,b!=0, r=#E prime, r!=p, r not | p^k-1 (k<=6),
  r>=ceil(0.85 p), >=8 points on C, p=1 mod4. stage1: 7 primes 229..12841;
  stage2: 25601..102409. (select_only.tsv already fixes p,u,a,b,r,n for all 10.)

SEEDS  seed0=0x5EED0415; seed_k=seed0 ^ (k*0x1000); IC seed base+j+10k
  (Q 700, D 500); LA stream seed ^ 0x4C41; BSGS salt 0xB565; rho seed 1000+j+100k.
  all seeds printed; cold runs built from (p,a,b,r,G,T) only (A6).

AGGREGATION  R(p)=ratio of means (Q over 3x4, rho over 3x16); OLS of ln R on
  ln p over 7 stage-1 points with t95(5); stage2 separate.

FALSIFICATION CRITERIA (preregistered)
  CLOSED (expected): verdict Q_above_rho, counted_below_rho=0 in every cell at
    every band corner, AND lower-95% bound of the ln R vs ln p slope > 0
    (gap grows). Predicted slope ~ +0.5.
  LEAD SURVIVES (would reopen): any cell with counted Q cost < matched rho at
    some band corner, OR lower-95% slope bound <= 0 (gap flat/shrinking).
    This is the only outcome that would justify scaling p further.
  Instrument invalid: dense Gauss-Jordan cross-check (p<=421) disagrees with
    Wiedemann; or 32-column BSGS check mismatches; or tamper_fails control
    passes a tampered certificate; or any verify_relation / factor-log replay /
    [t]G==T failure -> run void, no verdict.

VERIFICATION (outside the counted total, each reported)
  verify_relation every row; factor-log replay; fibre witness; final [t]G==T;
  determinism second-pass on k=0 of 229/421/821 (identical counts).

GATES THAT BLOCK THIS CONTAINER (why not run here)
  scripts/checkpoint.sh init (.resume/RESUME.md -> S3), bench.record/v1, and the
  experiment-director task card are all required before the launcher, and the
  hooks refuse the launcher without them; no AWS/isolab/S3 here. So the
  measurement is isolab-only. The INSTRUMENT (lib+bin, 16 unit tests) is
  committed and, per PROGRESS.md:149-167, already passes its controls.

EXPECTED OUTCOME  Q_above_rho, gap growing ~sqrt(p); closes the proof's one
  open (g,d)=(3,2) hyperelliptic/Castelnuovo-Severi gap with a certified row.
  Does NOT beat rho. One-shot confirmatory run only.
