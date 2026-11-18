# F3 — Pilot: gate reachability and rate calibration (2026-11-17)

Status `[pcc:unverified]`. Per lesson (a): every correctness gate must be shown
**reachable on a pilot instance before it is frozen**, and rate gates must be
calibrated **at the instance sizes the study uses**, not at smaller ones. This
pilot touches **only** gate quantities and instrument sanity — it never computes the
decision metric (total-cost `P/W`) on the decision instances.

## What was run

`python3 -I pilot/pilot.py` (Python 3 standard library only). It reuses the frozen
L01 genus-2 package copied into `pilot/l01/` (provenance and sha256:
`pilot/COPIED_FROM_L01.sha256`; source
`g2plus_leads_20261116/L01_anchored_pencil_sieve/l01/`, 12 files). New pilot code in
`pilot/pilot.py`: the cheap `a1 + a2`-BSGS order method, a standalone negation-map
Pollard rho with fruitless-cycle escape, a pure walk-arm solve, and the two
split-rate / rho-step calibrations. Raw output: `pilot/raw/pilot.json` (4.2 KB),
console transcript `pilot/raw/pilot_stdout.txt`. Deterministic: two runs printed
identical numbers. Wall ≈ 76 s on one core (diagnostic only).

## What it printed (verbatim summary)

```
=== F3 PILOT SUMMARY ===
G-order  cheap #J == F_p^2 jac_order on 8 tiny primes, mismatches=0; study p=8191 #J=66919357 a1=23 a2=15291 bsgs_ops=459  PASS=True
G-solve P (pencil end-to-end) at p=131,257: pass=True
   g2_p131: t_planted=29 t_recovered=29 hom_rank=73 n_cols=74 pencil_rows_bad=0 labelled_bad=0 pass=True
   g2_p257: t_planted=12740 t_recovered=12740 hom_rank=128 n_cols=129 pencil_rows_bad=0 labelled_bad=0 pass=True
G-solve W (walk end-to-end) at p=251: rows=138 t=409 t_rec=409 pass=True
G-solve R (rho) recovery: pass=True
   p=131 r=2477 t=1004 t_rec=1004 verify=True steps=40
   p=257 r=13297 t=10559 t_rec=10559 verify=True steps=146
   p=521 r=66739 t=56847 t_rec=56847 verify=True steps=432
G-split pencil (p=8191): obs/tuple=1332.8 model=1365.2 ratio=0.976 PASS=True
G-split walk   (p=8191): obs=0.50693 model=0.50117 ratio=1.011 PASS=True
R step calib  (p=8191): r=8388581 expected=3630 mean_steps/expected=1.295 all_verify=True
   seed=1 steps=4520 ratio=1.245 verify=True
   seed=2 steps=2790 ratio=0.769 verify=True
   seed=3 steps=5480 ratio=1.510 verify=True
   seed=4 steps=6009 ratio=1.655 verify=True
wall_seconds=76.0
```

## Gate-by-gate: reachable, with the pilot value that justifies the frozen threshold

| gate (PROTOCOL §6) | what the pilot showed | frozen threshold it justifies |
|---|---|---|
| **G-order** | cheap `#J` (a1 from N1, a2 by BSGS in the ≤4p Weil band) equals the `O(p²)` F_{p²} formula on all 8 tiny primes (0 mismatches); at the study prime `p=8191` it returns a verified `#J=66919357` in **459** group operations | the brief's O(p²) method is replaced; the cheap method is correct and cheap. Threshold: 0 mismatches at p≤101, and a verified #J (annihilation + Weil band) at study size |
| **G2 verify (P)** | pencil end-to-end solve recovers the planted `t` at p=131 and p=257 (`t_rec==t`), 0 bad pencil rows, 0 bad labelled rows | "every P run recovers and verifies `t`" is reachable |
| **G2 verify (W)** | pure walk-arm solve (138 inhomogeneous relations, no pencil rows) recovers `t=409` at p=251 | "every W run recovers and verifies `t`" is reachable |
| **G2 verify (R)** | negation-map rho recovers and verifies `t` at p=131,257,521 (steps 40/146/432) | "every R run recovers and verifies `t`" is reachable |
| **G4 rank** | homogeneous pencil `hom_rank = n_cols − 1` (73 of 74 at p=131; 128 of 129 at p=257): corank exactly 1, as the rank-based stop rule requires | corank-1 stop rule is reachable and correct |
| **G6 pencil split** | relations/tuple at **p=8191** = 1332.8 vs model `p/6=1365.2`, ratio **0.976** | `[0.85,1.15]` band, calibrated at the study prime (not a smaller one) |
| **G7 walk split** | split fraction at **p=8191** = 0.50693 vs model `C(n,2)·4/#J=0.50117`, ratio **1.011** | `[0.85,1.15]` band, calibrated at the study prime |
| **G8 rho sanity** | at **p=8191**, `r≈2^23`: all 4 seeds verify; `mean steps/√(πr/2)=1.295`, range 0.77–1.66 | all-verify + mean band `[0.3,3.0]` (generous around the calibrated 1.295) |

## The defect the pilot caught before freeze (lesson a in action)

The first pilot run of the negation-map rho (a plain 2-cycle escape) **trapped 2 of
4 study-size seeds** in fruitless cycles (173820 steps, no collision, verify=False).
Had G8 been frozen on tiny instances only, this would have surfaced at assessment,
not here. Fix (now in `pilot/pilot.py` and SPEC §8): a rolling-window cycle detector
with a deterministic minimal-key doubling escape. After the fix all 4 seeds verify
(above). The frozen SPEC's arm R is this fixed version, plus a hard step cap and
charged retries.

## What the pilot deliberately did NOT do

- It did not compute any `P/W`, `P/R` or `W/R` cost ratio on the decision primes
  (that is the decision metric, forbidden to the coordinator).
- It did not build or run the Rust decision instrument; it validated the mathematics
  and the gate thresholds in Python so the executor can freeze and run SPEC §0–12.
- The pencil/walk end-to-end recoveries ran at tiny p (131/251/257/521) where a
  dense solve is cheap; the study's counted **sparse** solve mod r (SPEC §7.S) at
  p up to 2^16 is the executor's measurement, not piloted here.
