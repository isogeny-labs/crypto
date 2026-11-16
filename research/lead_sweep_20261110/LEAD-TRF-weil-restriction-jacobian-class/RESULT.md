# RESULT: LEAD-TRF-weil-restriction-jacobian-class

## Outcome
- status: ran (single run, no retries). exit code 0, wall 1 s (run.status: exit=0 wall=1). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): C >= 1 and H >= 1 => consistent_with_prediction; C=0, A/A mismatch or Hasse-Weil violation => inconclusive; C>=1, H=0 => falsified (hyperelliptic locus only).

Measured:
- n=2 control (genus 2, q=3): 1296 curves, C = 258 hits; traces t in {-5..2} (counts 24,24,48,36,24,48,24,30); weil_viol = 0; null_hits = 248.
- n=3 census (genus 3 hyperelliptic, q=3): 11664 squarefree curves, H = 408 hits; t in {-8,-4,-2,0,2,4,8} (12,120,24,96,24,120,12); weil_viol = 0; null_hits = 0.
- A/A recount: 204 curves sampled, 0 mismatches.
- Verdict from run.py: consistent_with_prediction. Controls passed: positive control (C>=1) yes; A/A yes; Hasse-Weil yes. Null/negative control reported only (not decisive).

## Reading
- The toy tests the narrow claim: some hyperelliptic genus-3 curve over F_3 has L-polynomial 1 - tT^3 + 27T^6. 408 do. By Tate this puts their Jacobians in a Weil-restriction isogeny class. It does not test anything about ECDLP cost.
- Discrepancies/caveats: (a) A/A sample is 204, not the 300 stated in PROTOCOL.md s3/s4 (likely a sample drawn from a filtered pool or capped); mismatches 0 either way, but the stated size was not met. (b) The n=2 null filter is barely selective (248 null hits vs 258 hits), so the n=2 control shows the detector can count, not that the filter discriminates; the n=3 null is 0 hits, as expected if no N_1=5 curve satisfies the n=3 criterion. (c) Hit count H=408 exceeds the heuristic 10-100 (not preregistered as a number); hits are highly structured (many are plausibly twists/translates/isomorphic copies under PGL2 action, not 408 distinct isomorphism classes), so H counts coefficient vectors, not classes. (d) Trace values were not independently checked for Waterhouse admissibility beyond what run.py does; t=+-8 hits are the rarest (12 each).
- Verdict field matches the numbers (C>=1, H>=1, controls clean).

## Bearing on the sub-sqrt(p) question
none: existence of such Jacobians at q=3 is a point-count fact; PROTOCOL.md s6 notes transfer needs a cover/GHS-type map and the atlas records same-field genus-g embeddings as costing >= q, so no speedup or rho gap is shown.

## Standing
[pcc:unverified], single local toy run, no independent replay.
