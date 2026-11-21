# Phase 5 protocol: per-curve cost of rho and index calculus across isogenous curves, with randomised curve order

Status: **frozen before data** (2026-11-12T04:40Z), after the pilot of §8 and before any study data.

Requested by the user on 2026-11-12: the phase-4 measurement (min and max per-curve cost for the same instance)
for rho and for index calculus, on all 48 classes. This is the follow-up that phase 4's decision rule requires
(`../phase4/REPORT_PHASE4.pdf`). Phase 4 found a curve-level effect of about 1% that was confounded with execution
order: ecbench runs workloads in spec order within a session and shuffles only the arms. Classification: a
null-hypothesis characterization of a constant factor. It leaves the IC baseline unchanged and adds no leaderboard
row. Findings are [pcc:unverified].

## 1. Question

For the same instance on every curve of a class (`planted_shared`: the same `k`, with `Q' = [k]G'`), does the
wall time per unit of work differ between curves once the curve order is randomised?
- **rho:** wall ns per GAE.
- **IC, m = 3 and m = 2:** decomposition-phase ns per attempt (primary); wall ns per relation (secondary).

Expectation: no. The arithmetic does not use the curve coefficients (rho), and the decomposition cost per attempt
depends only on |F| and the field.

## 2. Population

The 48 prime-order classes of `../phase3/classes.json`: 332 isogenous curves plus 96 isomorphic models. This
includes c046–c048, which phase 4 missed.

## 3. Design (`fp31-class-census p5-spec`, frozen)

- **Sessions.** One ecbench session per (class, kind, session index). The session lists every curve of the class
  in a seeded random order, a different permutation per session.
- **Targets.** Fresh `planted_shared` targets per session (`target_seed = 20261113 + 1000·kind + session`).
- **Run settings.** 1 round, `warmup 0`, arm order `shuffle`, isolation L2 required.

| kind | sessions | targets per curve per session | arms |
|---|---|---|---|
| rho | 8 | 8 | `plain` (rho.plain), `plain-aa` (rho.plain), `control` (rho.negation) |
| ic3 | 4 | 2 | `main`, `main-aa` (ic.pipeline, prime-abscissa \|F\|, mitm:m=3), `pc` (\|F\|·1.2^{1/3}) |
| ic2 | 4 | 2 | the same with m = 2 |

IC arms use p2tool's fixed-|F| rule exactly as in phase 3: |F| = 378 (pc 402), 5,965 attempts per run for m = 3;
|F| = 1448 (pc 1586), 102,425 attempts for m = 2.

## 4. Commands (frozen, `run_phase5.sh`)

For each class, for each kind in (rho, ic3, ic2), for each session `s`:
```
fp31-class-census p5-spec --classes phase3/classes.json --class c --kind K --session s --targets T --seed 20261113 --warmup 0 --out runs/c/K_s<s>/spec.json
ecbench run --wait --spec runs/c/K_s<s>/spec.json --out runs/c/K_s<s>/session
ecbench verify --dir runs/c/K_s<s>/session --replay 2 --out runs/c/K_s<s>/audit.json
```
Then `fp31-class-census p5-analyze --kind K --sessions S` for each kind.
- A refused or failed session is recorded as missing and not replaced.
- Nothing else may run on the host during the card: no builds, and no inspection commands by the main session.

## 5. Tests (`p5-analyze`, frozen; per kind)

- **T1 (primary).** Within each class, a Friedman test across curves on the primary metric of the main arm, with
  blocks (session, target). The p-values are tested for uniformity with one-sample KS.
- **T2.** The number of classes with Friedman p < 0.05/48.
- **Family.** 3 kinds × (T1, T2), Bonferroni α = 0.05/3 for T1. T2 must be 0.
- **S1 (secondary).** An isomorphic model against its base curve, compared with all other curve pairs. Reported is
  the probability that the isomorphic pair is closer. If a curve effect is intrinsic to the curve, this is near 1;
  if it is noise or order, about 0.5.
- **Reported, not tested:** the per-class max/min of curve means; the between-curve SD with a one-sided 95% upper
  bound; and, for IC, the per-relation ratio of the planted arm.

## 6. Decision rules (per kind)

- **Null supported.** T1 p > 0.05/3, T2 = 0, the A/A log ratio lies within ±0.01, and the positive control passes:
  - rho: `rho.negation` ÷ `plain` in [1.03, 1.15] with |z| > 5;
  - ic3: pc ÷ main ns per attempt in [1.03, 1.15] with |z| > 5;
  - ic2: pc ÷ main ns per relation in [0.78, 0.92] with |z| > 5. The per-attempt difference at m = 2 is small:
    1.016 in the pilot.
- **Detector failed.** A positive control falls outside its window. No null is claimed for that kind.
- **Effect suspected.** T1 or T2 fails. The flagged curves and S1 are reported. A curve effect is called intrinsic
  only if S1 is above 0.75 and the follow-up replicates on fresh seeds.

## 7. Route

Local, as in phases 2–4; this session has no AWS identity. ecbench grades every row.

## 8. Pilot (non-study class p000: phase 2's pilot walk, 18 curves; excluded and not committed)

| kind | settings | runs | time | results |
|---|---|---|---|---|
| rho | 1 session, 8 targets, warmup 0 | 432/432 verified | 9 s | control 1.081 (log SE 0.013); A/A −0.005 ± 0.012 |
| ic3 | 1 session, 2 targets | 108 runs | 56 s | pc 1.091 per attempt (SE 0.014), 0.908 per relation; A/A −0.007 ± 0.018 |
| ic2 | 1 session, 2 targets | 108 runs | 48 s | pc 1.016 per attempt (SE 0.024), 0.843 per relation; A/A +0.013 ± 0.031 |

`warmup 0` is accepted. Budget: study classes have about 9 curves, so a class takes about 4.5 min and the 48
classes about 3.6 h.

Binaries: `ecbench` sha256 `86a9421d…` is unchanged. `fp31-class-census` was rebuilt for phase 5; its hash is in
`PROTOCOL.sha256`. The phase-4-bound binary hash `632d1b77…` no longer exists on disk; the phase 4 sources are unchanged.
