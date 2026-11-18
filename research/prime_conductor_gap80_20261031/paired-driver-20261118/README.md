# Transported endpoint input preparation

PR [#4721](https://github.com/isogeny-labs/crypto/pull/4721), stacked on
[#4703](https://github.com/isogeny-labs/crypto/pull/4703). This component
prepares actual transported targets for the existing ecbench harness.
It contains no new timer or solver. The original [measurement
protocol](../MEASUREMENT_PROTOCOL.md) and 80-bit crossing criterion remain open.

## Implemented behavior

[`paired_ecbench_export.cpp`](../native/paired_ecbench_export.cpp) reads
`p a b ell Kx Ky r Gx Gy`. It checks primes, ordinary source, nonsingularity,
canonical points, exact prime kernel/subgroup orders, complete kernel-pair
coverage, quotient order, nonzero transported generator and each equality
phi(sG)=s phi(G). Kernel and infinity images must be infinity. A subgroup with
r=ell is accepted only if its actual transported generator survives; no
coprime-degree assumption is silently added.

The default freezes 32 distinct shared-law scalars. An explicitly marked
development option allows fewer targets. The exporter writes `prepared.json`,
a prospective `spec.json`, and `scalar-workload.json` into a fresh directory
reserved after the arithmetic checks. Existing paths are refused. A write
failure may leave a partial directory; discard it rather than treating it as
a valid packet. No clock or solver is called by the exporter.

The coordinate control uses u=2: (x,y) maps to (4x,8y), and (a,b) to
(16a,64b). This is a field coordinate isomorphism. It is separate from the
47-bit pilot's quadratic-twist normalization.

## Observed correctness and planning controls

| Public field p | Degree ell | Targets | Planner rows, three models | PARI recoveries per endpoint |
|---|---:|---:|---:|---:|
| 6666679 | 37 | 32 distinct | 96 | 32 |
| 223 | 5 | 8, development | 24 | 8 |
| 229 | 5 | 8, development | 24 | 8 |

Direct PARI quotient and map comparisons cover 48 target images, three
generator images and six exceptional inputs. PARI recovers all 48 supplied
known scalars on each endpoint. The consumer agrees on 144 exact scalar and
coordinate rows. These are finite public correctness controls with no
measured runtimes or certified conductor gaps. [pcc:unverified]

The frozen 202-bit target and 96-bit pilot reject before construction because
the current `PrimeExplicit` consumer requires `5 <= p < 2^62`. Other negative
controls cover a killed generator, off-curve generator, zero target count,
invalid seed, existing directory, tampered plan row and tampered point.
Initial GP catalog attempt A has a syntax error despite exit 0; its source,
stderr and pre-call hash are retained. Attempt v2 has empty stderr. Neither
catalog is a conductor search or crossing result. [pcc:unverified]

The official v0.1.6221 consumer is pinned to release commit
`a2be660914e8e485eefd45f2958eb6d18344dadf`, with binary SHA-256
`e93a0d2f0f109836b4aa52573f92a7df84477ff76bcf2672b4d00e8dba994e95`.
Its archive matches the published SHA256SUMS. It was used only for `plan`;
it is not a build of the current source or a measured session. Raw plan bytes
retain full u64 algorithm seeds; do not round them through JSON doubles.
[pcc:unverified]

## Requirement status

| Original requirement | Status and exact remaining work |
|---|---|
| At least 80-bit certified prime endomorphism-conductor gap | Open: frozen source/kernel evidence exists; full quotient, images and endpoint argument required |
| Explicit ordinary-curve crossing | Implemented small-input map path; actual 80-bit construction still needs feasible full schedule |
| Dozens of points and scalars | 32 distinct transported targets prepared in one small case; large case open |
| Several fields, repeated scalar timings | Three small correctness fields; no repeated timing session |
| Same scalars on both endpoints | Planner scalar/coordinate equality checked; endpoint order and algorithm-seed pairing need harness integration |
| Scalar multiplication costs | Concrete cartesian input matrix emitted; canonical measurement integration pending |
| Rho, BSGS and index calculus | Existing arms appear in the accepted prospective spec; no arm executed here |
| Five rounds, warmup, L2 and A/A | Retained in spec; admission/execution and actual control observations pending |
| Strongest wall reference | Documented rho.prime_fast absent from pinned consumer; integration pending |
| 202-bit field support | Current PrimeExplicit unsupported; no truncation or smaller-field substitution |
| S3 / EKS execution | No new job; source/image identity and live budget/reservations/capacity needed |
| World-record comparison | Not established by this component |

`high_fidelity_admitted` is false. Alternate arm order alone does not alternate
endpoint curve order. Matching scalar values alone does not establish
algorithm-seed pairing or equal external construction/conversion costs.
Planner-generated curve slugs are unregistered, and are not promoted as
registry identities. No benchmark/leaderboard result is added.

## Replay and evidence

Extract `runs.tar.xz` here to restore `results/`. Its deterministic checksum is
in `runs.tar.xz.sha256`. Named PCC proof subjects also remain loose. Use a
fresh output path for every replay. The historical frozen scripts and pre-call
manifests preserve the exact executed version; do not overwrite their outputs.

Build with the parent's FLINT/PARI/GMP dependencies and OpenSSL:

```sh
prefix=/path/to/native-prefix
c++ -O2 -Wall -Wextra -std=c++17 \
  -isystem "$prefix/include" -isystem "$prefix/include/x86_64-linux-gnu" \
  ../native/paired_ecbench_export.cpp /path/to/optimized.o \
  -L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" \
  -lpari -lflint -lgmpxx -lgmp -lcrypto -o /tmp/exporter
```

From repository root, the original preparation command is the separately
copied `qualifier/qualify-frozen.sh EXPORTER PINNED_ECBENCH CHECK_PLAN FRESH_OUT`.
Compile `check_plan.cpp` with GMP. Compile `check_transport.cpp` with the
parent backend object and the same PARI/FLINT/GMP flags, without OpenSSL.
`qualifier/transport-frozen.sh CHECK_TRANSPORT FRESH_OUT` checks the archived
prepared cases, so it requires the raw packet extracted first. Process
deadlines are ceilings, not elapsed-time measurements.

PCC-20261118-07cc48 and PCC-20261118-83a506 bind producer evidence. Both have
verdict `not_covered` and standing `unverified`; they are not independent
execution certificates. Native build stderr is empty and scoped qualifiers
exit 0. Cargo/conductor tools are unavailable in this executor.
[pcc:unverified]

## Sources checked

KB unavailable (repository wrapper exit 127); no novelty claim is made.
Primary implementation contracts read at parent commit 979ab9100a36:

- [ecbench workload and exact shared-scalar law](../../../src/cryptanalysis/ecbench/workload.rs), especially PrimeExplicit construction and `SHARED_TARGET_LAW`.
- [ecbench spec](../../../src/cryptanalysis/ecbench/spec.rs), [canonical codec](../../../src/cryptanalysis/ecbench/canonical.rs), [harness guide](../../../docs/ecbench/README.md).
- [PARI ellisogeny, ellisogenyapply, elllog documentation](https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html).
- [Official pinned release](https://github.com/isogeny-labs/crypto/releases/tag/v0.1.6221); consumer metadata and hashes in `consumer/`.
- Parent [complete evaluator](../native/complete_pipeline.cpp) and [projective control evidence](../batch-walk-20261118/README.md).

The source-linked diagram and report are in [reports/report.pdf](reports/report.pdf).
