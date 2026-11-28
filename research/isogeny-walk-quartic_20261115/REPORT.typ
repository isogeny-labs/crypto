#set page(margin: 1.5cm)
#set text(size: 9pt)
= Walking genus-3 hyperelliptic Jacobians to a plane quartic (Smith)
_Status: \[pcc:unverified\] local development check at small p. No improvement is claimed. Plan: PLAN.md (frozen at 3c24520f, amendments A1, A2, A2b)._

== Verdict
After the skeptic's review the route now exists end to end at toy scale. The canonical map of Recillas' curve X is the trace of the holomorphic differentials of H along the correspondence, $h_i = [x^2](x^i D b^(-1) mod G)$. It puts X on a unique smooth plane quartic in 58 of 60 instances (2 of 60 have a conic image, X hyperelliptic). Push-forward of point pairs into $"Pic"^0(C)$ sends principal divisors to one class (1538 pairs, 57 of 57 instances; the negative control separates). BSGS on the quartic gives logs that pull back to H and verify there, up to the kernel of order dividing 8 (141 of 160 verified, all 19 others differ by 8-torsion). The route costs about 10.6 times the analytic matched rho at every price (BSGS on a dense quartic, not Diem IC).

#figure(image("graph.svg", width: 100%), caption: [A: one-step Smith success. B: exhaustive existence ceilings at p = 7. C: plane-curve kernel dimension, old projection (grey) against the canonical map (orange). D: priced cost per DLP, quartic BSGS (measured) against rho (analytic), with S/rho above the bars.])

== Measured (all \[pcc:unverified\])
#table(columns: 3, stroke: 0.4pt,
  [*cell*], [*measured*], [*outcome*],
  [P1 one-step, p = 101 / 211 / 521], [0.194 [0.186, 0.203] / 0.186 [0.178, 0.194] / 0.184 [0.177, 0.193]], [confirmed],
  [P4 projection (b0:b1:b2)], [degree 10 model], [refuted for that projection only],
  [P6 canonical map, p = 37, 101], [unique quartic 58 of 60; conic 2 of 60], [confirmed 29 of 30 per p],
  [P7 principal divisors], [1538 pairs, one value in 57 of 57; control: several values 57 of 57], [confirmed; 254 pairs untestable (library panic)],
  [P8 DLP pullback, p = 101 / 211 / 521], [50 / 60 / 50 logs on C; 44 / 54 / 43 verified; failures 6 / 6 / 7, all in 8-torsion], [confirmed up to the kernel],
  [P10 S/rho at w = 0, 0.1, 1], [10.5 / 10.5 / 10.5 (p = 101), 11.0 (211), 10.6 (521), ranges 6.2 to 15.2], [S/rho > 1 at all prices],
  [P9 exponent, 3 points], [1.37 (BSGS on C) against 1.45 (rho)], [near 1.5, no interval],
)

== Distance to matched rho
The rho reference is analytic: $sqrt(pi N / 4)$ group operations times the measured Cantor field operations per add on H (about 600). It is not a measured run. The quartic counter omits additions that the Cantor tally includes, so the ratio is if anything understated. Native unpriced work (hash probes, Phi in $F_(p^2)$ estimated at 800 ops) is priced at 0, 0.1 and 1 per operation and does not move the ratio.

== Verifier disagreement
The matched rho is analytic, not measured, and the arm is BSGS, not Diem IC; the walk for W > 1 is still not built. The skeptic's minor P4 point is accepted.

== Limitations
Small p, toy BSGS, partly estimated native work, monic degree-7 Smith-positive curves only (the 19 percent search is not charged), uniqueness and smoothness at d = 4 as the evidence for the canonical model, 254 library panics in test pairs, no replay or validator.

== Next step
Run Diem line relations with sparse block Wiedemann on the quartic that `build_quartic` produces, keep Phi as the transfer, and measure rho on the same Jacobian with the negation map.

== Repro
`cargo build --release -j2 --bin smith_walk_dlp && bash research/isogeny-walk-quartic_20261115/repro.sh`
