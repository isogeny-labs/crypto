# Manual accounting before code or measured results

Read exact15B FpPoly and CubicField sources before assigning these oracles.
All arithmetic expressions remain unchanged by the planned instrumentation.
The model counts source operators, not their internal limb cost. [pcc:unverified]

Each constant/coefficient normalization invokes one BigUint remainder, including
zero coefficients. Addition invokes one addition/remainder per output coefficient.
Subtraction uses lhs+p−rhs then remainder: one addition, subtraction and remainder
per coefficient. Negation of each nonzero coefficient uses one subtraction and
remainder. For x=2+3X,y=4+X over7 these give the declared calibration table.
[pcc:unverified]

Their four nonzero polynomial coefficient products each invoke one multiplication
and remainder, followed by one accumulation addition and remainder. Thus x*y
has [4,4,0,8,0,0] and polynomial[1,0,3]. Scalar multiplication first reduces the
scalar once, then invokes two multiplication/remainder pairs. Two-step Horner
evaluation invokes two multiplications/additions/remainders, including the first
zero accumulator. [pcc:unverified]

fp_inv first reduces the argument. Zero returns immediately. For nonzero a it
subtracts2 from p, then invokes the actual library modpow once; all internal
Montgomery/limb work is included in that atomic I term, without an invented
expansion. The inversion-input histogram records the reduced argument.
[pcc:unverified]

For (2+3X).divrem(1), the leading inverse contributes [0,0,1,1,1,0]. There are
two long-division steps. Each coefficient computation and its one nonzero
subproduct contribute two multiplications/three remainders including quotient
accumulation; quotient accumulation adds once. The successive r.sub calls have
length2 and1, together [0,3,3,3,0,0]. Totals are [4,5,4,10,1,0]. The inverse
argument is1 once. CubicField.c(203) at101 separately performs203%101 in u64
then its BigUint constant normalization, yielding [0,0,0,1,0,1].
[pcc:unverified]

The17 sites cover all explicit BigUint arithmetic in this backend and the one
u64 remainder used inside CubicField.inv's constant check. A scope around every
Field.inv records complete inverse-component budgets, including the old norm
wrappers. Whole-map backend budgets omit direct Fp2/u64 work elsewhere and are
not complete map costs. Allocation, comparison, cloning, bookkeeping, compiler
lowering and CPU time are not assigned empirical prices. [pcc:unverified]

Aggregate vector comparison treats I uniformly; expanded comparison retains
one coordinate for every actual inversion argument. Both are descriptive
component comparisons without a runtime ranking. No cost improvement is an
acceptance criterion. All prior15 mathematical output rows stay fixed.
[pcc:unverified]
