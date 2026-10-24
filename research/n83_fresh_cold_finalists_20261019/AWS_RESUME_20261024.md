# AWS handoff: N83 five-base shared-binary run 40114927017

This handoff is stacked on the shared-binary retry branch. Its executable source and sealed dispatch remain at GitHub head `8455179d125b5b451377ac9a9c38426fb714e9b1`. The terminal, machine-readable receipt is [results/run_40114927017/terminal.json](results/run_40114927017/terminal.json), with the [raw terminal provider response](results/run_40114927017/provider_jobs_raw.json); the earlier [provider.json](results/run_40114927017/provider.json) is an immutable *in-progress* snapshot. GitHub [run 40114927017](https://github.com/aburan28/crypto/actions/runs/40114927017) has a successful shared build and two failed stratum jobs. Both `m7i.2xlarge` Spot hosts in `us-west-2` are terminated. The five current-source factor bases remain in `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_current_source_finalists_20261019/`; their exact object URIs, sizes, hashes, and download replays are in [base_custody_prior_three_receipt.json](base_custody_prior_three_receipt.json) and [base_custody_missing_receipt.json](base_custody_missing_receipt.json).

Both strata completed all 320 scheduled events (eight public points, five bases, per-pair rho and calibration controls). For each stratum, all 40 IC rows and all 40 rho rows passed independent replay; base identity, command order, exact targets, executable bytes, and topology passed. Admission nevertheless failed. The EC2 host's `cpu.stat` omitted `nr_throttled`, so all 80 IC/rho resource receipts per stratum carry a null throttling delta and fail the frozen resource gate. Nonzero steal time appeared in 43 of 320 events in stratum 0 and 17 of 320 in stratum 1, failing the frozen host-noise gate. Calibration pairs report `REPLAY_OR_RESOURCE_FAILED` because their constituent resource receipts are inadmissible. The terminal analysis is `BOUNDED_UNKNOWN`/`NOT_ADMITTED` for each stratum, and `selected_best_total_runtime` remains null. Measured timings may be inspected as censored exploratory observations; they cannot select a base or support a speedup claim.

The retained GitHub artifacts are `n83-shared-binaries` (ID `12569745417`, ZIP SHA-256 `e206bbc5354f149b06da6bc3d345b3aa9099e37513755aaa1fc7e3472f30e973`), stratum 0 (ID `12573434670`, ZIP SHA-256 `eb77015ecc8182472ae72676586560769a1bd35bcd8b9fd66c9981ada0ab2a3c`), and stratum 1 (ID `12572808198`, ZIP SHA-256 `c2f0ca02cf777c09a1635351544055454b78ba73dabc64d75edfb20cebc09e2b`). All three ZIPs were independently fetched and matched these digests. GitHub reports expiry on **2026-11-07**, between 17:34Z and 20:00Z. Fetch them promptly and retain their bytes and provider API response under immutable, hash-bound custody before expiry. The repository receipt records artifact, provider-response, and extracted analysis/verify-log digests; the original two stratum archives are roughly 293 MB each.

From a checkout of the frozen run head, an AWS agent can fetch and inspect the two archives with:

```sh
mkdir -p /tmp/n83-40114927017
gh api repos/aburan28/crypto/actions/artifacts/12569745417/zip > /tmp/n83-40114927017/shared-binaries.zip
gh api repos/aburan28/crypto/actions/artifacts/12573434670/zip > /tmp/n83-40114927017/stratum0.zip
gh api repos/aburan28/crypto/actions/artifacts/12572808198/zip > /tmp/n83-40114927017/stratum1.zip
sha256sum /tmp/n83-40114927017/*.zip
unzip -tq /tmp/n83-40114927017/stratum0.zip
unzip -tq /tmp/n83-40114927017/stratum1.zip
mkdir -p /tmp/n83-40114927017/stratum0 /tmp/n83-40114927017/stratum1
unzip -q /tmp/n83-40114927017/stratum0.zip -d /tmp/n83-40114927017/stratum0
unzip -q /tmp/n83-40114927017/stratum1.zip -d /tmp/n83-40114927017/stratum1
gh api 'repos/aburan28/crypto/actions/runs/40114927017/jobs?per_page=100' > /tmp/n83-40114927017/provider_jobs_raw.json
python3 research/n83_fresh_cold_finalists_20261019/filter_shared_retry_jobs.py /tmp/n83-40114927017/provider_jobs_raw.json --run-id 40114927017 --run-attempt 1 --head-sha 8455179d125b5b451377ac9a9c38426fb714e9b1 --output /tmp/n83-40114927017/provider_jobs.json
sha256sum /tmp/n83-40114927017/stratum{0,1}/results/analysis.json
```

To bind the raw ZIP members, provider jobs, and independent local replay, run the aggregate as a module from the repository root:

```sh
R=/tmp/n83-40114927017
python3 -m research.n83_fresh_cold_finalists_20261019.aggregate \
  --stratum0-results "$R/stratum0/results" --stratum0-zip "$R/stratum0.zip" --stratum0-provider-sha256 eb77015ecc8182472ae72676586560769a1bd35bcd8b9fd66c9981ada0ab2a3c \
  --stratum1-results "$R/stratum1/results" --stratum1-zip "$R/stratum1.zip" --stratum1-provider-sha256 c2f0ca02cf777c09a1635351544055454b78ba73dabc64d75edfb20cebc09e2b \
  --provider-jobs "$R/provider_jobs.json" \
  --validated research/n83_fresh_cold_finalists_20261019/upstream_validated.json \
  --selection research/n83_fresh_cold_finalists_20261019/upstream_selection.json \
  --run-id 40114927017 --run-attempt 1 --head-sha 8455179d125b5b451377ac9a9c38426fb714e9b1 \
  --output "$R/aggregate"
```

The provider-job filter confirms the three-job identity even when both stratum conclusions are `failure`; the aggregate binds those conclusions to local replay exit 2 and `BOUNDED_UNKNOWN` analyses. Re-running `verify.py` on either extracted `results` directory should produce that status and exit 2. Preserve the complete raw archives before interpreting any descriptive timing.

The next legitimate AWS experiment needs a **new protocol and run identity**. Preflight the actual host for observable throttle accounting and zero steal during a representative measured interval; record the host, kernel, cgroup mode, counter presence, and resource envelope before dispatch. The current root cgroup has `cpu.max = max` and `cpu.stat` lacks `nr_throttled`; a host-level counter absence must be handled by a prospective, justified measurement contract, never by backfilling zero in this run. If the host still shows steal time, use isolated capacity or prospectively specify and validate a robust noise threshold. Keep the original exact bases and source pinned for a direct retry, charge all setup, failed attempts, import, relation search, linear algebra, individual-log work, and matched same-point rho, and obtain a disjoint-point confirmation before wider inference. Do not treat this run's already inspected 16 points as a fresh holdout.

The broader sweep is separate from this five-base panel. [PAIRED_FRONTIER_V4.md](../koblitz_n83_factor_base_sweep_20261008/PAIRED_FRONTIER_V4.md) indexes 26,939,278,080 finite design ordinals, crossing the declared split, symmetry, WDSat, Gray/FES, large-prime, and orbit settings; most are classified as unavailable or mathematically incompatible. Only 2,592 cross-stream suffix cases reach a source-availability gate, and that gate is not a measured solver result. The v2/v3/v4 design checks and bounded natural-relation controls give the AWS agents an addressable queue, not an exhaustive empirical runtime ranking. Keep these coverage and measurement claims distinct when extending the campaign.
