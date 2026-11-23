# First A: a stable composite kernel without a rational first step

First A source61f7b10c passed format, strict release compilation, Clippy and
one native process under2GiB/120s/one worker. Full small raw, all135 span
bitsets/F images, all64 K points/orders/F images, complete orthogonal and
doubling lists, versions and pre/post source/runtime/readonly-ELF guards
are retained. Native stdout and whole compiled-output hashes are in
ACTUAL_FIRST_A_RESULT.json. Large whole ELF/packet custody remains private
and pending; this is not a signed PCC verdict. [pcc:unverified]

Observed: |K|=64, |K[2]|=16, |2K|=4, K=K^perp, and no nonzero fixed vector
in ambient two-torsion. Of all135 Lagrangians, none is F-stable. Exactly
three compatible first kernels lie in K[2] and contain2K; F cycles them.
Replacing M by identity gives pairing2 on points1088,272; restoration passes.
Identity F retains all135 first kernels. No old genus-two population was
replayed. An independent canonical-RREF verifier is still a separate task.
[pcc:unverified]

## Written realization bridge, not an evaluated divisor map

All25 affine pairs of E/F5:y^2=x^3+2x+1 were checked; #E=7, trace=-1 and
nonzero discriminant. Standard elliptic Frobenius theory gives
pi^2+pi+[5]=0. Modulo4 this becomes F^2+F+I=0; modulo2 it is irreducible.
For any primitive v in E[4], reductions of v,pi(v) form a basis of E[2],
so v,pi(v) form a Z/4 basis by Nakayama. The resulting Frobenius matrix is
[[0,3],[1,3]]. This establishes a written module identification; no geometric
torsion coordinates were computed. [pcc:unverified]

The actual endomorphism [1]+[2]pi restricts to M on E[4]. Its mod4 determinant
is3, so its graph is anti-isometric for the alternating Weil pairing.
Thus E1[2] direct-sum graph(M) is a rational maximal4-isotropic subgroup of
the product E^3. Standard polarized quotient theory yields a rational
4-polarized degree64 isogeny with this kernel. The three compatible first
kernels have Frobenius orbit3, hence require F125; each geometric quotient
has degree8, with a second degree8 quotient completing the composite.
These geometric statements use standard torsion/quotient facts and are not
yet a shared Lean theorem or an implemented morphism. [pcc:unverified]

## Why this does not qualify a Jacobian or ECDLP candidate

The source is the product PPAV E^3. The kernel itself splits across
E1 and E2xE3, so the polarized target is the product
(E1/E1[2]) x ((E2xE3)/graph(M)), hence decomposable too. No smooth genus-three
Jacobian with its canonical principal polarization is produced at either
endpoint. Even an indecomposable genus-three quotient would need an actual
base-field curve/isogeny descent check; a rational PPAV alone is insufficient.
[pcc:unverified]

Any degree64 isogeny is injective on odd-prime point subgroups. On the known
E1 factor the quotient identifies with multiplication by2, so this example
adds no nontrivial ECDLP solver route there. No actual divisor images,
transfer/solve/recovery timing, relation matrix, setup/LA/memory pricing or
matched rho/BSGS/IC gain was measured. The result marks the dimension boundary
of the genus-two layer argument and provides a controlled positive composite
example, with the transfer-admissibility failure preserved. [pcc:unverified]

Sources read: Smith arXiv0806.2995v2 §9 for rational PPAV/Jacobian descent
scope; Magma Richelot handbook text1615 and Bruin--Doerksen0902.3480 for
polarized-kernel context. The finite computations and source are local to
this new model. No novelty clearance is claimed. [pcc:unverified]
