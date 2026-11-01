# First measured extended N83 holdout: calibration-gated attempt

The first measured 16-point, five-arm current-source finalist holdout
[run 39253164581](https://github.com/aburan28/crypto/actions/runs/39253164581)
finished with one admitted stratum and one noisy stratum. The frozen
[protocol at its exact execution head](https://github.com/aburan28/crypto/blob/448a07d5e6554592425af1d089be527986de5737/research/n83_final_extended_20261018/PROTOCOL.md)
requires every candidate-point calibration pair to stay within 10% drift
between before and after medians. Point 14's K1024/root11 pair measured
**10.757247%** drift. The 16-point selection is therefore
`FROZEN_CALIBRATION_GATE_FAIL`; both online and complete-cold selections
remain null. This is a measurement-admission failure, not an arithmetic
counterexample or a zero-yield observation.

| Stratum | Points | IC arms replayed by provider verifier | Same-point rho replays | Calibrations passing | Result |
| --- | --- | ---: | ---: | ---: | --- |
| 0 | 0–7 | 40/40 | 40/40 | 40/40 | `STRATUM_ADMITTED` |
| 1 | 8–15 | 40/40 | 40/40 | 39/40 | `BOUNDED_UNKNOWN` |

The failed K1024 calibration's three before times were 516.340, 522.342
and 506.684 ms; its three after times were 457.308, 472.403 and
463.631 ms. The provider verifier reported passing base identity,
execution order, scalar/relation replay, resource and telemetry checks for
both strata. Its stratum-1 exit code 2 enforces the calibration gate. The
[log-and-artifact audit](receipt.json) independently parses both GitHub job
logs, requires the exact failure, validates both provider ZIPs (2,320 safe,
unique, CRC-clean members each), and checks that each archived analysis,
archived verifier log and GitHub job log agree. This audit binds and checks
the provider's replay output; it does not reexecute all raw arms locally.

The exact ZIP bytes are retained under the requested S3 research prefix:

| Stratum | Provider artifact | ZIP SHA-256 | S3 object |
| --- | ---: | --- | --- |
| 0 | `12196822837` | `fa2252377835ec0db62a773001770ad01998a9b64fc2b1924c8e04e45bb49e20` | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_final_extended_attempt_20261019/a1/provider-artifacts/fa2252377835ec0db62a773001770ad01998a9b64fc2b1924c8e04e45bb49e20.zip` |
| 1 | `12194257314` | `56bf5856e9b86d03a5bafec50f62fb33fe35116347cbf24a68216f2ec3a8c8e4` | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_final_extended_attempt_20261019/a1/provider-artifacts/56bf5856e9b86d03a5bafec50f62fb33fe35116347cbf24a68216f2ec3a8c8e4.zip` |

Fresh S3 downloads matched both provider digests. The original GitHub
artifacts expire on 2026-11-02; S3 custody preserves this failed attempt
for later full raw replay. The five candidate bases and both public-point
strata are now exposed. A new selection attempt must be separately frozen
on fresh points, retain this failed attempt, and set its host and calibration
rules before seeing new timings. The admitted eight-point subset can inform
engineering, but cannot replace the 16-point selector or establish the
minimum total index-calculus runtime. The [upstream six-arm selection](https://github.com/aburan28/crypto/blob/0021730ef93baae79522b848877879e6284d2911/research/n83_final_extended_20261018/upstream_selection.json) had already
selected K1107/root11 for cold time within its six declared candidates
(mean 94.211 s over 32 fresh points), while its online selector remained
`BOUNDED_UNKNOWN`. This failed extension does not compare K992 or K1024
against that cold selection under an admitted fresh-point rule.

[Validation receipts](validation/README.md) retain release, example and Python
test logs and repeat S3 checks for these custody scripts.
