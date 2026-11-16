# Development preflight and runtime planning

This is **input preparation and a heuristic scheduling estimate**, filed before
the primary comparison. It does not run the M=8 selectors, compare their quality,
produce primary rank/work observations or establish a runtime advantage. The
original p=7,a=0,b=3,B=2 known-answer development fixture uses M=3 for selector
timing. Its separately known rank-control audit uses the existing W=8 fixture.
No threshold, field, B, budget or decision rule was selected from these timings.

## Observed inputs

- [calibration.json](preflight/calibration.json) records 13,430,371 ns for the
  known-control producer audit, independent replay and fixed-control checks
  together. There is no isolated producer/replay timing claim. The reference
  has 12 nonzero targets and coefficient dimension three. The maximum of its
  seven M=3 selector-call timings is 132,580 ns; every recorded timing remains
  visible. These are local development clocks, without admitted isolation or
  independent physical-host attestation.
- [observed-fresh-helper-build.log](preflight/observed-fresh-helper-build.log)
  records one fresh development helper/native-dependency build at 36.88 s.
  This original helper enumerated public input models only; the estimator
  plumbing was added later. The portable helper's incremental rebuild is
  retained separately in [reproduce-build.log](preflight/reproduce-build.log)
  and is not relabelled as a fresh build.
- [native-checks-exit.json](preflight/native-checks-exit.json) records the
  existing final standalone development checks over 34 seconds. This is not
  root-library validation or a primary research run.
- [estimate.json](preflight/estimate.json) records the native public-input
  census and complete planning arithmetic. It reports 8,20,51 eligible models
  for p=7,11,19 respectively, 79 total. These are public group-order/input
  facts from `tr04_verify::model_info`, not completed primary model records.
  The later primary run still must independently retain all 531 dispositions
  and its 1,106 model/B/method cases, including every failure.

The original full known-answer preflight manifest and raw records remain in
[development-evidence.tar.xz](development-evidence.tar.xz). The compact copied
[source manifest](preflight/calibration-source-manifest.json) preserves its
original source, binary, protocol and calibration hashes; not all paths named
by that old manifest are repeated in this directory.

## Exact input-size ceilings and heuristic time scaling

An eligible prime-order model has `(n-1)/2` canonical x coordinates. For each B,
there is at most one relation cache per canonical B-point base and prepaid flag:
`2*C((n-1)/2,B)`. The factor two conservatively allows every base under both
flags; collision avoidance may use fewer. Multiply each cache count by `n-1`
to retain its target exposure. The helper does not construct any primary pair
table or inspect rank/selection outcomes to obtain these ceilings.

| Field | Eligible models | B2/B3 cache-count ceiling | B2/B3 target-cache ceiling | Heuristic field work, seconds | Rounded planning allowance, seconds |
| --- | ---: | ---: | ---: | ---: | ---: |
| p7 | 8 | 114 / 108 | 1,104 / 1,128 | 8.802 | 100 |
| p11 | 20 | 560 / 870 | 7,460 / 12,420 | 85.109 | 100 |
| p19 | 51 | 3,366 / 7,986 | 61,380 / 150,732 | 961.142 | 1,000 |

For cache work, scale the observed control duration linearly in target count
and cubically in coefficient dimension: B2 uses dimension three; B3 uses four.
Use a predeclared factor two for unpriced execution, serialization, hashing and
I/O variability. This is a planning assumption, **not a proved wall-time bound**.
The reference timing already includes a production audit and independent replay.
The implementation computes a cache once in `run_shard` and independently
replays it once in the wrapper's `verify`; `summarize` checks hashes/receipts
and parses records without repeating the relation census.

For selector work, use the largest recorded M=3 selector duration for every arm.
Scale its public DP-state/candidate ceiling using all B-sized partial subsets,
and the affine-stream candidate ceiling for progression, then multiply by seven
arms and a factor eight for replay, trial-table width and other implementation
variation. The exact formulas and each term are retained in `estimate.json`
and the [native helper](preflight-helper/src/main.rs). No clock price is applied
to the primary native operation counters.

The schema's `per_sample_seconds=1000` is the **maximum** rounded field-work
allowance, covering all nine logical arms in one field job. It does not mean
27 isolated pods or 1,000 measured seconds. `fixed_seconds=219` assumes five
build roles (hosted preflight, three fields, hosted aggregate) at the observed
36.88 s rounded to 37, plus the observed 34-second development checks:
`5*37+34=219`. The conservative serial planning envelope is therefore
`219+3*1000=3219` seconds. The three field jobs may run concurrently; this is
not a prediction of their aggregate elapsed time. Runner startup, transport,
network, host differences and tail latency are not independently measured.

The existing **1,800-second field job cap stays fixed**. Each field's largest
planning allowance plus nominal build is 1,037 seconds, below that cap, but
the estimate does not guarantee completion. Checkpoint, deadline, interruption,
runner and evidence-store failures remain visible and may yield incomplete
coverage. No capacity, credentials, account limits, settings or funding change
is implied by this estimate.

## Reproduce the input preparation

From the repository root, using the standalone native helper:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/tr04-preflight-target \
  cargo run --release --locked \
  --manifest-path research/twist_relations_tr04_20261029/preflight-helper/Cargo.toml \
  -- research/twist_relations_tr04_20261029/preflight/calibration.json
```

The complete helper source, manifest and lockfile are tracked. Its recorded
[command/exit receipt](preflight/command-exit.json), raw output, build log and
SHA-256 list preserve the actual development invocation. A reproduction's
`elapsed_ns` will change; its public model counts and planning arithmetic should
agree. Re-running this preparation does not discharge any primary coverage,
timing, durable-store readback or completed root-library validation requirement.
