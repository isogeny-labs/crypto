# Amendment 1: container reclaimed during Part A (n = 19); resume

Written 2026-10-18, before any Part B or Part C run and before the n = 19
rerun.

`run_all.sh` started 2026-10-12 02:30 UTC. Sessions `A-n11`, `A-n13` and
`A-n17` completed (48, 48 and 32 records, all `verified`, per their logs).
Session `A-n19` was cut off by the cloud container being reclaimed: 14 of
40 executions recorded, `session.json` still `running`. Parts B and C had
not started. Six days later the container was restored with the same
checkout and the same release binaries (`BINARIES.sha256` matches).

Changes, none to the protocol's inputs:

- `A-n19` is kept as recorded and audited with `--allow-interrupted`. It is
  not a result.
- `resume_all.sh` reruns `SPEC-A-n19.json`, unchanged, into `A-n19-02`,
  then runs Parts B and C and every audit, with the same binaries.
- `progress.txt` records both launches.

Nothing was read from the interrupted session before the rerun beyond its
record count and its log's last lines.
