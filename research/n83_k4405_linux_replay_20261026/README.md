# Independent replay for the K4405 Linux control

`verify.py` checks the completed a1 K4405 control against its exact
731,230-point current-source factor-base prefix, frozen public point,
answer sidecar, algorithm flags, source and Linux executable hashes. It
recomputes the scalar and four-point relation in the independent N83 group
model, checks rank and exclusive online phase accounting, and rejects new
cgroup OOM or CPU-throttling counters. The producer's zero exit code alone
does not pass this check.

Copy the actual executed Linux binary together with the complete control
output before terminating the Pod. In the checked-out source commit, run:

```sh
python3 -m research.n83_k4405_linux_replay_20261026.verify \
  /path/to/copied/control-output \
  --binary /path/to/copied/koblitz_orbit_dlp_fast
```

The verifier accepts only the earlier public fixture-zero workload. The
candidate is
`IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h76ff6ddee28c`.
Its `PASS_INDEPENDENT_REPLAY` result establishes correctness and this
control's resource envelope. The sealed Tier 1 same-point IC/rho comparison
and cold-cost selection require separate completed measurements.

The mathematical replay path passed a positive control on the published
current-source K1107 run and rejected altered scalar and relation witnesses.
The K4405 Linux control is pending cloud execution; there is no completed
K4405 output to promote yet.
