# Frozen continuation: two languages on a standard hosted runner

This is a prospective infrastructure continuation of the incomplete
independent S3 check. The earlier manual dispatch returned HTTP404 and ran
no job; its protocol and export remain unchanged. This continuation uses
a new research workflow triggered only by adding its file on `main`,
after normal PR and merge gates. No trusted workflow is repurposed.
No hosted execution is asserted before actual job output. [pcc:unverified]

Two fresh native calls are planned in one standard public Ubuntu24.04
job: original C++ arithmetic, then the separate Rust implementation.
Both cover all nonsingular short models over 5,7,11,13,17,19 and every
ordered base-field x-coordinate triple, with actual signed point lifts
in the quadratic field. The known-answer reference was seen previously:
942 models, 4,188,162 triples and 244,236 zeros. Every per-field aggregate
must match, with zero false or missing zeros and all inline checks passing.
This is deterministic known-answer replication, not a blind discovery.
[pcc:unverified]

Source checkout is the already published commit
e3658b4d6ffc6839e32bad1f5f732f796a2c8e4d. All original Rust, driver,
comparator, protocol, workflow and reference hashes remain binding.
The C++ arithmetic/header files remain unchanged. A generated translation
unit copies the original C++ file prefix byte for byte, excludes its
broader main entry point, and appends a fixed S3-only entry. The original
and prefix hashes, entry hash and generated-unit hash are recorded.
The entry adds no field/group arithmetic. No historical local row is
silently presented as a new execution. [pcc:unverified]

The new canonical experiment.protocol/v1 record fixes both arms and all
component/source hashes before any hosted execution. Freeze and commit
it before opening the ready main-based infrastructure PR. The public
event/workflow/protocol identities are logged before source checkout and
native work. Compilers, binaries, OS/kernel/CPU and complete build/run/
comparison exit records are logged; source and binary identity are checked
before each native call. The first hosted job attempt is the only admitted
attempt; infrastructure or correctness failures are retained, never retried
for a favorable outcome. [pcc:unverified]

Limits: C++ compile60s/native60s; Rust compile60s/native30s; whole job5min.
No warmups, RNG, new sampling, statistical inference or valid clock metric.
The resource estimate is a conservative reservation based on prior local
native checks, not an invented hosted timing. The C++ entry receives a
syntax-only local preflight; this emits no new scientific population.
Both native arms are attempted when their instrument builds permit it;
a failed build or native call remains a failure. [pcc:unverified]

Budget is zero paid spend. The repository is public and official GitHub
billing documentation says standard public runners remain free; larger
runners are billed. Use only ubuntu24.04 standard, contents-read and no
secrets, EKS, GPU, larger runner, cache, artifact upload, recurring schedule,
PR trigger or feature-branch trigger. Public job logs are the evidence
channel. A missing/rejected job is not-completed, not independent validation.
[pcc:unverified]

Existing Lean square-lift/actual AdjoinRoot/cardinality and actual
Mathlib point/S3 equivalence proofs remain the mathematical foundation.
They do not prove the native compiler/runtime, physical hardware identity
or signed execution standing. Actual independent-host provenance requires
the real GitHub-hosted job. Binary formulas, S4/S5, generic degrees,
performance and full author independence are outside this continuation.
[pcc:unverified]
