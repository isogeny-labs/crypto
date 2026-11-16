# RESULT: LEAD-ICB-koblitz-rho-reference

## Outcome
- Status: ran to completion. Exit code 0, wall 15 s (run.status: exit=0 wall=15). One run only, no retries.
- run.py verdict field: "inconclusive". run.stderr was empty (nothing to report).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): R_m = mean(neg)/mean(frob) >= 0.6*sqrt(2m), i.e. >= 2.81, 3.12, 3.50 for m = 11, 13, 17.
Controls: A/A CI contains 1; none/neg in [1.1, 1.8]; zero failed solves in the frob arm.

Measured (mean f-evals, 200 targets):
- m=11 (n=991): none 166.2, neg 734.7, neg_AA 669.7, frob 23.1. R=31.8 CI [21.4, 44.0]. A/A 1.10 CI [0.67, 1.65]. none/neg 0.226. frob failures 0.
- m=13 (n=79): none 82.0, neg 180.6, neg_AA 145.0, frob 20.9. R=8.65 CI [3.63, 41.8]. A/A 1.25 CI [0.76, 2.11]. none/neg 0.454. frob failures 2.
- m=17 (n=65587): none 856.7, neg 16739, neg_AA 16888, frob 484.8. R=34.5 CI [14.3, 112.1]. A/A 0.99 CI [0.80, 1.22]. none/neg 0.051. frob failures 1.

Controls:
- Null/A-A: passed at all three m (CIs contain 1).
- Positive control: FAILED at all three m. none/neg is 0.05 to 0.45, not in [1.1, 1.8]; the raw-point walk is faster than the negation-only walk.
- Zero frob failures: FAILED for m=13 (2) and m=17 (1).
Verdict: inconclusive (controls invalid). Point estimates exceed the thresholds, but under the preregistered rules this is not a consistent_with_prediction result.

## Reading
- The toy does not test the lead's claim cleanly. The none arm finishes in far fewer evaluations than neg (mean 857 vs 16739 at m=17, where sqrt(n)=256). The neg arm shows huge restart counts (38412 at m=17) and 16 to 51 failures per 200 targets. This is the classic fruitless-cycle problem of negation walks (2-cycles with a 16-way partition) with no cycle-escape rule; the cap and restart rule dominate. So R_m mostly measures how badly the neg arm handles fruitless cycles, not the sqrt(2m) orbit saving.
- The frob means are below sqrt(n) at m=11 (23 evals vs sqrt(991)=31) and about 2x sqrt(79)=9 at m=13. The m=11 value in particular suggests early collisions that may be short or degenerate rather than genuine rho collisions. Failures in frob at m=13 and 17 mean some returned k did not verify, so the frob arm is not fully correct as implemented.
- m=13 has a tiny prime n=79 (N=8374=2*53*79), so that row is a very small group; its CI is wide [3.6, 41.8].
- The R values (8.7 to 34.5) far exceed the ideal sqrt(2m)=4.7 to 5.8, which is itself a sign of an artefact (the neg baseline is inflated) rather than a real super-ideal saving.
- Nothing contradicts the verdict field: "inconclusive" matches the numbers and failed controls.

## Bearing on the sub-sqrt(p) question
none. The toy neither supports nor refutes the lead: controls failed, and the ratio is dominated by baseline fruitless-cycle behaviour. Even a valid result would only be a constant-factor (sqrt(2m)) effect already known for Koblitz curves.

## Standing
[pcc:unverified]. Single local toy run, no independent replay.
