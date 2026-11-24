# Identity-image tangent splitting diagnostic

Separate static successor on the unchanged frozen points and model. Do not
modify prior protocols/results. No new curve point or parameter is generated.
H-D:the tangent at the unique recorded point with elliptic image O has a
quadratic residual that splits into two distinct rational points over Fp.

Find that point using the recorded image list,not a selected outcome. Form
its exact gradient tangent,restrict the quartic using the same fixed affine
parameterization as the canonical audit,and divide out the known double
root. Reconstruct the full restriction from that root and the residual.
If degree or multiplicity differs,retain the exceptional outcome and do not
replace the point. For a genuine quadratic compute discriminant and its
Euler criterion by GMP modular exponentiation and independently OpenSSL
BN_mod_exp. Nonzero square supports H-D;nonresidue rejects it;zero is a
repeated-contact exception. Do not extract roots or generate a new pool.

Geometric motivation,conditional on existing map/principality qualifications:
if tangent section=2P+D and f(P)=O,then the aggregate image of D is kappa,
the same canonical trace as the reference. Its difference from the reference
is a principal divisor. A rational split D would therefore expose a two-point
residual relation collector. It does not by itself give extra closed rank:
the two residual images might be new independent columns. Nonrational D may
still be a rational closed-point divisor;do not label its support rational.

Controls:require exact quartic equation,nonzero gradient,identity image,
exact double division and full product reconstruction;reject changed
constant on the restriction and a changed zero-root witness. Independent
modular exponentiation must agree. Retain every exceptional category.

120seconds/1GiB worker,300seconds/1GiB strict native build. Fresh write-once
source/input/binary snapshots and exit records. One deterministic diagnostic,
not a sampling/performance campaign. Prior sources:PROTOCOL.md,BRMap degree6
interface and RELATION_PIPELINE.md;standard tangent intersection and norm
functoriality. KB unavailable as recorded previously. [pcc:unverified]
