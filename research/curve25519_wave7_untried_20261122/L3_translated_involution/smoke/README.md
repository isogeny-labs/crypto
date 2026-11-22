# Smoke, not data

`scripts/smoke.sh` ran on 2026-11-22. It does three things:

- runs the desk checks at k = 16;
- runs the census at k = 16, t = 4, 5, 6 with K = 100 frames per arm, all 15 arms, plus the fixed
  frames;
- runs the analysis script with `--smoke`.

It used about 9 s CPU in total (census 3.9 s CPU, 48 MB RSS).

What it shows is that the pipeline runs end to end:

- the desk checks pass 30/30;
- CAL1 reproduces the 6 `frame_search` fixed-frame counts it covers exactly;
- C1 has no pointwise control violation (one frame was unfittable, with < 3 fibres).

Verdict lines here use 100 frames, not the protocol's 2000. They are not evidence for or against any
prediction, and several statistical verdicts fail by design at this K [pcc:unverified].
