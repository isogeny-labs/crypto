# P-192 mixed-3 quotient and projected relation solver

- **Status:** open
- **Kind:** implementation gap, independent-verification gap, resource/access limit
- **Affected work:** [test report](../research/nist_cover_three_tests_20261116/README.md), [protocol](../research/nist_cover_three_tests_20261116/PROTOCOL.md), [odd-isogeny execution](odd-isogeny-quartic-transfer-execution.md), [projected relation cost](prime-cover-projected-relation-cost-missing.md)
- **Next action owner:** lane/nist-p192-polarized-quotient
- **Last updated:** 2026-11-16

## Problem

The original P-192 speedup test remains open. All26 actual kernel witnesses and two exact height algorithms now support the written same-field degree27/polarization3 quotient proof;see [POLARIZED_QUOTIENT.md](../research/nist_cover_three_tests_20261116/POLARIZED_QUOTIENT.md),PCC-20261116-3f440b,PCC-20261116-ce4a7b and PCC-20261116-8559fc. The source evaluator and public bridge are executed;six setup rows have rank6 on225 projected points. Generic transpose-dual execution,exceptional-chart handling and restricted individual descent remain. Paid performance runs still require restored,live-confirmed headroom. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** native development replays,actual kernel witnesses,coset-image quotient/dual checks and OpenSSL elliptic endpoints are in the report. The two small controls recover all11/109 subgroup targets; their explicit bases contain all nonzero subgroup points and their quotient working fields are extensions6/9. These are unsigned producer assertions; statement claims remain not_covered and cannot become verified by adding a signature alone. [pcc:unverified]
- **Derived or proved:** the coset implementation executes TF=[6] and inverse6 source-point recovery. For transported source bases, projection scales the old norm by3; this does not test a new target-geometry base. No formal or signed independent proof is asserted. [pcc:unverified]
- **Suspected explanation:** unknown whether the candidate quotients improve the projected relation equations. Kernel existence and point-field degree do not decide it.
- **Constructed geometry:** native normalized Weil functions, quotient level2 moduli, Riemann candidate and explicit curve descent for actual kernel1; seven/28 bitangent checks, Frobenius/Hilbert90 and geometric smoothness are retained in the [geometric report](../research/nist_cover_three_tests_20261116/GEOMETRY.md). A fixed234-point destination base supplies five principal relations, with no source-image binding. PCC-20261116-901321 and PCC-20261116-d2bdb3 are unsigned. [pcc:unverified]
- **Further checks:** normalized target functions and exact differential forms give a conditional nonsquare scalar for kernel1; the full fresh kernel2 chain agrees. The cover involution pairs(1,2),(3,7),(4,8),(5,6), and an explicit Fp point map/inverse between the first two candidate curves passes all28 bitangent and full equation checks. See the [symmetry report](../research/nist_cover_three_tests_20261116/SYMMETRY.md), PCC-20261116-08b9be, PCC-20261116-0d2e60, PCC-20261116-f14049 and PCC-20261116-f8faa8, all unsigned. The conditional normalization argument still needs independent review. [pcc:unverified]
- **Missing capability:** polarized quotient/Jacobian identification including its Serre twist, generic isogeny-image conversion and divisor-class source map/dual, a useful restricted-base P-192 solver, signed independent replay and complete admitted costs. Candidate curve reconstruction/descent, rational-support target Jacobian arithmetic, finite-image quotient/dual and small full-base recoveries are implemented; do not list them as missing. The native target backend passes addition against a direct effective divisor, cancellation and a principal relation; PCC-20261116-94a153 is unsigned. [pcc:unverified]
- **Alternatives inspected:** existing Tian controls replay published maps but do not construct this quotient; the existing prime-cover collector lacks the proposed mixed-3 relation mechanism; Milio/Tian primary papers describe quotient methods but the inspected local native APIs do not implement this input path. The public Magma route previously reached its request limit; no new public-service campaign was started. No universal implementation-absence claim follows.

## What would unblock it

- [x] Execute the coset-image quotient/dual and source-point recovery for all eight P-192 kernels, plus additional public small-curve IC recoveries. This is component coverage, not completion of the original advantage claim. [pcc:unverified]
- [x] Reconstruct a smooth nonhyperelliptic candidate from actual kernel1 quotient moduli and descend its curve equation to Fp. This is candidate-curve geometry, with no same-field polarized Jacobian identification asserted. [pcc:unverified]
- [x] Identify one same-field polarized quotient with actual kernel witnesses,exact heights,written geometric proof and a regular-chart source evaluator. Producer standing;generic transpose-dual execution and exceptional inputs remain separate. [pcc:unverified]
- [ ] Implement generic transpose-dual evaluation and complete exceptional-chart interfaces.
- [ ] Implement an explicit projected relation solver and independent checking on original elliptic points, merging duplicate projected base images before rank accounting.
- [ ] Freeze public inputs, standard-panel controls and complete per-phase costs before a paid experiment; obtain current budget/capacity, source-image pin and S3 checkpoint admission.
- [ ] Run and independently replay the complete one-target comparison; retain null/failure outcomes.

## Next discriminating step

Implement the generic transpose-dual/exceptional-chart interface and restricted individual descent,starting from the [completed quotient proof](../research/nist_cover_three_tests_20261116/POLARIZED_QUOTIENT.md). Do not repeat the successful source/public/kernel/height stages without a changed input or concern. Preserve the declared seven-variable timeout and all witnesses. The original paid envelope remains2CPU/8GiB/1800s,zero retries and totalUSD5 after admission;native correctness units do not constitute that cost evaluation. [pcc:unverified]

## Resolution and history

Unresolved for the original P-192 advantage. 2026-11-16: actual kernels, source maps, coset-image quotient/dual and small-curve recovery implemented; later actual kernel1 moduli, smooth candidate quartic, Fp curve descent and destination principal relations added. Target Jacobian/twist/map identification, scalable restricted-base solver and full measurements remain open. Native correctness replays are distinguished from paid scientific jobs.
