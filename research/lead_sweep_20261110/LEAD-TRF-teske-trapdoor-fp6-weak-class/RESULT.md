# RESULT: LEAD-TRF-teske-trapdoor-fp6-weak-class

## Outcome
- status: ran (single run, no retries); exit code 0; wall 3 s (run.status: exit=0 wall=3).
- run.stderr empty. RESULT.json is one JSON object, verdict "falsified".

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): pooled public_ratio = mean blind-walk steps * w, w = 0.05, in [0.85, 1.15]. Falsified if outside the band while controls are valid.
- Measured: public_ratio_pooled = 1.3528 (arm A 1.385, arm B 1.321). Above 1.15.
- Null (random weak sets) is the primary arm: ratio 1.35, mean steps 27.7 / 26.4 (A/B).
- Positive control (BFS-informed): ratio 0.2405 (< 0.5), valid.
- A/A: relative difference 0.0469 (<= 0.08), valid.
- Descriptive ball arm (graph-aligned): ratio 3.37, no verdict role.
- Verdict: falsified (controls passed), i.e. blind walk costs about 1.35x sampling in this toy.

## Reading
- Graphs are tiny: n_vertices = 14, 4, 52, 88, 6 for p = 1009, 1511, 2003, 2503, 3001; weak_size = 1, 1, 3, 4, 1.
- Weak size is rounded to at least 1, so the actual weak fraction is 1/14, 1/4, 3/52, 4/88, 1/6 (about 0.07, 0.25, 0.058, 0.045, 0.17), not 0.05. The ratio "steps * 0.05" is therefore not a clean steps/(1/w) test. Primes with a larger actual fraction should give smaller mean steps, so the 1.35 is not simply explained by this, but the normalisation is off. Also starts are non-weak and the walk is non-backtracking on small, cycle-like graphs, which raises hitting times above geometric. A hitting time well above 1/w is plausible from slow mixing, not from any isogeny-class locality effect.
- run.py's verdict field matches the numbers (1.353 outside band, controls valid). No crash or discrepancy in the code path.
- The toy does not test the F_{p^6} / Joux-Vitse claim (PROTOCOL s6). It is a prime-field proxy with random weak sets.
- The informed BFS ratio of 0.24 shows only that graph knowledge helps, as expected on tiny graphs.

## Bearing on the sub-sqrt(p) question
none: a random-weak-set, tiny prime-field graph toy says nothing about F_{p^6} trapdoor structure or sub-sqrt(p) cost; the falsification is of the toy's own mixing assumption.

## Standing
[pcc:unverified], single local toy run, no independent replay.
