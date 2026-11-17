# K2816 four-worker pilot: capped before target timing

The current-source K2816/root11 four-worker cancellation pilot reached its
900-second producer cap. The verifier records `BOUNDED_UNKNOWN` with raw
producer status `TIMEOUT`; no online target interval, relation, or four-worker
speed comparison is available from this run.

| Frozen item | Recorded value |
| --- | --- |
| Candidate | `IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h62c132d4bafe` |
| IC source SHA-256 | `8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829` |
| Executed archived binary SHA-256 | `c93716a49d7db070666af021d1add78df3529a23d674e53e50ea816f8b5ca04a` |
| Exposed target SHA-256 | `2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d` |
| Target policy | Four target workers, cooperative cancellation, indexed5 ID30 disabled |
| Process cap / observed wall | 900 s / 900.739068 s |
| Exit / stop | `-9` / `TIMEOUT` |
| Verifier result | `BOUNDED_UNKNOWN`; raw `TIMEOUT` |

The immutable raw directory is
`/Volumes/SSD990/crypto/artifacts/n83_thread4_pilot_20261017_raw`.
Its `source.json`, `host.json`, `usage.json`, `launch.json`, and `replay.json`
SHA-256 digests are respectively
`87f0614f12a70a7465c85391256a023230cd522f20dedb44ec55882233ea50ae`,
`b50f649b0bb9d57a197b94875f1f2922bb6c74ac809dfd093b64ad5b3ec22447`,
`faaa8dcec17f705f9d7214c6d991ba65a0df3d518f75fd5380c0509d9b1faf35`,
`04baf70b37fb262cbe4c39d9456cf0fc030337446828935a0377eddb752d1919`,
and `a95f9e85592a51d3c9f93f1f2ddfbe3365d54167b8ca40d1b19860b12ad1fd10`.

The macOS `peak_rss_bytes` raw field is not a validated memory measurement:
the collector multiplies the platform's `ru_maxrss` by 1024 and the sampled
RSS field is zero. Keep this resource envelope unverified. The raw usage
records 43.277 user CPU seconds, 155.914 system CPU seconds, and 2,449,364
involuntary context switches. This run was on an ordinary local host while
other I/O work was active; its cap does not refute the four-worker method.

The prewritten verifier initially stopped at an imported older-source hash
check before reading the producer status. The [failure record](results/first_verifier_failure.json)
preserves that exact boundary. The correction binds the imported verifier to
this pilot's frozen current-source hash; the rerun returned exit code 2 and
wrote `replay.json` with the capped status. A future fresh same-point Tier 1
comparison needs a host that completes K2816 construction under its resource
cap before four-worker online timing can be compared with two workers.
