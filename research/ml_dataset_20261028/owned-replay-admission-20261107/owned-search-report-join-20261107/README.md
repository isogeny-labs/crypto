# Restricted owned-search report and capture join

Adds a read-only independent join from the native partial worker report to the latest complete snapshot and final committed event head. The snapshot provides the equations and committed rows used by the existing mathematical checker. Full prefix verification completes before any output. Explicit source hashes and expected pattern remain caller-supplied dependencies; this helper does not yet perform native cell admission.

The actual native report contract in src/ml_dataset/owned_search_report.rs and existing independent capture/row checker informed this implementation. Its output is restricted-only, with no public feature record. It does not verify producer counters, signed lifts, original descent, complete search or degree labels. Failed or capped mathematics remains failed or inconclusive. Report metadata reads are bounded to8192bytes, source metadata to65536bytes; existing capture/retention/replay budgets remain in force.

54 software controls pass:13 new joins/CLI controls plus41 donor controls. The fixtures are synthetic capture streams with mathematical row identities, not completed native dataset runs. Tests include changed source/head/pattern, false scalar/degree labels, invalid numeric types, corrupt payload, trailing committed event, stale report after a newer snapshot, latest-only row labels and capped CLI exit2. Independent review found the trailing committed-event gap in the initial draft; the final implementation rejects it and retains the review/probe separately.

Run:

```sh
python3 -m unittest -v test_report_join test_replay_capture test_capture test_inspect_wire
python3 report_join.py /sealed/cell/live --binding CAPTURE_SHA --sources sources.json --report outcome.json --pattern 0
```

This packet copies its self-contained dependencies from owned-search-semantic-replay-20261107. Only check_capture.py changes among donor modules: its internal verified callback now also returns chunk count and the exact last-chunk acknowledgement. Frozen donor packets remain unchanged.
