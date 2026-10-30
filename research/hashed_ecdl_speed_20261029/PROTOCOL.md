# Native hashed-ECDL optimization protocol — 2026-10-29

User request: speed up the Certicom ECC2K-130/ECCp-131 work while preserving
correctness. Both published challenge solves remain open. This protocol is
frozen before optimization or performance runs; local builds/unit tests are
development work. The Python companion is historical reference, not a new
performance execution path in crypto.

## Baseline and proposed changes

Crypto baseline: `d188c5a6775dff07129084ab2f3f7ae39547c9fc`.
Cryptanalysis baseline: `1caa747d9bb888e0b98c739a59b17fbb1bd92e99`.
Preserve the baseline release binary/hash, compiler flags and host manifest.
Existing primitive step counters are diagnostics, not calibrated common units.

Investigate separately: (1) exact primality for bounded integers/certified
challenge parameters, retaining AKS and rejecting unproved claims; (2) avoid
recounting all solutions at each interface prefix in the built-in functional
leaf path. Preserve the lexicographically selected witness, pins, exact counts,
resource-error propagation, recursive admission guards and final graph replay.
Do not change source theorem constants, factor-base selection or RNG draws.

Sources: existing native sources and pinned theorem references; Handbook of
Applied Cryptography §4.3.2/Fact 4.40 and its chapter notes (Pocklington);
Sorenson/Webster, https://arxiv.org/abs/1509.00864 (bounded deterministic
strong-pseudoprime result). No new theorem or asymptotic result is proposed.

## Required evidence and controls

1. Profile with tools/profile.sh before tuning; retain unsupported/failed probes.
2. Preserve exact public ECCp-131 p/n/P/Q preflight, both baseline and candidate,
   with the same 2,000,000-step policy. Any added proof-generation/setup cost
   must be recorded separately and included in a cold comparison. A changed
   failure stage is not a complete solve or a speed ratio.
3. Compare count-based and optimized witnesses on all small assignments,
   identity/UNSAT, pins on free/derived/aliased signals, arbitrary caller
   predicates, empty interface, counterfeit certificates and work exhaustion.
   Counts and selected words must match exactly. Preserve all failures.
4. Full cold known-answer pipeline panel: curves (5,2,1; order 7; generator
   (0,1)), (7,0,3; order 13; generator (1,2)), and (17,2,2; order 19; generator
   (5,1)); cofactor one. For each curve use fixture seeds 1..10 and public
   target scalars 1 and r-2, giving 60 inputs. The solver receives points,
   never the target-generating scalar. Two slots, one thread, same parameters
   and explicit limits, 3 repetitions. Fresh holdout seeds 101..104 with the
   same two public target scalars add 24 inputs per repetition. Validate full
   cardinalities and every returned scalar/certificate independently. Retain
   limits/timeouts; do not discard unsuccessful baseline or candidate rows.
5. The WDSat binary-polynomial suite is not the same encoder/field/backend.
   This native prime-field panel is an equivalent-suite proposal under
   research/index_calculus_baseline_20260914/ec_index_calculus_contract.json;
   it does not establish ECC2K-130 scaling or replace either actual challenge.

## Measurement and execution

Before scientific/profile/benchmark execution, complete and push the S3 resume
checkpoint and pass scripts/checkpoint.sh gate. Default x86 execution is Iso
Lab EKS Spot. Use one bounded job, at most one c8a.2xlarge worker, 4 GiB,
1,800-second original deadline, no automatic measurement retries. Verify actual
source/binary/image identities, native L2 isolation, warmup, at least five A/A
and five interleaved A/B rounds and all S3 payloads. A profile explains costs;
its wall time is not throughput evidence. Do not launch a deployed ecbench
integration fixture in place of this native tensor workload.

Measure cold setup, parameter/proof validation, hash/base enumeration,
encoding, all relation attempts, tensor witness search, matrix/extraction and
final replay separately without double counting. Retain failed attempts and
setup for both arms. Native Meter values may be reported only as diagnostics;
unpriced common-operation totals, S/rho and unsupported hardware metrics remain
null. No end-to-end gain follows from stage work reductions or candidate-only
completions. Runtime claims require admitted matched hardware and a 95% paired
interval excluding no improvement, in addition to the parent accounting rules.

## Completion and blockers

Publish the code, regression controls, raw receipts and benchmark comparison
in paired regular PRs, preserving the original audit and solve requirements.
If checkpoint, profiler, image construction or EKS access prevents measurement,
record the exact failed command/access result and leave profiling/A-B acceptance
open. Continue feasible implementation and correctness work; do not silently
replace an unavailable benchmark with unisolated local timing. No challenge
scalar, practical subexponential improvement or general proof is presumed.
