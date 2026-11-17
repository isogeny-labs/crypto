# Complete first-chart point-source identities

Exact transform executed once at3ae4ff9195b exited0. All four inverse polynomial substitutions pass over the integers; SHIFT.tsv contains the transformed coefficients. This is stronger than sampled equality but not an independently implemented algebra verifier or an arithmetic/runtime test. [pcc:unverified]

Write X=K1, Y=K2, Z=K4, H=Y-rX, U=K3-rY+r²X and S=r5+c. Then the complete first column, in one-based notation, is

    B11 = U²
    B12 = U*(Z-r²Y-2r³X) + 2S*X²
    B13 = U*(rZ-r²U-r³Y+3r4X) + 2S*X*H
    B14 = U*(r²Z+2r³U+4r4Y-(4r5+2c)X) + 2S*H².

Expanding these expressions gives SHIFT.tsv after H=Y-rX. In particular B14's remaining square contributes2S*Y²-4rS*XY+2r²S*X². The linear factor's XU coefficient is -(4r5+2c), matching the transformed entries. Together these are a concrete arithmetic DAG candidate, not a priority claim. Fixed source powers and coefficients may be precomputed and charged once.

A straightforward generic-constant evaluation ledger, before common-subexpression improvements: H costs1 constant multiplication; U costs2; B11 costs1 square; B12 costs2 constant multiplications+1 product+1 square+1 constant multiplication; B13 costs4 constant multiplications+2 products+1 constant multiplication; B14 costs4 constant multiplications+1 product+1 square+1 constant multiplication. Thus this schedule uses23 multiplication/square operations when every multiplication by a nontrivial fixed coefficient is charged like a multiplication. Small constants2/3/4 can instead use additions. Shared r²Z/r²U etc. can reduce this schedule further. This is an explicit upper-cost schedule, not a lower bound, measured REDC count or proof no smaller circuit exists.

For first difference chart j=1, reconstruction still computes W1=B11*D1 and Wi=2*Bi1*D1-B11*Di for i=2,3,4: ten generic products before zero/unit specializations. Other difference charts require the retained generic evaluator or a separately checked circuit. Identity/theta transitions cannot be discarded. Splitting, normalization, signed recovery, setup, useful rank and final scalar verification remain additional.

When r=0 the formulas simplify to K3², K3*K4+2c*K1², 2c*K1*K2, 2c*(K2²-K1*K3). This is only an eligible source when c has a rational nonzero square root; the controls cannot all be replaced by such instances. A point with a convenient coordinate is not automatically a known-log step. Any new basis/source normalization remains charged.

Decision: retain this circuit as a concrete factorization improvement, but its present cost ledger does not justify a full collector campaign for the tenfold objective. Next work must either supply a substantially smaller all-cost schedule or increase useful independent relation yield with an explicit invariant and cost model. No actual runtime speedup, complete-solve gain, generic impossibility or mathematical breakthrough is established.
