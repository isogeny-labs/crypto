# RESULT: LEAD-EXO-cryptoscale-candidate-screen

## Outcome
- Status: ran to completion. Exit code 0. Wall 20 s (run.status). One run only, no retries.
- Files: RESULT.json, run.stderr (empty tail), run.status.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): candidate flag false for rhoA, rhoB, ph_generic; true for ph_smooth_planted. Falsified if any generic arm is flagged; inconclusive if the positive control is not flagged or any arm has < 3 solved instances.

Measured (median group ops / sqrt(d), 95% bootstrap CI, ratio to bsgs):
- rhoA: n=42, 10.89 [8.28, 12.59], ratio 6.20, candidate false
- rhoB: n=42, 9.36 [8.39, 12.68], ratio 5.32, candidate false
- bsgs: n=42, 1.757 [1.570, 1.960], ratio 1.00, candidate false
- ph_generic: n=42, 1.764 [1.598, 1.967], ratio 1.004, candidate false
- ph_smooth_planted: n=28, 0.0514 [0.0290, 0.0708], ratio 0.029, candidate TRUE
- A/A rhoB/rhoA median ratio: 0.859 (expected near 1; within the noise implied by the overlapping CIs).

Verdict: consistent_with_prediction. Controls: null (three generic arms unflagged) passed; positive control flagged; A/A near 1 and CIs overlap. All arms had n >= 28.

## Reading
- The numbers support run.py's verdict field; no discrepancy found.
- Caveat 1: the bsgs reference n=42 corresponds to the generic sizes k=12,14,16 only, while the planted arm is at k=28,32. The planted arm is therefore compared with a generic reference from other sizes. This is tolerable only because ops/sqrt(d) is roughly size-independent for BSGS, and the margin (ratio 0.03 vs threshold 0.8) is far larger than any such effect.
- Caveat 2: per PROTOCOL s6, the 0.8 threshold and the planted sizes were tuned on pilots, so detectability of the positive control is by design. The planted attack (Pohlig-Hellman on a smooth order) is a textbook weakness, not an unknown one.
- Caveat 3: "ph_generic ~ bsgs" is expected, since prime-order PH reduces to BSGS; it is not an independent null.
- The toy tests soundness and sensitivity of the screen statistic. It does not test the recorded 13 batches, any crypto-size curve, or any externally sourced candidate. Operation counts are unpriced (no s5a 0/0.1/1 GAE band).
- Nothing surprising: rho constant ~9-11 sqrt(d) is in the protocol's stated 5-30 range.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction (narrowly): the screen is not blind at toy scale, so FINAL_NO_CANDIDATE on generic inputs is not an artefact of insensitivity to a planted smooth-order weakness; it says nothing about unknown weaknesses.

## Standing
[pcc:unverified], single local toy run, no independent replay.
