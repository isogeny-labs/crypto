# Frozen continuation after shell-label failure

The image-restored card started at 2026-10-20 01:10:50 UTC. Its first
K2048 cache-off constructed control passed with checked rank one, followed
by an independent general-field replay. The shell then stopped with exit
2 because `run()` assigned the caller's global `label`, making the next
path end in `-replay-replay`. There was no natural query. This was a driver
bug, not a solver cap or a failed mathematical control.

The failed card and its 39-file archive are retained under
`xor-cache-v1/driver-failure/`; the original script remains unchanged.
`run-xor-cache-card-v2.sh` uses a subshell function to isolate its variables.
Its SHA-256 is
`9c9a07cca9ded5c4afe7db03217580c55c806c1c7b0ca120eedf0494721c1a4d`.
The native fixture test `n83_xor_cache_card_check` runs this actual shell
card with file-copy receipt fixtures. Both release tests passed at commit
`f1667c313910a1021daed455c9e48cd03524f42a`, checking all 24 calls in exact
order, completion of all sixteen public cells after a retained wall cap,
and rejection of output-directory reuse. This validates driver sequencing;
actual curve/row correctness remains the worker and independent replayer's
responsibility. The test log is `xor-cache-card-tests-v2.log`.

This continuation is committed before execution. Stage the same pinned
worker and replayer into `/home/ec2-user/n83-m23-20261019/xor-cache-v2`
using `prepare-xor-cache-v2.sh`. Only the run root and driver script/hash
change. Recheck every input hash and the exact restored image; rerun all
four controls and independent replays, then all sixteen public searches.
Worker source stays `1de47a9d4186bff08abc84c224a111183e820106`.
The protocol's budgets, order, host conditions and stop rules still apply.
No further retry is implicit.
