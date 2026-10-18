# K2816 wins N83 root11 cold-process comparison on 32 fresh points

The sealed four-host Tier 1 comparison selects the K2816 root11 dynamic
factor base for its specified fresh-process cost: it beat K3072 on all 32
previously unseen public points. The mean paired K3072 minus K2816 cost is
61.167 seconds (descriptive 95% paired interval 59.102–63.233 seconds;
two-sided exact sign p = 4.6566 × 10⁻¹⁰). The primary single-target online
selector remains `BOUNDED_UNKNOWN`: K3072 was faster on 19 points and K2816
on 13, below the frozen 23-win gate. These are two distinct, prespecified
timing objectives, and only the cold-process one selected a base.

| Exact source and measurement | K2816 root11 dynamic | K3072 root11 dynamic |
| --- | ---: | ---: |
| IC1 candidate suffix | `hf693b1b33c8b` | `hec7ae07d8b9a` |
| Actual usable factor-base points | 467,456 | 509,952 |
| Folded columns | 2,816 | 3,072 |
| One-target online median | 14.469 ms | 9.798 ms |
| One-target online geometric mean | 14.973 ms | 10.634 ms |
| Fresh-process internal wall median | 352.961 s | 415.363 s |
| Fresh-process internal wall mean | 354.639 s | 415.806 s |
| Geometric paired same-point rho / IC online ratio | 156.23 | 214.98 |
| Individually verified points faster than paired rho | 32/32 | 32/32 |

The geometric online K3072/K2816 ratio is 0.7102, with a descriptive paired
95% interval of 0.3798–1.3282 after exponentiating the frozen log-difference
interval. Its two-sided exact sign p is 0.3771. The rho/IC ratios use each
candidate's own same-public-point rho solve and the same measured cloud
allocation for that pair; all 64 online ratios exceed one. The online interval
starts after reusable factor-base, index, and log preparation and ends after
target recovery; the fresh-process interval comes from the binary's
`timing_ms.process_total` and includes index construction, rank stage, and
target computation. Process launch and external archive replay are outside
that supplementary fresh-process interval. A full cold-start accounting would
also charge process launch and the separately recorded external checks.

The four archives from [GitHub Actions run 39061060775](https://github.com/aburan28/crypto/actions/runs/39061060775)
were produced at source commit
`9b9a0f3842de6cf921e38573cb99fcaed2a4683c`. Four distinct provider
runners overlapped in execution. Each stratum admitted eight points with
`PASS_TIER1`, `PASS_10_PERCENT_PER_PAIR`, and `PASS_INDEPENDENT_REPLAY`.
The 32 points generated 64 IC runs, 64 same-point rho runs, and 384 frozen
rho calibration runs. The [validated rows](results/run_39061060775/validated_rows.json),
[selection](results/run_39061060775/selection.json), and
[provider jobs](results/run_39061060775/provider_jobs.json) bind all points,
failed-attempt status fields, host receipts, and runner identity. Their
SHA-256 digests are respectively
`2951fccbc9fa2cd7ec60e026a2e18161debb322a8fa9912bbebe174d0ac2c8db`,
`45d537214924e1d8de23037cc6872443c4ea2ee8bc7de0f7cc06036f8ac76a3d`,
and `f39f03966a071e8282d7a43af7e9685d60ee58a5d84c7e09739b865992f56b4f`.

The unmodified provider ZIPs remain under
`/Volumes/SSD990/crypto/artifacts/n83_root11_finalists_tier1_20261017/run_39061060775/`.
For strata 0–3, their provider SHA-256 digests are
`333b8d9b4ab4c26721a2d54d5deccc1fc8b4123c4c198113f6fe933f1426816d`,
`36a03a4609c21756d6fc0a948e3121ab65d24df85d48a3498b9fc43c055c8155`,
`0c03a53b49831f9b2d387eecb0b8459c29b097567702746f23c798b5dbfce9c1`,
and `7f2e1cf81fee4511057976f31ef2513f54ebb966bce3b05ebe8ba47f646b3084`.
The aggregate bound every extracted member to those ZIPs and the provider
job receipts, then independently replayed each admitted analysis.

Local replay used [verify_cached.py](verify_cached.py), SHA-256
`a40cc97baa658646e57c47555dbda7515b2b4381f8afd8a7472012d8758677d8`.
It calls the frozen verifier and caches the full factor-base orbit replay by
SHA-256 of the complete parsed dump plus all summary fields consumed by that
check. Each stratum had two distinct base dumps, repeated eight times per
candidate. Every target witness and scalar check still ran separately. Each
cached verifier JSON was identical to its original provider-verifier JSON,
and each stderr was empty. The four
[cached replay receipts](results/run_39061060775/) preserve wrapper and
stdout hashes. The original local replay attempts for strata 0, 2, and 3
were interrupted under severe shared-host contention; their
`stratum*_original_interruption.json` receipts preserve that boundary. The
initial direct-script aggregate invocation was also stopped by Python's
`select.py` module-name shadowing of the standard-library `select` module;
the successful aggregation ran the same module through `python3 -m` and
`--reuse-local-replay` with unchanged ZIP and provider bindings.

An unrelated central 16–16 edge case was found in the sealed selector: its
two-sided sign p value would exceed one. The observed splits are 19–13 and
32–0, so this edge case does not change either decision. Running the original
sealed selector on the committed validated rows reproduces the committed
selection JSON exactly. The edge correction is preserved separately for a
prospective selector revision. The next comparison tests both bases against
the K3450 wide3 and K3500 root4 families under the same two-worker target
cancellation policy on a separate 24-point fixture.
