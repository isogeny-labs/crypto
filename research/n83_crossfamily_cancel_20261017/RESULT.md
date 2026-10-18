# N83 four-family holdout: K2816 wins the complete cold process panel

The frozen 24-point, four-stratum Tier 1 panel selects **K2816/root11 dynamic
for complete fresh-process IC time among its four prespecified families**.
The independent aggregate is `PASS_INDEPENDENT_REPLAY`, with four distinct
concurrent provider runners, identical source and binaries across strata,
identical candidate bases across points, and passing per-pair calibration.
Its [validated rows](results/run_39075872898/validated_rows.json) and
[frozen selector](results/run_39075872898/selection.json) retain every
candidate-point IC and same-point rho measurement. The online selector is
`BOUNDED_UNKNOWN` with a null selected ID.

| Factor base and index | Usable points | Folded columns | Mean complete cold process, s | Geometric online mean, ms |
| --- | ---: | ---: | ---: | ---: |
| K2816/root11 dynamic | 467,456 | 2,816 | **334.291** | 5.937 |
| K3072/root11 dynamic | 509,952 | 3,072 | 393.983 | 6.370 |
| K3450/root11 wide3 Bloom | 572,700 | 3,450 | 577.128 | **5.474** |
| K3500/root4 standard Bloom | 581,000 | 3,500 | 586.219 | 7.019 |

The selector compares direct paired cold-time differences and online log
ratios on the same 24 public points. For each of K3072, K3450 and K3500
minus K2816, all 24 cold differences are positive. Their mean advantages
for K2816 are **59.691**, **242.837** and **251.927 seconds**, respectively;
each exact two-sided sign-flip probability is `1.1920928955078125e-07`,
below the frozen six-pair Bonferroni threshold `0.05/6`. K3450 has the
lowest descriptive online geometric mean, but none of its three online
pairwise tests reaches that threshold (`p=0.8408`, `0.6363`, `0.4410`).
All 96 same-point online IC/rho pairs pass scalar replay and have rho/IC
ratios above one; those after-setup online ratios do not include the cold
base, index, relation, rank and log setup.

This result is for the a=1 degree-83 diagnostic curve with a 53-bit prime
subgroup, the exact two-worker target-cancellation source, and these four
bases. It does not compare the a=0 SAT sweep or smaller a=1 bases such as
K1107 and K1120 under matched conditions. The selected K2816 ID is a
**within-panel cold-process choice**; `selected_best_total_runtime` for the
broader N83 search remains null. The separately dispatched K1107-versus-K2304
bridge is one relevant next rung.

## Provenance and replay

GitHub Actions [provider run 39075872898](https://github.com/aburan28/crypto/actions/runs/39075872898),
attempt 1, used source commit `577c8cbe9ddfa20a2b6a62d8c39eb1dd339497ed`.
The source, public-target and scalar-sidecar SHA-256 digests are
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`,
`ac0b84bd916291331e0dd13bfb312aedcd50d7cd3bbc1c3cd5f81eeca22ea73d`,
and `2c47e48393346ca742eddff8e0cbfeb59b9bd486b154428f28aee2a90a5fa49b`.
The [provider artifact receipt](results/run_39075872898/provider_artifacts.json)
pins all four artifact IDs and ZIP hashes; the
[provider job snapshot](results/run_39075872898/provider_jobs.json) binds
the four distinct runner IDs and overlapping jobs.

Each stratum was independently replayed from its extracted raw provider
archive on the local host. All four verifier exits were zero, stderr was
empty, and parsed stdout exactly equaled its ZIP-bound `analysis.json`.
The [local replay receipt](results/run_39075872898/local_replay_receipts.json)
and four compressed full stdout files retain those identities. The unchanged
frozen aggregate then rebound the ZIP members, source, bases, hosts and
points before executing the selector; its final exit was zero with empty
stderr. The [attempt history](results/run_39075872898/attempt_history.json)
also retains an initial direct-path Python `select.py` import-shadow failure
before a verdict and an intentionally interrupted sequential replay after
stratum 0, when the separate parallel replays were already running. The
module-based final aggregate used `--reuse-local-replay` over the verified
receipts. The raw ZIPs remain outside Git under
`/Volumes/SSD990/crypto/artifacts/n83_crossfamily_independent_audit_20261018/run_39075872898/`.

The four factor bases themselves are content-addressed in the approved S3
prefix, with [fresh-download and tamper audits](https://github.com/aburan28/crypto/blob/44519dc5b4/research/n83_crossfamily_s3_custody_20261018/RESULT.md)
in the separate custody PR. The additional
[four-host independent replay](https://github.com/aburan28/crypto/blob/44519dc5b4/research/n83_crossfamily_independent_replay_20261018/RESULT.md)
passed on distinct runners disjoint from the provider runners. Each full
replayed JSON matched both the pinned provider analysis and the local replay.
That cross-check adds provenance evidence, not new timing samples or a
broader factor-base comparison.

The unchanged source passed its 768-event preflight, 12-case selector suite,
and sealed dispatch check. Re-executing the frozen selector over the committed
validated rows reproduced `selection.json` byte for byte
([validation receipt](results/run_39075872898/validation.json)). No runtime
producer or selector source changed in this evidence commit.
