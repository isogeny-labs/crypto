# Erratum to REPORT.pdf (PR #4380): the "unlocated" 0.085 ms per call is a one-time start-up

Written 2026-11-16 after `research/ic_gpu_wake_probe_20261116/`. The files in this directory are unchanged (the record
and its raw data pin them); this note corrects how they should be read. Unverified assertions of the producing session
(pcc:unverified).

- The "unlocated" remainder (round trip minus worker busy minus null-worker floor: 0.085 ms at 24 unknowns, 0.079 ms at
  26) is not a per-call cost. The cumulative mean round trip logged every 500 calls follows b + F/n to within 1 us, with a
  one-time F of about 150 ms per process (the CUDA worker's initialisation, paid by the first call). Spread over 1878 and
  2338 calls, F/n is 80 and 64 us. See `research/ic_gpu_wake_probe_20261116/REPORT.pdf` and `startup_fit.json`.
- The whole-call means in the table (0.5009 to 0.2028 ms at 24 unknowns, 0.6310 to 0.2753 ms at 26; "59.5 and 56.4
  percent less") include that start-up in both arms. Steady state, derived from the fits: about 0.423 to 0.119 ms
  (72 percent less) and 0.566 to 0.199 ms (65 percent less).
- The steady-state call is fully accounted for: round trip 108 us = worker busy 93 + pipe floor 13.5 + 1.5 at 24
  unknowns; the candidate causes listed in the report (cold wake-ups, `cudaStreamSynchronize` behaviour, first-launch
  latency after idle) were tested by a preregistered probe and not supported as per-call costs.
- The pipe-floor measurement (13.5 us in the container, 28 us on the host) and the counted-answer identity between the
  two arms stand.
