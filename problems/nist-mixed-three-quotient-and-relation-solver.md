# P-192 mixed-3 quotient and projected relation solver

- **Status:** open
- **Kind:** implementation gap, independent-verification gap, resource/access limit
- **Affected work:** [test report](../research/nist_cover_three_tests_20261116/README.md), [protocol](../research/nist_cover_three_tests_20261116/PROTOCOL.md), [odd-isogeny execution](odd-isogeny-quartic-transfer-execution.md), [projected relation cost](prime-cover-projected-relation-cost-missing.md)
- **Next action owner:** lane/nist-cover-three-tests
- **Last updated:** 2026-11-16

## Problem

The original P-192 speedup test remains open: all eight kernels now have an executable coset-image quotient/dual transfer, but no nonhyperelliptic target model or useful restricted-base P-192 algorithm has been constructed. A small full-base index-calculus control now completes scalar recovery on public F109/F1009 fixtures. Paid performance runs additionally require restored, live-confirmed budget headroom. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** native development replays, actual kernel witnesses, coset-image quotient/dual checks and OpenSSL elliptic endpoints are in the report. The two small controls recover all11/109 subgroup targets; their explicit bases contain all nonzero subgroup points and their quotient working fields are extensions6/9. These are producer assertions awaiting signed PCC verification. [pcc:unverified]
- **Derived or proved:** the coset implementation executes TF=[6] and inverse6 source-point recovery. For transported source bases, projection scales the old norm by3; this does not test a new target-geometry base. No formal or signed independent proof is asserted. [pcc:unverified]
- **Suspected explanation:** unknown whether the candidate quotients improve the projected relation equations. Kernel existence and point-field degree do not decide it.
- **Missing capability:** native theta/Weil-set plane-quartic reconstruction and geometric descent, full target divisor/dual-coordinate arithmetic, a useful restricted-base P-192 solver, signed independent replay and complete admitted costs. The finite-image quotient/dual and small full-base recovery controls are implemented; do not list them as missing.
- **Alternatives inspected:** existing Tian controls replay published maps but do not construct this quotient; the existing prime-cover collector lacks the proposed mixed-3 relation mechanism; Milio/Tian primary papers describe quotient methods but the inspected local native APIs do not implement this input path. The public Magma route previously reached its request limit; no new public-service campaign was started. No universal implementation-absence claim follows.

## What would unblock it

- [x] Execute the coset-image quotient/dual and source-point recovery for all eight P-192 kernels, plus additional public small-curve IC recoveries. This is component coverage, not completion of the original advantage claim. [pcc:unverified]
- [ ] Construct one quotient from an emitted actual kernel, identify smooth nonhyperelliptic Jacobian/product/hyperelliptic target and descent field; verify actual kernel and dual composition, including exceptional inputs.
- [ ] Implement an explicit projected relation solver and independent checking on original elliptic points, merging duplicate projected base images before rank accounting.
- [ ] Freeze public inputs, standard-panel controls and complete per-phase costs before a paid experiment; obtain current budget/capacity, source-image pin and S3 checkpoint admission.
- [ ] Run and independently replay the complete one-target comparison; retain null/failure outcomes.

## Next discriminating step

Compute the theta/Weil-set quotient for kernel1 with the emitted six-generator witness and coordinate basis. Verify smoothness/descent and its actual kernel before running a destination solver. Use a deliberately non-isotropic kernel, published Tian map, extension-only target and corrupted maps as controls. First construction envelope is2CPU/8GiB/1800s, zero retries and totalUSD5 only after admission; larger solver panels need a separate frozen budget. A construction failure is bounded to this method/limit. A successful construction enables the map/solver stage and does not itself settle the speedup. [pcc:unverified]

## Resolution and history

Unresolved for the original P-192 advantage. 2026-11-16: actual kernels, source maps and finite coset-image quotient/dual implemented; full small-curve principal-function IC recovery added. Nonhyperelliptic target geometry, scalable restricted base and full measurements remain open. Native correctness replays are distinguished from paid scientific jobs.
