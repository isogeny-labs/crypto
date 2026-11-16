# Sharp genus floor for the selected mask-167 subgroup

16 November 2026. For the exact order-`r=185302017921931` subgroup of the mask-167 negative source over `F_(3^35)`, **any nonzero transfer into the rational points of an abelian variety over `k=F_(3^7)` needs dimension at least five**. In particular, no Jacobian of a curve of genus zero through four over `k` can receive this subgroup. The explicit transfer to the genus-five carrier in [the native GHS replay](../ghs_isogenous2_native_20261116/RESULT.md) attains the bound. This is an unconditional size obstruction for the selected subgroup, independent of the cover shape or isogeny class. [pcc:unverified]

## Proof

Let `A/k` have dimension `g`, and let the image of the prime-order source group under a homomorphism into `A(k)` be nonzero. Its image has order `r`, so `r` divides `#A(k)` and `r≤#A(k)`. The [Weil bound for abelian varieties](https://hal.science/hal-00662352v2/file/Jacobians.pdf) gives `#A(k)≤(sqrt(q)+1)^(2g)`, where `q=3⁷=2187`. Since `2187<47²`, we have `sqrt(q)+1<48`. For `g≤4`,

`#A(k)≤48^(2g)≤48⁸=28,179,280,429,056<185,302,017,921,931=r`,

a contradiction. [GenusFloorArithmetic.lean](GenusFloorArithmetic.lean) checks every integer inequality with `lean` and no admitted assertions. The standard Weil bound and the group-homomorphism step are mathematical inputs to that finite arithmetic file; they are not formalized in Lean.

The previously checked degree-seven cover, two-isogeny bridge, and five-term trace give a nonzero order-`r` image in `J(Y⁻)(k)` for the exact mask-167 public point. The [independent native replay audit](../ghs_isogenous2_native_20261116/audit_native.json) binds the source points and trace image to the completed solve. As `Y⁻` has genus five, the dimension bound is **sharp for this target**.

The same inequality does not classify all subgroups of all curves in the trace-26 class. A smaller prime or different base field can have a lower dimension floor. It is a target-size criterion, not a proof of attack speed; the separate same-target GHS and rho receipts supply cost evidence.
