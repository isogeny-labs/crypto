# Reproduce the prepared comparison

No production cohort execution has occurred. See PROTOCOL.md and REPORT.md
for the current access dependency and the distinction between synthetic
development checks and real-curve results.

Use C++17, PARI 2.17.3, GMP 6.3.0 and OpenSSL from the recorded native prefix.
The independent verifier uses its own GMP affine group implementation and
Tonelli–Shanks square roots. Both programs share OpenSSL for SHA functions;
that joint is not claimed as independently implemented hashing.

The development-only script builds three programs, generates six tiny
prime-field examples repeated into a synthetic 21-row parser fixture, and
checks the complete output and nine refusal cases. It never opens the real
cohort inputs. Use a fresh output, for example:

```sh
GB_NATIVE_PREFIX=/workspace/tools/sage_skeleton_env \
GB_BUILD=/workspace/new-generator-controls sh build-controls.sh
```

Real inputs/cohort.tsv and published-points.tsv retain all 21 rows. The eleven
unavailable historical generators remain NA. supplied-points.tsv preserves
the four supplied canonical points exactly. The 21 inherited feature files
are copied from the final factor study's full84-replay-016/features members,
with every member digest checked in evidence/existing-feature-custody.json.
They are available in that already published study's final-native-003 archive;
do not substitute its earlier1100- or1118-feature snapshots. Native code refuses
an incomplete or differing catalog. Preserve the historical-b convention of
these reused features when examining the changed-b point view.

The local review bundle includes input copies, tested source/binaries and all
retained development attempts. Its local custody record must not be presented
as an uploaded or independently replayed preparation. Rebuilding changes binary
identity and needs fresh source/binary admission before cohort use.

The production launcher run-prepared.sh requires the bound input.sha256,
binaries.sha256, a successful S3 checkpoint gate and an actual preparation
delivery/member-verification receipt. That receipt intentionally does not
exist yet. Freeze the supported execution route and actual source identity
before launch; the local launcher is not authorization to bypass default EKS
routing or its build contract. Once admitted it applies a 300-second watchdog
and 768 MiB address ceiling to each native phase, retains failure/status files
and performs no automatic retry. No wall/CPU/RSS clock is admitted.

```sh
GB_ANALYZER=/path/to/admitted/analyze \
GB_VERIFIER=/path/to/admitted/verify \
GB_OUT=/path/to/fresh/run sh run-prepared.sh
```

Finish raw archive, independent checks, unified export, report/PDF, remote
readback and committed bindings after real execution. The prepared source and
the synthetic test results do not fulfill that measurement requirement or
identify a historical skip criterion.
