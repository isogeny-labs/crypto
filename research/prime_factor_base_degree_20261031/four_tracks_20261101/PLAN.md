# Four complete selector presentations

Request: pursue all four ideas proposed after PR3108: (1) invertible XOR
recoding, (2) bounded-weight labels, (3) symmetric chord coordinates,
(4) determinantal membership. Preserve complete rational relations, actual
canonical highest forms and all variables/branches. Runtime improvement and
P-256/17-term gates remain separate obligations.

Frozen mathematical controls: existing F17 curve icv1-fp5-tm1-a77cc6af,
FB1h29cc024e755c (eight columns/sixteen signed points); graph4x²+9x+9,
offsets0,2,10,12. Compare exact same signed point inventory. The old two-bit
presentation has canonical7, dense9. These are not affine F4 degrees.

1. XOR: t=2u+12v-4uv; Boolean map(u,v)->(u XOR v,v) is invertible.
   No added variables; target canonical degree at most6, with lower witness.
2. At-most-one-of-three bits, weights2,10,12, plus all pair products zero.
   Count all six slot bits and constraints; target canonical degree5.
3. Symmetric chord: sigma=m²-r, pi=r²+r*m²+A+2ms, D=sigma²-4pi.
   Replace x,z with(sigma±d)/2 and keep d²=D. Reduce the two incidence
   equations to linear equations in d. Cover either nonzero pivot and the
   zero/zero rational-square chart; no discarded branch or free saturation.
4. A3x3 affine pencil with fixed identity block realizes quadratic graph
   membership. Prove normalized-kernel and Schur equivalence, count ordinate
   and witness variables. Check the expanded highest ideal for free
   directions before claiming any finite regularity or bilinear speedup.

Primary review: PKM2016 prime-field paper; Faugere/Safey El Din/Spaenlehauer
2011 bihomogeneous paper (genericity/zero-dimensionality obligations); pinned
Mathlib coefficient/ideal/matrix sources; repository24-design register,
prior pencil proof and prior matched F4 study. No external novelty claim.

Native proof/correctness development is not a timing/yield experiment.
Freeze a separate EKS protocol before empirical comparisons; no prior local
exception carries over. No current arbitrary-command EKS client or AWS
identity; GitHub workflow presently builds specific ecbench executables.
Implement the requested sources and keep the actual routing gap explicit.
