# Complete F5control before native calibration

Lean proves that the complete reduced-polynomial-pair model for
y²=x⁵−x over F5has16elements, all16with split u and every v=0.
The curve polynomial is monic, degree5and separable. Identity is included.
This is the fixed all-smooth control for the native calibration, complementing
the complete F3model's8pairs/7smooth. [pcc:unverified]

`ReducedPair` ranges over every actual polynomial pair with u monic,
degree u≤2, degree v<degree u and u dividing v²−f. The proof classifies
all651coefficient shapes, proves the actual monic-division test equivalent
to scalar coefficient equations, and constructs a complete bijection.
It does not assume a selected list is complete. [pcc:unverified]

Every valid quadratic u has a root in F5, by a kernel finite certificate.
For a root r, the proof establishes the actual factorization
u=(X−r)(X+a+r). Thus the splitting count uses `Polynomial.Splits`,
not a substituted scalar predicate. The scalar v-zero classification is
also transferred back to the complete polynomial model. [pcc:unverified]

The method adapts the frozen F3coefficient-shape proof but checks
characteristic-five conditions directly. Structural signature injectivity
avoids an expensive pair enumeration. A recursion bound4096permits finite
kernel reduction; it changes no foundation or trust rule. Failed first
build and passing second build remain in evidence. [pcc:unverified]

| Obligation | Declaration in MumfordFiveControlAudit |
|:--|:--|
| Complete actual coefficient shape | `canonical_valid_shape` |
| Actual division equals scalar condition | `valid_iff_coefficientValid` |
| Actual complete count16 | `reducedPair_cardinality` |
| Actual splitting of every u | `valid_quadratic_has_root`, `quadratic_splits_of_root`, `every_pair_smooth` |
| Smooth count16and everyv0 | `smooth_reducedPair_cardinality`, `every_pair_v_zero` |
| Monic, degree5and separable | `curve_monic`, `curve_degree`, `curve_separable` |
| Combined binding | `mumford_five_control_certificate` |

The current whole tracked project gate passes240declarations,0failures
and0trusted-native exceptions, including generated declarations. Only
propext, Classical.choice and Quot.sound are accepted. Lean4.35.0 and
Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
See [gate](evidence/five-control-gate.log),
[source](lean/MumfordFiveControl.lean) and
[source/identity plan](five-control-sources.json). [pcc:unverified]

Source commit11100a8512bac3840bf1f85b1e7f0f523cd78464 precedes packet
PCC-20261109-e81718 and supplement IDC1ha387b9e22b14b9cf.
The full local packet check passes at21:56:51Z on producerhost762f5fc120c9. Signed standing
remains unverified. The supplement accepts32derived/11extra seeded points
for the supplied-root quadratic factorization only:3variables/degree2over
2⁶¹−1. It does not establish the F5counts, completeness, splitting, curve
separability or geometry; those finite statements rest on Lean.
[pcc:unverified]

Exact source/packet/native bindings and final report digest are in
[five-control-verification.json](five-control-verification.json).
Both pages of the [report PDF](reports/proof-20261109-five-control/report.pdf)
were visually inspected before publication. [pcc:unverified]

Formal geometric Picard/Jacobian identification and a Lean F5Cantor group
proof remain separate. This finite control does not refute Gaudry's
fixed-genus limit or a qualified large-prime/typical-subgroup heuristic.
The original fourteen-item audit remains partial. [pcc:unverified]

The native driver and separate C++arithmetic checker are built, not yet run
on curves. Calibration protocol f2572b4dd547db0c3177611d575be15fe3fd1abf2b2ab134db618c3fcd2913a6
was frozen at21:52:34Zand must be committed before either calibration call.
It names the two known fixtures, positive/negative detector checks, exact
identities, metadata-only seeds,120sper-call cap, all failure rules and
zero-paid-compute local backend. See [native source and plan](native-census-v1/README.md).
[pcc:unverified]
