# N83 cooperative cancellation: fresh Tier 1 result

Cooperative cancellation reduced the verified one-target online interval on
all 24 fresh public points relative to waiting for both target workers. The
preregistered paired selector chose cancellation: its geometric paired
join/cancellation ratio was 4.37, with exact two-sided sign-flip
`p = 1.1920928955078125e-7`. Both arms used the same 164,672-point factor
base, 992 folded columns, root11 index, two target workers, and frozen source;
only the cooperative cancellation flag differed.

## Measured result

| Arm | Candidate suffix | Mean online, ms | Median online, ms | Mean cold process, ms | Median same-point rho/IC |
| --- | --- | ---: | ---: | ---: | ---: |
| Join | `33c9c3b2f933` | 254.016 | 208.275 | 112,253.247 | 7.7058 |
| Cancel | `58ec337f71f7` | 81.176 | 54.427 | 111,769.789 | 35.8132 |

For `log(cancel_online / join_online)`, the paired mean was `-1.474735`, its
descriptive 95% interval was `[-2.161442, -0.788028]`, and cancellation was
faster on 24/24 points. The exact paired sign-flip test used all `2^24`
assignments. The cold-process mean paired difference, cancel minus join, was
`-483.458 ms`, with descriptive 95% interval `[-934.478, -32.438] ms` and
exact `p = 0.036473989486694336`. Cold process includes the target-independent
base and index construction, so its relative gain is much smaller. The
one-target online interval is the primary comparison under the repository's
measurement contract.

Each arm independently solved the same 24 public points with a matching
single-target rho solve. All 24 same-point rho/IC ratios were greater than
one for each arm. The four six-point host strata passed `PASS_TIER1` and
`PASS_10_PERCENT_PER_PAIR`; the cancellation arm passed the all-six-point
same-point speedup gate on each host. The recorded panel includes 48 IC
relations and scalar checks, 48 corresponding rho solves, and 288
calibration solves. The local replay returned `PASS_INDEPENDENT_REPLAY` on
every stratum, and the combined selector returned `SELECTED` for cancellation
on both online and cold process time.

## Custody and reproduction

The public target file has SHA-256
`aa02c031dedd97a5f4bb214838e550e4597f5af44b556c3f24f7e5f3eefd99f5`;
the answer sidecar has SHA-256
`c0134002e32ff898c8799f2024649ef286d0fbf881dc5a1a3eff5b589e4ce7d2`.
The source SHA-256 is
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`.
GitHub Actions run
[`39055982074`](https://github.com/aburan28/crypto/actions/runs/39055982074),
attempt 1, ran at commit
`389abba5dbbe6b0c112e25096eceadb516d4e724` on four distinct,
overlapping runners. The versioned
[provider-jobs snapshot](results/panel_39055982074/provider_jobs.json) has SHA-256
`368895594c4f993c77691d055bbdf10b5055f0c7e01a5a5e6db9755e15f98f9e`.

| Stratum | Public indices | Artifact ID | Raw ZIP SHA-256 | Local replay exit |
| --- | --- | ---: | --- | ---: |
| 0 | 0–5 | 12113664744 | `a051e5e2d610e3c35422b59df7ea87caad1c8e3c0ad16ba610766841833c393c` | 0 |
| 1 | 6–11 | 12114856398 | `45bbfd2921253b301e98350ada552da37572c8f998e8f6d533305b969e0dfe0f` | 0 |
| 2 | 12–17 | 12114770711 | `628fc06b951e5b7ea1e2095e77c5af9c3eaa88c706f118ca5de9341108c3ac1d` | 0 |
| 3 | 18–23 | 12114990866 | `871bd97306a820b79f62eb64f56e38ea8b17f005bf39e12a2fa48d4b35938de5` | 0 |

Download each GitHub artifact ZIP, verify its SHA-256, extract it to a
separate `stratumN/results` directory, and save the run's provider jobs JSON.
Run this directory's `aggregate.py` with all four `--stratumN-results`,
`--stratumN-zip`, and `--stratumN-provider-sha256` arguments, plus
`--provider-jobs`, `--run-id 39055982074`, `--run-attempt 1`, the commit above
as `--head-sha`, and an empty `--output` directory. The aggregator binds every
extracted member to the ZIP, independently reruns `verify.py` for each
stratum, checks provider runner identities and overlap, then applies the
frozen `selector.py`. The versioned
[`validated_rows.json`](results/panel_39055982074/validated_rows.json) and
[`selection.json`](results/panel_39055982074/selection.json) are the exact
aggregated outputs. Their SHA-256 digests are respectively
`b0fd64f95867b4d273d18998549b42e4d86323223aedf8e57a189acfa28f1d53`
and `27fbc9510953384bd5de50578d98d9b2b3306ba936aa6655ef802097acd7585b`.

This result isolates cooperative cancellation on the frozen 992-column
factor base. The separate factor-base finalist study compares 2,816 and
3,072 folded columns on fresh points under the same Tier 1 measurement
contract.
