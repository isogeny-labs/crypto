#set page(margin: 1.6cm)
#set text(size: 10pt)
= K-233 derived row of the admissible-degree inequality (L8)
*Class: derived calculation; no S, no measured cost.* Calculation:
`examples/k233_admissible.rs`; formulas as in `dreg_score slope`
(derived table). Curve sect233k1, r = 0x8000...bdf (log2 r = 231), cofactor 4,
log2 T_rho = 110.894.

*Formulas.* R = 2^l/(2n), A = max(1, m! 2^(n-ml)), B = T_rho/(R A); admissible D is
the largest D in 2..40 with w log2 C(N, <=D) <= log2 B, for N = ml + (m-2)n (chained)
and N = ml, w in {1, 2.2}.

*Stable bases (computed).* ord_233(2) = 29: nine 2-cyclotomic cosets, one of size 1
and eight of size 29. Stable dimensions 29a + c, a in 0..8, c in 0..1.

#table(columns: 9, align: right, inset: 3pt,
[m],[l],[ml],[log2 B],[D chained w=1],[D chained w=2.2],[D elim w=1],[D elim w=2.2],[D=4 fits],
[3],[58],[174],[0.2],[none],[none],[none],[none],[no],[3],[59],[177],[2.2],[none],[none],[none],[none],[no],[3],[87],[261],[32.8],[4],[none],[4],[none],[yes],[3],[88],[264],[31.8],[4],[none],[4],[none],[yes],[3],[116],[348],[3.8],[none],[none],[none],[none],[no],[4],[58],[232],[56.2],[7],[2],[9],[3],[yes],[4],[59],[236],[59.2],[7],[3],[10],[3],[yes],[4],[87],[348],[32.8],[3],[none],[4],[none],[yes],[5],[29],[145],[-4.1],[none],[none],[none],[none],[no],[5],[30],[150],[-0.1],[none],[none],[none],[none],[no],[5],[58],[290],[61.8],[7],[3],[10],[3],[yes],[5],[59],[295],[60.8],[7],[3],[10],[3],[yes],[6],[29],[174],[22.3],[2],[none],[3],[none],[no],[6],[30],[180],[27.3],[2],[none],[4],[none],[yes],[6],[58],[348],[61.8],[7],[2],[9],[3],[yes])

#figure(image("admissible_vs_ell.svg", width: 100%),
caption: [Admissible D against stable l for each m (derived, from rows.jsonl). Dashed: D = 4. Hollow marker at 1: none.])

*Conclusion (derived).* First D >= 4 by ml: chained w=1 at m=4, l=58 (ml=232, D=7);
N=ml, w=1 at m=6, l=30 (ml=180, D=4); w=2.2: none (maximum 3). Measured closure degree
is 4 at ml <= 18 and above 4 at ml = 21; every row here has ml >= 145, so the closure degree
there is unmeasured and D = 4 fitting is not sufficient (needs closure <= 7 to 10 at w=1,
<= 3 at w=2.2). The inequality counts the relation phase only. Distance to rho remains
a distance, not a measurement.
