# Hosted canonical six-summand K600 Gray/hash pair

[GitHub Actions run 39082501084](https://github.com/aburan28/crypto/actions/runs/39082501084) completed on 2026-10-18 from source commit `cdaa0a96391084de881064e99ca7a8da0f527370`. Its uploaded artifact is ID `12121096452`, named `n83-hosted-m6-canonical-39082501084-1`; the [run metadata](github-run.json) and [artifact metadata](github-artifacts.json) preserve those identities. The [frozen protocol](../canonical-m6-paired-20261017/PROTOCOL.md) fixes the prior 4+2 plan (SHA-256 `7788514127328a78cf2b0f634003ddae3ff18a9ff0c0c8cd75b50fcc006152cd`), one public target and coefficients, the sequential K600 base, and the Gray-prefix and hash-derived K600 pools. Each arm uses equal-domain canonical x ordering, retains all 16 intermediate-identity patterns in its search space, and has one CPU, a 4-GiB hard memory limit with no swap, a networkless read-only container, and a 20,000-conflict cap.

Both arms imported the exact archives and reached **`UNKNOWN_search_cap`** after one SAT call, one attempted identity pattern, 20,000 conflicts and zero models. Neither first pattern was refuted. These capped calls are censored solver-access observations; zero observed models does not estimate natural relation yield or establish a ranking.

| Measurement | Gray-prefix pool | Hash-derived pool |
| --- | ---: | ---: |
| Sequential-base signed-Frobenius orbits | 600 | 600 |
| Shared signed orbits | 503 | 0 |
| Residual signed orbits / points | 97 / 16,102 | 600 / 99,600 |
| Domain clauses | 237,280 | 323,796 |
| Peak variables / clauses / XOR rows | 118,930 / 548,356 / 6,059 | 120,942 / 634,872 / 6,059 |
| Peak cgroup bytes | 380,084,224 | 446,128,128 |
| Search-container wall, local observation | 1,289.221 s | 1,384.766 s |
| Solver calls / conflicts / models | 1 / 20,000 / 0 | 1 / 20,000 / 0 |
| Audited status | `UNKNOWN_search_cap` | `UNKNOWN_search_cap` |

The [independent audit](audit.json), implemented by [audit.py](../canonical-m6-paired-20261017/audit.py), passed as `PASS_MATCHED_CANONICAL_ONE_QUERY_REPLAY`. It checked the pinned source files, plan and public target; the regenerated 54-object pilot manifest and three exact archived bases; fresh S3 archive bytes; the worker and image identities; both container commands, limits, bind mounts, imports, raw log digests, search counters, exit and cleanup states; and the outer result. It also records SHA-256 for every one of the **65** uploaded files, totaling **11,211,938** uncompressed bytes. Their path/size/hash tree digest is `12fa07b0adcbe5fefde26d298fad0062fa9c575b9b1865a8a1deff1ba1138061`. The downloaded raw GitHub artifact is retained by GitHub until 2027-01-16 according to its metadata.

The three bases used in this pair were freshly downloaded from the user-specified `s3://crypto-autoresearcher/factor-bases/icv1/etc/` prefix and passed SHA-256, BLAKE3, byte-length and single-part ETag checks in the [S3 byte receipt](s3-bases-refresh.json). A separate [live inventory](pilot-s3-presence-refresh.json) confirms all **54** manifest-listed pilot bases at their expected sizes; it did not rehash the other 51 contents.

The hosted job ran from 00:28:38 to 01:21:16 UTC, **3,158 seconds** including preparation, regeneration, build, imports, searches and upload. The paired supervisor reported 2,680.213 seconds. This screen conservatively charges the **full 7,500-second hosted envelope** in addition to prior rounds. The prior first-pair run used a different hosted job, so its search-wall differences are not a matched speed comparison.

For replay, download artifact `12121096452` from the run and the three `s3_uri` objects in the S3 receipt, then run:

```sh
PYTHONPATH=/path/to/blake3 python3 research/koblitz_n83_factor_base_sweep_20261008/verification/canonical-m6-paired-20261017/audit.py \
  --artifact ARTIFACT_DIR --s3-bases S3_BASE_DOWNLOAD_DIR --run-id 39082501084
```

This is one public-target query per arm. Repeated disjoint-query partial yield, double-large-prime graph closure, useful rank, sparse linear algebra, an individual logarithm and its verification, a matched rho run, and fully charged cold single-target runtime remain unmeasured. `selected_best_total_runtime` and `total_index_calculus_runtime_ms` remain null.
