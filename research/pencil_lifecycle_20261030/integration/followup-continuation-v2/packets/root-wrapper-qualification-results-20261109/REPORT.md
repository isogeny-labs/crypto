# Root wrapper: 26-case software qualification

The recorded single invocation has26 passing case assertions and process exit0. Harness elapsed time is0.22215310897445306 seconds; conservative charge is60 active worker-seconds. Source pins, exact case names, publication and exit receipts are preserved. There was no native unit, compiler or network invocation according to the harness result.

## Scope and retained evidence

Cases exercise valid waiver and ordinary finite allocations; invalid waiver, numeric and structural inputs; worker permit identity/expiry; mocked heartbeat failures; fixed log-prefix hashing; atomic write and sealing fsync failures. The exact 26-case harness and QUALIFICATION.md were prospectively reviewed under T1961 before root execution. Internal50-second BaseException deadline and outer55+5-second timeout bound one invocation. The command uses512MiB address space and1MiB per-file limit in a dedicated subshell. CPU time and peak RSS were not recorded.

This independent archival readback checked26 unique case names, all passing result rows, stdout equality to result.json and exit0. It inspected the retained synthetic admission JSON, verified the fixed six-byte log-prefix hash, old atomic value and absence of terminal-output-manifest.json after failure. No test was rerun. The two heartbeat exceptions have case assertions but no separate exception receipt; this limitation is retained.

The raw archive holds199 files, including copied source inputs, synthetic admission variants and control outputs. No failed fixture result is removed: expected rejection is a passing assertion, while the underlying inputs remain invalid. The graph groups recorded assertions by fixed case category; no population inference or timing comparison is made.

## Publication and remaining boundary

The source commit is recorded in publication.json. .artifacts.json provides the content-addressed archive digest and byte count; raw-manifest.json gives each uncompressed file hash. Artifact-tool push and remote verification completed with no problems; local hydration remains available. Use ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull with this packet path.

These are software helper/permit/heartbeat checks. Actual supervisor loss, systemd cleanup, native OOM, disk exhaustion and power-loss durability were not exercised. The separate1,800-second validation retains its own native containment, permit, heartbeat, input, output and custody checks.
