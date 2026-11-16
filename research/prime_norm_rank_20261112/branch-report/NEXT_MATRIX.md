# Next actual matrix transfer

Unexecuted next proof obligations, not a completed corank result
[pcc:unverified].

1. Use the actual degreeLT.basisProd coordinate equivalence. Prove its two
   polynomial projections are precisely NormLift.assemble of the existing
   upperFamily/lowerFamily [pcc:unverified]. Derive the degree bounds and coordinate injectivity.
   Existing familiesComplete supplies surjectivity.
2. Expand the actual matrix bilinear expression for arbitrary w,z into
   mu(A_w A_z-f B_w B_z). Keep both vectors; a diagonal identity alone does not
   justify this in characteristic2.
3. Use the actual linear branch evaluation and a genuine basis of its kernel.
   Transfer restrictedRadicalWitnessLine through the faithful coordinates to
   identify the restricted matrix kernel as one nonzero line.
4. Use rank-nullity to prove exact corank1 and then a nonzero adjugate minor.
   Derive coefficient-form stationarity from the actual supplied norm and
   apply guarded recovery with every guard present.

Before changing code, read the pinned Basis.equivFun, equivFun_symm_apply,
Matrix/BilinearForm and rank-nullity APIs; no guessed module paths or assumed
matrix rank. Preserve failed proof logs. This is a symbolic correctness task,
not an admitted timing experiment. A public-input certificate finder and its
uniform bit complexity remain additional obligations.

## Dated continuation

NormMatrixFaithful supplies the actual projections, degree caps, coordinate
injectivity, arbitrary-pair bilinear identity and coefficient-space branch
radical transfer. See ../faithful-report/README.md and its exact bindings.
The reduced branch-basis corank/adjugate step and public-input finder remain
separate; the earlier four-step plan is retained as its original scope
[pcc:unverified].
