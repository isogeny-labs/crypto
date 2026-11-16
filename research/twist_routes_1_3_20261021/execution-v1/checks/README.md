# Validation scope and preserved failures

The final root release library suite passed 4,479 tests, with zero failures and
119 existing ignored tests, at source freeze cb7e324. This includes the added
scalar-Frobenius/zero-discriminant regression. The earlier 4,478-test pass at
4471be410e1f4152eb8767e9e99930edb76c5eab is retained separately. The revised
isogeny runner and its full supporting suite also pass (128 tests). E15 independently point-checks every answer and
its native audit reproduces all 1,704 verified records.

The earlier root-suite attempt had missing sparse runtime fixtures and restricted
local-socket tests. Materializing the tracked fixtures and granting the test
process network access resolved them. The next attempt passed 4,477 tests and
failed only the roster assertion fixed at 1,324 extension models. The current
registry contains 1,358: the original cohort plus 33 execution extensions and
one pairing-control extension. The expected count was updated without removing
any per-model field or subgroup checks, and the full suite passed again.

Raw setup failures and earlier test logs remain here. The final outcomes above,
not an earlier compile/test failure, define current validation. Historical
binary hashes identify the executables used at their time; those binaries are
not bundled and a newer build may replace the same local target path.

CAS_BINARIES.sha256 and CAS_PACKAGES.sha256 identify the locally extracted
native tool packages. These scratch paths are provenance, not required remote
paths. PARI raw member hashes and the solver split archives are verified beside
their respective outputs. Full-cost calibration and independent-host replay
remain open. No missing formal bridge is treated as a proved theorem.

The native catalog cover suite passes 11 tests and the updated standalone
constructor suite passes 13, including every one of the 2,637 current models.
The Node catalog assessment verifier additionally replays the declared-field
identities and reports zero problems. The native browser projection preserves
all historical yields and existing values exactly; its initial one-ULP parse
failure and the float-roundtrip correction are recorded. The first trait
rebuild's factor(0) panic was preserved and its remaining workers terminated
before rebuilding with the corrected native code. Backlog V2 used invalid
path arguments; V3 validates all three card IDs. These metadata/setup failures
do not alter the sealed experiment records.
