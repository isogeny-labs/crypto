# Frozen producer completion check

Review on 2026-10-20 found that the frozen runner labels its summary
`COMPLETE_REPORTED_OUTPUT` based only on the number of attempted events.
If the final event fails verification, that count still equals the schedule
length. Therefore the summary label alone does not establish completion.

The runner, protocol and fixture are preserved unchanged because their hashes
belong to the frozen study. Before consuming a producer summary, run:

```sh
python3 -m research.n83_load80_online_pair_20261020.check_reported_completion RESULTS_DIRECTORY
```

This additional check binds the frozen protocol, requires the exact ordered
schedule, rejects any failed process/resource/reported-output event, checks
each event's on-disk receipts, and rejects a producer stop marker. It returns
exit 2 and `BOUNDED_UNKNOWN` for incomplete or inconsistent output, including
a failure in the final event. It never alters the raw records.

`PASS_REPORTED_COMPLETION` is only a check of producer-reported completion.
Independent scalar, relation, subgroup, timing, calibration, resource and
immutable-base verification remain required by the original protocol before
publishing a measured result. No selection or speedup is asserted by this check.

The nine regression cases include the final-event failure, partial and
duplicate schedules, inconsistent/missing receipts and source/protocol mismatch:

```sh
python3 -m unittest research.n83_load80_online_pair_20261020.test_reported_completion
```
