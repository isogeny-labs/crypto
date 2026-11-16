# P-192 mixed-3 quotient and projected relation solver

- **Status:** open
- **Kind:** implementation gap, independent-verification gap, resource/access limit
- **Affected work:** [test report](../research/nist_cover_three_tests_20261116/README.md), [protocol](../research/nist_cover_three_tests_20261116/PROTOCOL.md), [odd-isogeny execution](odd-isogeny-quartic-transfer-execution.md), [projected relation cost](prime-cover-projected-relation-cost-missing.md)
- **Next action owner:** lane/nist-cover-three-tests
- **Last updated:** 2026-11-16

## Problem

The original P-192 speedup test remains open. All eight kernels have coset-image quotient/dual transfer; actual kernel1 quotient moduli now yield a smooth plane-quartic candidate explicitly descended to Fp. The polarized Jacobian identification, possible Serre twist, divisor-class isogeny and useful restricted-base P-192 solver remain open. Small full-base controls recover scalars on public F109/F1009 fixtures. Paid performance runs additionally require restored, live-confirmed budget headroom. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** native development replays, actual kernel witnesses, coset-image quotient/dual checks and OpenSSL elliptic endpoints are in the report. The two small controls recover all11/109 subgroup targets; their explicit bases contain all nonzero subgroup points and their quotient working fields are extensions6/9. These are producer assertions awaiting signed PCC verification. [pcc:unverified]
- **Derived or proved:** the coset implementation executes TF=[6] and inverse6 source-point recovery. For transported source bases, projection scales the old norm by3; this does not test a new target-geometry base. No formal or signed independent proof is asserted. [pcc:unverified]
- **Suspected explanation:** unknown whether the candidate quotients improve the projected relation equations. Kernel existence and point-field degree do not decide it.
- **Constructed geometry:** native normalized Weil functions, quotient level2 moduli, Riemann candidate and explicit curve descent for actual kernel1; seven/28 bitangent checks, Frobenius/Hilbert90 and geometric smoothness are retained in the [geometric report](../research/nist_cover_three_tests_20261116/GEOMETRY.md). A fixed234-point destination base supplies five principal relations, with no source-image binding. PCC-20261116-901321 and PCC-20261116-d2bdb3 are unsigned. [pcc:unverified]
- **Missing capability:** polarized quotient/Jacobian identification including its Serre twist, generic isogeny-image conversion and divisor-class source map/dual, a useful restricted-base P-192 solver, signed independent replay and complete admitted costs. Candidate curve reconstruction/descent, rational-support target Jacobian arithmetic, finite-image quotient/dual and small full-base recoveries are implemented; do not list them as missing. The native target backend passes addition against a direct effective divisor, cancellation and a principal relation; PCC-20261116-94a153 is unsigned. [pcc:unverified]
- **Alternatives inspected:** existing Tian controls replay published maps but do not construct this quotient; the existing prime-cover collector lacks the proposed mixed-3 relation mechanism; Milio/Tian primary papers describe quotient methods but the inspected local native APIs do not implement this input path. The public Magma route previously reached its request limit; no new public-service campaign was started. No universal implementation-absence claim follows.

## What would unblock it

- [x] Execute the coset-image quotient/dual and source-point recovery for all eight P-192 kernels, plus additional public small-curve IC recoveries. This is component coverage, not completion of the original advantage claim. [pcc:unverified]
- [x] Reconstruct a smooth nonhyperelliptic candidate from actual kernel1 quotient moduli and descend its curve equation to Fp. This is candidate-curve geometry, with no same-field polarized Jacobian identification asserted. [pcc:unverified]
- [ ] Settle the possible Serre twist and construct/check an actual divisor-class isogeny and dual, including exceptional inputs.
- [ ] Implement an explicit projected relation solver and independent checking on original elliptic points, merging duplicate projected base images before rank accounting.
- [ ] Freeze public inputs, standard-panel controls and complete per-phase costs before a paid experiment; obtain current budget/capacity, source-image pin and S3 checkpoint admission.
- [ ] Run and independently replay the complete one-target comparison; retain null/failure outcomes.

## Next discriminating step

Extend the implemented rational-support target arithmetic to generic image divisors and construct the Kummer-to-divisor lift for the emitted Fp quartic. Decide the possible Serre twist and check an actual public source-point map, kernel and homomorphism identities before a destination DLP claim. Preserve the exact kernel/moduli/descent witnesses. The original proposed experiment envelope remains2CPU/8GiB/1800s, zero retries and totalUSD5 after admission; larger solver panels need a frozen budget. Native development correctness checks do not constitute the paid cost evaluation. [pcc:unverified]

## Resolution and history

Unresolved for the original P-192 advantage. 2026-11-16: actual kernels, source maps, coset-image quotient/dual and small-curve recovery implemented; later actual kernel1 moduli, smooth candidate quartic, Fp curve descent and destination principal relations added. Target Jacobian/twist/map identification, scalable restricted-base solver and full measurements remain open. Native correctness replays are distinguished from paid scientific jobs.
