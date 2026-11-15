#set page(margin: 2cm)
#set text(size: 10pt)
= Normalised points and Lanczos dot reuse in the plane-quartic IC

_Study ic\_gw\_nrm\_lz\_20261115. All numbers \[pcc:unverified\], L0 local run (EKS budget hold), counted cost only._

*Hypotheses.* NRM: normalised factor-base and residual points (first coordinate 1) skip the products by that coordinate in relation finding. LZ\_REUSE: scalar Lanczos (arms D, DL1) takes $v^T v_("prev")$ and $w_i^T y$ from the local and global A-orthogonality identities. Candidate = BLR1 plus the two env-gated changes; REF = BLR1 (sha256 e236855d7fef...); 5 arms x 30 cells (k = 18..22, seeds 1-6) = 150 units, c#sub[D] = 1.7.

*Gates.* G0 (A0 = REF) pass; G1 (only the allowed fields change) pass; G2 (all ok, exit 0) pass; G3 (all s.\* statistics equal) pass.

#image("nrmlz.png", width: 100%)

*Decision (frozen rule: ratio < 1 at all 30 cells and w_add in 0, 0.5, 1).* BOTH: DL improvement (max 0.9872), DL1 improvement (max 0.9443), D improvement (max 0.9788); also below 1 for every w#sub[h]. NRM alone: all three arms improve. LZ alone: only DL1 and D (DL is unchanged, ratio exactly 1).

*BOTH/REF, DL cold M-eq*, geomean (max) over seeds 1-6, $w_h = 0$:
#table(columns: 4, align: left,
[q (k)], [w_add = 0], [w_add = 0.5], [w_add = 1],
[262111 (18)], [0.9614 (0.9624)], [0.9795 (0.9799)], [0.9869 (0.9872)],
[524287 (19)], [0.9577 (0.9585)], [0.9786 (0.9791)], [0.9866 (0.9869)],
[1048507 (20)], [0.9585 (0.9588)], [0.9790 (0.9792)], [0.9868 (0.9869)],
[2097091 (21)], [0.9589 (0.9590)], [0.9794 (0.9796)], [0.9871 (0.9872)],
[4194271 (22)], [0.9569 (0.9570)], [0.9791 (0.9793)], [0.9871 (0.9872)],
)

*BOTH/REF, DL1 cold M-eq*, geomean (max) over seeds 1-6, $w_h = 0$:
#table(columns: 4, align: left,
[q (k)], [w_add = 0], [w_add = 0.5], [w_add = 1],
[262111 (18)], [0.8503 (0.8563)], [0.9166 (0.9186)], [0.9390 (0.9410)],
[524287 (19)], [0.8353 (0.8407)], [0.9148 (0.9175)], [0.9391 (0.9411)],
[1048507 (20)], [0.8434 (0.8452)], [0.9182 (0.9189)], [0.9413 (0.9418)],
[2097091 (21)], [0.8487 (0.8493)], [0.9210 (0.9221)], [0.9433 (0.9443)],
[4194271 (22)], [0.8382 (0.8386)], [0.9200 (0.9209)], [0.9435 (0.9443)],
)

*BOTH/REF, D cold M-eq*, geomean (max) over seeds 1-6, $w_h = 0$:
#table(columns: 4, align: left,
[q (k)], [w_add = 0], [w_add = 0.5], [w_add = 1],
[262111 (18)], [0.9713 (0.9728)], [0.9742 (0.9755)], [0.9762 (0.9774)],
[524287 (19)], [0.9716 (0.9723)], [0.9746 (0.9753)], [0.9767 (0.9773)],
[1048507 (20)], [0.9725 (0.9726)], [0.9753 (0.9754)], [0.9773 (0.9774)],
[2097091 (21)], [0.9733 (0.9739)], [0.9761 (0.9766)], [0.9780 (0.9785)],
[4194271 (22)], [0.9737 (0.9741)], [0.9765 (0.9769)], [0.9784 (0.9788)],
)

NRM only, DL geomean at w_add = 0 / 1: 0.9569 / 0.9871 at k = 22 (relations.mul ratio 0.8838); LZ only, DL1 at w_add = 0 / 1: 0.9074 / 0.9601 at k = 22 (DL1 linear\_algebra.lmul ratio 0.7623).

*Gap to the matched negation-map rho at k = 22* (frozen band; REF reproduces its IC numbers to $<10^(-9)$: yes): rho/IC (DL) at w = 0, 0.1, 1 GAE goes from 449.7, 484.1, 586.8 (REF) to 470.0, 502.1, 595.3 (BOTH). Distance remaining IC/rho: DL 0.00213, 0.00199, 0.00168; DL1 0.00116, 0.00125, 0.00143. Not validated: no replay certificate, validator report or new matched-rho run, so no leaderboard row moves.
