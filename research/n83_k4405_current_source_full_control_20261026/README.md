# K4405 current-source full-process resource control

The exact current-source K4405 wide3 worker ran on an earlier public N83
point for **873.169 seconds** under a 1,200-second outer cap. It was
manually stopped when host swap pressure became substantial. The run
preserves the source and executable identities, input, flags, resource
samples and terminal status `BOUNDED_UNKNOWN_HOST_SWAP_ABORT`. It does not
provide a completed rank or target result for this candidate.

This control uses curve `EC1N83Ckb1a1haacb69ee9f37`, 4,405 signed-Frobenius
representatives and the independently replayed current-source 731,230-point
base. The candidate identity is
`IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h76ff6ddee28c`.
The producer source SHA-256 is
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`;
the static binary SHA-256 is
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`.
The one-point input is fixture zero of the earlier public N83 cold-base
study, leaving the sealed cross-family holdout untouched. The worker used
the fixed five-byte ID30 table, `wide3` Bloom profile, root-only-11 state
layout, four root threads, two target threads and cooperative cancellation.

The original wrapper admitted the host at 53% system-wide free memory and
watched child RSS and the same free-memory percentage. Its largest sampled
child RSS was 11,680,759,808 bytes, below its 24 GiB stop limit. During
the run, the [host observation](evidence-01/host_pressure_observation.json)
recorded 3,746 immediately free 16 KiB pages and 1,551,744 pages occupied
by the compressor; the child remained live. The agent sent
SIGTERM to the child process group to protect concurrent jobs. The
post-stop `vm.swapusage` reading retained in
[`manual_abort.json`](evidence-01/manual_abort.json) was above 12 GiB.
The available telemetry does not isolate which concurrent process caused
each swapped page, and compressed/swapped pages make child RSS an incomplete
host-pressure gate.

The versioned [`runner_v1.py`](evidence-01/runner_v1.py) matches the executed
wrapper byte for byte. Its config, terminal receipt, samples, empty producer
output and file hashes are in [`evidence-01`](evidence-01/). Run
`python3 audit.py` here to check that bundle; it returns
`PASS_BOUNDED_HOST_SWAP_ABORT_RECEIPT`. A producer summary, base dump,
relation matrix and target receipt were not emitted before the stop.

The current [`run.py`](run.py) is a revised guard for a later attempt. It
requires at most 2 GiB host swap at admission and stops if swap crosses
that threshold during execution, in addition to the existing wall, child
RSS and host-free gates. Its preflight currently rejects this host with
more than 11 GiB swap in use. This change prevents repeating the same
high-memory launch while the host remains unsuitable; it does not
reclassify the interrupted result.

The next full-process K4405 control needs a 64 GiB-class host with a
working provider stop guard or a cleared local memory envelope. Once rank,
relation and scalar replay pass, the exact candidate can enter the
separately sealed same-point Tier 1 IC/rho comparison. The K5500 split
index requires a still larger measured memory envelope.
