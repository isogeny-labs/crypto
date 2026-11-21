# RESULT: LEAD-EXO-gag-holant-semaev-chain

## Outcome
- status: ran (single run of run.py, unmodified). exit code 0, wall 5 s (run.status: exit=0 wall=5; far below the 5 min estimate).
- RESULT.json is one JSON object; run.stderr empty. smoke=false, 8 curves, seeds 1000..1007, bits=8, rank modulus 1000003.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): S = median over 8 curves of rank_x / mean(rank_rand1, rank_rand2). Consistent if S >= 0.95 with valid controls; falsified if S < 0.9 with valid controls; else inconclusive. Controls: positive DL/null <= 0.7; A/A within 0.03 of 1.

Measured:
- S = 0.7273 (< 0.9)
- positive control median rank_dl/rank_null = 0.1332 (<= 0.7, valid)
- A/A median = 1.0059 (within 0.03, valid)
- null control: two random labellings gave ranks 234-256 of ceiling 256, so the null sits near ceiling and is a usable baseline.
- controls_ok = true. Verdict (run.py): falsified.

Per curve (p: rank_x / rank_r1 / rank_r2 / rank_dl): 151: 100/247/240/28; 163: 120/238/247/21; 181: 144/253/251/30; 199: 169/248/234/30; 211: 191/252/255/36; 223: 196/256/254/40; 241: 228/256/255/44; 251: 256/255/255/42.

## Reading
- The verdict field matches the numbers: 0.727 < 0.9, controls pass. No discrepancy between run.py and the data.
- The result is not uniform: rank_x/null rises monotonically with p, from about 0.41 (p=151) to about 1.0 (p=251, rank 256 = ceiling). The median is driven by the middle curves. The ratio tracks the class count (84 to 129 of 256 labels), so the x-compression looks like a small-size / label-occupancy effect, which vanishes as the classes fill the 8-bit label space. Whether it persists, vanishes or scales at cryptographic size is not tested; this is a four-curve trend at best, not a scaling law.
- The x-label rank is still far above the DL-label rank (about 100-256 vs 21-44), consistent with the hypothesis's weaker half, that only DL labelling compresses strongly.
- Caveats: orders about 2^8, one-step tensor, one cut, one FB shape, one labelling family. Rank is structure, not cost. Falsifiers A and B (GGM simulability, Cai-Chen) are not covered. No GAE or matched-rho pricing (section 5a). "Falsified" here refers only to the 0.95 threshold of toy falsifier C as the protocol reads it; the protocol's own reading of falsifier C was an inference from truncated text.

## Bearing on the sub-sqrt(p) question
constant_factor_only. x-labelling gives a partial rank reduction at tiny sizes that fades toward the ceiling as p grows, and DL labelling (needs the secret) remains the real compressor. No indication of a sub-sqrt(p) route.

## Standing
[pcc:unverified], single local toy run, no independent replay.
