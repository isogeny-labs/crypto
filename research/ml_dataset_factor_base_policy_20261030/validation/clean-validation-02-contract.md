# Clean package validation 02

Prospective SOFTWARE validation of the isolated restore-policy lint repair.
Snapshot10 manifest SHA-256:
`912405bf9cfadea7e39781fa2bb6e05c00bacd139acd29b0992fc9828db02b60`.
Base commit `1d858fb9e4b8035b38f93db4423880627b09211f`; sole overlay
`e3852ec4bdf5b256a7a0360b085fc0ffdb02dc8ec9dcc3f7248b0c8b5bd28cbd`.
Path list and modes exactly match package01, and every other file hash matches.

Admission requires a successful package02 receipt, verified complete Git bundle,
and fresh S3 archive readback. Record its actual clean root commit and receipt
hash before invoking the existing guarded build helper. Do not relabel package01
or its retained executables. Source files are on local /var/tmp scratch; logs,
receipts, binary copies and archived bundles remain in durable workspace/S3.

Sequential commands, stop on first failure:

1. Existing `run-clean-build.sh`: locked release build of crypto and the native
   fixture controller, actual package02 commit, clean source guards and retained
   executable hashes.
2. Existing `run-clean-validation.sh` clippy phase: maintained targets and
   unchanged strict policy. Require exit0; no allow-list or target weakening.
3. The same helper's fixture phase:31 passed,0 failed,0 ignored.
4. The same helper's dataset phase:479 passed,0 failed,2 ignored. Preserve the
   known FIFO child summary separately, using the corrected result qualification.
5. Existing four-command CLI surface check against package02's retained binaries.

Each Cargo command retains the prior1800-second timeout plus30-second grace,
Rust1.98,8 Cargo jobs,8 Rayon threads and one test thread. No concurrent own
Cargo. No automatic retries. All source and retained-binary checks must pass
before and after. Record this source's actual results; prior successes are
context, not substitute receipts.

Keep package01's clippy101 and dataset wrapper1 with its qualified Cargo479-pass
result. Preserve all seven unbound native plans already prepared; defer final
protocol-bound plan invocations until their source/binary/protocol identities
are ready. This repair neither runs the26-cell acceptance workflow nor changes
its native runtime-preflight, official schema, V2 API, full-library or export
requirements. No cloud generation is authorized by this contract.
