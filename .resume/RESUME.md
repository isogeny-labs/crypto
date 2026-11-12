# Resume: crypto @ claude/ecstatic-thompson-0jpmjv

## Goal
Owner (2026-11-12): run the frozen wide-modulus sparse-LA pack on RunPod ("1 but on runpod").
Protocol: research/ic_techniques_20261012/wide_sparse_la/PROTOCOL.md + PROTOCOL_AMENDMENT_1.md
(RunPod cpu5g 4 vCPU/16 GiB, one pod per seed, A/A per pod, USD 25 ceiling, 40 h pod deadline).

## Lead / hypothesis
Size law: row_ops/D grows as D^(beta-1), beta in [1.9, 2.1] (Krylov). Width law: ns per row_op
t(L) = c0 + c2 L^2 over limbs L = 1..4. Extrapolate U192@129 (ECC2K-130 order) to D = 1e9 with
interval; falsifiers in PROTOCOL.md "How the extrapolation model is falsified".

## Spec
research/ic_techniques_20261012/wide_sparse_la/runpod/{launch.py, pod_run.sh, make_bundle.py}.
launch.py bundles HEAD, uploads to s3://setec-astronomy-artifacts/lab-results/wide-sparse-la/<stamp>-<sha>/,
creates 3 RunPod CPU pods (REST v1) that build la_engine_bench and run the grid for their seed.
Record: benchmarks/records/<id>-wide-sparse-la.json (benchrec).

## State
2026-11-12: amendment + launcher committed; local smoke test of pod_run.sh (D=2^8..2^9) run.
Launch manifest: research/ic_techniques_20261012/wide_sparse_la/launch.json (pod ids, S3 prefix).

## Runs in flight
3 RunPod pods (see launch.json). Progress: aws s3 cp <prefix>/seed<s>/runs.jsonl -; done.json appears at end.
Pods self-terminate via podTerminate; if not, terminate via REST DELETE /v1/pods/<id>.

## Stop and success conditions
Fixed grid; per-run 3600 s timeout and per-arm stop rule; pod deadline 40 h; A/A determinism failure voids.
Success = complete raw data + frozen analysis (fits, intervals, falsification outcomes), whatever the outcome.

## Next steps
1. Monitor pods (~13-16 h). 2. Pull raw/*.jsonl into wide_sparse_la/raw/, sha256 + S3.
3. analysis.py per frozen Analysis section -> results.json; research-visuals report/PDF/graphs.
4. benchrec add-raw/metric/close; benchmarks/<date>-wide-sparse-la/ + README row; progress report; PR.
