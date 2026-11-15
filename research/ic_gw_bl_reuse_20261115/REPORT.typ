#set page(margin: 2cm)
#set text(size: 10pt)
= Exact block-Lanczos gram reuse in the plane-quartic IC
_Study ic\_gw\_bl\_reuse\_20261115. All numbers \[pcc:unverified\], L0 local run (EKS budget hold), counted cost only._

*Hypothesis.* Two exact identities (M-orthogonality gives $(M V_(i-1))^T (M V_i)$ from this iteration's full gram; the three-term
recurrence gives $V_i^T V_0$) remove two $n times k$ products per Lanczos iteration at identical counted work elsewhere
(c#sub[D] = 1.7, k = 18..22, seeds 1-6, REF = fasthash COMBO, 60 units, protocol frozen at `5c9dc91c48`).

*Gates.* G1 pass (only DL `linear_algebra.lmul`/`ladd` differ, in 30 of 30 cells); G2 pass (`bl_iters`, `bl_narrowed` equal);
G3 pass (all records ok, exit 0); G4 pass (REF reproduces the committed COMBO records of ic\_gw\_fasthash, seeds 1-3).

#image("blreuse.png", width: 100%)

*Decision (frozen rule).* *Supported*: DL cold M-eq BLR/REF below 1 at all 30 cells for `w_add` in 0, 0.5, 1 and `w_h` in 0, 0.1, 1.

#table(columns: 4, align: left,
[q (k)], [w_add = 0], [w_add = 0.5], [w_add = 1],
[262111 (18)], [0.8182 (max 0.8221)], [0.8634 (0.8654)], [0.8833 (0.8858)],
[524287 (19)], [0.8104 (0.8136)], [0.8621 (0.8650)], [0.8836 (0.8863)],
[1048507 (20)], [0.8152 (0.8163)], [0.8656 (0.8663)], [0.8864 (0.8870)],
[2097091 (21)], [0.8189 (0.8193)], [0.8688 (0.8700)], [0.8892 (0.8905)],
[4194271 (22)], [0.8126 (0.8128)], [0.8677 (0.8687)], [0.8894 (0.8905)],
)
DL ratio, geomean (max) over seeds 1-6, $w_h = 0$. DL1 and D ratios are exactly 1. DL linear-algebra lmul ratio 0.729..0.738.

*Gap to the matched negation-map rho at k = 22* (frozen band; REF reproduces its IC numbers to $<10^(-9)$): rho/IC at w = 0, 0.1, 1 GAE
goes from 365.5, 406.2, 545.1 (REF) to 449.8, 484.1, 586.8 (BLR). The distance remaining is IC/rho = 0.00222, 0.00207, 0.00170 for BLR.
Not validated: no replay certificate, validator report or new matched-rho run, so no leaderboard row moves.
