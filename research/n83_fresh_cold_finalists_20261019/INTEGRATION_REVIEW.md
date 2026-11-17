# Parent integration review

The confirmation run used commit `8bf07aefb3fb2a65248ce379da7284cb58a5581d`. Its frozen dispatch and records remain unchanged. All sealed file digests were checked against that commit; see `integration-review.json`.

The parent integration includes the later opt-in load80 implementation in `examples/koblitz_orbit_dlp_fast.rs`. The sealed dispatch deliberately rejects this integrated checkout because that source digest differs. Do not reseal the old experiment or interpret its measurements as validation of the later source. To replay the original dispatch, use an isolated checkout of the recorded commit with Python 3.12, materialize all files named by `dispatch.json`, and run `python3 research/n83_fresh_cold_finalists_20261019/check_dispatch.py` there.

Native inputs (`src`, `isogeny_algos`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `examples`, and `docs/curves`) in the integrated tree are identical to commit `c0274ce6e4c995e9e06c233b6ff223bb84b8a5c2`, whose retained validation records 4,162 passing library tests and 22 passing IC example tests. This reuses that native validation; it is not a new execution of those tests or of the research campaign.

The push workflow triggers only when the frozen dispatch changes. This integration does not change it. Final local archive replay, aggregation, custody, figures and any scientific admission remain separate requirements; the parent merge does not complete those requirements.

After integration, six arm tests, two provider tests and nine selector tests pass. The frozen preflight rejects the changed IC source, as does dispatch validation; both checks passed at the original source revision. These rejections are the intended seal behavior and are not reported as passing validation of the current producer.
