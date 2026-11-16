# P-192 mixed-3 quotient and projected relation solver

- **Status:** open
- **Kind:** implementation gap, independent-verification gap, resource/access limit
- **Affected work:** [test report](../research/nist_cover_three_tests_20261116/README.md), [protocol](../research/nist_cover_three_tests_20261116/PROTOCOL.md), [odd-isogeny execution](odd-isogeny-quartic-transfer-execution.md), [projected relation cost](prime-cover-projected-relation-cost-missing.md)
- **Next action owner:** lane/nist-cover-three-tests
- **Last updated:** 2026-11-16

## Problem

The comprehensive speedup test cannot yet execute because the eight actual P-192 d7 mixed kernels have no implemented quotient/dual map in this study, and the hypothesized restricted-base relation solver has no explicit algorithm. Paid EKS runs also require restored, live-confirmed budget headroom; the latest shared journal reports the monthly allowance exceeded. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** native development replays, exact input witnesses, failures and corruption controls are in the report. Six actual genus3 torsion generators realize the eight abstract kernels after common-root alignment. These are producer assertions awaiting signed PCC verification. [pcc:unverified]
- **Derived or proved:** the standard polarized-isogeny identities would give TF=[6] after construction of phi; this is conditional, not an executed quotient identity. Source pullback/norm has a separate executed fixture check.
- **Suspected explanation:** unknown whether the candidate quotients improve the projected relation equations. Kernel existence and point-field degree do not decide it.
- **Missing capability:** native theta/Weil-set quotient, field-of-definition/descent and actual-kernel check, dual evaluation, restricted-base algebraic relation solver, independent replay and complete cost panel. No missing generic genus3 arithmetic is asserted for the tested monic cover.
- **Alternatives inspected:** existing Tian controls replay published maps but do not construct this quotient; the existing prime-cover collector lacks the proposed mixed-3 relation mechanism; Milio/Tian primary papers describe quotient methods but the inspected local native APIs do not implement this input path. The public Magma route previously reached its request limit; no new public-service campaign was started. No universal implementation-absence claim follows.

## What would unblock it

- [ ] Construct one quotient from an emitted actual kernel, identify smooth nonhyperelliptic Jacobian/product/hyperelliptic target and descent field; verify actual kernel and dual composition, including exceptional inputs.
- [ ] Implement an explicit projected relation solver and independent checking on original elliptic points, merging duplicate projected base images before rank accounting.
- [ ] Freeze public inputs, standard-panel controls and complete per-phase costs before a paid experiment; obtain current budget/capacity, source-image pin and S3 checkpoint admission.
- [ ] Run and independently replay the complete one-target comparison; retain null/failure outcomes.

## Next discriminating step

Compute the theta/Weil-set quotient for kernel1 with the emitted six-generator witness and coordinate basis. Verify smoothness/descent and its actual kernel before running a destination solver. Use a deliberately non-isotropic kernel, published Tian map, extension-only target and corrupted maps as controls. First construction envelope is2CPU/8GiB/1800s, zero retries and totalUSD5 only after admission; larger solver panels need a separate frozen budget. A construction failure is bounded to this method/limit. A successful construction enables the map/solver stage and does not itself settle the speedup. [pcc:unverified]

## Resolution and history

Unresolved. 2026-11-16: actual kernel realization and source cover maps added; quotient and candidate relation algorithm remain open. No scientific job is running.
