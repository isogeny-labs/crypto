# Retained primary N83 solver-setting evidence, run 40193751812

This directory retains exact provider ZIPs and independent audits for the
ongoing [frozen eight-stratum panel](../solver-setting-panel-20261025/PROTOCOL.md).
The provider run is [40193751812, attempt 1](https://github.com/aburan28/crypto/actions/runs/40193751812),
at head `a5af8d5023a644e9e3b347af4d8495540c8dea99`. The inputs are
the same eight SHA-256-pinned, S3-backed XCNFs from the earlier natural-query
panel on the primary `a=0` GF(2^83) curve. This is a bounded solver-setting
experiment, not a complete factor-base runtime measurement.

The current [partial summary](partial-summary.json) binds four successful
provider artifacts, each of their five raw solver streams, provider metadata,
terminal job snapshot, and independent audit. All 20 arms reported
`INDETERMINATE` within the 600-second solver cap; every audited arm is censored.
The aggregate 12,062.753 process-wall seconds are parallel provider-host work,
not a one-target cold index-calculus time. The other four strata remain
pending in this retained snapshot. Conflict counts and wall times under a
time cap cannot identify a faster factor base or setting.

| Stratum | Provider artifact ID | ZIP SHA-256 | Five-arm status |
| --- | ---: | --- | --- |
| `gray_4_2_t1_m0` | 12599809244 | `f85714e38b1375dd9cdeb8c7a272277e766cb87b99a19c9446e9c1baaa38441f` | five `INDETERMINATE` |
| `gray_4_2_t2_m0` | 12599694537 | `e0a8d720ad878bfb31cc67bc53d743b5e0533a7ce38ad4eda67fd10b798abd1d` | five `INDETERMINATE` |
| `gray_5_1_t1_m0` | 12599749878 | `239114d768819ae3869043e23b8f7a0362e309bb8528a4f4db0569ac43a85cbd` | five `INDETERMINATE` |
| `hash_4_2_t1_m0` | 12600483557 | `ee7163aeb4d99f5f348d2d0d5dc72350e1e36fb5a776fa366d688baf81fceb93` | five `INDETERMINATE` |

[`retain.py`](retain.py) verifies the provider's artifact SHA-256 and byte
count, exact run/head/attempt, successful terminal stratum job, audit identity,
and frozen arm order. With the original source and solver ZIPs it reruns the
[independent raw-stream auditor](../solver-setting-panel-20261025/audit_artifact.py)
and requires byte-for-byte agreement with each saved `audit.json` before
returning `PASS_RETAINED_PARTIAL_CENSORED` for the current snapshot. Without
those ZIPs it returns
`BINDING_ONLY_SOURCE_ZIPS_REQUIRED`; this weaker check is useful for checking
Git custody but must not be represented as full replay. The [exact-run
collector](../solver-setting-collector-20261025/README.md) is the supported
way to reacquire source ZIPs and audit all eight once terminal. Its `--probe`
never dispatches another run.

For local replay after fetching the two pinned source ZIPs and solver ZIP as
described by the collector, invoke:

```sh
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/solver-setting-result-20261025/retain.py \
  --root research/koblitz_n83_factor_base_sweep_20261008/verification/solver-setting-result-20261025/run-40193751812-attempt-1 \
  --source-dir /path/to/n83-run-40185426122-attempt-1 \
  --solver-zip /path/to/n83-preflight-solver-12593635829.zip
```

The candidate minimum remains unset: `selected_best_total_runtime=null`.
Further evidence needs independently group-checked natural relations, graph
closure and rank, verified factor-base logarithms, unseen-target descent,
fully charged one-target cold accounting, and matched same-target rho. The
diagnostic `a=1` curve uses a different subgroup and is outside this ranking.
