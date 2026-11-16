# Local N83 five-base screening protocol

This screen compares the same five exact current-source finalist bases as the
frozen GitHub trial on one additional deterministic public N83 point. Resolve
the ordered arms and candidate manifests from the sealed
`research/n83_fresh_cold_finalists_20261019/dispatch.json`, without inspecting
this new point. Run a fresh process for each arm in forward order, then a fresh
process for each in reverse order. Run one same-point rho process before and
after the IC sequence. Use the original source commit
`8bf07aefb3fb2a65248ce379da7284cb58a5581d`, the original current-source
environment flags, rank seed 29, and the original per-arm caps. Preserve every
status, including timeouts and nonzero exits. All child processes start with
empty output directories and construct the factor base from scratch.

`generate.py` seals the public target and answer before any timing and checks
that its point is outside the 16 frozen finalists and the prior local
root-layout point. `local_run.py` checks source, dispatch, manifest and fixture
hashes before measurement. Its two rho seeds are 2026102402 and 2026102403.
The independent audit must rehash every raw file, replay the scalar from the
public generator, check every IC/rho answer and full-rank receipt, and compare
each constructed base with the exact S3-custodied base. The S3 objects already
exist for all five arms; this screen does not create a different base.

This is a local macOS process screen with two repetitions on one point. It
does not meet the frozen cloud host/calibration gate or provide population
precision, so `selected_best_total_runtime` stays null regardless of the
ordering. The goal remains the fully charged total runtime across all
supported grid combinations, not the winner of this narrow screen.
