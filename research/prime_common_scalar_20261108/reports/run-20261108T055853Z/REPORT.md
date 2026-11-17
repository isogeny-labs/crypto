# Physical identical replay execution1

Same binary, seed0, instance0, replicate0 under the replay control arm.
Command: run.sh STUDY /workspace/audit/common-scalar-check 1 (final argument
is token1). Same1CPU,2GiB address cap,900s hard timeout, no retries,
no restores, no valid clock. Start05:58:53Z and finish05:59:18Z are in runs/1/receipt.json.
Receipt exit0;20928 records, byte-identical to execution0. Grouped generating-
function analyzer recounts41856 tuple totals and every prospective key.
Requested1, attempted1, completed1. Producer statement PCC-20261108-4f9871;
no independent signature. [pcc:unverified]

The shared [PDF](../report.pdf), [graph](../census.svg),
[exact summary](../summary.json) and [all-level table](../per-level.csv)
cover the identical results without treating replay as extra population data.
Raw archive ../../runs/1/panel.csv.xz; host and controls unchanged. Metadata
errors and the premature analyzer failure are preserved in the study root.
