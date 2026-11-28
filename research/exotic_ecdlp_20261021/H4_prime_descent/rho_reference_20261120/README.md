# H4 n=23 same-target Sage rho reference — 2026-11-20 UTC

Installed Sage's 20-partition Pollard rho recovered the scalar
**14,984** for the same public point `(62002,638929)` in all five frozen
walks. The five online intervals, including point replay, were 38.059,
17.633, 41.522, 27.904, and 22.069 ms. Their median is **27.904 ms**;
the original three-bucket Floyd control in the [complete candidate
run](../full_pipeline_20261120/README.md) took 63.006 ms on this point.

| Frozen walk seed | Status | Verified scalar | Online wall time |
| ---: | --- | ---: | ---: |
| 92024 | verified | 14,984 | 38.059 ms |
| 92025 | verified | 14,984 | 17.633 ms |
| 92026 | verified | 14,984 | 41.522 ms |
| 92027 | verified | 14,984 | 27.904 ms |
| 92028 | verified | 14,984 | 22.069 ms |

[`protocol.json`](protocol.json) pins the candidate, workload, and
original-run hashes, the installed Sage function source hash, the wrapper
hash, five seeds, same target, and a 30-second per-run cap. It was
committed in `e318fbdd906` before the measurements. The runner
constructs the curve and point outside timing, resets Sage's RNG for
each walk, and calls `sage.groups.generic.discrete_log_rho` with the
prime subgroup order, additive operation, and a deterministic coordinate
hash. Sage's function uses 20 random-increment partitions and a
four-entry collision table at this order. Every elapsed interval starts
before its target-dependent walk setup and ends after checking
`[s]G=Q` and the recorded fixture scalar. All five statuses, CPU times,
and peak RSS observations are in [`result.json`](result.json).

The independent [audit](verify.py) reconstructs the curve and public
point from the immutable candidate and workload records, checks the
runner and Sage source hashes, and replays every returned scalar as a
group multiple. Its [receipt](audit.json) confirms five verified runs,
the 17.633–41.522 ms range, and 27.904 ms median. The checked Sage
runtime is in [`sage-runtime-info.json`](sage-runtime-info.json).

This is a reference-quality correction to the local exploratory
control: the earlier Floyd interval was slower than each of these five
recorded Sage intervals. The runs reuse one public point on one ordinary
macOS host, so they do not satisfy the five **distinct** fresh-target
pairs, alternating order, tuned implementation, calibration, and Tier 1
resource receipts required for a speedup report. The prior IC online
interval was 1.227 ms, and its cold preparation was 1,019.717 ms;
neither timing is rerun or changed here. A Tier 1 panel should use a
preselected rho reference at least as capable as this one and retain
every paired outcome.

Reproduce from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/rho_reference_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/rho_reference_20261120/run.py
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/rho_reference_20261120/verify.py
```
