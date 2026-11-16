# Native follow-up: square classes, geometric charts, and repeated diagnostics

Read the [round report](reports/REPORT.md) or [inspected PDF](reports/REPORT.pdf).

The universal extra-step assertion is still open. This follow-up makes a
symbolic step toward it, derives a geometric chart-availability bound, adds
native projective-divisor recovery, and records repeated native stage runs.
It does not complete the native elliptic-to-Jacobian transfer or qualify
an end-to-end performance result. All historical Sage evidence is unchanged.

2026-10-26 continuation: compute is online, but the mandatory S3 checkpoint
gate fails and no compatible isolated-runner connection is configured. The
[cold-comparison requirements](../native_end_to_end/REQUIREMENTS.md) and
[observed access record](../native_end_to_end/ACCESS_STATUS.md) describe the
remaining native-map and execution prerequisites. No new benchmark has run.

2026-10-28 continuation: the supplied AWS identity now passes the S3 checkpoint
gate, with verified readback; all five pending historical artifacts are uploaded
and hydrated. The [AWS access receipt](../native_end_to_end/access/20261028-aws.json)
records a denial of the lab's required CodeBuild status action. Native transfer
construction/inversion and the qualified full comparison remain unfinished.

| Requested requirement | Status | Evidence and remaining gap |
| --- | --- | --- |
| Prove or disprove one extra step after preparation | Partial | Explicit rational three-torsion lift with exact polynomial checks; 27 prepared native replays. Level-nine neighbor labels and obstruction divisors remain uncomputed. |
| Complete exceptional-input handling | Partial | Geometric availability argument for p>=19 under stated point-count/order hypotheses; 2,492 exhaustive effective-divisor checks. Forward-formula poles and a complete native inverter remain open. |
| Isolated, repeated end-to-end benchmarks | Blocked in full / partial diagnostics | 58 native stage/control runs, all correct, A/A counters equal. PSI missing, incomplete CPU reservation, no native map construction/inversion. |
| Independent checks | Partial | Existing Rust checker replays C++ polynomial certificates; FLINT replays all source answers. Same host/session; no independent-host or formal geometric proof. |

## Mathematical results and exact scope

* [Explicit rational torsion action](THETA_ACTION.md): M^3=I and the
  translation formula are checked as universal integer polynomial identities
  after denominators are cleared. They are not fitted from numerical signs.
  Six identity records in results are checked by the existing Rust checker,
  each with 64 additional checker-chosen points and a false-identity control.
  These are not Lean proof artifacts.
* [Geometric chart argument](GEOMETRIC_CHART_BOUND.md): for a smooth plane
  quartic with N1=p+1, N2=p^2+1 and prime Jacobian order r satisfying the
  stated Weil bound, at most 5*p^2+7*p+6 classes fail to have a unique,
  reduced affine degree-three representative. For p>=19 there is a common
  nonzero multiplier for S,T,S+T, assuming all three are nonzero. The proof
  does not establish that the present forward formulas and inverter succeed
  everywhere in that geometric domain.
* [Square-class obligation](SQUARE_CLASS_OBLIGATION.md): a compatible
  relation among labelled obstruction functions would settle a stratum.
  The conditional neighbor-product observation has only two failing centers
  behind it; it is not a universal identity or a new novelty claim.

## Executed native evidence

* 26 saved prepared component curves plus the p=157 bad prepared curve
  reproduce their usable-kernel counts. Two unprepared source fixtures are
  retained separately: 29 curves total, 232 nonzero torsion actions checked.
* All rational effective degree-three divisors on the declared smooth
  projective quartics were encoded and checked: 64 at p=5, 568 at p=7,
  and 1,860 at p=11. Repeated support, infinity, closed points and rejection
  of nonrational support are included. These are divisors, not an exhaustive
  classification of Jacobian classes or transfer inputs.
* Five seeds for each of source rho, source negation rho, target IC,
  target rho and target negation rho at p=109 and p=157: 50 runs. Eight
  additional A/A runs complete 58 correct answers. The source control uses
  direct adjugate inversion in F_(p^3), rather than slow exponentiation.
* Target inputs are synthetic points on the fixed historical quartics with
  the same public known answer. They are not computed images of the original
  source points. Every recovered scalar also passes a separate FLINT source
  check; this does not certify an unimplemented forward map.

![Native solver counts](results/native-counts.svg)

The operation graph uses only the 50 main runs and shows medians/min/max
of base-field multiplication counts. Five-seed median M counts for target
IC are 16,001,836 (p=109) and 3,160,099 (p=157); the corresponding source-rho
medians are 280,521 and 366,579. Inversions are recorded separately; these
counts are not calibrated GAE or a complete native-cost price.

No wall time is qualified. Strict preflight refused the host because CPU
and memory PSI are unavailable; the diagnostic attempt also records system
threads that could not be moved. The final diagnostic run held the exclusive
lock, pinned workers to CPU 3, moved 16 movable threads, and recorded visible
other-process ticks, context switches, faults and peak RSS. No visible-tick
contention was detected, but this does not erase the two qualification gaps.

## Review and reproduce

* [Protocol](PROTOCOL.md), [frozen run manifest](RUN_MANIFEST.json), and
  [build/fixture hashes](results/benchmark-input-sha256.txt).
* [Native benchmark summary](results/benchmark-summary.json),
  [raw receipts](results/pinned-diagnostic/reservation.json), and
  [FLINT source replay](results/source-replay.json).
* [Polynomial checks](results/theta-symbolic-audit.json),
  [theta certificate replay](results/theta-certificate-replay.json),
  [other certificate replay](results/certificate-replay.json), and
  [normalization compatibility](results/normalization-family-audit.json).
* [Development failures and open obstructions](results/development-obstructions.json).

Native dependencies: Rust, a C++17 compiler, OpenSSL, jq, FLINT/GMP and the
pinned sources in vendor/. Host tools build with `bash build_native.sh`;
FLINT tools with `bash build_flint.sh` in the pinned SageMath 10.6 image.
No Sage or Python research process is invoked. The image digest is
`sha256:19995db6194f4a4bab18ce9a88556fd15b9ed5e916b4504fefe618a7796ddbdb`.

Run the generated quartic test binary with CUBIC_FIXTURE pointing to either
fixture. Recompute proof records with build/proof_audit and
build/theta_identity. The standalone certificate-checker Cargo package
replays them with `cargo run --release --locked --manifest-path
certificate-checker/Cargo.toml -- results/*.identity.json`.
Regenerate the manifest with `jq -n -f run_manifest.jq` before a fresh run;
use a new output directory. Strict isolation is the default. Diagnostic mode
does not satisfy the strict measurement requirement.

The earlier root `cargo test --release --lib` attempt was blocked by the original
partial import's absent Cargo.toml. The 2026-10-28 continuation uses a proper
sparse clone. Its root release build stopped at compiler allocation failure
under both 4-GiB and 12-GiB virtual ceilings; library tests did not execute.
The [validation record](../native_end_to_end/access/20261028-local-validation.json)
retains all attempts. The earlier standalone tests are distinct.
The five original S3 uploads and fresh-cache byte checks
were completed on 2026-10-28. Conductor, Lean/Lake and the remaining native
measurement/release obligations stay open. Benchmark/leaderboard admission
is not requested.
